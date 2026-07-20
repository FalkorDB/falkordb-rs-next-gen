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
//! ## Node lifetime
//!
//! Multi-edge nodes are freed by version chains, not refcounts or a live-
//! version registry. When a writer supersedes a foreign node it chains the
//! fresh node to the old one (`VersionNode::next`), so the newest word of a
//! cell reaches every older version of its vector. Every instance in a
//! tensor's version chain shares one graveyard of unlinked chains — an
//! `Arc`-shared list whose reference count doubles as the chain's instance
//! count. Nodes are freed at exactly two points:
//!
//! - **Rollback**: dropping an uncommitted instance frees the nodes it
//!   created — stamped with its own epoch, visible only to itself. Its
//!   retire buffer is discarded (the buffered chains are still linked in
//!   committed versions).
//! - **Chain death**: dropping the last instance (the one that takes the
//!   graveyard `Arc` to zero) frees every node reachable from its effective
//!   matrix, the graveyard, and its own retire buffer, deduplicated by
//!   address (a graveyard chain can reach the same node an old snapshot's
//!   word points at).
//!
//! Unlinking a word (edge deletion, demotion to scalar) buffers the pair's
//! chain in the instance's `retired` list; [`Tensor::commit`] moves the
//! buffer to the shared graveyard and marks the instance committed. A
//! writer's own head (same epoch, never published) is freed on the spot and
//! only its committed tail is buffered.
//!
//! Superseded committed nodes therefore stay allocated until their pair's
//! chain reaches the graveyard or the tensor dies. One bounded, deliberate
//! leak: if the newest version drops while an older snapshot is still live
//! (graph deletion racing an in-flight read), nodes reachable only from the
//! newest matrix are never freed.
//!
//! ## Iteration
//!
//! [`Iter`] walks the forward (or backward) adjacency matrix and yields
//! `(src, dst, edge_id)` triples, expanding multi-edge entries in ascending
//! edge-id order.

use std::{ptr::null_mut, sync::Arc};

use parking_lot::Mutex;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::graph::graphblas::{
    GrB_Index, GrB_IndexUnaryOp, GrB_IndexUnaryOp_free, GrB_IndexUnaryOp_new, GrB_Info, GrB_Matrix,
    GrB_Matrix_apply, GrB_Matrix_apply_IndexOp_UDT, GrB_Matrix_apply_IndexOp_UINT64,
    GrB_Matrix_free, GrB_Matrix_new, GrB_Matrix_reduce_UINT64, GrB_PLUS_MONOID_UINT64, GrB_Type,
    GrB_Type_new, GrB_UINT64, GrB_UnaryOp, GrB_UnaryOp_new,
    matrix::{BoolExtract, Uint64Extract},
    versioned_vector::{IdsIter, TensorEntryVector, TensorEntryVectorRef, VersionedVector},
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
    /// is mutated in place.
    epoch: u64,
    /// Total edge count.
    count: u64,
    /// Number of pairs whose entry is a multi-edge vector word.
    multi_pair_count: u64,
}

impl Drop for Tensor {
    fn drop(&mut self) {
        self.release(self.epoch);
    }
}

impl Tensor {
    #[must_use]
    pub fn new(
        nrows: u64,
        ncols: u64,
        epoch: u64,
    ) -> Self {
        Self {
            m: VersionedMatrix::<VersionedVector>::new(nrows, ncols),
            mt: VersionedMatrix::<bool>::new(ncols, nrows),
            epoch,
            count: 0,
            multi_pair_count: 0,
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
                    // word. The old node stays chained behind it (`next`) for
                    // older snapshots and end-of-life freeing — no unlink.
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
    /// Membership probes run before any write to `m`, so `m` syncs pending
    /// GraphBLAS work at most once for the whole batch (a per-edge
    /// get-after-set pattern would re-sync per edge, going quadratic).
    /// In-batch duplicates are caught by batch-local maps, keeping the cost
    /// O(batch) instead of scanning all committed pairs.
    pub fn set_all_from_slices(
        &mut self,
        srcs: &[u64],
        dsts: &[u64],
        ids: &[u64],
    ) {
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
        // Pre-existing multi-edge pairs whose node another transaction owns:
        // the replacement word (this transaction's node, so later pushes
        // mutate in place). Written back after the probe loop so no dp write
        // lands between membership probes.
        let mut updated: FxHashMap<(u64, u64), VersionedVector> = FxHashMap::default();

        for ((&s, &d), &id) in srcs.iter().zip(dsts.iter()).zip(ids.iter()) {
            if let Some(pair_ids) = new_pairs.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(pair_ids) = promoted.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(&w) = updated.get(&(s, d)) {
                // This transaction owns the node — pushed in place.
                let _ = w.push(self.epoch, id);
            } else if let Some(vv) = self.m.get(s, d) {
                if vv.is_scalar() {
                    promoted.insert((s, d), vec![vv.scalar(), id]);
                } else {
                    let new_vv = vv.push(self.epoch, id);
                    if new_vv != vv {
                        // Foreign node cloned; the old one stays chained
                        // behind the new word — no unlink.
                        updated.insert((s, d), new_vv);
                    }
                }
            } else {
                new_pairs.insert((s, d), vec![id]);
            }
        }
        self.count += srcs.len() as u64;

        // Publish the replacement words for multi-edge pairs whose old nodes
        // were superseded (and chained) above.
        for ((s, d), w) in updated {
            self.m.set(s, d, w);
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
            self.count -= rels.len() as u64;
            return rels.iter().map(|&(_, src, dst)| (src, dst)).collect();
        }

        // Slow path: some pairs have multi-edge vectors. Handle per edge:
        //  - scalar entry: the pair becomes empty.
        //  - vector entry: drop the id from the vector; demote to a scalar
        //    word when one id remains, empty the pair at zero.
        let mut emptied = Vec::new();
        for &(id, src, dst) in rels {
            let Some(vv) = self.m.get(src, dst) else {
                debug_assert!(false, "removing edge {id} from missing pair ({src}, {dst})");
                continue;
            };
            if vv.is_scalar() {
                debug_assert_eq!(vv.scalar(), id);
                self.m.remove(src, dst);
                self.mt.remove(dst, src);
                emptied.push((src, dst));
            } else {
                let (remaining, new_vv) = vv.remove(self.epoch, id);
                if remaining == 0 {
                    // `new_vv` is this transaction's own node (fresh clone
                    // chained to `vv`, or `vv` itself mutated in place), so
                    // one unlink frees it and retires the committed chain.
                    self.m.remove(src, dst);
                    self.mt.remove(dst, src);
                    self.multi_pair_count -= 1;
                    emptied.push((src, dst));
                } else if remaining == 1 {
                    // Demote back to an inline scalar; old snapshots keep
                    // reading the vector word from their own matrix copy.
                    // As above, one unlink retires the pair's whole chain.
                    let last = new_vv.ids(self.epoch).next().unwrap();
                    self.m.set(src, dst, VersionedVector::new_scalar(last));
                    self.multi_pair_count -= 1;
                } else if new_vv != vv {
                    // Another transaction's node was cloned; publish the new
                    // word. The old node stays chained behind it — no unlink.
                    self.m.set(src, dst, new_vv);
                }
            }
            self.count -= 1;
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

    /// Snapshot this tensor for a new write transaction: `Cow`-shares the
    /// matrices, pins the copy to the transaction's graph version, and joins
    /// the chain's shared graveyard.
    #[must_use]
    pub fn dup(
        &self,
        epoch: u64,
    ) -> Self {
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            epoch,
            count: self.count,
            multi_pair_count: self.multi_pair_count,
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

    pub fn release(
        &self,
        version: u64,
    ) {
        unsafe {
            let mut release_op: GrB_IndexUnaryOp = null_mut();
            GrB_IndexUnaryOp_new(
                &raw mut release_op,
                Some(release_fn),
                GrB_UINT64,
                GrB_UINT64,
                GrB_UINT64,
            );

            GrB_Matrix_apply_IndexOp_UDT(
                self.m.m().inner(),
                null_mut(),
                null_mut(),
                release_op,
                self.m.m().inner(),
                (&raw const version).cast(),
                null_mut(),
            );
        }
    }
}

unsafe extern "C" fn release_fn(
    z: *mut std::os::raw::c_void,
    x: *const std::os::raw::c_void,
    _i: crate::graph::graphblas::GrB_Index,
    _j: crate::graph::graphblas::GrB_Index,
    y: *const std::os::raw::c_void,
) {
    let version = unsafe { *y.cast::<u64>() };
    let raw = unsafe { *x.cast::<u64>() };
    let vv = VersionedVector::from_raw(raw);
    if !vv.is_scalar() {
        // TODO
        println!("vector");
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
                    // any writer epoch.
                    let vec = VersionedVector::from_committed(v);
                    m.set(src, dst, vec);
                    multi_pair_count += 1;
                }
            }
        }
        debug_assert_eq!(multi_pair_count, total_tensors);

        // Backward matrix is rebuilt from `m` by the caller (`rebuild_backward`)
        // after decode, so leave it empty here. Decoded graphs start at
        // version 0, matching the epoch the entries were registered with.
        Ok(Self {
            m,
            mt: VersionedMatrix::<bool>::new(0, 0),
            epoch: 0,
            count,
            multi_pair_count,
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
                let vv = self
                    .t
                    .m
                    .get(src, dest)
                    .unwrap_or(VersionedVector::new_scalar(0));
                (src, dest, vv)
            }
        };
        self.src = src;
        self.dest = dest;

        if vv.is_scalar() {
            return Some((src, dest, vv.scalar()));
        }
        self.buf = vv.ids(self.t.epoch).collect();
        self.buf_pos = 1;
        self.buf.first().map(|&id| (src, dest, id))
    }
}
