//! 3D sparse tensor for multi-edge relationship storage.
//!
//! This module provides [`Tensor`], which extends the adjacency matrix model
//! to support multiple edges of the same type between the same pair of nodes.
//! While a plain adjacency matrix can only record whether an edge exists,
//! the tensor stores individual edge IDs so that each edge can carry its own
//! properties.
//!
//! ## Internal Structure
//!
//! Mirrors FalkorDB C's tensor design: the forward adjacency is a UINT64
//! matrix whose entries are tagged words ([`VersionedVector`]) — either a
//! scalar edge id (MSB clear) or a pointer to a heap-allocated vector of all
//! edge ids of a multi-edge pair (MSB set).
//!
//! ```text
//!   Tensor
//!     |
//!     |-- m  (forward adjacency)   src --> dst  (UINT64 tagged word)
//!     |-- mt (backward adjacency)  dst --> src  (boolean, structure only)
//!
//!   m[s, d] = edge_id                 single edge
//!   m[s, d] = ptr | MSB               multi edge (ids live in the inner)
//! ```
//!
//! ## MVCC
//!
//! The matrices get snapshot isolation from [`VersionedMatrix`] (delta
//! tracking + `Cow`). The heap nodes behind multi-edge entries are *shared*
//! across snapshots (`GrB_Matrix_dup` bit-copies the tagged words) but
//! immutable once published — a writer never mutates a node it did not
//! create in the current transaction; it clones the node and replaces the
//! tagged word in its own COW delta instead (see [`VersionedVector`]). Each
//! `Tensor` value carries an **epoch**: the graph version of the transaction
//! that created it (passed into [`Tensor::dup`] per write transaction),
//! which stamps new nodes and identifies the writer's own nodes for in-place
//! mutation.
//!
//! ## Transaction lifecycle and node lifetime
//!
//! Only **released** vector versions are tracked (see the
//! `versioned_vector` module docs). Current heads belong to the matrices;
//! nothing registers them. The tensor's part of the contract:
//!
//! - Every *foreign* (committed) node this transaction supersedes or
//!   unlinks is buffered in `retire_buf`. [`Tensor::commit`] drains the
//!   buffer into the shared [`Lineage`] stamped with the writer's epoch:
//!   from then on the lineage knows the version is only readable by
//!   snapshots below that epoch, and frees it once none remains.
//! - Unlinking the writer's **own** node (pair emptied or demoted within
//!   the creating transaction) frees it on the spot — it was never
//!   published — and its committed tail was already buffered when the node
//!   was cloned.
//! - Dropping an instance **without** commit is rollback: one epoch-filtered
//!   `GrB_apply` over its effective forward matrix frees the nodes stamped
//!   with its epoch (nothing else can reach them), its retire buffer is
//!   discarded (the buffered versions are still current in the committed
//!   graph), and dropping its COW matrices is the return to the previous
//!   version. This requires each write transaction to use a **fresh**
//!   epoch, strictly above every committed one.
//! - The **last** instance of the version chain (tracked by the `chain`
//!   token) frees the current heads with the same apply, unfiltered (heads
//!   only — every superseded tail is in a release list), and the graph-wide
//!   lineage frees whatever released versions still had potential readers
//!   once its last live version deregisters. The sets are disjoint, so each
//!   node is freed exactly once, and graph deletion racing an in-flight
//!   read no longer leaks.
//!
//! Instance creation and drop are serialized by the graph layer, which is
//! what makes the last-instance check (`Arc::strong_count`) reliable, and
//! there is **one `Tensor` instance per graph version** — readers of a
//! version share it through the graph object. The [`Lineage`] is **per
//! graph**, shared by all of its tensors: epochs are graph versions, and
//! the graph layer registers/deregisters each version exactly once.
//!
//! ## Iteration
//!
//! [`Iter`] walks the forward (or backward) adjacency matrix and yields
//! `(src, dst, edge_id)` triples, expanding multi-edge entries in ascending
//! edge-id order.

use std::{ptr::null_mut, sync::Arc};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::graph::graphblas::{
    GrB_Index, GrB_IndexUnaryOp, GrB_IndexUnaryOp_free, GrB_IndexUnaryOp_new, GrB_Info, GrB_Matrix,
    GrB_Matrix_apply_IndexOp_UINT64, GrB_Matrix_free, GrB_Matrix_new, GrB_Matrix_reduce_UINT64,
    GrB_PLUS_MONOID_UINT64, GrB_UINT64,
    matrix::{BoolExtract, Uint64Extract},
    versioned_vector::{
        IdsIter, Lineage, NodePtr, TensorEntryVector, TensorEntryVectorRef, VersionedVector,
    },
};

use super::{
    matrix::{Dup, Matrix},
    serialization::{Decode, Encode, Reader, Writer},
    versioned_matrix::{self, VersionedMatrix},
};

/// Maximum GraphBLAS index value (2^60 - 1).
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

/// Pack a `(src, dst)` node-id pair into a compound key, used by the
/// graph-wide `edge_id → endpoints` reverse index.
///
/// The encoding `(src << 32) | dst` reserves 32 bits for each side, so both
/// values must fit in a `u32`. We check this unconditionally (not just under
/// `debug_assert!`) because silent truncation would corrupt the key and
/// conflate edges between different node pairs.
///
/// NOTE: this caps node ids at 2^32 - 1, tighter than the 60-bit ids the C
/// implementation supports. Widen the key (e.g. `u128` or a struct key) if
/// larger graphs are ever needed.
#[inline]
#[must_use]
pub fn compound_key(
    src: u64,
    dst: u64,
) -> u64 {
    assert!(
        u32::try_from(src).is_ok() && u32::try_from(dst).is_ok(),
        "Tensor compound key overflow: src={src}, dst={dst} (each must fit in u32)",
    );
    (src << 32) | dst
}

/// MSB flag used by C FalkorDB to mark multi-edge entries in the serialized
/// UINT64 forward matrix (`count | MSB`).
const MSB_MASK: u64 = 1u64 << 63;

/// Edge storage for one relationship type, with inline edge ids.
///
/// The forward adjacency `m` is UINT64-valued: each entry is a
/// [`VersionedVector`] tagged word (scalar edge id, or pointer to the pair's
/// multi-edge id vector). The backward adjacency `mt` is structure-only.
///
/// Invariants:
/// - `mt[d, s]` has an entry iff `m[s, d]` does.
/// - `count` == total edges visible at `epoch`.
/// - `multi_pair_count` == number of pairs whose entry is a vector word.
/// - Write methods must not be called after [`Tensor::commit`]; start a new
///   transaction with [`Tensor::dup`] instead.
pub struct Tensor {
    /// Forward adjacency (src → dst), UINT64 tagged word.
    m: VersionedMatrix<VersionedVector>,
    /// Backward adjacency (dst → src), BOOL structure only. Edge ids are never
    /// stored here — they are recovered from `m` when iterating incoming
    /// edges, avoiding a redundant copy of every id.
    mt: VersionedMatrix<bool>,
    /// Epoch this tensor writes multi-edge nodes at: the graph version of
    /// the transaction that created it (see [`Tensor::dup`]). New nodes are
    /// stamped with it; a node whose stamp matches is the writer's own and
    /// is mutated in place. Write transactions must use a fresh epoch —
    /// rollback identifies their nodes by this stamp.
    epoch: u64,
    /// Total edge count.
    count: u64,
    /// Number of pairs whose entry is a multi-edge vector word.
    multi_pair_count: u64,
    /// Graph-wide released-version tracker, shared by every tensor of the
    /// graph (epochs are graph versions). Live-version registration is the
    /// graph layer's job; the tensor only publishes releases into it.
    lineage: Arc<Lineage>,
    /// Aliveness token of this tensor's version chain: cloned by [`Tensor::dup`],
    /// so a strong count of 1 marks the last instance of the chain, which
    /// frees the current heads on drop.
    chain: Arc<()>,
    /// Foreign versions superseded or unlinked by this transaction; released
    /// into the lineage on commit, discarded on rollback.
    retire_buf: Vec<NodePtr>,
    /// Set by [`Tensor::commit`]; gates rollback-on-drop and (debug) writes.
    committed: bool,
}

impl Drop for Tensor {
    fn drop(&mut self) {
        // Instance creation/drop is serialized by the graph layer, so the
        // strong count is stable here.
        if Arc::strong_count(&self.chain) == 1 {
            // Last instance of the chain: free the current heads. Every
            // superseded tail is in a release list — either this buffer (if
            // we never committed) or the lineage's, freed once the graph
            // deregisters the last live version.
            self.free_versions(FREE_ALL_VERSIONS);
            for n in self.retire_buf.drain(..) {
                unsafe { n.free() };
            }
        } else if !self.committed {
            // Rollback: this transaction's nodes carry its (fresh) epoch and
            // are reachable only from these matrices, so one epoch-filtered
            // apply frees them; the buffered releases never happened
            // publicly — those versions are still current, and dropping our
            // COW matrices is the return to the previous version.
            self.free_versions(self.epoch);
            self.retire_buf.clear();
        } else {
            debug_assert!(
                self.retire_buf.is_empty(),
                "committed instance holds an undrained retire buffer",
            );
        }
    }
}

/// `free_versions` thunk meaning "free every vector node", used by the last
/// instance's teardown. Graph versions never reach `u64::MAX`.
const FREE_ALL_VERSIONS: u64 = u64::MAX;

impl Tensor {
    #[must_use]
    pub fn new(
        nrows: u64,
        ncols: u64,
        epoch: u64,
        lineage: Arc<Lineage>,
    ) -> Self {
        Self {
            m: VersionedMatrix::<VersionedVector>::new(nrows, ncols),
            mt: VersionedMatrix::<bool>::new(ncols, nrows),
            epoch,
            count: 0,
            multi_pair_count: 0,
            lineage,
            chain: Arc::new(()),
            retire_buf: Vec::new(),
            committed: false,
        }
    }

    /// Publish this transaction's releases: every superseded or unlinked
    /// foreign version becomes readable only below this epoch, and is freed
    /// by the lineage once no such snapshot remains. Must be called exactly
    /// once, when the owning write transaction commits; dropping without
    /// calling it is rollback.
    pub fn commit(&mut self) {
        debug_assert!(!self.committed, "double commit");
        self.lineage
            .retire_committed(self.epoch, self.retire_buf.drain(..));
        self.committed = true;
    }

    /// Re-home a freshly decoded tensor onto the graph's shared lineage.
    /// Only valid right after [`Tensor::decode`], before any dup or write:
    /// the dummy decode lineage must not have accumulated any state.
    pub(crate) fn set_lineage(
        &mut self,
        lineage: Arc<Lineage>,
    ) {
        debug_assert!(self.committed && self.retire_buf.is_empty());
        self.lineage = lineage;
    }

    /// Buffer a foreign version this transaction superseded or unlinked; it
    /// is released (at this epoch) when the transaction commits.
    fn retire(
        &mut self,
        vv: VersionedVector,
    ) {
        debug_assert!(!vv.is_scalar());
        debug_assert_ne!(vv.epoch(), self.epoch, "own nodes are freed, not released");
        self.retire_buf.push(vv.node_ptr());
    }

    /// Handle the node behind a word this transaction is unlinking (pair
    /// emptied, or demoted to a scalar word). Own nodes were never published
    /// and are freed on the spot — their committed tail was already buffered
    /// when they were cloned. Foreign nodes are buffered for release.
    fn unlink(
        &mut self,
        vv: VersionedVector,
    ) {
        if vv.is_scalar() {
            return;
        }
        if vv.epoch() == self.epoch {
            unsafe { vv.node_ptr().free() };
        } else {
            self.retire(vv);
        }
    }

    /// Free the nodes behind this instance's forward matrix in one
    /// `GrB_apply` pass, the way C's `Tensor_free` walks its matrix with a
    /// unary op. `only_epoch == FREE_ALL_VERSIONS` frees every vector node
    /// (final teardown); otherwise only nodes stamped with `only_epoch`
    /// (rollback of the transaction that created them).
    ///
    /// The apply runs over the **extracted effective** matrix
    /// (`(m − dm) ∪ dp`), not the raw base: words this transaction removed
    /// still sit in the base masked by delta-minus, and their nodes were
    /// already handled when they were unlinked — touching them again would
    /// double-free. Extraction also sidesteps relying on a mask to suppress
    /// side effects, which GraphBLAS does not guarantee.
    fn free_versions(
        &self,
        only_epoch: u64,
    ) {
        let effective = self.m.extract();
        unsafe {
            let mut free_op: GrB_IndexUnaryOp = null_mut();
            let info = GrB_IndexUnaryOp_new(
                &raw mut free_op,
                Some(free_versions_fn),
                GrB_UINT64,
                GrB_UINT64,
                GrB_UINT64,
            );
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

            let mut sink: GrB_Matrix = null_mut();
            let info = GrB_Matrix_new(&raw mut sink, GrB_UINT64, self.m.nrows(), self.m.ncols());
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

            let info = GrB_Matrix_apply_IndexOp_UINT64(
                sink,
                null_mut(),
                null_mut(),
                free_op,
                effective.inner(),
                only_epoch,
                null_mut(),
            );
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

            GrB_Matrix_free(&raw mut sink);
            GrB_IndexUnaryOp_free(&raw mut free_op);
        }
    }

    /// Edge ids for the `(src, dest)` pair, in ascending edge-id order.
    /// Lock-free: a multi-edge pair's vector is read in place (see
    /// [`VersionedVector::ids`](super::versioned_vector::VersionedVector::ids)).
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> IdsIter {
        self.m
            .get(src, dest)
            .map_or_else(IdsIter::empty, |vv| vv.ids(self.epoch))
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        debug_assert!(!self.committed, "write after commit; dup a new instance");
        match self.m.get(src, dest) {
            None => {
                // First edge for this pair: store the id inline.
                self.m.set(src, dest, VersionedVector::new_scalar(id));
                self.mt.set(dest, src, true);
            }
            Some(vv) if vv.is_scalar() => {
                // Second edge: promote scalar → vector.
                let vec = VersionedVector::new_vec(self.epoch, [vv.scalar(), id]);
                self.m.set(src, dest, vec);
                self.multi_pair_count += 1;
            }
            Some(vv) => {
                let new_vv = vv.push(self.epoch, id);
                if new_vv != vv {
                    // Another transaction's node was cloned; publish the new
                    // word and buffer the superseded version for release at
                    // commit. Older snapshots keep reading it through their
                    // own copies of the word until they drop.
                    self.retire(vv);
                    self.m.set(src, dest, new_vv);
                }
            }
        }
        self.count += 1;
    }

    /// Set entries from parallel slices. The first edge of each pair lands
    /// inline in `m`/`mt` as a scalar word; pairs that gain additional edges
    /// get (or extend) a multi-edge vector.
    ///
    /// The loop over the input only *classifies* — no matrix write and no
    /// heap-vector mutation happens until every membership probe is done, so
    /// `m` syncs pending GraphBLAS work at most once for the whole batch (a
    /// per-edge get-after-set pattern would re-sync per edge, going
    /// quadratic). Each touched multi-edge vector is then mutated **once**
    /// with all of its batch ids (one clone, one `GrB_wait`), instead of a
    /// wait per id. In-batch duplicates are caught by batch-local maps,
    /// keeping the cost O(batch) instead of scanning all committed pairs.
    pub fn set_all_from_slices(
        &mut self,
        srcs: &[u64],
        dsts: &[u64],
        ids: &[u64],
    ) {
        debug_assert!(!self.committed, "write after commit; dup a new instance");
        debug_assert_eq!(srcs.len(), dsts.len());
        debug_assert_eq!(srcs.len(), ids.len());
        if srcs.is_empty() {
            return;
        }

        // Pairs first seen in this batch → all their ids (queued below).
        let mut new_pairs: FxHashMap<(u64, u64), Vec<u64>> = FxHashMap::default();
        // Committed/pending scalar pairs gaining edges this batch:
        // [inline first id, new ids...].
        let mut promoted: FxHashMap<(u64, u64), Vec<u64>> = FxHashMap::default();
        // Pre-existing multi-edge pairs: the word captured at probe time
        // (words are stable during the probe — nothing writes `m`) plus all
        // ids this batch adds to the pair.
        let mut existing: FxHashMap<(u64, u64), (VersionedVector, Vec<u64>)> = FxHashMap::default();

        // Probe/classify phase: reads only.
        for ((&s, &d), &id) in srcs.iter().zip(dsts.iter()).zip(ids.iter()) {
            if let Some(pair_ids) = new_pairs.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(pair_ids) = promoted.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some((_, pair_ids)) = existing.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(vv) = self.m.get(s, d) {
                if vv.is_scalar() {
                    promoted.insert((s, d), vec![vv.scalar(), id]);
                } else {
                    existing.insert((s, d), (vv, vec![id]));
                }
            } else {
                new_pairs.insert((s, d), vec![id]);
            }
        }
        self.count += srcs.len() as u64;

        // Existing multi-edge pairs: one batched mutation per pair. A clone
        // (foreign node) is published and supersedes a version that is
        // buffered for release.
        for ((s, d), (vv, pair_ids)) in existing {
            let new_vv = vv.push_all(self.epoch, &pair_ids);
            if new_vv != vv {
                self.retire(vv);
                self.m.set(s, d, new_vv);
            }
        }

        // New pairs: single-edge pairs go through the bulk scalar path;
        // in-batch multi-edge pairs get a vector word directly.
        let mut m_srcs: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut m_dsts: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut m_ids: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut mt_entries: Vec<(u64, u64)> = Vec::with_capacity(new_pairs.len());
        for ((s, d), pair_ids) in new_pairs {
            mt_entries.push((d, s));
            if let [id] = pair_ids[..] {
                m_srcs.push(s);
                m_dsts.push(d);
                m_ids.push(id);
            } else {
                let vec = VersionedVector::new_vec(self.epoch, pair_ids);
                self.m.set(s, d, vec);
                self.multi_pair_count += 1;
            }
        }
        for ((s, d), pair_ids) in promoted {
            debug_assert!(pair_ids.len() >= 2);
            let vec = VersionedVector::new_vec(self.epoch, pair_ids);
            // Old effective word was a scalar, never a pointer — no unlink.
            self.m.set(s, d, vec);
            self.multi_pair_count += 1;
        }

        self.m.set_all(
            m_srcs
                .iter()
                .zip(m_dsts.iter())
                .zip(m_ids.iter())
                .map(|((&s, &d), &id)| (s, d, id)),
        );
        self.mt.set_all(mt_entries.into_iter());
    }

    /// Bulk-remove specific edges from this tensor.
    ///
    /// Each entry in `rels` is `(edge_id, src, dst)`.
    /// Returns the list of `(src, dst)` pairs that became completely empty
    /// in this tensor (no remaining edges of this type between those nodes).
    pub fn remove_all(
        &mut self,
        rels: &[(u64, u64, u64)],
    ) -> Vec<(u64, u64)> {
        debug_assert!(!self.committed, "write after commit; dup a new instance");
        if rels.is_empty() {
            return Vec::new();
        }

        // Fast path: no multi-edge pairs exist, so every edge is the scalar
        // entry of its pair. Bulk-remove from the forward/backward adjacency
        // in two GraphBLAS ops; every touched pair becomes empty.
        if !self.has_multi_edge() {
            let nrows = self.m.nrows();
            let ncols = self.m.ncols();
            let mut m_rows = Vec::with_capacity(rels.len());
            let mut m_cols = Vec::with_capacity(rels.len());
            let mut mt_rows = Vec::with_capacity(rels.len());
            let mut mt_cols = Vec::with_capacity(rels.len());
            for &(_, src, dst) in rels {
                m_rows.push(src);
                m_cols.push(dst);
                mt_rows.push(dst);
                mt_cols.push(src);
            }
            let mut m_mask = Matrix::<bool>::new(nrows, ncols);
            m_mask.build(&m_rows, &m_cols);
            let mut mt_mask = Matrix::<bool>::new(ncols, nrows);
            mt_mask.build(&mt_rows, &mt_cols);
            self.m.remove_mask(&m_mask);
            self.mt.remove_mask(&mt_mask);
            debug_assert!(self.count >= rels.len() as u64);
            self.count -= rels.len() as u64;
            return rels.iter().map(|&(_, src, dst)| (src, dst)).collect();
        }

        // Slow path: some pairs have multi-edge vectors. Group the doomed
        // ids per pair so each pair is probed once (a get-after-remove
        // pattern re-syncs `m` per edge — quadratic), then:
        //  - every edge of the pair doomed: unlink the word, pair empties;
        //  - all but one doomed: demote to an inline scalar without touching
        //    the vector (the survivor is read straight off the visible
        //    version), unlink the old word;
        //  - otherwise: one batched removal (one clone + one wait for a
        //    foreign node), publish/release as in the insert paths.
        // Matrix writes are deferred until all probes are done.
        let mut per_pair: FxHashMap<(u64, u64), Vec<u64>> = FxHashMap::default();
        for &(id, src, dst) in rels {
            per_pair.entry((src, dst)).or_default().push(id);
        }

        enum Op {
            /// Scalar word: remove the pair.
            ClearScalar,
            /// Every id of the vector doomed: unlink the word, remove pair.
            ClearVector(VersionedVector),
            /// One id survives: write it as a scalar, unlink the old word.
            Demote(u64, VersionedVector),
            /// New (cloned) word to publish over the old one.
            Publish(VersionedVector, VersionedVector),
        }

        // Probe phase — no matrix writes.
        let mut ops: Vec<((u64, u64), Op)> = Vec::with_capacity(per_pair.len());
        for (&(src, dst), doomed) in &per_pair {
            let Some(vv) = self.m.get(src, dst) else {
                debug_assert!(
                    false,
                    "removing edges {doomed:?} from missing pair ({src}, {dst})"
                );
                continue;
            };
            debug_assert!(self.count >= doomed.len() as u64);
            self.count -= doomed.len() as u64;

            if vv.is_scalar() {
                debug_assert_eq!(doomed[..], [vv.scalar()]);
                ops.push(((src, dst), Op::ClearScalar));
                continue;
            }

            let n = vv.count(self.epoch);
            let k = doomed.len() as u64;
            debug_assert!(k <= n, "removing more edges than the pair holds");
            if k == n {
                // No point mutating a vector we are about to drop entirely
                // (the old code cloned a foreign node here and leaked the
                // clone) — just unlink the word.
                ops.push(((src, dst), Op::ClearVector(vv)));
            } else if n - k == 1 {
                // Read the survivor straight off the visible version; no
                // clone, no mutation.
                let dead: FxHashSet<u64> = doomed.iter().copied().collect();
                let survivor = vv
                    .ids(self.epoch)
                    .find(|id| !dead.contains(id))
                    .expect("survivor must exist when n - k == 1");
                ops.push(((src, dst), Op::Demote(survivor, vv)));
            } else {
                let (remaining, new_vv) = vv.remove_ids(self.epoch, doomed);
                debug_assert_eq!(remaining, n - k, "doomed id missing from vector");
                if new_vv != vv {
                    ops.push(((src, dst), Op::Publish(new_vv, vv)));
                }
                // In-place removal on our own node: nothing to write back.
            }
        }

        // Write phase.
        let mut emptied = Vec::new();
        for ((src, dst), op) in ops {
            match op {
                Op::ClearScalar => {
                    self.m.remove(src, dst);
                    self.mt.remove(dst, src);
                    emptied.push((src, dst));
                }
                Op::ClearVector(vv) => {
                    self.unlink(vv);
                    self.m.remove(src, dst);
                    self.mt.remove(dst, src);
                    self.multi_pair_count -= 1;
                    emptied.push((src, dst));
                }
                Op::Demote(survivor, vv) => {
                    self.unlink(vv);
                    // Old snapshots keep reading the vector word from their
                    // own matrix copies until the release frees the node.
                    self.m.set(src, dst, VersionedVector::new_scalar(survivor));
                    self.multi_pair_count -= 1;
                }
                Op::Publish(new_vv, old_vv) => {
                    self.retire(old_vv);
                    self.m.set(src, dst, new_vv);
                }
            }
        }
        emptied
    }

    pub fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.m.resize(nrows, ncols);
        self.mt.resize(ncols, nrows);
    }

    /// Rebuild the backward matrix as the transpose of the forward matrix.
    ///
    /// `mt` is structure-only (`bool`). The forward matrix's *effective*
    /// structure (`(m − dm) ∪ dp`) is materialized first, then transposed into
    /// a clean base with empty deltas.
    pub fn rebuild_backward(&mut self) {
        self.mt = VersionedMatrix::from_matrix(self.m.extract().transpose());
    }

    /// Snapshot this tensor for a new transaction: `Cow`-shares the
    /// matrices, pins the copy to the transaction's graph version, and joins
    /// the graph's shared [`Lineage`]. Each graph version gets exactly one
    /// instance (readers share it through the graph object); the graph layer
    /// registers the version in the lineage. Write transactions must use a
    /// fresh epoch (strictly above every committed one) and call
    /// [`Tensor::commit`] to publish; dropping without commit is rollback.
    #[must_use]
    pub fn dup(
        &self,
        epoch: u64,
    ) -> Self {
        debug_assert!(
            self.committed,
            "dup must snapshot a committed instance, not an in-flight writer",
        );
        debug_assert!(epoch >= self.epoch, "snapshot epoch precedes its source");
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            epoch,
            count: self.count,
            multi_pair_count: self.multi_pair_count,
            lineage: Arc::clone(&self.lineage),
            chain: Arc::clone(&self.chain),
            retire_buf: Vec::new(),
            committed: false,
        }
    }

    /// Forward adjacency (src → dst); UINT64 tagged words. Structural
    /// consumers (`ANY_PAIR` mxm, masks) may use it directly.
    #[must_use]
    pub const fn matrix(&self) -> &VersionedMatrix<VersionedVector> {
        &self.m
    }

    /// Transposed/backward pair-level adjacency (dst → src), structure only.
    #[must_use]
    pub const fn matrix_t(&self) -> &VersionedMatrix<bool> {
        &self.mt
    }

    /// Total number of edges in this tensor.
    #[must_use]
    pub const fn edge_count(&self) -> u64 {
        self.count
    }

    /// Iterate every `(src, dst, edge_id)` triple in the tensor.
    ///
    /// Streams the forward matrix, expanding multi-edge vector entries. On a
    /// single-edge graph this is one streaming pass over `m` with no
    /// per-pair work.
    pub fn iter_edges(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        let epoch = self.epoch;
        self.m.iter(0, u64::MAX).flat_map(move |(src, dst, raw)| {
            VersionedVector::from_raw(raw)
                .ids(epoch)
                .map(move |id| (src, dst, id))
        })
    }

    #[must_use]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Iter<'_> {
        Iter::new(self, min_row, max_row, transpose)
    }

    /// Whether this tensor has any (src, dst) pair with more than one edge.
    #[must_use]
    pub const fn has_multi_edge(&self) -> bool {
        self.multi_pair_count != 0
    }

    pub fn wait(&mut self) {
        self.m.wait();
        self.mt.wait();
    }

    /// Wait on all matrices for fork safety (takes &self, not &mut self).
    pub fn wait_all(&self) {
        self.m.wait_all();
        self.mt.wait_all();
    }

    /// Returns true if every internal matrix has no pending GraphBLAS
    /// operations queued.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.m.is_synced() && self.mt.is_synced()
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        let mut usage = self.m.memory_usage() + self.mt.memory_usage();
        if self.multi_pair_count != 0 {
            usage += self.multi_edge_memory_usage();
        }
        usage
    }

    /// Heap usage of the multi-edge id vectors, computed like C's
    /// `Tensor_memoryUsage`: a GraphBLAS index op maps every tagged word to
    /// the size of its node visible at this tensor's epoch (scalar words map
    /// to 0), and a plus-reduction sums the per-entry sizes. Runs over the
    /// base and delta-plus matrices; as in C, entries masked in delta-minus
    /// still count.
    fn multi_edge_memory_usage(&self) -> usize {
        self.m.wait();
        let mut total = 0u64;
        unsafe {
            let mut size_op: GrB_IndexUnaryOp = null_mut();
            let info = GrB_IndexUnaryOp_new(
                &raw mut size_op,
                Some(memory_usage_fn),
                GrB_UINT64,
                GrB_UINT64,
                GrB_UINT64,
            );
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

            for words in [self.m.m().inner(), self.m.dp().inner()] {
                let mut sizes: GrB_Matrix = null_mut();
                let info =
                    GrB_Matrix_new(&raw mut sizes, GrB_UINT64, self.m.nrows(), self.m.ncols());
                debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

                let info = GrB_Matrix_apply_IndexOp_UINT64(
                    sizes,
                    null_mut(),
                    null_mut(),
                    size_op,
                    words,
                    self.epoch,
                    null_mut(),
                );
                debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);

                let mut sum = 0u64;
                let info = GrB_Matrix_reduce_UINT64(
                    &raw mut sum,
                    null_mut(),
                    GrB_PLUS_MONOID_UINT64,
                    sizes,
                    null_mut(),
                );
                debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
                total += sum;

                GrB_Matrix_free(&raw mut sizes);
            }

            GrB_IndexUnaryOp_free(&raw mut size_op);
        }
        total as usize
    }
}

/// `GrB_IndexUnaryOp` used by [`Tensor::free_versions`]: frees the node
/// behind the word `x` when it is a vector entry and the thunk `y` is
/// [`FREE_ALL_VERSIONS`] or matches the node's epoch. Always writes `z`
/// (GraphBLAS stores it into the sink matrix; leaving it uninitialized is
/// UB).
unsafe extern "C" fn free_versions_fn(
    z: *mut std::os::raw::c_void,
    x: *const std::os::raw::c_void,
    _i: GrB_Index,
    _j: GrB_Index,
    y: *const std::os::raw::c_void,
) {
    unsafe {
        let only_epoch = *y.cast::<u64>();
        let vv = VersionedVector::from_raw(*x.cast::<u64>());
        if !vv.is_scalar() && (only_epoch == FREE_ALL_VERSIONS || vv.epoch() == only_epoch) {
            vv.node_ptr().free();
        }
        *z.cast::<u64>() = 0;
    }
}

/// `GrB_IndexUnaryOp`: `z` = heap size of the entry's id vector visible at
/// epoch `y`, `0` for scalar words — the Rust counterpart of C's
/// `_multiedge_memory`.
unsafe extern "C" fn memory_usage_fn(
    z: *mut std::os::raw::c_void,
    x: *const std::os::raw::c_void,
    _i: GrB_Index,
    _j: GrB_Index,
    y: *const std::os::raw::c_void,
) {
    unsafe {
        let epoch = *y.cast::<u64>();
        let vv = VersionedVector::from_raw(*x.cast::<u64>());
        *z.cast::<u64>() = vv.memory_usage(epoch) as u64;
    }
}

impl Encode<19> for Tensor {
    #[allow(clippy::similar_names)]
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        let nrows = self.m.nrows();
        let ncols = self.m.ncols();

        // Serialize the C-compatible UINT64 forward matrix from the effective
        // state. Single-edge pairs store the edge id directly; multi-edge
        // pairs store `(edge_count | MSB)` and push their full id list into
        // the tensor section below.
        let mut f_rows: Vec<u64> = Vec::new();
        let mut f_cols: Vec<u64> = Vec::new();
        let mut f_vals: Vec<u64> = Vec::new();
        let mut multi: Vec<(u64, u64, VersionedVector)> = Vec::new();
        for (src, dst, raw) in self.m.iter(0, u64::MAX) {
            let vv = VersionedVector::from_raw(raw);
            f_rows.push(src);
            f_cols.push(dst);
            if vv.is_scalar() {
                f_vals.push(vv.scalar());
            } else {
                let n = vv.count(self.epoch);
                if n == 1 {
                    f_vals.push(vv.ids(self.epoch).next().unwrap());
                } else {
                    f_vals.push(n | MSB_MASK);
                    multi.push((src, dst, vv));
                }
            }
        }

        // Forward VersionedMatrix layout: base (effective), empty delta-plus,
        // empty delta-minus. Folding dp into the base keeps the on-disk form
        // canonical and matches what decode expects.
        let empty = Matrix::<VersionedVector>::new(nrows, ncols);
        if f_rows.is_empty() {
            empty.encode(w);
        } else {
            let mut fm = Matrix::<VersionedVector>::new(nrows, ncols);
            fm.build(&f_rows, &f_cols, &f_vals);
            fm.encode(w);
        }
        empty.encode(w); // delta-plus
        empty.encode(w); // delta-minus

        // Tensor section (C v19 format): leading word = number of multi-edge
        // pairs; zero means nothing else follows. Otherwise two groups (base
        // TM, then delta-plus TDP), each a count followed by per-pair
        // (src, dst, GxB_Vector_serialize blob of the BOOL id vector). All
        // multi-edge pairs live in the base group; the delta-plus group is
        // empty since dp was folded into the base above.
        w.write_unsigned(multi.len() as u64);
        if multi.is_empty() {
            return;
        }
        w.write_unsigned(multi.len() as u64);
        for (src, dst, vv) in &multi {
            w.write_unsigned(*src);
            w.write_unsigned(*dst);
            vv.read(self.epoch, |v| TensorEntryVectorRef(v).encode(w));
        }
        w.write_unsigned(0); // empty delta-plus tensor group
    }
}

impl Decode<19> for Tensor {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let forward = VersionedMatrix::<VersionedVector>::decode(r)?;
        let nrows = forward.nrows();
        let ncols = forward.ncols();

        // The on-disk forward matrix (C-compatible) stores single-edge ids
        // directly (MSB clear) and `(count | MSB)` for multi-edge pairs,
        // whose real id lists follow in the tensor section. `forward` holds
        // count words, not pointers — the real pointer words are built into
        // the fresh `m` below.
        let mut m = VersionedMatrix::<VersionedVector>::new(nrows, ncols);
        let mut count = 0u64;
        let mut multi_pair_count = 0u64;

        m.set_all(
            forward
                .iter(0, u64::MAX)
                .filter(|&(_, _, value)| value & MSB_MASK == 0)
                .inspect(|_| count += 1),
        );

        // Tensor section (C v19 format): number of multi-edge pairs, then —
        // only when non-zero — two groups (base TM, delta-plus TDP) of
        // (src, dst, GxB blob of the BOOL id vector) entries.
        let total_tensors = r.read_unsigned()?;
        if total_tensors > 0 {
            for _ in 0..2 {
                let group_count = r.read_unsigned()?;
                for _ in 0..group_count {
                    let src = r.read_unsigned()?;
                    let dst = r.read_unsigned()?;
                    let v = TensorEntryVector::decode(r)?.0;
                    count += v.nvals();
                    // Decoded state is the committed baseline: epoch 0, below
                    // any writer epoch. Nothing to track — as current heads,
                    // these nodes are owned by `m` and freed by the last
                    // instance's drop.
                    m.set(src, dst, VersionedVector::from_committed(v));
                    multi_pair_count += 1;
                }
            }
        }
        debug_assert_eq!(multi_pair_count, total_tensors);

        // Backward matrix is rebuilt from `m` by the caller (`rebuild_backward`)
        // after decode, so leave it empty here. Decoded graphs start at
        // version 0, matching the epoch the entries were stamped with.
        // The trait signature can't carry the graph's lineage, so a dummy
        // unregistered one is used; `Graph::restore` re-homes the tensor via
        // `set_lineage` before the graph goes live.
        Ok(Self {
            m,
            mt: VersionedMatrix::<bool>::new(0, 0),
            epoch: 0,
            count,
            multi_pair_count,
            lineage: Lineage::new(),
            chain: Arc::new(()),
            retire_buf: Vec::new(),
            committed: true, // decoded state is the committed baseline
        })
    }
}

/// Base adjacency iterator. Forward iteration streams tagged words directly
/// from `m`; backward iteration streams the BOOL structure of `mt` (which
/// carries no ids) and recovers each word from `m`.
enum BaseIter {
    Forward(versioned_matrix::Iter<Uint64Extract>),
    Backward(versioned_matrix::Iter<BoolExtract>),
}

pub struct Iter<'a> {
    t: &'a Tensor,
    base: BaseIter,
    src: u64,
    dest: u64,
    /// Buffered, ascending edge ids for the current multi-edge pair.
    buf: Vec<u64>,
    buf_pos: usize,
}

impl<'a> Iter<'a> {
    fn new(
        t: &'a Tensor,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Self {
        Self {
            t,
            base: if transpose {
                BaseIter::Backward(t.mt.iter(min_row, max_row))
            } else {
                BaseIter::Forward(t.m.iter(min_row, max_row))
            },
            src: 0,
            dest: 0,
            buf: Vec::new(),
            buf_pos: 0,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Drain buffered (ascending) ids for the current multi-edge pair.
            if self.buf_pos < self.buf.len() {
                let id = self.buf[self.buf_pos];
                self.buf_pos += 1;
                return Some((self.src, self.dest, id));
            }

            // Next base pair, oriented as (src, dest) with its tagged word.
            let (src, dest, vv) = match &mut self.base {
                BaseIter::Forward(it) => {
                    let (row, col, raw) = it.next()?;
                    (row, col, VersionedVector::from_raw(raw))
                }
                BaseIter::Backward(it) => {
                    let (row, col) = it.next()?;
                    let (src, dest) = (col, row);
                    let Some(vv) = self.t.m.get(src, dest) else {
                        // `mt` must mirror `m`'s structure; don't fabricate
                        // an edge id out of a broken invariant.
                        debug_assert!(false, "mt entry ({row}, {col}) has no matching m entry");
                        continue;
                    };
                    (src, dest, vv)
                }
            };
            self.src = src;
            self.dest = dest;

            if vv.is_scalar() {
                return Some((src, dest, vv.scalar()));
            }
            self.buf = vv.ids(self.t.epoch).collect();
            self.buf_pos = 0;
            // Loop back to drain the buffer (defensively handles an empty
            // visible vector instead of yielding a phantom id).
        }
    }
}
