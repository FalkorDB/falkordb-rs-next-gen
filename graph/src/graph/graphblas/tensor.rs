//! 3D sparse tensor for multi-edge relationship storage.
//!
//! This module provides [`Tensor`], which extends the adjacency matrix model
//! to support multiple edges of the same type between the same pair of nodes.
//! While a plain adjacency matrix can only record whether an edge exists,
//! the tensor stores individual edge IDs so that each edge can carry its own
//! properties.
//!
//! ## Internal Structure (decoupled representation)
//!
//! ```text
//!   Tensor
//!     |-- m   (forward adjacency)   src --> dst   VersionedMatrix<bool>
//!     |-- mt  (backward adjacency)  dst --> src   VersionedMatrix<bool>
//!     |-- ids (edge-id store)       compound_key(src,dst) -> {edge ids}
//! ```
//!
//! The adjacency matrices `m`/`mt` are **structure only** (`bool`) — they carry
//! no edge ids. Traversal (`mxm`) uses the `GxB_ANY_PAIR_BOOL` semiring, so it
//! only ever needs the sparsity pattern. **All** edge ids live in the native
//! copy-on-write [`EdgeIdStore`], keyed by `compound_key(src, dst)`.
//!
//! This keeps a single uniform MVCC model (the `bool` `VersionedMatrix`) for the
//! GraphBLAS matrices and removes the in-place-value-update machinery (promotion,
//! order-sensitive flush) that an inline-edge-id `UINT64` forward matrix requires.
//!
//! The one consumer that needs edge ids inside GraphBLAS — weighted `algo.MSF` —
//! rebuilds an ephemeral `UINT64` forward matrix and `bool` overflow matrix on
//! demand in one store walk (`build_msf_matrices`, colocated with MSF in
//! `runtime::functions::algo_procedures`).
//!
//! ## Compound Key Encoding
//!
//! ```text
//!   key = (src << 32) | dst      (both must fit in u32)
//! ```

use itertools::Either;
use rustc_hash::FxHashSet;

use super::{
    edge_id_store::EdgeIdStore,
    matrix::{Dup, Matrix},
    serialization::{Decode, Encode, Reader, Writer},
    vector::Vector,
    versioned_matrix::VersionedMatrix,
};

/// Maximum GraphBLAS index value (2^60 - 1).
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

const COL_MASK: u64 = 0xFFFF_FFFF;

/// Pack a `(src, dst)` node-id pair into the compound row key used by the
/// edge-id store.
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

/// Split a `compound_key` back into `(src, dst)`.
#[inline]
pub(crate) const fn split_key(key: u64) -> (u64, u64) {
    (key >> 32, key & COL_MASK)
}

/// Edge storage for one relationship type: bool adjacency + a native edge-id
/// store.
pub struct Tensor {
    /// Forward adjacency (src → dst), structure only.
    m: VersionedMatrix<bool>,
    /// Backward adjacency (dst → src), structure only.
    mt: VersionedMatrix<bool>,
    /// `compound_key(src, dst) → {edge ids}` — the sole source of truth for ids.
    ids: EdgeIdStore,
}

impl Tensor {
    #[must_use]
    pub fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: VersionedMatrix::<bool>::new(nrows, ncols),
            mt: VersionedMatrix::<bool>::new(ncols, nrows),
            ids: EdgeIdStore::new(),
        }
    }

    /// Edge ids for the `(src, dest)` pair, in ascending edge-id order. Returns
    /// a lazy owned cursor (borrows nothing, allocates nothing) over a snapshot
    /// of the store.
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> impl Iterator<Item = u64> + use<> {
        self.ids.ids_iter(compound_key(src, dest))
    }

    /// The smallest edge id for `(src, dest)`, or `None` if the pair has no edge.
    /// Cheap single descent (no cursor) — for the "one representative edge" hot
    /// path; callers needing all ids or a specific one use [`get`](Self::get).
    #[must_use]
    pub fn first_edge(
        &self,
        src: u64,
        dest: u64,
    ) -> Option<u64> {
        self.ids.first_id(compound_key(src, dest))
    }

    /// Insert edge `id` for `(src, dest)`. Structure flips on the 0→1
    /// transition; the id always lands in the store.
    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        if self.m.get(src, dest).is_none() {
            self.m.set(src, dest, true);
            self.mt.set(dest, src, true);
        }
        self.ids.set(compound_key(src, dest), id);
    }

    /// Bulk insert from parallel slices. The forward/backward structure gets only
    /// genuinely-new pairs (the store absorbs multiplicity). A pair is new iff it
    /// isn't already persisted (`pair_nonempty`, an O(log N) store point lookup on
    /// the pre-batch state) and hasn't been seen earlier in this batch (the
    /// batch-local `m_new` set). This avoids rebuilding the whole present-set —
    /// O(N) per call — which dominated incremental writes on large graphs.
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
        // New (src, dst) pairs this batch — bounded by the batch size, not the
        // graph. Doubles as the set_all source (order-independent: each set is
        // an independent write).
        let mut m_new: FxHashSet<(u64, u64)> = FxHashSet::default();
        let mut store_batch: Vec<(u64, u64)> = Vec::with_capacity(srcs.len());
        for ((&s, &d), &id) in srcs.iter().zip(dsts.iter()).zip(ids.iter()) {
            let key = compound_key(s, d);
            store_batch.push((key, id));
            if !m_new.contains(&(s, d)) && !self.ids.pair_nonempty(key) {
                m_new.insert((s, d));
            }
        }
        self.ids.insert_batch(&store_batch);
        self.m.set_all(m_new.iter().copied());
        self.mt.set_all(m_new.into_iter().map(|(s, d)| (d, s)));
    }

    /// Bulk-remove specific edges. Each entry is `(edge_id, src, dst)`. Returns
    /// the `(src, dst)` pairs that became completely empty. A pure store drop +
    /// structure clear on emptied pairs — no overflow-into-inline promotion.
    pub fn remove_all(
        &mut self,
        rels: &[(u64, u64, u64)],
    ) -> Vec<(u64, u64)> {
        if rels.is_empty() {
            return Vec::new();
        }
        let mut store_batch: Vec<(u64, u64)> = rels
            .iter()
            .map(|&(id, src, dst)| (compound_key(src, dst), id))
            .collect();
        store_batch.sort_unstable();
        self.ids.remove_batch(&store_batch);
        let touched: FxHashSet<(u64, u64)> = rels.iter().map(|&(_, s, d)| (s, d)).collect();
        let mut emptied = Vec::new();
        for (s, d) in touched {
            if !self.ids.pair_nonempty(compound_key(s, d)) {
                self.m.remove(s, d);
                self.mt.remove(d, s);
                emptied.push((s, d));
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

    /// Rebuild the backward matrix as the layerwise transpose of the forward
    /// matrix. Both are pure bool, so the three MVCC layers transpose
    /// independently.
    pub fn rebuild_backward(&mut self) {
        self.mt = self.m.transpose();
    }

    #[must_use]
    pub fn dup(&self) -> Self {
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            ids: self.ids.dup(),
        }
    }

    /// Forward pair-level adjacency (src → dst), structure only.
    #[must_use]
    pub const fn matrix(&self) -> &VersionedMatrix<bool> {
        &self.m
    }

    /// Transposed/backward pair-level adjacency (dst → src), structure only.
    #[must_use]
    pub const fn matrix_t(&self) -> &VersionedMatrix<bool> {
        &self.mt
    }

    /// The native edge-id store (`compound_key(src,dst) → {edge ids}`).
    #[must_use]
    pub const fn store(&self) -> &EdgeIdStore {
        &self.ids
    }

    /// Total number of edges (inline first edges plus multi-edge overflow).
    #[must_use]
    pub fn edge_count(&self) -> u64 {
        self.ids.nvals()
    }

    /// Iterate every `(src, dst, edge_id)` triple, ascending by `(src, dst, id)`.
    /// Lazy: a streaming pass over the store, no allocation, droppable mid-scan.
    pub fn iter_edges(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.ids.range_iter(0, u64::MAX).map(|(k, id)| {
            let (s, d) = split_key(k);
            (s, d, id)
        })
    }

    /// Lazily iterate `(src, dst, edge_id)` triples over a row range. Forward:
    /// a streaming merge over the store's contiguous `src`→`compound_key` range
    /// (structure + ids from one pass, no per-pair lookup). Backward: walk bool
    /// `mt` (dst-major) and stream each pair's ids from the store.
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        if transpose {
            Either::Right(self.mt.iter(min_row, max_row).flat_map(move |(dst, src)| {
                self.ids
                    .ids_iter(compound_key(src, dst))
                    .map(move |id| (src, dst, id))
            }))
        } else {
            // A row > COL_MASK can't fit the 32-bit src half, so no key exists at
            // or above it — clamp to u64::MAX (empty lower bound). Mirrors the
            // max_key guard; `min_row << 32` would otherwise truncate the high
            // bits and select a wrong range.
            let min_key = if min_row > COL_MASK {
                u64::MAX
            } else {
                min_row << 32
            };
            let max_key = if max_row >= COL_MASK {
                u64::MAX
            } else {
                (max_row << 32) | COL_MASK
            };
            Either::Left(self.ids.range_iter(min_key, max_key).map(|(k, id)| {
                let (s, d) = split_key(k);
                (s, d, id)
            }))
        }
    }

    /// Whether this tensor has any `(src, dst)` pair with more than one edge.
    /// `O(1)`: by the no-shadow invariant, `m`'s bool structure holds exactly one
    /// cell per pair with ≥1 live id, so `m.nvals()` is the distinct-pair count
    /// and `ids.nvals()` the total id count — a multi-edge exists iff the latter
    /// exceeds the former. No store scan.
    #[must_use]
    pub fn has_multi_edge(&self) -> bool {
        self.ids.nvals() > self.m.nvals()
    }

    pub fn wait(&mut self) {
        self.m.wait();
        self.mt.wait();
    }

    /// Wait on all matrices for fork safety (takes &self, not &mut self). The
    /// store is plain Rust memory with no GraphBLAS pending state.
    pub fn wait_all(&self) {
        self.m.wait_all();
        self.mt.wait_all();
    }

    /// Returns true if every internal GraphBLAS matrix has no pending
    /// operations queued.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.m.is_synced() && self.mt.is_synced()
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.m.memory_usage() + self.mt.memory_usage() + self.ids.memory_usage()
    }
}

#[cfg(test)]
mod repro_tests {
    use super::*;
    use crate::graph::graphblas::test_init::ensure_init;

    #[test]
    fn repro_engine_pattern() {
        ensure_init();
        let mut t = Tensor::new(8, 8);
        // a=0,b=1,c=2; edges (0,1,100),(0,1,101),(1,2,102),(0,2,103)
        t.set_all_from_slices(&[0, 0, 1, 0], &[1, 1, 2, 2], &[100, 101, 102, 103]);
        let mut e: Vec<_> = t.iter_edges().collect();
        e.sort_unstable();
        assert_eq!(e, vec![(0, 1, 100), (0, 1, 101), (0, 2, 103), (1, 2, 102)]);
        assert_eq!(t.get(0, 1).collect::<Vec<_>>(), vec![100, 101]);
        assert_eq!(t.get(0, 2).collect::<Vec<_>>(), vec![103]);
        assert_eq!(t.edge_count(), 4);
        // O(1) multi-edge: 4 ids over 3 distinct pairs ⇒ true.
        assert_eq!(t.matrix().nvals(), 3);
        assert!(t.has_multi_edge());
        // A single-edge tensor: ids == pairs ⇒ false.
        let mut single = Tensor::new(8, 8);
        single.set_all_from_slices(&[0, 1], &[1, 2], &[10, 11]);
        assert!(!single.has_multi_edge());
        assert_eq!(
            t.iter(0, u64::MAX, false).collect::<Vec<_>>(),
            vec![(0, 1, 100), (0, 1, 101), (0, 2, 103), (1, 2, 102)]
        );
        // dup + mutate (MVCC), old snapshot unchanged
        let mut t2 = t.dup();
        t2.set(2, 0, 200);
        assert_eq!(t.edge_count(), 4);
        assert_eq!(t2.edge_count(), 5);
    }
}

/// MSB flag used by C FalkorDB to indicate multi-edge entries in the
/// UINT64 forward matrix.
const MSB_MASK: u64 = 1u64 << 63;

impl Encode<19> for Tensor {
    #[allow(clippy::similar_names)]
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        let nrows = self.m.nrows();
        let ncols = self.m.ncols();

        // Build the C-compatible UINT64 forward matrix from the store: single-
        // edge pairs store the edge id directly; multi-edge pairs store
        // `(edge_count | MSB)` and push their full ascending id list into the
        // tensor section below.
        let pairs = self.ids.all_pairs(); // sorted by (key, id)
        let mut f_rows: Vec<u64> = Vec::new();
        let mut f_cols: Vec<u64> = Vec::new();
        let mut f_vals: Vec<u64> = Vec::new();
        // Each multi-edge pair is a `[start, end)` slice of `pairs` (already
        // materialized, ascending by id) — no per-pair Vec allocation.
        let mut multi: Vec<(u64, u64, usize, usize)> = Vec::new();

        let mut i = 0;
        while i < pairs.len() {
            let key = pairs[i].0;
            let start = i;
            while i < pairs.len() && pairs[i].0 == key {
                i += 1;
            }
            let (src, dst) = split_key(key);
            f_rows.push(src);
            f_cols.push(dst);
            let count = i - start;
            if count == 1 {
                f_vals.push(pairs[start].1);
            } else {
                f_vals.push(count as u64 | MSB_MASK);
                multi.push((src, dst, start, i));
            }
        }

        // Forward VersionedMatrix layout: base (effective), empty delta-plus,
        // empty delta-minus.
        let empty = Matrix::<u64>::new(nrows, ncols);
        if f_rows.is_empty() {
            empty.encode(w);
        } else {
            let mut fm = Matrix::<u64>::new(nrows, ncols);
            fm.build(&f_rows, &f_cols, &f_vals);
            fm.encode(w);
        }
        empty.encode(w); // delta-plus
        empty.encode(w); // delta-minus

        let total = self.ids.nvals();
        w.write_unsigned(total);
        if total == 0 {
            return;
        }

        // Tensor section: two groups (base TM, delta-plus TDP). All multi-edge
        // pairs live in the base group; the delta-plus group is empty.
        let mut v = Vector::<u64>::new(GrB_INDEX_MAX);
        w.write_unsigned(multi.len() as u64);
        for &(src, dst, start, end) in &multi {
            v.clear();
            for (idx, &(_, edge_id)) in pairs[start..end].iter().enumerate() {
                v.set(idx as u64, edge_id);
            }
            w.write_unsigned(src);
            w.write_unsigned(dst);
            v.encode(w);
        }
        w.write_unsigned(0); // empty delta-plus tensor group
    }
}

impl Decode<19> for Tensor {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let forward = VersionedMatrix::<u64>::decode(r)?;
        let nrows = forward.nrows();
        let ncols = forward.ncols();

        // The on-disk forward matrix (C-compatible) stores single-edge ids
        // directly (MSB clear) and `(count | MSB)` for multi-edge pairs, whose
        // real id lists follow in the tensor section.
        let mut m = VersionedMatrix::<bool>::new(nrows, ncols);
        let mut store_pairs: Vec<(u64, u64)> = Vec::new();

        for (src, dst, value) in forward.iter(0, u64::MAX) {
            m.set(src, dst, true);
            if value & MSB_MASK == 0 {
                // Single edge: value is the edge id.
                store_pairs.push((compound_key(src, dst), value));
            }
            // Multi-edge (MSB set): ids come from the tensor section below.
        }

        let total_tensor_count = r.read_unsigned()?;
        if total_tensor_count > 0 {
            // Two groups: base (TM) then delta-plus (TDP).
            for _ in 0..2 {
                let count = r.read_unsigned()?;
                for _ in 0..count {
                    let src = r.read_unsigned()?;
                    let dst = r.read_unsigned()?;
                    let v = Vector::<u64>::decode(r)?;
                    m.set(src, dst, true);
                    let key = compound_key(src, dst);
                    for (_, edge_id) in v.iter() {
                        store_pairs.push((key, edge_id));
                    }
                }
            }
        }

        let ids = EdgeIdStore::from_pairs(store_pairs);
        // Backward matrix is rebuilt from `m` by the caller (`rebuild_backward`)
        // after decode, so leave it empty here.
        let mt = VersionedMatrix::<bool>::new(0, 0);
        Ok(Self { m, mt, ids })
    }
}
