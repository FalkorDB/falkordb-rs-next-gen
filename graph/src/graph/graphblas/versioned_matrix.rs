//! Copy-on-write sparse matrix with MVCC delta tracking.
//!
//! This module provides [`VersionedMatrixT`], which wraps a base [`Matrix`] with
//! two delta matrices to track pending additions and deletions. This is the
//! building block for snapshot isolation: readers see the committed base state
//! while writers accumulate changes in separate delta matrices.
//!
//! ## Cell type
//!
//! The matrix is generic over its cell-value type [`CellValue`]:
//!
//! - [`VersionedMatrix`] = `VersionedMatrixT<bool>` — a **presence** matrix. This
//!   is the engine-wide default (adjacency, label, and tensor matrices, and the
//!   node index) and the only instantiation the rest of the engine uses, so its
//!   API is unchanged.
//! - `VersionedMatrixT<u64>` — a **value-carrying** matrix. The edge index uses
//!   it to pack each edge's `(src, dst)` endpoints into the cell value, so an
//!   edge-index scan yields endpoints inline (no separate resolution structure).
//!
//! The delta-minus (deletion mask) is always `BOOL` regardless of `V` — it marks
//! *which* cells are deleted, never a value.
//!
//! ## Internal Structure
//!
//! ```text
//!   VersionedMatrixT<V>
//!     |
//!     |-- m   Cow<Matrix>   Base matrix (committed / shared with readers)
//!     |-- dp  Cow<Matrix>   Delta-plus  (pending additions)
//!     |-- dm  Cow<Matrix>   Delta-minus (pending deletions, BOOL)
//!
//!   Effective state = (m UNION dp) MINUS dm
//! ```
//!
//! Each inner matrix is wrapped in [`Cow`] (copy-on-write). When a new version
//! is created via [`Dup`], the `Cow` clones share the underlying `Arc<Matrix>`
//! until a mutation triggers a deep copy.
//!
//! ## Flush
//!
//! When delta matrices exceed 10,000 entries, [`flush`](VersionedMatrixT::flush)
//! merges them into the base matrix (`dp` via [`CellValue::fold`] — a
//! value-preserving union — `dm` via masked removal) and clears the deltas.
//!
//! ## Iterators
//!
//! - [`Iter`] (from [`iter`](VersionedMatrixT::iter)) — the **scan** iterator:
//!   chains the base matrix iterator (skipping entries present in `dm`) with the
//!   delta-plus iterator, yielding `(row, col)`. Reads no cell value, so it works
//!   for any `V`.
//! - [`ValueIter`] (from [`iter_values`](VersionedMatrixT::iter_values), `u64`
//!   only) — yields `(row, col, value)` for value-carrying matrices.

use std::marker::PhantomData;

use super::{
    GxB_Print_Level,
    matrix::{
        self, Dup, Get, MaskedElementWiseAdd, Matrix, New, Remove, Set, Size, Transpose,
        Uint64Extract,
    },
    serialization::{Decode, Encode, Reader, Writer},
};
use crate::graph::cow::Cow;

/// The cell-value type of a [`VersionedMatrixT`]. Abstracts the few
/// GraphBLAS-type-specific operations (base/delta-plus matrix constructor,
/// element set, and the value-preserving delta-plus fold) so one MVCC
/// matrix core serves both presence (`bool`) and value-carrying (`u64`) matrices.
pub trait CellValue: Copy + 'static {
    /// Construct an empty base / delta-plus matrix of this cell type.
    fn new_matrix(
        nrows: u64,
        ncols: u64,
    ) -> Matrix;
    /// Set `(i, j) = value` in a base / delta-plus matrix of this cell type.
    fn set(
        m: &mut Matrix,
        i: u64,
        j: u64,
        value: Self,
    );
    /// Fold `base = base ∪ delta_plus`, preserving values. `base` and `dp` never
    /// share a cell (a `set` only writes `dp` when `base` lacks the cell), so
    /// this is a pure value-preserving copy of `dp`'s entries into `base`.
    fn fold(
        base: &mut Matrix,
        dp: &Matrix,
    );
}

impl CellValue for bool {
    fn new_matrix(
        nrows: u64,
        ncols: u64,
    ) -> Matrix {
        Matrix::new(nrows, ncols)
    }

    fn set(
        m: &mut Matrix,
        i: u64,
        j: u64,
        value: bool,
    ) {
        m.set(i, j, value);
    }

    fn fold(
        base: &mut Matrix,
        dp: &Matrix,
    ) {
        // PAIR/BOOL union — the engine's long-standing presence fold.
        base.element_wise_add(None, None, Some(dp), None);
    }
}

impl CellValue for u64 {
    fn new_matrix(
        nrows: u64,
        ncols: u64,
    ) -> Matrix {
        Matrix::new_uint64(nrows, ncols)
    }

    fn set(
        m: &mut Matrix,
        i: u64,
        j: u64,
        value: u64,
    ) {
        m.set_uint64(i, j, value);
    }

    fn fold(
        base: &mut Matrix,
        dp: &Matrix,
    ) {
        // Value-preserving union (ANY/SECOND over UINT64) — see
        // `Matrix::element_wise_add_uint64`.
        base.element_wise_add_uint64(dp);
    }
}

/// A matrix with MVCC delta tracking for snapshot isolation, generic over the
/// cell-value type `V`. See the module docs; [`VersionedMatrix`] is the `bool`
/// alias used throughout the engine.
#[derive(Clone)]
pub struct VersionedMatrixT<V: CellValue = bool> {
    /// Base committed matrix (cell type `V`).
    m: Cow<Matrix>,
    /// Delta-plus: entries added in the current transaction (cell type `V`).
    dp: Cow<Matrix>,
    /// Delta-minus: entries removed in the current transaction (always `BOOL`).
    dm: Cow<Matrix>,
    _v: PhantomData<V>,
}

/// The engine-wide presence matrix: adjacency, label, tensor, and node-index
/// matrices. This is the only instantiation the engine outside the index uses.
pub type VersionedMatrix = VersionedMatrixT<bool>;

unsafe impl<V: CellValue> Send for VersionedMatrixT<V> {}
unsafe impl<V: CellValue> Sync for VersionedMatrixT<V> {}

impl<V: CellValue> Size for VersionedMatrixT<V> {
    fn nrows(&self) -> u64 {
        self.m.nrows()
    }

    fn ncols(&self) -> u64 {
        self.m.ncols()
    }

    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.wait();
        self.m.resize(nrows, ncols);
        self.dp.resize(nrows, ncols);
        self.dm.resize(nrows, ncols);
    }

    fn nvals(&self) -> u64 {
        self.wait();
        self.m.nvals() + self.dp.nvals() - self.dm.nvals()
    }
}

impl<V: CellValue> New for VersionedMatrixT<V> {
    fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: Cow::new(V::new_matrix(nrows, ncols)),
            dp: Cow::new(V::new_matrix(nrows, ncols)),
            // Deletion mask is structural: always BOOL.
            dm: Cow::new(Matrix::new(nrows, ncols)),
            _v: PhantomData,
        }
    }
}

impl<V: CellValue> VersionedMatrixT<V> {
    #[must_use]
    pub fn m(&self) -> &Matrix {
        &self.m
    }

    #[must_use]
    pub fn dp(&self) -> &Matrix {
        &self.dp
    }

    #[must_use]
    pub fn dm(&self) -> &Matrix {
        &self.dm
    }

    /// Set `(i, j) = value`. If the cell is already in the committed base, this
    /// only un-deletes it (the index never rewrites an existing cell's value —
    /// updates tombstone the old cell and add a new one), so the base value is
    /// authoritative; otherwise the value lands in delta-plus.
    pub fn set_cell(
        &mut self,
        i: u64,
        j: u64,
        value: V,
    ) {
        debug_assert!(!self.m.pending());
        if self.m.get(i, j).is_some() {
            debug_assert!(self.dp.get(i, j).is_none());
            self.dm.remove(i, j);
        } else {
            debug_assert!(self.dm.get(i, j).is_none());
            V::set(&mut self.dp, i, j, value);
        }
    }

    pub fn flush(&mut self) {
        self.wait();
        if self.dp.nvals() >= 10000 {
            V::fold(&mut self.m, &self.dp);
            self.dp.clear();
        }
        if self.dm.nvals() >= 10000 {
            self.m.remove_all(&self.dm);
            self.dm.clear();
        }
    }

    pub fn wait(&self) {
        debug_assert!(!self.m.pending());
        self.dp.wait();
        self.dm.wait();
    }

    /// Wait on all three internal matrices (m, dp, dm).
    /// Used for fork safety — ensures no GrB internal locks are held.
    pub fn wait_all(&self) {
        self.m.wait();
        self.dp.wait();
        self.dm.wait();
    }

    /// Returns true if every internal matrix has no pending GraphBLAS
    /// operations — i.e. wait_all was effective.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.m.is_synced() && self.dp.is_synced() && self.dm.is_synced()
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.m.memory_usage() + self.dp.memory_usage() + self.dm.memory_usage()
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> Iter {
        self.wait();
        Iter::new(self, min_row, max_row)
    }

    pub fn print(
        &self,
        level: GxB_Print_Level,
    ) {
        self.m.print(level);
        self.dp.print(level);
        self.dm.print(level);
    }
}

impl<V: CellValue> Dup<Self> for VersionedMatrixT<V> {
    fn dup(&self) -> Self {
        Self {
            m: self.m.new_version(),
            dp: self.dp.new_version(),
            dm: self.dm.new_version(),
            _v: PhantomData,
        }
    }
}

impl<V: CellValue> Remove for VersionedMatrixT<V> {
    fn remove(
        &mut self,
        i: u64,
        j: u64,
    ) {
        if self.m.get(i, j).is_some() {
            debug_assert!(self.dp.get(i, j).is_none());
            self.dm.set(i, j, true);
        } else {
            self.dp.remove(i, j);
        }
    }
}

impl<V: CellValue> Get for VersionedMatrixT<V> {
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<bool> {
        self.wait();
        self.m.get(i, j).map_or_else(
            || self.dp.get(i, j),
            |value| {
                if self.dm.get(i, j).is_some() {
                    None
                } else {
                    Some(value)
                }
            },
        )
    }
}

// --- `bool`-only API used by the engine (adjacency / tensor / label matrices).
// These keep the presence-matrix surface unchanged; the alias `VersionedMatrix`
// resolves to `VersionedMatrixT<bool>`, so every existing caller is unaffected.

impl VersionedMatrix {
    /// Wrap an owned `Matrix` as a `VersionedMatrix` with empty delta-plus /
    /// delta-minus.  Used when callers materialize a merged matrix and then
    /// want to expose it through the versioned-matrix iter API without the
    /// dup overhead of re-building inside the versioned wrapper.
    #[must_use]
    pub fn from_matrix(m: Matrix) -> Self {
        let nrows = m.nrows();
        let ncols = m.ncols();
        Self {
            m: Cow::new(m),
            dp: Cow::new(Matrix::new(nrows, ncols)),
            dm: Cow::new(Matrix::new(nrows, ncols)),
            _v: PhantomData,
        }
    }

    #[must_use]
    pub fn to_matrix(&self) -> Matrix {
        self.wait();
        let mut m = self.m.dup();
        if self.dm.nvals() > 0 {
            m.remove_all(&self.dm);
        }
        if self.dp.nvals() > 0 {
            m.element_wise_add(None, None, Some(&self.dp), None);
        }
        m
    }

    #[must_use]
    pub fn extract_m_dp(&self) -> (Matrix, Matrix) {
        if self.dm.nvals() == 0 {
            // Fast path: no deletions, return dups of m and dp directly
            (self.m.dup(), self.dp.dup())
        } else {
            let mut m = Matrix::new(self.m.nrows(), self.m.ncols());
            let mut dp = Matrix::new(self.dp.nrows(), self.dp.ncols());
            m.select(&self.dm, &self.m);
            dp.select(&self.dm, &self.dp);
            (m, dp)
        }
    }

    /// Bulk-extract all effective entries as (row, col) arrays.
    ///
    /// Returns `(rows, cols)` from `(m - dm) ∪ dp`, avoiding iterator overhead
    /// on matrices with huge dimensions (e.g., GrB_INDEX_MAX).
    #[must_use]
    pub fn extract_all_tuples(&self) -> (Vec<u64>, Vec<u64>) {
        self.wait();
        if self.dm.nvals() == 0 {
            // Fast path: no deletions, just combine m and dp tuples
            let (mut rows_m, mut cols_m) = self.m.extract_tuples_bool();
            let (rows_dp, cols_dp) = self.dp.extract_tuples_bool();
            rows_m.extend_from_slice(&rows_dp);
            cols_m.extend_from_slice(&cols_dp);
            (rows_m, cols_m)
        } else {
            // Slow path: materialize effective matrix then extract
            let effective = self.to_matrix();
            effective.extract_tuples_bool()
        }
    }

    /// Bulk-extract tuples from base `m` and delta-plus `dp` separately.
    ///
    /// Returns `((m_rows, m_cols), (dp_rows, dp_cols))`.
    /// Only valid when `dm` is empty (asserted in debug builds).
    #[must_use]
    pub fn extract_m_dp_tuples(&self) -> ((Vec<u64>, Vec<u64>), (Vec<u64>, Vec<u64>)) {
        self.wait();
        debug_assert_eq!(self.dm.nvals(), 0, "extract_m_dp_tuples requires empty dm");
        let m_tuples = self.m.extract_tuples_bool();
        let dp_tuples = self.dp.extract_tuples_bool();
        (m_tuples, dp_tuples)
    }

    /// Bulk-remove all entries matching a mask matrix.
    ///
    /// Equivalent to calling `remove(i, j)` for every entry `(i, j)` in `mask`,
    /// but executes in two GraphBLAS bulk operations instead of N individual calls:
    /// - Entries in base `m` matching `mask` are marked deleted in `dm`
    /// - Entries in delta-plus `dp` matching `mask` are removed from `dp`
    pub fn remove_mask(
        &mut self,
        mask: &Matrix,
    ) {
        // dm |= (m & mask): for each entry in mask that exists in m, add to dm
        self.dm
            .element_wise_add(Some(&self.m), None, Some(mask), None);
        // dp &= ~mask: remove entries from dp that exist in mask
        self.dp.remove_all(mask);
    }

    /// Returns true if the base matrix has UINT64 element type.
    ///
    /// C-produced relation matrices store edge IDs as UINT64, while
    /// Rust-produced ones use BOOL.
    #[must_use]
    pub fn is_uint64(&self) -> bool {
        self.m.is_uint64()
    }

    /// Iterate UINT64 entries from the base M and delta-plus DP matrices.
    ///
    /// Used during RDB decode to read C-produced relation matrices where
    /// single-edge entries store the edge ID as a UINT64 value.
    /// Returns an empty iterator for Rust-produced BOOL matrices.
    pub fn uint64_iter(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.m.uint64_iter().chain(self.dp.uint64_iter())
    }

    /// Set multiple entries, checking dm emptiness once upfront.
    ///
    /// If dm is empty, uses the fast path (1 FFI call per entry).
    /// Otherwise falls back to the full `set` path (2+ FFI calls per entry).
    pub fn set_all(
        &mut self,
        entries: impl Iterator<Item = (u64, u64)>,
    ) {
        if self.dm.nvals() == 0 {
            for (i, j) in entries {
                self.dp.set(i, j, true);
            }
        } else {
            for (i, j) in entries {
                self.set_cell(i, j, true);
            }
        }
    }
}

impl Set for VersionedMatrix {
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: bool,
    ) {
        // Engine entry point (`.set(..)`); the typed core lives in `set_cell`.
        self.set_cell(i, j, value);
    }
}

impl Transpose for VersionedMatrix
where
    Self: New,
{
    /// Transposes the matrix.
    ///
    /// # Returns
    /// A new matrix that is the transpose of the original.
    fn transpose(&self) -> Self {
        Self {
            m: Cow::new(self.m.transpose()),
            dp: Cow::new(self.dp.transpose()),
            dm: Cow::new(self.dm.transpose()),
            _v: PhantomData,
        }
    }
}

impl Encode<19> for VersionedMatrix {
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        self.m.encode(w);
        self.dp.encode(w);
        self.dm.encode(w);
    }
}

impl Decode<19> for VersionedMatrix {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let m = Matrix::decode(r)?;
        let dp = Matrix::decode(r)?;
        let dm = Matrix::decode(r)?;
        Ok(Self {
            m: Cow::new(m),
            dp: Cow::new(dp),
            dm: Cow::new(dm),
            _v: PhantomData,
        })
    }
}

// --- `u64`-only API used by the edge index (value-carrying cells).

impl VersionedMatrixT<u64> {
    /// Iterate the effective `(row, col, value)` triples whose row is in
    /// `[min_row, max_row]`. The edge index uses this to read each matched
    /// edge's packed `(src, dst)` endpoints directly from the cell, so a scan
    /// yields endpoints inline with no separate resolution structure.
    #[must_use]
    pub fn iter_values(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> ValueIter {
        self.wait();
        ValueIter::new(self, min_row, max_row)
    }
}

/// Scan iterator: chains the base matrix iterator (skipping entries present in
/// `dm`) with the delta-plus iterator, yielding `(row, col)`. Reads no value, so
/// it is valid for any cell type.
pub struct Iter {
    mit: matrix::Iter,
    dpit: matrix::Iter,
    dm: Cow<Matrix>,
    /// True when both the deletion mask and the delta-plus matrix are empty,
    /// so iteration can stream `mit` without per-edge `dm.get` lookups or a
    /// `dpit` tail. Hot path for read-only queries on a freshly loaded graph.
    dm_empty: bool,
    dp_empty: bool,
}

unsafe impl Send for Iter {}
unsafe impl Sync for Iter {}

impl Iter {
    /// Creates a new scan iterator over `[min_row, max_row]`.
    #[must_use]
    pub fn new<V: CellValue>(
        m: &VersionedMatrixT<V>,
        min_row: u64,
        max_row: u64,
    ) -> Self {
        let dm_empty = m.dm.nvals() == 0;
        let dp_empty = m.dp.nvals() == 0;
        Self {
            mit: m.m.iter(min_row, max_row),
            dpit: m.dp.iter(min_row, max_row),
            dm: m.dm.clone(),
            dm_empty,
            dp_empty,
        }
    }

    /// Re-seek both inner GraphBLAS iterators to a new row range without
    /// re-allocating them. Hot-loop callers (e.g. `CondTraverseOp` and
    /// `ExpandInto` looking up edges by `(src, dst)`) use this to amortize
    /// the per-pair iterator allocation.
    pub fn seek(
        &mut self,
        min_row: u64,
        max_row: u64,
    ) {
        self.mit.seek(min_row, max_row);
        self.dpit.seek(min_row, max_row);
    }
}

impl Iterator for Iter {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dm_empty {
            if let Some(item) = self.mit.next() {
                return Some(item);
            }
            if self.dp_empty {
                return None;
            }
            return self.dpit.next();
        }
        for (i, j) in &mut self.mit {
            if self.dm.get(i, j).is_none() {
                return Some((i, j));
            }
        }
        self.dpit.next()
    }
}

/// Value-yielding iterator over a `u64` versioned matrix: chains base + delta-plus
/// `(row, col, value)` triples, skipping entries present in `dm`.
pub struct ValueIter {
    mit: matrix::Iter<Uint64Extract>,
    dpit: matrix::Iter<Uint64Extract>,
    dm: Cow<Matrix>,
    dm_empty: bool,
    dp_empty: bool,
}

unsafe impl Send for ValueIter {}
unsafe impl Sync for ValueIter {}

impl ValueIter {
    #[must_use]
    fn new(
        m: &VersionedMatrixT<u64>,
        min_row: u64,
        max_row: u64,
    ) -> Self {
        let dm_empty = m.dm.nvals() == 0;
        let dp_empty = m.dp.nvals() == 0;
        Self {
            mit: m.m.iter_values(min_row, max_row),
            dpit: m.dp.iter_values(min_row, max_row),
            dm: m.dm.clone(),
            dm_empty,
            dp_empty,
        }
    }

    /// Re-seek both inner iterators to a new row range without re-allocating.
    pub fn seek(
        &mut self,
        min_row: u64,
        max_row: u64,
    ) {
        self.mit.seek(min_row, max_row);
        self.dpit.seek(min_row, max_row);
    }
}

impl Iterator for ValueIter {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dm_empty {
            if let Some(item) = self.mit.next() {
                return Some(item);
            }
            if self.dp_empty {
                return None;
            }
            return self.dpit.next();
        }
        for (i, j, v) in &mut self.mit {
            if self.dm.get(i, j).is_none() {
                return Some((i, j, v));
            }
        }
        self.dpit.next()
    }
}
