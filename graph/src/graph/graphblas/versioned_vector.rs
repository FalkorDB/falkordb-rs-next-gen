//! MVCC-aware multi-edge entry for [`Tensor`](super::tensor::Tensor) forward
//! matrices.
//!
//! Mirrors FalkorDB C's tensor entry encoding: every entry in the UINT64
//! forward adjacency matrix is a tagged word that is either
//!
//! - a **scalar** edge id (MSB clear — edge ids are < 2^60), or
//! - a **pointer** to a [`VersionedVectorInner`] (MSB set), holding all edge
//!   ids of a multi-edge `(src, dst)` pair.
//!
//! ## MVCC model
//!
//! Matrices are shared across graph snapshots via `Cow`, and `GrB_Matrix_dup`
//! bit-copies the tagged words — so the *same* inner allocation is reachable
//! from many snapshots. Snapshot isolation is therefore provided *inside* the
//! inner: it stores a stack of `(epoch, committed, vector)` versions.
//!
//! - Each write transaction gets a fresh, never-reused **epoch** from a
//!   global counter ([`next_epoch`]; graph `version` numbers are reused
//!   after rollback, so they cannot tag shared state).
//! - A writer only mutates a vector in place when its epoch tag equals the
//!   writer's own epoch (i.e. it was created in this transaction); otherwise
//!   it clones the newest *committed* vector and appends a new version.
//! - Commit marks the transaction's versions `committed`. A reader at epoch
//!   `E` sees the newest version that is either its own (`epoch == E`) or
//!   committed with `epoch <= E`. Rolled-back versions are never marked
//!   committed, stay invisible to everyone, and are pruned lazily by the next
//!   writer that touches the same inner.
//!
//! ## Ownership
//!
//! Inners are reference counted. The count equals the number of GrB matrices
//! whose entries contain the tagged pointer word, plus the number of
//! `Tensor::touched` entries pointing at the inner. `GrB_Matrix_dup` of an
//! owning matrix retains every pointer word it copies; dropping the last
//! handle to an owning matrix walks its entries and releases them. Old
//! versions inside an inner are kept until the inner itself is freed, since
//! arbitrarily old snapshots may still read them.

use std::{
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::{NonNull, null_mut},
    sync::atomic::{AtomicU64, AtomicUsize, Ordering, fence},
};

use parking_lot::Mutex;
use thin_vec::ThinVec;

use super::{
    GrB_Info, GxB_Vector_deserialize, GxB_Vector_serialize,
    serialization::{Decode, Encode, Reader, Writer},
};
use crate::graph::graphblas::vector::{Remove, Set, Vector};

/// MSB tag: set = pointer to [`VersionedVectorInner`], clear = scalar edge id.
pub(crate) const MSB_MASK: u64 = 1u64 << 63;

/// Global write-epoch counter. Epochs are never reused, unlike graph
/// `version` numbers which are reclaimed after rollback.
static NEXT_EPOCH: AtomicU64 = AtomicU64::new(1);

/// Hand out a fresh, never-reused write epoch.
pub fn next_epoch() -> u64 {
    NEXT_EPOCH.fetch_add(1, Ordering::Relaxed)
}

/// Tagged word stored as the UINT64 value of a tensor forward matrix entry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VersionedVector {
    raw: u64,
}

impl VersionedVector {
    #[must_use]
    pub const fn new_scalar(id: u64) -> Self {
        debug_assert!(id & MSB_MASK == 0, "edge id must fit in 63 bits");
        Self { raw: id }
    }

    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(self) -> u64 {
        self.raw
    }

    #[must_use]
    pub const fn is_scalar(self) -> bool {
        self.raw & MSB_MASK == 0
    }

    /// The scalar edge id. Only valid when [`Self::is_scalar`].
    #[must_use]
    pub const fn scalar(self) -> u64 {
        debug_assert!(self.raw & MSB_MASK == 0);
        self.raw
    }

    pub(crate) fn inner(self) -> NonNull<VersionedVectorInner> {
        debug_assert!(self.raw & MSB_MASK != 0);
        NonNull::new((self.raw & !MSB_MASK) as *mut VersionedVectorInner).unwrap()
    }

    /// Allocate a new multi-edge inner containing `ids`, tagged with the
    /// writer's `epoch` (uncommitted), and return the tagged pointer word.
    /// The inner is born with a refcount of 1, owned by whoever stores the
    /// returned word.
    #[must_use]
    pub fn new_vec(
        epoch: u64,
        ids: impl IntoIterator<Item = u64>,
    ) -> Self {
        let mut v = Vector::<bool>::new(super::tensor::GrB_INDEX_MAX);
        for id in ids {
            v.set(id, true);
        }
        Self::register(VectorVersion {
            epoch,
            committed: false,
            vector: v,
        })
    }

    /// Wrap an already-populated bool vector (index = edge id) as the
    /// committed baseline (epoch 0) of a new inner. Used by the RDB decode
    /// path.
    #[must_use]
    pub fn from_committed(v: Vector<bool>) -> Self {
        Self::register(VectorVersion {
            epoch: 0,
            committed: true,
            vector: v,
        })
    }

    fn register(version: VectorVersion) -> Self {
        let mut versions = ThinVec::new();
        versions.push(version);
        let inner = Box::new(VersionedVectorInner {
            refs: AtomicUsize::new(1),
            versions: Mutex::new(versions),
        });
        let ptr = NonNull::from(Box::leak(inner));
        debug_assert!(ptr.as_ptr() as u64 & MSB_MASK == 0);
        Self {
            raw: ptr.as_ptr() as u64 | MSB_MASK,
        }
    }

    /// Increment the inner's refcount. Only valid on vector entries.
    pub(crate) fn retain(self) {
        unsafe { self.inner().as_ref() }.retain();
    }

    /// Decrement the inner's refcount, freeing the inner when it reaches
    /// zero. Only valid on vector entries.
    pub(crate) fn release(self) {
        unsafe { release_inner(self.inner()) };
    }

    /// Run `f` on the newest vector version visible at `epoch`. Only valid on
    /// vector entries.
    pub fn read_visible<R>(
        self,
        epoch: u64,
        f: impl FnOnce(&Vector<bool>) -> R,
    ) -> Option<R> {
        let inner = unsafe { self.inner().as_ref() };
        let versions = inner.versions.lock();
        VersionedVectorInner::visible_idx(&versions, epoch).map(|i| f(&versions[i].vector))
    }

    /// Edge ids visible at `epoch`, ascending. Empty for a vector entry whose
    /// versions are all invisible (cannot happen for entries reachable from a
    /// consistent snapshot).
    #[must_use]
    pub fn ids(
        self,
        epoch: u64,
    ) -> Vec<u64> {
        if self.is_scalar() {
            return vec![self.raw];
        }
        unsafe { self.inner().as_ref() }.ids(epoch)
    }

    /// Number of edge ids visible at `epoch`.
    #[must_use]
    pub fn count(
        self,
        epoch: u64,
    ) -> u64 {
        if self.is_scalar() {
            return 1;
        }
        unsafe { self.inner().as_ref() }.count(epoch)
    }

    /// Add `id` for the writer at `epoch`. Returns the inner pointer so the
    /// caller can record it for commit marking. Only valid on vector entries.
    pub fn push(
        self,
        epoch: u64,
        id: u64,
    ) -> NonNull<VersionedVectorInner> {
        let inner = self.inner();
        unsafe { inner.as_ref() }.mutate(epoch, |v| v.set(id, true));
        inner
    }

    /// Remove `id` for the writer at `epoch`; returns the remaining count at
    /// that epoch and the inner pointer for commit marking. Only valid on
    /// vector entries.
    pub fn remove(
        self,
        epoch: u64,
        id: u64,
    ) -> (u64, NonNull<VersionedVectorInner>) {
        let inner = self.inner();
        let remaining = unsafe { inner.as_ref() }.mutate(epoch, |v| v.remove(id));
        (remaining, inner)
    }

    /// Approximate heap usage of the inner (vector entries only).
    #[must_use]
    pub fn memory_usage(self) -> usize {
        if self.is_scalar() {
            return 0;
        }
        let inner = unsafe { self.inner().as_ref() };
        let versions = inner.versions.lock();
        versions.len() * size_of::<VectorVersion>()
            + versions
                .iter()
                .map(|v| v.vector.nvals() as usize * 16)
                .sum::<usize>()
    }
}

impl From<u64> for VersionedVector {
    fn from(raw: u64) -> Self {
        Self::from_raw(raw)
    }
}

/// Borrowed view of a multi-edge entry's visible id vector, encoded as a
/// `GxB_Vector_serialize` blob — the wire format FalkorDB C's tensor section
/// uses for the id vectors of multi-edge `(src, dst)` pairs.
///
/// Kept as its own type (rather than another `impl Encode<19> for
/// Vector<bool>`) so it can't collide with `Vector<bool>`'s own
/// `Encode`/`Decode` (the container/unload format used for matrix container
/// fields in `matrix.rs`) — Rust forbids two impls of the same trait for the
/// same type, and the two formats are not interchangeable on disk.
pub struct TensorEntryVectorRef<'a>(pub &'a Vector<bool>);

impl Encode<19> for TensorEntryVectorRef<'_> {
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        unsafe {
            let mut blob: *mut c_void = null_mut();
            let mut blob_size: u64 = 0;

            let info =
                GxB_Vector_serialize(&raw mut blob, &raw mut blob_size, self.0.ptr(), null_mut());
            assert_eq!(
                info,
                GrB_Info::GrB_SUCCESS,
                "GxB_Vector_serialize failed: {info:?}"
            );

            let blob_slice = std::slice::from_raw_parts(blob.cast::<u8>(), blob_size as usize);
            w.write_buffer(blob_slice);

            let layout = std::alloc::Layout::from_size_align(blob_size as usize, 8).unwrap();
            std::alloc::dealloc(blob.cast::<u8>(), layout);
        }
    }
}

/// Owned counterpart of [`TensorEntryVectorRef`]: decodes a
/// `GxB_Vector_deserialize` blob back into a `Vector<bool>` id vector for a
/// multi-edge entry.
pub struct TensorEntryVector(pub Vector<bool>);

impl Decode<19> for TensorEntryVector {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let blob = r.read_buffer()?;
        unsafe {
            let mut v: MaybeUninit<super::GrB_Vector> = MaybeUninit::uninit();
            let info = GxB_Vector_deserialize(
                v.as_mut_ptr(),
                null_mut(),
                blob.as_ptr().cast(),
                blob.len() as u64,
                null_mut(),
            );
            if info != GrB_Info::GrB_SUCCESS {
                return Err(format!("GxB_Vector_deserialize failed: {info:?}"));
            }
            Ok(Self(Vector::from(v.assume_init())))
        }
    }
}

struct VectorVersion {
    epoch: u64,
    committed: bool,
    vector: Vector<bool>,
}

/// Heap side of a multi-edge entry: a refcount and a version stack guarded
/// by a mutex. Readers only hold the lock long enough to collect ids into a
/// `Vec`.
pub struct VersionedVectorInner {
    refs: AtomicUsize,
    versions: Mutex<ThinVec<VectorVersion>>,
}

// The mutex serializes all access to the version stack; `Vector` is a plain
// owning wrapper over a `GrB_Vector` handle.
unsafe impl Send for VersionedVectorInner {}
unsafe impl Sync for VersionedVectorInner {}

impl VersionedVectorInner {
    /// Index of the newest version visible at `epoch`.
    fn visible_idx(
        versions: &[VectorVersion],
        epoch: u64,
    ) -> Option<usize> {
        versions
            .iter()
            .rposition(|v| v.epoch == epoch || (v.committed && v.epoch <= epoch))
    }

    fn ids(
        &self,
        epoch: u64,
    ) -> Vec<u64> {
        let versions = self.versions.lock();
        Self::visible_idx(&versions, epoch)
            .map(|i| versions[i].vector.iter().collect())
            .unwrap_or_default()
    }

    fn count(
        &self,
        epoch: u64,
    ) -> u64 {
        let versions = self.versions.lock();
        Self::visible_idx(&versions, epoch).map_or(0, |i| versions[i].vector.nvals())
    }

    /// Apply `f` to the writer's version at `epoch`, creating it by cloning
    /// the newest visible version if needed. Prunes rolled-back garbage
    /// (uncommitted versions from other epochs — unreachable by definition).
    /// Returns the resulting entry count.
    fn mutate(
        &self,
        epoch: u64,
        f: impl FnOnce(&mut Vector<bool>),
    ) -> u64 {
        let mut versions = self.versions.lock();
        versions.retain(|v| v.committed || v.epoch == epoch);
        if versions.last().is_none_or(|v| v.epoch != epoch) {
            let base = Self::visible_idx(&versions, epoch).map_or_else(
                || Vector::<bool>::new(super::tensor::GrB_INDEX_MAX),
                |i| versions[i].vector.dup(),
            );
            versions.push(VectorVersion {
                epoch,
                committed: false,
                vector: base,
            });
        }
        let last = versions.last_mut().unwrap();
        f(&mut last.vector);
        last.vector.nvals()
    }

    /// Mark the writer's version(s) at `epoch` committed. Called from the
    /// MVCC commit hook.
    pub fn mark_committed(
        &self,
        epoch: u64,
    ) {
        let mut versions = self.versions.lock();
        for v in versions.iter_mut() {
            if v.epoch == epoch {
                v.committed = true;
            }
        }
    }

    /// Increment the refcount.
    pub(crate) fn retain(&self) {
        self.refs.fetch_add(1, Ordering::Relaxed);
    }
}

/// Decrement the inner's refcount and free it when the count reaches zero.
///
/// # Safety
///
/// `ptr` must point to a live inner previously created by
/// [`VersionedVector::new_vec`]/[`VersionedVector::from_committed`], and the
/// caller must own one reference (which this call consumes).
pub(crate) unsafe fn release_inner(ptr: NonNull<VersionedVectorInner>) {
    if unsafe { ptr.as_ref() }.refs.fetch_sub(1, Ordering::Release) == 1 {
        fence(Ordering::Acquire);
        drop(unsafe { Box::from_raw(ptr.as_ptr()) });
    }
}
