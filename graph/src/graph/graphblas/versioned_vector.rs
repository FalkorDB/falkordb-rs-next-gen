//! Multi-edge entry for [`Tensor`](super::tensor::Tensor) forward matrices.
//!
//! Mirrors FalkorDB C's tensor entry encoding: every entry in the UINT64
//! forward adjacency matrix is a tagged word that is either
//!
//! - a **scalar** edge id (MSB clear — edge ids are < 2^60), or
//! - a **pointer** to a version-chained [`VersionNode`] (MSB set), holding
//!   all edge ids of a multi-edge `(src, dst)` pair.
//!
//! ## MVCC model
//!
//! Matrices are shared across graph snapshots via `Cow`, and `GrB_Matrix_dup`
//! bit-copies the tagged words — so the *same* node may be reachable from
//! many snapshots. Snapshot isolation falls out of the matrix copy-on-write
//! itself: a node's id vector is immutable once the transaction that created
//! it is published, and each snapshot reads whichever node its own copy of
//! the word points at.
//!
//! Every node is stamped with the graph version (*epoch*) of the transaction
//! that created it, and chained (`next`) to the version it superseded, so
//! chain epochs strictly decrease. Every read walks from its own copy of the
//! word to the newest version stamped at or below the reader's epoch.
//!
//! The single serialized writer mutates a node in place only when the stamp
//! equals its own epoch — the node was created in this transaction and is
//! reachable only from its unpublished matrices. Otherwise it clones the
//! visible vector into a fresh node chained to the old head and replaces the
//! tagged word in its own COW delta; older snapshots keep reading their own
//! visible version.
//!
//! Rollback needs no bookkeeping: nodes created by a rolled-back transaction
//! are referenced only by that transaction's matrices, so dropping the write
//! graph frees them. Graph versions are reused after rollback, but a fresh
//! writer dups from the committed graph, whose words never point at
//! rolled-back nodes.
//!
//! ## Ownership
//!
//! Nodes are neither reference counted nor tracked in a registry. Instead
//! every node keeps a `next` pointer to the version of the pair's vector it
//! superseded, so the newest word of a cell reaches every older version of
//! its vector. Nodes are freed at exactly two points (see the tensor's
//! module docs): when the transaction that created them rolls back, and when
//! the tensor's whole version chain dies — superseded committed nodes stay
//! chained (readable by older snapshots) until then.

use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::{NonNull, null_mut},
};

use super::{
    GrB_Info, GxB_Vector_deserialize, GxB_Vector_serialize,
    serialization::{Decode, Encode, Reader, Writer},
};
use crate::graph::graphblas::vector::{Iter as VectorIter, Remove, Set, Vector};

/// MSB tag: set = pointer to [`VersionNode`], clear = scalar edge id.
pub(crate) const MSB_MASK: u64 = 1u64 << 63;

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

    fn node(self) -> NonNull<VersionNode> {
        debug_assert!(self.raw & MSB_MASK != 0);
        NonNull::new((self.raw & !MSB_MASK) as *mut VersionNode).unwrap()
    }

    /// Allocate a new multi-edge node containing `ids`, stamped with the
    /// writer's `epoch`, and return the tagged pointer word. The node starts
    /// a fresh chain (its cell had no vector before); it lives until its
    /// transaction rolls back or the tensor's version chain dies (see the
    /// module docs).
    #[must_use]
    pub fn new_vec(
        epoch: u64,
        ids: impl IntoIterator<Item = u64>,
    ) -> Self {
        let mut v = Vector::<bool>::new(super::tensor::GrB_INDEX_MAX);
        for id in ids {
            v.set(id, true);
        }
        Self::register(epoch, null_mut(), v)
    }

    /// Wrap an already-populated bool vector (index = edge id) as a committed
    /// node (epoch 0, below any writer epoch). Used by the RDB decode path.
    #[must_use]
    pub fn from_committed(v: Vector<bool>) -> Self {
        Self::register(0, null_mut(), v)
    }

    fn register(
        epoch: u64,
        next: *mut VersionNode,
        mut v: Vector<bool>,
    ) -> Self {
        // Materialize pending GraphBLAS work before the vector becomes
        // reachable: readers attach iterators to published vectors in place,
        // and an attach on a vector with pending tuples triggers an internal
        // GB_wait that *mutates* it — racing concurrent readers.
        v.wait();
        let node = Box::new(VersionNode {
            epoch,
            next,
            vector: UnsafeCell::new(v),
        });
        let ptr = NonNull::from(Box::leak(node));
        debug_assert_eq!(ptr.as_ptr() as u64 & MSB_MASK, 0);
        Self {
            raw: ptr.as_ptr() as u64 | MSB_MASK,
        }
    }

    /// Creation epoch of the node. Only valid on vector entries.
    #[must_use]
    pub(crate) fn epoch(self) -> u64 {
        unsafe { self.node().as_ref() }.epoch
    }

    /// The older vector version this node superseded, if any. Only valid on
    /// vector entries.
    #[must_use]
    pub(crate) fn next(self) -> Option<Self> {
        let next = unsafe { self.node().as_ref() }.next;
        (!next.is_null()).then_some(Self {
            raw: next as u64 | MSB_MASK,
        })
    }

    /// Free this node alone — its `next` chain is untouched. Only valid on
    /// vector entries, and only when the caller has proven no live graph
    /// version can reach the node (rollback of its creating transaction, or
    /// tensor chain death — see the tensor's module docs).
    pub(crate) fn free(self) {
        drop(unsafe { Box::from_raw(self.node().as_ptr()) });
    }

    /// The newest version of this entry visible at `epoch`: the first node
    /// in the chain stamped at or below it (chain epochs strictly decrease).
    /// Only valid on vector entries reachable from a consistent snapshot,
    /// which always holds a version at or below its own epoch.
    fn visible(
        self,
        epoch: u64,
    ) -> NonNull<VersionNode> {
        let mut node = self.node();
        while unsafe { node.as_ref() }.epoch > epoch {
            node = NonNull::new(unsafe { node.as_ref() }.next)
                .expect("no vector version visible at epoch");
        }
        node
    }

    /// Run `f` on the id vector visible at `epoch`. Only valid on vector
    /// entries.
    pub fn read<R>(
        self,
        epoch: u64,
        f: impl FnOnce(&Vector<bool>) -> R,
    ) -> R {
        f(unsafe { self.visible(epoch).as_ref() }.vector())
    }

    /// Edge ids of this entry visible at `epoch`, ascending, as a streaming
    /// iterator.
    ///
    /// Lock-free: a published node's vector is immutable, so the iterator
    /// reads it in place. As with every method here, the entry must stay
    /// alive for the iterator's whole lifetime.
    #[must_use]
    pub fn ids(
        self,
        epoch: u64,
    ) -> IdsIter {
        if self.is_scalar() {
            return IdsIter::scalar(self.raw);
        }
        IdsIter {
            scalar: None,
            vector: Some(unsafe { self.visible(epoch).as_ref() }.vector().iter()),
        }
    }

    /// Number of edge ids in this entry visible at `epoch`.
    #[must_use]
    pub fn count(
        self,
        epoch: u64,
    ) -> u64 {
        if self.is_scalar() {
            return 1;
        }
        unsafe { self.visible(epoch).as_ref() }.vector().nvals()
    }

    /// Add `id` for the writer at `epoch`. Mutates the node in place when the
    /// writer owns it (created it in this transaction); otherwise clones the
    /// vector into a fresh node and returns its tagged word — the caller must
    /// store it back over this entry. Only valid on vector entries.
    #[must_use]
    pub fn push(
        self,
        epoch: u64,
        id: u64,
    ) -> Self {
        self.mutate(epoch, |v| v.set(id, true)).1
    }

    /// Remove `id` for the writer at `epoch`; returns the remaining count and
    /// the (possibly fresh — see [`Self::push`]) tagged word holding the
    /// result. Only valid on vector entries.
    #[must_use]
    pub fn remove(
        self,
        epoch: u64,
        id: u64,
    ) -> (u64, Self) {
        self.mutate(epoch, |v| v.remove(id))
    }

    fn mutate(
        self,
        epoch: u64,
        f: impl FnOnce(&mut Vector<bool>),
    ) -> (u64, Self) {
        let node = unsafe { self.node().as_ref() };
        if node.epoch == epoch {
            // The writer's own node: created in this transaction, reachable
            // only from its unpublished matrices — no reader can observe the
            // vector mid-mutation.
            let vector = unsafe { &mut *node.vector.get() };
            f(vector);
            // Keep the vector materialized even mid-transaction: it is
            // published as-is on commit, and readers must never inherit
            // pending work (see `register`).
            vector.wait();
            return (vector.nvals(), self);
        }
        let mut v = unsafe { self.visible(epoch).as_ref() }.vector().dup();
        f(&mut v);
        let nvals = v.nvals();
        // Chain the fresh node to the one it supersedes: older snapshots keep
        // reading their own visible version through the chain, and the chain
        // keeps every superseded version reachable for end-of-life freeing.
        (nvals, Self::register(epoch, self.node().as_ptr(), v))
    }

    /// Approximate heap usage of the node visible at `epoch` (vector entries
    /// only).
    #[must_use]
    pub fn memory_usage(
        self,
        epoch: u64,
    ) -> usize {
        if self.is_scalar() {
            return 0;
        }
        let node = unsafe { self.visible(epoch).as_ref() };
        size_of::<VersionNode>() + node.vector().nvals() as usize * 16
    }
}

impl From<u64> for VersionedVector {
    fn from(raw: u64) -> Self {
        Self::from_raw(raw)
    }
}

/// Borrowed view of a multi-edge entry's id vector, encoded as a
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

/// Heap side of a multi-edge entry: an id vector stamped with the graph
/// version (epoch) of the transaction that created it, chained to the vector
/// version it superseded. Freed on rollback or at chain death (see the
/// module docs).
pub struct VersionNode {
    /// Graph version of the creating transaction. The writer mutates the
    /// vector in place only when this equals its own epoch.
    epoch: u64,
    /// The older version of this pair's vector that this node superseded
    /// (null when none). Reads walk it to the newest version stamped at or
    /// below their epoch, and it keeps every superseded version reachable
    /// from the newest word for end-of-life freeing.
    next: *mut VersionNode,
    /// Only mutated by the single writer that created the node, before its
    /// transaction is published — an unpublished node is invisible to every
    /// reader, so no reader ever dereferences a vector under mutation.
    vector: UnsafeCell<Vector<bool>>,
}

// The writer only mutates its own unpublished (reader-invisible) node; every
// published node is immutable — so shared references across threads are sound.
unsafe impl Send for VersionNode {}
unsafe impl Sync for VersionNode {}

impl VersionNode {
    /// Shared view of the id vector. Sound for any node the caller may
    /// legitimately read: published nodes are immutable, and an unpublished
    /// node is only reachable by the writer that owns it.
    fn vector(&self) -> &Vector<bool> {
        unsafe { &*self.vector.get() }
    }
}

/// Streaming iterator over the edge ids of a tensor entry, ascending.
///
/// The multi-edge arm reads the node's vector in place — no lock, no `Vec`
/// materialization; the vector is immutable while published. See
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
