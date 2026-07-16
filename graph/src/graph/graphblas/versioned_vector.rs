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
//!   it clones the newest *committed* vector and prepends a new version.
//! - Commit marks the transaction's versions `committed`. A reader at epoch
//!   `E` sees the newest version that is either its own (`epoch == E`) or
//!   committed with `epoch <= E`. Rolled-back versions are never marked
//!   committed, stay invisible to everyone, and remain linked (as unreachable
//!   garbage) until the inner itself is freed.
//!
//! The version list is **lock-free**: writes are serialized system-wide, so
//! the single writer prepends fully-built nodes with a release store and
//! readers traverse with acquire loads. Nodes are never unlinked or freed
//! while the inner is alive, so traversal needs no reclamation scheme.
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
    cell::UnsafeCell,
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::{NonNull, null_mut},
    sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering, fence},
};

use super::{
    GrB_Info, GxB_Vector_deserialize, GxB_Vector_serialize,
    serialization::{Decode, Encode, Reader, Writer},
};
use crate::graph::graphblas::vector::{Iter as VectorIter, Remove, Set, Vector};

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
        Self::register(epoch, false, v)
    }

    /// Wrap an already-populated bool vector (index = edge id) as the
    /// committed baseline (epoch 0) of a new inner. Used by the RDB decode
    /// path.
    #[must_use]
    pub fn from_committed(v: Vector<bool>) -> Self {
        Self::register(0, true, v)
    }

    fn register(
        epoch: u64,
        committed: bool,
        v: Vector<bool>,
    ) -> Self {
        let node = Box::into_raw(Box::new(VersionNode {
            epoch,
            committed: AtomicBool::new(committed),
            vector: UnsafeCell::new(v),
            next: null_mut(),
        }));
        let inner = Box::new(VersionedVectorInner {
            refs: AtomicUsize::new(1),
            head: AtomicPtr::new(node),
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
        inner.visible(epoch).map(|node| f(node.vector()))
    }

    /// Edge ids visible at `epoch`, ascending, as a streaming iterator.
    /// Empty for a vector entry whose versions are all invisible (cannot
    /// happen for entries reachable from a consistent snapshot).
    ///
    /// Lock-free: the visible vector is immutable while reachable, so the
    /// iterator reads it in place. As with every method here, the entry must
    /// stay alive for the iterator's whole lifetime.
    #[must_use]
    pub fn ids(
        self,
        epoch: u64,
    ) -> IdsIter {
        if self.is_scalar() {
            return IdsIter::scalar(self.raw);
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
        let mut usage = 0;
        let mut p = inner.head.load(Ordering::Acquire);
        while !p.is_null() {
            let node = unsafe { &*p };
            usage += size_of::<VersionNode>() + node.vector().nvals() as usize * 16;
            p = node.next;
        }
        usage
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

/// One version of a multi-edge id vector: a node in the inner's lock-free,
/// prepend-only version list (newest first).
struct VersionNode {
    epoch: u64,
    /// Flipped (release) by the commit hook; readers check it with acquire
    /// loads, which also makes the fully-built `vector` visible to them.
    committed: AtomicBool,
    /// Only mutated by the single writer that created the node, while it is
    /// still uncommitted — and an uncommitted node is invisible to every
    /// reader, so no reader ever dereferences a vector under mutation.
    vector: UnsafeCell<Vector<bool>>,
    /// Next-older version. Immutable after the node is published.
    next: *mut VersionNode,
}

impl VersionNode {
    /// Shared view of the id vector. Sound for any node the caller may
    /// legitimately read: committed nodes are immutable, and an uncommitted
    /// node is only visible to the writer that owns it.
    fn vector(&self) -> &Vector<bool> {
        unsafe { &*self.vector.get() }
    }
}

/// Streaming iterator over the edge ids of a tensor entry, ascending.
///
/// The multi-edge arm reads the visible version's vector in place — no lock,
/// no `Vec` materialization; the vector is immutable while reachable. See
/// [`VersionedVector::ids`] for the keep-alive contract.
pub struct IdsIter {
    scalar: Option<u64>,
    vector: Option<VectorIter<bool>>,
}

impl IdsIter {
    const fn scalar(id: u64) -> Self {
        Self {
            scalar: Some(id),
            vector: None,
        }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            scalar: None,
            vector: None,
        }
    }
}

impl Iterator for IdsIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if let Some(id) = self.scalar.take() {
            return Some(id);
        }
        self.vector.as_mut()?.next()
    }
}

/// Heap side of a multi-edge entry: a refcount and a lock-free version list
/// (newest first). The single serialized writer prepends fully-built nodes
/// with release stores; readers traverse with acquire loads. Nodes are never
/// unlinked or freed before the inner itself drops.
pub struct VersionedVectorInner {
    refs: AtomicUsize,
    head: AtomicPtr<VersionNode>,
}

// The writer only mutates its own uncommitted (reader-invisible) node, every
// other node is immutable, and nodes live as long as the inner — so shared
// references across threads are sound.
unsafe impl Send for VersionedVectorInner {}
unsafe impl Sync for VersionedVectorInner {}

impl Drop for VersionedVectorInner {
    fn drop(&mut self) {
        let mut p = *self.head.get_mut();
        while !p.is_null() {
            let node = unsafe { Box::from_raw(p) };
            p = node.next;
        }
    }
}

impl VersionedVectorInner {
    /// Newest version visible at `epoch`, walking newest → oldest.
    fn visible(
        &self,
        epoch: u64,
    ) -> Option<&VersionNode> {
        let mut p = self.head.load(Ordering::Acquire);
        while !p.is_null() {
            let node = unsafe { &*p };
            if node.epoch == epoch
                || (node.committed.load(Ordering::Acquire) && node.epoch <= epoch)
            {
                return Some(node);
            }
            p = node.next;
        }
        None
    }

    fn ids(
        &self,
        epoch: u64,
    ) -> IdsIter {
        self.visible(epoch)
            .map_or_else(IdsIter::empty, |node| IdsIter {
                scalar: None,
                vector: Some(node.vector().iter()),
            })
    }

    fn count(
        &self,
        epoch: u64,
    ) -> u64 {
        self.visible(epoch).map_or(0, |node| node.vector().nvals())
    }

    /// Apply `f` to the writer's version at `epoch`, creating it (from a
    /// clone of the newest visible version) if this is the transaction's
    /// first write to this inner. Returns the resulting entry count.
    ///
    /// Rolled-back nodes from dead epochs stay linked: they are invisible to
    /// every reader and are freed with the inner.
    fn mutate(
        &self,
        epoch: u64,
        f: impl FnOnce(&mut Vector<bool>),
    ) -> u64 {
        let head = self.head.load(Ordering::Acquire);
        if !head.is_null() && unsafe { &*head }.epoch == epoch {
            // The writer's own node — pushed by this transaction, still
            // uncommitted (a transaction never writes after its commit), so
            // no reader can observe the vector mid-mutation.
            let vector = unsafe { &mut *(*head).vector.get() };
            f(vector);
            return vector.nvals();
        }
        let mut base = self.visible(epoch).map_or_else(
            || Vector::<bool>::new(super::tensor::GrB_INDEX_MAX),
            |node| node.vector().dup(),
        );
        f(&mut base);
        let nvals = base.nvals();
        let node = Box::into_raw(Box::new(VersionNode {
            epoch,
            committed: AtomicBool::new(false),
            vector: UnsafeCell::new(base),
            next: head,
        }));
        self.head.store(node, Ordering::Release);
        nvals
    }

    /// Mark the writer's version(s) at `epoch` committed. Called from the
    /// MVCC commit hook.
    pub fn mark_committed(
        &self,
        epoch: u64,
    ) {
        let mut p = self.head.load(Ordering::Acquire);
        while !p.is_null() {
            let node = unsafe { &*p };
            if node.epoch == epoch {
                node.committed.store(true, Ordering::Release);
            }
            p = node.next;
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
