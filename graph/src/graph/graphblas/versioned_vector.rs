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
//! chain epochs strictly decrease (asserted in `register`). Every read walks
//! from its own copy of the word to the newest version stamped at or below
//! the reader's epoch.
//!
//! The single serialized writer mutates a node in place only when the stamp
//! equals its own epoch — the node was created in this transaction and is
//! reachable only from its unpublished matrices. Otherwise it clones the
//! visible vector into a fresh node chained to the old head and replaces the
//! tagged word in its own COW delta; older snapshots keep reading their own
//! visible version.
//!
//! ## Node lifetime: released-version tracking
//!
//! Only **released** versions are tracked — nodes a transaction superseded
//! (clone-on-write) or unlinked (pair emptied / demoted to a scalar). The
//! *current* heads are never registered anywhere: they are reachable from
//! the matrices and freed by the last `Tensor` instance's drop (one pass
//! over its effective forward matrix, heads only — their tails are all in
//! the released lists).
//!
//! All tensors of a graph share one [`Lineage`] (`Arc`) — epochs are graph
//! versions, so live-version tracking is graph-wide. It holds a bitmap of
//! live version numbers plus an ordered map with the released node
//! addresses per version:
//!
//! - **`live`** — the graph versions with a live `Graph` instance
//!   (registered by the graph layer once per version, deregistered on its
//!   drop). This is what answers "which released version can be freed":
//!   a node released at epoch `R` is readable only by snapshots with epoch
//!   `< R` (snapshots at `≥ R` see the replacement word in their matrix
//!   copy), so
//! - **`nodes`** — the released nodes keyed by release version; everything
//!   released at `R` is freed as soon as `min(live) ≥ R`, re-checked on
//!   every commit and drop.
//!
//! A release becomes visible to the lineage only when the releasing
//! transaction **commits**
//! ([`Tensor::commit`](super::tensor::Tensor::commit) drains the instance's
//! retire buffer into the lineage). Until then it is provisional:
//!
//! - **Rollback** (dropping an uncommitted instance) discards the buffer —
//!   the buffered nodes are still current in the committed graph — and
//!   frees the transaction's own nodes with one `GrB_apply` over its
//!   effective forward matrix, filtered by its epoch stamp (nothing else
//!   can reach them). Dropping the instance's COW matrices *is* the return
//!   to the previous version.
//! - A writer's **own** node unlinked mid-transaction was never published
//!   and is freed on the spot; its committed tail was already buffered for
//!   release when the node was cloned.

use std::{
    cell::UnsafeCell,
    collections::BTreeMap,
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::{NonNull, null_mut},
    sync::Arc,
};

use parking_lot::Mutex;
use roaring::RoaringTreemap;

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

    /// Address handle of this entry's node for release bookkeeping.
    /// Only valid on vector entries.
    #[must_use]
    pub(crate) fn node_ptr(self) -> NodePtr {
        NodePtr(self.node())
    }

    /// Allocate a new multi-edge node containing `ids`, stamped with the
    /// writer's `epoch`, and return the tagged pointer word. The node starts
    /// a fresh chain (its cell had no vector before). It is not registered
    /// anywhere: while current it is owned by the matrices; it is only
    /// tracked once *released* (superseded or unlinked).
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
        // Chain epochs must strictly decrease — `visible()` relies on it to
        // terminate, and a violation means the single-writer discipline (or
        // the dup-per-transaction protocol) was broken upstream.
        debug_assert!(
            next.is_null() || unsafe { (*next).epoch } < epoch,
            "version chain epoch not strictly decreasing",
        );
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
    /// store it back over this entry and **buffer this superseded node for
    /// release**. Only valid on vector entries.
    #[must_use]
    pub fn push(
        self,
        epoch: u64,
        id: u64,
    ) -> Self {
        self.push_all(epoch, &[id])
    }

    /// Add every id in `ids` for the writer at `epoch`, materializing the
    /// vector **once** for the whole batch (a per-id `push` would `GrB_wait`
    /// per element, going quadratic on large multi-edge batches). Same
    /// release contract as [`Self::push`].
    #[must_use]
    pub fn push_all(
        self,
        epoch: u64,
        ids: &[u64],
    ) -> Self {
        let before = self.count(epoch);
        let (after, w) = self.mutate(epoch, |v| {
            for &id in ids {
                v.set(id, true);
            }
        });
        debug_assert_eq!(
            after,
            before + ids.len() as u64,
            "duplicate edge id inserted into multi-edge vector",
        );
        let _ = (before, after);
        w
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
        self.remove_ids(epoch, &[id])
    }

    /// Remove every id in `ids` for the writer at `epoch`, materializing the
    /// vector once for the whole batch. Same release contract as
    /// [`Self::push`].
    #[must_use]
    pub fn remove_ids(
        self,
        epoch: u64,
        ids: &[u64],
    ) -> (u64, Self) {
        self.mutate(epoch, |v| {
            for &id in ids {
                v.remove(id);
            }
        })
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
        // reading their own visible version through the chain. The caller
        // buffers the superseded node for release.
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

//------------------------------------------------------------------------------
// Node lifetime
//------------------------------------------------------------------------------

/// Address handle of a [`VersionNode`]. Used by the release bookkeeping.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NodePtr(NonNull<VersionNode>);

// An inert address; the pointee is Send/Sync (see `VersionNode`) and only
// freed under the release discipline described in the module docs.
unsafe impl Send for NodePtr {}
unsafe impl Sync for NodePtr {}

impl NodePtr {
    /// Free the node this handle points at — its `next` chain is untouched.
    ///
    /// # Safety
    /// The caller must guarantee no live snapshot can still reach the node
    /// (the release rules in the module docs) and that each node is freed
    /// exactly once: released nodes are freed by the [`Lineage`], current
    /// heads by the last `Tensor` instance's drop, a rolled-back writer's
    /// own nodes by its drop — three disjoint sets.
    pub(crate) unsafe fn free(self) {
        drop(unsafe { Box::from_raw(self.0.as_ptr()) });
    }
}

/// Shared, `Arc`-owned tracker of **released** vector versions for one
/// graph — the authority on *which released version can be freed*. All of a
/// graph's tensors publish into the same lineage (epochs are graph
/// versions). Current heads are never tracked here; they belong to the
/// matrices. See the module docs for the rules.
///
/// The version arithmetic lives in the `live` roaring bitmap plus an
/// ordered map of the node addresses released at each version.
///
/// `live` is a set, not a multiset: the graph layer registers each graph
/// version exactly once — one `Graph` value (holding one instance of every
/// tensor) per version, shared by its readers — which `register`
/// debug_asserts.
pub(crate) struct Lineage {
    inner: Mutex<LineageInner>,
}

#[derive(Default)]
struct LineageInner {
    /// Graph versions with a live `Graph` instance.
    live: RoaringTreemap,
    /// The nodes released at each version, keyed by release version. A
    /// version released at `R` is readable only by snapshots with epoch
    /// < `R`, so everything released at `R` is freed once `min(live) ≥ R` —
    /// the ordered map lets `reclaim` take exactly that prefix.
    nodes: BTreeMap<u64, Vec<NodePtr>>,
}

impl Lineage {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(LineageInner::default()),
        })
    }

    /// Register the live graph version reading/writing at `epoch`.
    pub(crate) fn register(
        &self,
        epoch: u64,
    ) {
        let fresh = self.inner.lock().live.insert(epoch);
        debug_assert!(fresh, "one Graph instance per version");
    }

    /// Deregister a dropped graph version and free newly unreachable versions.
    pub(crate) fn deregister(
        &self,
        epoch: u64,
    ) {
        let mut g = self.inner.lock();
        let present = g.live.remove(epoch);
        debug_assert!(present, "deregistering unregistered epoch {epoch}");
        Self::reclaim(&mut g);
    }

    /// Publish a committing transaction's release buffer: each buffered
    /// version is unreadable to snapshots at or above `release_epoch`, so it
    /// is freed once `min(live) >= release_epoch`.
    pub(crate) fn retire_committed(
        &self,
        release_epoch: u64,
        buf: impl IntoIterator<Item = NodePtr>,
    ) {
        let mut g = self.inner.lock();
        let mut buf = buf.into_iter().peekable();
        if buf.peek().is_some() {
            g.nodes.entry(release_epoch).or_default().extend(buf);
        }
        Self::reclaim(&mut g);
    }

    /// Free everything released at or below the oldest live version — no
    /// snapshot that could still read those nodes remains.
    fn reclaim(g: &mut LineageInner) {
        let freeable = match g.live.min() {
            None => std::mem::take(&mut g.nodes),
            // Graph versions never reach u64::MAX, so `m + 1` cannot wrap.
            Some(m) => {
                let keep = g.nodes.split_off(&(m + 1));
                std::mem::replace(&mut g.nodes, keep)
            }
        };
        for nodes in freeable.into_values() {
            for n in nodes {
                unsafe { n.free() };
            }
        }
    }
}

impl Drop for Lineage {
    /// Graph fully gone: free the released versions that still had
    /// potential readers. Current heads were already freed by each tensor
    /// chain's last instance (`Tensor::free_versions`) — the sets are
    /// disjoint, so every node is freed exactly once.
    fn drop(&mut self) {
        let g = self.inner.get_mut();
        debug_assert!(g.live.is_empty(), "lineage dropped with live instances");
        for nodes in g.nodes.values() {
            for &n in nodes {
                unsafe { n.free() };
            }
        }
    }
}

//------------------------------------------------------------------------------
// Serialization
//------------------------------------------------------------------------------

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
/// version it superseded. Freed under the release discipline described in
/// the module docs.
pub struct VersionNode {
    /// Graph version of the creating transaction. The writer mutates the
    /// vector in place only when this equals its own epoch.
    epoch: u64,
    /// The older version of this pair's vector that this node superseded
    /// (null when none). Reads walk it to the newest version stamped at or
    /// below their epoch. It may dangle once every snapshot that could walk
    /// past this node is gone (the pointee is reclaimed first) — by
    /// construction it is never dereferenced after that point.
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
