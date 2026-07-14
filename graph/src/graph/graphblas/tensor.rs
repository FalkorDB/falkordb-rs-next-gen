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
//! ```text
//!   Tensor
//!     |
//!     |-- m  (forward adjacency)      src --> dst  (boolean, GraphBLAS)
//!     |-- mt (backward adjacency)     dst --> src  (boolean, GraphBLAS)
//!     |-- e  (edge-id store)          compound_key --> sorted edge ids (Rust)
//! ```
//!
//! The GraphBLAS matrices carry only the pair-level sparsity pattern: they
//! feed the batched F·A mxm traversal, relationship-matrix merging, and bulk
//! mask removal. All edge ids — including multi-edge overflow — live in the
//! copy-on-write [`EdgeStore`], keyed by [`compound_key`].
//!
//! ## Compound Key Encoding
//!
//! ```text
//!   key = (src << 32) | dst
//! ```
//!
//! Both node ids must fit in a `u32`. Edge ids are capped at `u32` by the
//! store (asserted on insert).
//!
//! ## Iteration
//!
//! [`Iter`] walks the forward (or backward) adjacency matrix and, for each
//! (src, dst) pair found, yields the pair's edge ids from the store in
//! ascending order.

use crate::graph::graphblas::edge_store::{EdgeIds, EdgeStore, PairState};

use super::{
    matrix::{Dup, Matrix},
    serialization::{Decode, Encode, Reader, Writer},
    vector::Vector,
    versioned_matrix::{self, VersionedMatrix},
};

/// Maximum GraphBLAS index value (2^60 - 1).
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

/// Pack a `(src, dst)` node-id pair into the compound key used by the
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

/// Edge storage for one relationship type.
///
/// The forward (`m`) and backward (`mt`) adjacency matrices are **BOOL** and
/// record only which `(src, dst)` pairs are connected. Edge ids live in the
/// pure-Rust [`EdgeStore`] (`e`), which shares its committed base across MVCC
/// versions via `Arc` and keeps per-version changes in a bounded delta.
///
/// Invariants:
/// - `m[s, d]` is present iff `e` has at least one id for `compound_key(s, d)`,
///   and `mt[d, s]` mirrors `m[s, d]`.
/// - ids within a pair are sorted ascending and duplicate-free.
/// - total edges == `e.edge_count()`.
pub struct Tensor {
    /// Forward adjacency (src → dst), pair structure only.
    m: VersionedMatrix<bool>,
    /// Backward adjacency (dst → src), pair structure only.
    mt: VersionedMatrix<bool>,
    /// Edge-id storage keyed by `compound_key(src, dst)`.
    e: EdgeStore,
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
            e: EdgeStore::new(),
        }
    }

    /// Edge ids for the `(src, dest)` pair, in ascending edge-id order.
    /// Returns an owned iterator (borrows nothing).
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> std::vec::IntoIter<u64> {
        self.e
            .ids(compound_key(src, dest))
            .map_or_else(Vec::new, |ids| ids.iter().collect::<Vec<u64>>())
            .into_iter()
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        if self.e.add(compound_key(src, dest), id) {
            // First edge for this pair: record the pair in the forward and
            // backward adjacency.
            self.m.set(src, dest, true);
            self.mt.set(dest, src, true);
        }
    }

    /// Set entries from parallel slices. The store detects the first edge of
    /// each pair in O(1), so only new pairs are batched into the adjacency
    /// matrices (no per-edge GraphBLAS sync).
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

        let mut m_srcs: Vec<u64> = Vec::with_capacity(srcs.len());
        let mut m_dsts: Vec<u64> = Vec::with_capacity(srcs.len());
        for ((&s, &d), &id) in srcs.iter().zip(dsts.iter()).zip(ids.iter()) {
            if self.e.add(compound_key(s, d), id) {
                m_srcs.push(s);
                m_dsts.push(d);
            }
        }

        self.m
            .set_all(m_srcs.iter().zip(m_dsts.iter()).map(|(&s, &d)| (s, d)));
        self.mt
            .set_all(m_dsts.iter().zip(m_srcs.iter()).map(|(&d, &s)| (d, s)));
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

        let mut emptied = Vec::new();
        for &(id, src, dst) in rels {
            if self.e.remove(compound_key(src, dst), id) == PairState::Emptied {
                emptied.push((src, dst));
            }
        }
        if emptied.is_empty() {
            return emptied;
        }

        // Bulk-remove the emptied pairs from the forward/backward adjacency
        // in two GraphBLAS mask operations.
        let nrows = self.m.nrows();
        let ncols = self.m.ncols();
        let mut m_rows = Vec::with_capacity(emptied.len());
        let mut m_cols = Vec::with_capacity(emptied.len());
        let mut mt_rows = Vec::with_capacity(emptied.len());
        let mut mt_cols = Vec::with_capacity(emptied.len());
        for &(src, dst) in &emptied {
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
    /// The forward matrix's *effective* structure (`(m − dm) ∪ dp`) is
    /// materialized first, then transposed into a clean base with empty
    /// deltas.
    pub fn rebuild_backward(&mut self) {
        self.mt = VersionedMatrix::from_matrix(self.m.extract().transpose());
    }

    #[must_use]
    pub fn dup(&self) -> Self {
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            e: self.e.dup(),
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

    /// Edge-id store keyed by `compound_key(src, dst)`.
    #[must_use]
    pub const fn edge_store(&self) -> &EdgeStore {
        &self.e
    }

    /// Total number of edges.
    #[must_use]
    pub const fn edge_count(&self) -> u64 {
        self.e.edge_count()
    }

    /// Iterate every `(src, dst, edge_id)` triple in the tensor. Pair order
    /// is arbitrary; ids within a pair are ascending.
    pub fn iter_edges(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.e.iter_edges()
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
        self.e.multi_pairs() != 0
    }

    pub fn wait(&mut self) {
        self.m.wait();
        self.mt.wait();
        self.e.fold();
    }

    /// Wait on all matrices for fork safety (takes &self, not &mut self).
    pub fn wait_all(&self) {
        self.m.wait_all();
        self.mt.wait_all();
    }

    /// Fold the edge store's delta into its shared base when possible.
    /// Called at commit points; a no-op while an older version still shares
    /// the base.
    pub fn fold_edge_store(&mut self) {
        self.e.fold();
    }

    /// Returns true if every internal matrix has no pending GraphBLAS
    /// operations queued.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.m.is_synced() && self.mt.is_synced()
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.m.memory_usage() + self.mt.memory_usage() + self.e.memory_usage()
    }
}

/// MSB flag used by C FalkorDB to indicate multi-edge entries in the
/// UINT64 forward matrix.
const MSB_MASK: u64 = 1u64 << 63;

impl Encode<19> for Tensor {
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        let nrows = self.m.nrows();
        let ncols = self.m.ncols();

        // Serialize the C-compatible UINT64 forward matrix. Single-edge pairs
        // store the edge id directly; multi-edge pairs store
        // `(edge_count | MSB)` and push their full id list into the tensor
        // section below.
        let mut f_rows: Vec<u64> = Vec::new();
        let mut f_cols: Vec<u64> = Vec::new();
        let mut f_vals: Vec<u64> = Vec::new();
        let mut multi: Vec<(u64, u64, Vec<u64>)> = Vec::new();
        for (src, dst) in self.m.iter(0, u64::MAX) {
            let Some(ids) = self.e.ids(compound_key(src, dst)) else {
                debug_assert!(false, "Tensor adjacency/store mismatch at ({src}, {dst})");
                continue;
            };
            f_rows.push(src);
            f_cols.push(dst);
            match ids {
                EdgeIds::One(id) => f_vals.push(u64::from(*id)),
                EdgeIds::Many(_) => {
                    let ids: Vec<u64> = ids.iter().collect();
                    f_vals.push(ids.len() as u64 | MSB_MASK);
                    multi.push((src, dst, ids));
                }
            }
        }

        // Forward VersionedMatrix layout: base (effective), empty delta-plus,
        // empty delta-minus. Folding dp into the base keeps the on-disk form
        // canonical and matches what decode expects.
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

        let total = self.e.edge_count();
        w.write_unsigned(total);
        if total == 0 {
            return;
        }

        // Tensor section: two groups (base TM, delta-plus TDP). All multi-edge
        // pairs live in the base group; the delta-plus group is empty since
        // the forward matrix above is already the effective state.
        let mut v = Vector::<u64>::new(GrB_INDEX_MAX);
        w.write_unsigned(multi.len() as u64);
        for (src, dst, ids) in &multi {
            v.clear();
            for (idx, &edge_id) in ids.iter().enumerate() {
                v.set(idx as u64, edge_id);
            }
            w.write_unsigned(*src);
            w.write_unsigned(*dst);
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
        // directly (MSB clear) and `(count | MSB)` for multi-edge pairs,
        // whose real id lists follow in the tensor section. The bool pair
        // structure lands in `m`; every id lands in the store.
        let mut m = VersionedMatrix::<bool>::new(nrows, ncols);
        let mut e = EdgeStore::new();

        for (src, dst, value) in forward.iter(0, u64::MAX) {
            m.set(src, dst, true);
            if value & MSB_MASK == 0 {
                e.add(compound_key(src, dst), value);
            }
            // Multi-edge (MSB set): ids are supplied by the tensor section.
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
                    e.add_many(compound_key(src, dst), v.iter().map(|(_, id)| id));
                }
            }
        }

        // Backward matrix is rebuilt from `m` by the caller (`rebuild_backward`)
        // after decode, so leave it empty here.
        let backward = VersionedMatrix::<bool>::new(0, 0);
        Ok(Self { m, mt: backward, e })
    }
}

pub struct Iter<'a> {
    t: &'a Tensor,
    base: versioned_matrix::Iter,
    transpose: bool,
    src: u64,
    dest: u64,
    /// Remaining (ascending) ids of the current multi-edge pair, borrowed
    /// straight from the store.
    rest: &'a [u32],
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
                t.mt.iter(min_row, max_row)
            } else {
                t.m.iter(min_row, max_row)
            },
            transpose,
            src: 0,
            dest: 0,
            rest: &[],
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        // Drain buffered (sorted) ids for the current multi-edge pair.
        if let Some((&id, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Some((self.src, self.dest, u64::from(id)));
        }

        loop {
            // Next base pair, oriented as (src, dest).
            let (row, col) = self.base.next()?;
            let (src, dest) = if self.transpose {
                (col, row)
            } else {
                (row, col)
            };
            let Some(ids) = self.t.e.ids(compound_key(src, dest)) else {
                debug_assert!(false, "Tensor adjacency/store mismatch at ({src}, {dest})");
                continue;
            };
            self.src = src;
            self.dest = dest;
            return match ids {
                EdgeIds::One(id) => Some((src, dest, u64::from(*id))),
                EdgeIds::Many(v) => {
                    let (&first, rest) = v.split_first().expect("Many is never empty");
                    self.rest = rest;
                    Some((src, dest, u64::from(first)))
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::graph::graphblas::test_init::ensure_init;

    /// In-memory Writer/Reader pair: encode records typed tokens, decode
    /// replays them in order, so a roundtrip needs no Redis IO.
    #[derive(Default)]
    struct MemIo {
        unsigned: VecDeque<u64>,
        signed: VecDeque<i64>,
        doubles: VecDeque<f64>,
        buffers: VecDeque<Vec<u8>>,
    }

    impl Writer for MemIo {
        fn write_unsigned(
            &mut self,
            val: u64,
        ) {
            self.unsigned.push_back(val);
        }
        fn write_signed(
            &mut self,
            val: i64,
        ) {
            self.signed.push_back(val);
        }
        fn write_double(
            &mut self,
            val: f64,
        ) {
            self.doubles.push_back(val);
        }
        fn write_buffer(
            &mut self,
            data: &[u8],
        ) {
            self.buffers.push_back(data.to_vec());
        }
    }

    impl Reader for MemIo {
        fn read_unsigned(&mut self) -> Result<u64, String> {
            self.unsigned
                .pop_front()
                .ok_or_else(|| "mem: no more unsigned".to_string())
        }
        fn read_signed(&mut self) -> Result<i64, String> {
            self.signed
                .pop_front()
                .ok_or_else(|| "mem: no more signed".to_string())
        }
        fn read_double(&mut self) -> Result<f64, String> {
            self.doubles
                .pop_front()
                .ok_or_else(|| "mem: no more doubles".to_string())
        }
        fn read_buffer(&mut self) -> Result<Vec<u8>, String> {
            self.buffers
                .pop_front()
                .ok_or_else(|| "mem: no more buffers".to_string())
        }
    }

    fn sorted_triples(t: &Tensor) -> Vec<(u64, u64, u64)> {
        let mut v: Vec<_> = t.iter_edges().collect();
        v.sort_unstable();
        v
    }

    #[test]
    fn set_get_remove() {
        ensure_init();
        let mut t = Tensor::new(16, 16);
        t.set(0, 1, 9);
        t.set(0, 1, 5);
        t.set(2, 1, 7);

        assert_eq!(t.edge_count(), 3);
        assert!(t.has_multi_edge());
        assert_eq!(t.get(0, 1).collect::<Vec<_>>(), vec![5, 9]);
        assert_eq!(t.get(2, 1).collect::<Vec<_>>(), vec![7]);
        assert_eq!(t.get(1, 0).collect::<Vec<_>>(), Vec::<u64>::new());

        // Removing one edge of a multi pair keeps the pair alive.
        let emptied = t.remove_all(&[(5, 0, 1)]);
        assert!(emptied.is_empty());
        assert_eq!(t.edge_count(), 2);
        assert!(!t.has_multi_edge());

        // Removing the last edge empties the pair and clears the adjacency.
        let emptied = t.remove_all(&[(9, 0, 1)]);
        assert_eq!(emptied, vec![(0, 1)]);
        assert_eq!(t.matrix().iter(0, u64::MAX).count(), 1);
        assert_eq!(t.matrix_t().iter(0, u64::MAX).count(), 1);
    }

    #[test]
    fn iter_forward_and_transposed() {
        ensure_init();
        let mut t = Tensor::new(16, 16);
        t.set(1, 2, 10);
        t.set(1, 2, 3);
        t.set(4, 2, 8);

        let fwd: Vec<_> = t.iter(0, u64::MAX, false).collect();
        assert_eq!(fwd, vec![(1, 2, 3), (1, 2, 10), (4, 2, 8)]);

        // Backward iteration is dst-major but still yields (src, dst, id).
        let mut bwd: Vec<_> = t.iter(0, u64::MAX, true).collect();
        bwd.sort_unstable();
        assert_eq!(bwd, vec![(1, 2, 3), (1, 2, 10), (4, 2, 8)]);

        // Row-restricted forward iteration.
        let row4: Vec<_> = t.iter(4, 4, false).collect();
        assert_eq!(row4, vec![(4, 2, 8)]);
    }

    #[test]
    fn set_all_from_slices_batches_pairs() {
        ensure_init();
        let mut t = Tensor::new(16, 16);
        t.set_all_from_slices(&[0, 0, 3], &[1, 1, 4], &[7, 2, 5]);
        assert_eq!(t.edge_count(), 3);
        assert_eq!(t.get(0, 1).collect::<Vec<_>>(), vec![2, 7]);
        assert_eq!(t.get(3, 4).collect::<Vec<_>>(), vec![5]);
        assert_eq!(t.matrix().iter(0, u64::MAX).count(), 2);
    }

    #[test]
    fn dup_is_isolated() {
        ensure_init();
        let mut parent = Tensor::new(16, 16);
        parent.set(0, 1, 5);
        parent.wait();

        let mut child = parent.dup();
        child.set(0, 1, 6);
        child.remove_all(&[(5, 0, 1)]);

        assert_eq!(parent.get(0, 1).collect::<Vec<_>>(), vec![5]);
        assert_eq!(child.get(0, 1).collect::<Vec<_>>(), vec![6]);
    }

    #[test]
    fn encode_decode_roundtrip() {
        ensure_init();
        let mut t = Tensor::new(32, 32);
        t.set(0, 1, 5); // single
        t.set(2, 3, 6); // multi
        t.set(2, 3, 9);
        t.set(2, 3, 1);
        t.set(31, 0, 7); // single at edge of dimensions

        let mut io = MemIo::default();
        Encode::<19>::encode(&t, &mut io);
        let mut d = <Tensor as Decode<19>>::decode(&mut io).unwrap();
        d.rebuild_backward();

        assert_eq!(d.edge_count(), t.edge_count());
        assert!(d.has_multi_edge());
        assert_eq!(sorted_triples(&d), sorted_triples(&t));
        assert_eq!(d.get(2, 3).collect::<Vec<_>>(), vec![1, 6, 9]);

        let mut bwd: Vec<_> = d.iter(0, u64::MAX, true).collect();
        bwd.sort_unstable();
        assert_eq!(bwd, sorted_triples(&t));
    }

    #[test]
    fn encode_decode_empty() {
        ensure_init();
        let t = Tensor::new(8, 8);
        let mut io = MemIo::default();
        Encode::<19>::encode(&t, &mut io);
        let d = <Tensor as Decode<19>>::decode(&mut io).unwrap();
        assert_eq!(d.edge_count(), 0);
        assert!(!d.has_multi_edge());
    }
}
