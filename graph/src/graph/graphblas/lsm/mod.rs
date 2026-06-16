//! Log-structured matrix — the LSM core + compaction.
//!
//! An **append-only list of immutable segments**: a commit *appends* one small
//! immutable segment; it never mutates or copies anything that grows. MVCC
//! isolation comes from immutability — published segments are frozen and
//! `Arc`-shared, so a reader's pinned snapshot is stable while later commits add
//! (or compact) segments.
//!
//! ```text
//!   one band (rows = encoded-key low 60 bits, cols = id), newest segment last
//!     Segment { adds: Matrix<V>,  tombs: Matrix<bool>,  version, weight }   ...
//!
//!   effective = ⋃ seg.adds  −  ⋃ seg.tombs            (newest version wins per cell)
//! ```
//!
//! The oldest segment (`segs[0]`) is the "base": tombstones there suppress nothing
//! older, so a bottom-merge **drops** them (tombstone GC).
//!
//! - **Reads** — a lazy k-way merge over the segments with newest-wins per
//!   cell (see [`LsmCursor`]); this computes `(⋃ adds) − (⋃ tombs)`.
//! - **Compaction** — after each append, adjacent segments closer than a
//!   geometric factor `F` are merged ([`compact`]). This keeps the segment count
//!   `O(log N)` (bounded read amplification) and gives amortized `O(N log N)`
//!   ingest, insert-order-independent.

/// The value→id banded store built on this log-structured matrix.
pub(crate) mod store;

use std::marker::PhantomData;
use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::graph::graphblas::matrix::{Descriptor, MaskedElementWiseAdd, Matrix, New, Size};

/// One band's matrix dimension: `2^60`. Rows are an encoded key's low 60 bits,
/// columns are ids — both `< 2^60` (GraphBLAS's index ceiling). Banding (the
/// high 4 bits) is handled one level up, in the store; a single
/// [`LsmMatrix`] is one band.
const DIM: u64 = 1 << 60;

/// Compaction fan-out: adjacent segments are merged while the older is no more
/// than `F×` the newer (i.e. they are within the same size tier). Stable state
/// therefore has each segment `> F×` the next-newer one, so weights grow
/// geometrically and the segment count is `O(log_F N)`. `F = 2` balances read
/// amplification (segment count) against write amplification (merge work).
const F: u64 = 2;

/// Cell type of an LSM matrix: presence (`bool`) or value-carrying (`u64`,
/// packing a caller payload). Provides the type-specific GraphBLAS primitives the
/// LSM needs — build a run, scan it, and a value-preserving suppressing union.
/// Local to this module.
pub(crate) trait LsmCell: Clone + Send + Sync + 'static {
    /// Bulk-build an immutable, hypersparse run matrix from `(rows, cols, vals)`.
    /// Empty slices yield an empty matrix (the only constructor). Entries are
    /// unique per run (an id is added once per row in a commit), so no dup
    /// resolution is needed; GraphBLAS sorts internally, so ingest is
    /// insert-order-independent.
    fn build_run(
        rows: &[u64],
        cols: &[u64],
        vals: &[Self],
    ) -> Matrix;

    /// Iterate the run's cells whose row is in `[lo, hi]`, yielding
    /// `(row, col, value)`. For `bool` the value is presence (`true`).
    fn scan(
        m: &Matrix,
        lo: u64,
        hi: u64,
    ) -> Box<dyn Iterator<Item = (u64, u64, Self)> + Send>;

    /// Build `out = (a ∪ b) − mask` as a **fresh** matrix in one pass — no `dup`
    /// of the base, which `a` being `Arc`-shared and immutable would otherwise
    /// force; `a`/`b` are read, not mutated. Values are preserved (a shared cell
    /// takes one of two identical values, see `Matrix::element_wise_add_uint64`).
    /// `mask`, when given, is a structural complement: the cells it holds are
    /// dropped from the result, fusing a segment's tombstone application into the
    /// union. A missing `a` or `b` contributes nothing (`x ∪ ∅ = x`);
    /// `(None, None)` yields `None`.
    fn union_masked(
        a: Option<&Matrix>,
        b: Option<&Matrix>,
        mask: Option<&Matrix>,
    ) -> Option<Matrix>;
}

impl LsmCell for bool {
    fn build_run(
        rows: &[u64],
        cols: &[u64],
        _vals: &[Self],
    ) -> Matrix {
        let mut m = Matrix::new(DIM, DIM);
        m.pin_hypersparse();
        if !rows.is_empty() {
            m.build_bool(rows, cols);
        }
        m
    }

    fn scan(
        m: &Matrix,
        lo: u64,
        hi: u64,
    ) -> Box<dyn Iterator<Item = (u64, u64, Self)> + Send> {
        Box::new(m.iter(lo, hi).map(|(r, c)| (r, c, true)))
    }

    fn union_masked(
        a: Option<&Matrix>,
        b: Option<&Matrix>,
        mask: Option<&Matrix>,
    ) -> Option<Matrix> {
        if a.is_none() && b.is_none() {
            return None;
        }
        // Fresh, hypersparse, empty target: `c<!mask, replace> = a ∪ b`. A `None`
        // operand defaults to the empty `c`, so a lone run passes through
        // (`x ∪ ∅ = x`). `mask` (if any) is a structural complement applied in
        // the same op.
        let mut c = Self::build_run(&[], &[], &[]);
        c.element_wise_add(mask, a, b, mask.map(|_| Descriptor::RSC));
        Some(c)
    }
}

/// Generate a [`LsmCell`] impl for a **value-carrying** cell type by wiring its
/// `Matrix` primitives: the typed constructor `$new`, bulk builder `$build`,
/// value-reading range scan `$iter`, and value-preserving suppressing union
/// `$union`. Presence (`bool`) is the special case implemented explicitly above;
/// every value type shares this shape, so adding one (`f64`, `i64`, …) is one
/// line here plus its matching `Matrix` primitives.
macro_rules! impl_valued_lsm_cell {
    ($t:ty, $new:ident, $build:ident, $iter:ident, $union:ident) => {
        impl LsmCell for $t {
            fn build_run(
                rows: &[u64],
                cols: &[u64],
                vals: &[Self],
            ) -> Matrix {
                let mut m = Matrix::$new(DIM, DIM);
                m.pin_hypersparse();
                if !rows.is_empty() {
                    m.$build(rows, cols, vals);
                }
                m
            }

            fn scan(
                m: &Matrix,
                lo: u64,
                hi: u64,
            ) -> Box<dyn Iterator<Item = (u64, u64, Self)> + Send> {
                Box::new(m.$iter(lo, hi))
            }

            fn union_masked(
                a: Option<&Matrix>,
                b: Option<&Matrix>,
                mask: Option<&Matrix>,
            ) -> Option<Matrix> {
                if a.is_none() && b.is_none() {
                    return None;
                }
                // Fresh, hypersparse, empty target: `c<!mask, replace> = a ∪ b`.
                // A `None` operand defaults to the empty `c`, so a lone run
                // passes through (`x ∪ ∅ = x`).
                let mut c = Self::build_run(&[], &[], &[]);
                c.$union(a, b, mask);
                Some(c)
            }
        }
    };
}

impl_valued_lsm_cell!(
    u64,
    new_uint64,
    build_uint64,
    iter_values,
    element_wise_add_uint64
);

/// One immutable segment: a commit's (or a merge's) net add/tombstone runs.
/// `adds`/`tombs` are `None` when empty. `weight ≈ nnz(adds) + nnz(tombs)` drives
/// the compaction tiering. `version` orders segments for newest-wins reads.
/// `Clone` is shallow — the matrices are shared by `Arc`.
#[derive(Clone)]
struct Segment {
    version: u64,
    adds: Option<Arc<Matrix>>,
    tombs: Option<Arc<Matrix>>,
    weight: u64,
}

/// One published, immutable version of a band: a version-ordered segment list
/// (oldest first; `segs[0]` is the base). Everything is `Arc`-shared and never
/// mutated after publish, so a reader holding this is fully isolated from later
/// commits and compactions.
pub(crate) struct Layers<V: LsmCell> {
    segs: Vec<Segment>,
    _v: PhantomData<V>,
}

// `Layers`/`LsmMatrix`/`LsmCursor` hold GraphBLAS matrices behind `Arc`; they are
// immutable once published and only read concurrently, so sharing is safe.
unsafe impl<V: LsmCell> Send for Layers<V> {}
unsafe impl<V: LsmCell> Sync for Layers<V> {}

impl<V: LsmCell> Layers<V> {
    /// An empty band version (no segments).
    pub(crate) fn empty() -> Self {
        Layers {
            segs: Vec::new(),
            _v: PhantomData,
        }
    }

    /// Approximate resident bytes across this band's segment matrices.
    pub(crate) fn memory_usage(&self) -> usize {
        self.segs
            .iter()
            .map(|s| {
                s.adds.as_ref().map_or(0, |m| m.memory_usage())
                    + s.tombs.as_ref().map_or(0, |m| m.memory_usage())
            })
            .sum()
    }
}

/// Apply one commit's changes to a band, returning the new immutable version:
/// append a segment for `(adds, tombs)` and compact. Pure (no lock) so the
/// multi-band store can drive all bands under its own single lock. `adds` are
/// `(row, col, value)` cells; `tombs` are `(row, col)` cells to suppress. The
/// result shares all untouched old segments with `cur` by `Arc`.
///
/// Compaction runs **synchronously** on the commit here, so a writer pays the
/// (amortized `O(log N)`) merge cost inline. Moving it to a background thread —
/// the writer only appends, a compactor merges and republishes — is a write-path
/// optimization left for later.
pub(crate) fn commit_layers<V: LsmCell>(
    cur: &Layers<V>,
    version: u64,
    add_rows: &[u64],
    add_cols: &[u64],
    add_vals: &[V],
    tomb_rows: &[u64],
    tomb_cols: &[u64],
) -> Layers<V> {
    debug_assert_eq!(add_rows.len(), add_cols.len());
    debug_assert_eq!(add_rows.len(), add_vals.len());
    debug_assert_eq!(tomb_rows.len(), tomb_cols.len());

    // Hand the caller's structure-of-arrays straight to the matrix builder — no
    // array-of-structs rebuild. An empty side has no run.
    let adds_m = if add_rows.is_empty() {
        None
    } else {
        Some(Arc::new(V::build_run(add_rows, add_cols, add_vals)))
    };
    let tombs_m = if tomb_rows.is_empty() {
        None
    } else {
        Some(Arc::new(<bool as LsmCell>::build_run(
            tomb_rows,
            tomb_cols,
            &[],
        )))
    };

    let mut segs = cur.segs.clone();
    if adds_m.is_some() || tombs_m.is_some() {
        segs.push(Segment {
            version,
            adds: adds_m,
            tombs: tombs_m,
            weight: add_rows.len() as u64 + tomb_rows.len() as u64,
        });
        compact::<V>(&mut segs);
    }
    Layers {
        segs,
        _v: PhantomData,
    }
}

/// A published snapshot of one band — what a reader pins.
pub(crate) type Snapshot<V> = Arc<Layers<V>>;

/// One band's log-structured matrix: a single published [`Layers`] swapped
/// atomically on commit. The writer builds new immutable segments and publishes
/// a new `Layers` that shares the untouched old ones by `Arc`; readers `load`
/// the current `Arc` **lock-free** (writes are serialized upstream, so a single
/// atomic publish — no reader lock — suffices).
pub(crate) struct LsmMatrix<V: LsmCell> {
    committed: ArcSwap<Layers<V>>,
}

impl<V: LsmCell> Default for LsmMatrix<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: LsmCell> LsmMatrix<V> {
    /// An empty band.
    pub(crate) fn new() -> Self {
        Self {
            committed: ArcSwap::from_pointee(Layers::empty()),
        }
    }

    /// Pin the latest published version — the immutable view a reader scans.
    /// Lock-free: an atomic load plus an `Arc` bump.
    pub(crate) fn snapshot(&self) -> Snapshot<V> {
        self.committed.load_full()
    }

    /// Append one commit's changes as a new immutable segment, run compaction,
    /// and publish. Cells are passed structure-of-arrays — `add_rows`/`add_cols`/
    /// `add_vals` are the parallel `(row, col, value)` adds; `tomb_rows`/
    /// `tomb_cols` the `(row, col)` cells to suppress — so they reach the matrix
    /// builder without an array-of-structs rebuild (the form
    /// [`super::store::BandedLsmStore`] produces directly). The new version
    /// shares all untouched old segments by `Arc`; only the small segment list is
    /// cloned, so a quiet commit is `O(this commit's changes)` and a compacting
    /// commit pays `O(merged nnz)` (amortized `O(log N)` per element).
    pub(crate) fn commit(
        &self,
        version: u64,
        add_rows: &[u64],
        add_cols: &[u64],
        add_vals: &[V],
        tomb_rows: &[u64],
        tomb_cols: &[u64],
    ) {
        debug_assert!(
            !(add_rows.is_empty() && tomb_rows.is_empty()),
            "empty commit — the caller must not publish an empty version"
        );
        let cur = self.committed.load_full();
        let next = commit_layers::<V>(
            &cur, version, add_rows, add_cols, add_vals, tomb_rows, tomb_cols,
        );
        self.committed.store(Arc::new(next));
    }

    /// Collapse all segments into a single tombstone-free base (see
    /// [`major_compact_layers`]). Publishes a new version; pinned readers keep
    /// their old segments.
    pub(crate) fn major_compact(&self) {
        let cur = self.committed.load_full();
        let next = major_compact_layers::<V>(&cur);
        self.committed.store(Arc::new(next));
    }

    /// Number of live segments. Bounded to `O(log N)` by compaction.
    #[cfg(test)]
    pub(crate) fn seg_count(&self) -> usize {
        self.committed.load().segs.len()
    }

    /// Sum of segment weights — a proxy for resident cells (memory). Used by the
    /// tombstone-GC test to confirm churn doesn't grow the structure unboundedly.
    #[cfg(test)]
    pub(crate) fn total_weight(&self) -> u64 {
        self.committed.load().segs.iter().map(|s| s.weight).sum()
    }
}

/// Merge two adjacent segments `a` (older) and `b` (newer) into one, applying
/// newest-wins:
///
/// ```text
///   adds  = (a.adds ∪ b.adds[newer wins]) − b.tombs
///   tombs = (a.tombs ∪ b.tombs)           − b.adds      (dropped if `bottom`)
/// ```
///
/// Each side is one fused [`LsmCell::union_masked`] (union + masked tombstone
/// removal) into a fresh matrix. `a`'s matrices are `Arc`-shared and immutable;
/// `union_masked` reads them as operands rather than duplicating them, leaving
/// the originals intact for older readers. When `a` is the bottom segment
/// (`segs[0]`) its merged tombstones suppress nothing older, so they are GC'd
/// (`bottom = true`).
fn merge_segments<V: LsmCell>(
    a: &Segment,
    b: &Segment,
    bottom: bool,
) -> Segment {
    // adds = (a.adds ∪ b.adds) − b.tombs, built fresh (no dup of the base).
    let add = V::union_masked(a.adds.as_deref(), b.adds.as_deref(), b.tombs.as_deref());
    let add_n = add.as_ref().map_or(0, Matrix::nvals);
    let adds = add.filter(|_| add_n > 0).map(Arc::new);

    // tombs = (a.tombs ∪ b.tombs) − b.adds ; dropped at the bottom (nothing
    // older to suppress). Tombstone runs are always `bool`; `b.adds` is the
    // structural suppression mask.
    let (tombs, tomb_n) = if bottom {
        (None, 0)
    } else {
        let t = <bool as LsmCell>::union_masked(
            a.tombs.as_deref(),
            b.tombs.as_deref(),
            b.adds.as_deref(),
        );
        let n = t.as_ref().map_or(0, Matrix::nvals);
        (t.filter(|_| n > 0).map(Arc::new), n)
    };

    Segment {
        version: b.version,
        adds,
        tombs,
        weight: add_n + tomb_n,
    }
}

/// Size-tiered compaction. After an append, repeatedly merge the newest adjacent
/// pair whose older segment is within `F×` the newer (same tier), restarting
/// after each merge. Stable state has strictly `> F×` growth oldest→newest, so
/// the segment count is `O(log_F N)`. Merging the bottom pair GC's tombstones and
/// can drop a fully-cancelled (empty) segment.
fn compact<V: LsmCell>(segs: &mut Vec<Segment>) {
    loop {
        let n = segs.len();
        if n < 2 {
            return;
        }
        let mut merged = false;
        // Scan newest pair → oldest; merge the first that is within a tier.
        let mut i = n - 1;
        while i >= 1 {
            if segs[i - 1].weight <= F.saturating_mul(segs[i].weight) {
                let bottom = i == 1;
                let m = merge_segments::<V>(&segs[i - 1], &segs[i], bottom);
                let replacement = if m.adds.is_some() || m.tombs.is_some() {
                    vec![m]
                } else {
                    Vec::new() // both runs fully cancelled — drop the segment
                };
                segs.splice(i - 1..=i, replacement);
                merged = true;
                break;
            }
            i -= 1;
        }
        if !merged {
            return;
        }
    }
}

/// **Major compaction**: collapse a band's entire segment list into a single
/// tombstone-free base. Where [`compact`] (minor, per-commit) only bounds the run
/// count to `O(log N)` — leaving several matrices per band (k-way read merge) —
/// this folds *everything* into one matrix: all tombstones applied and dropped,
/// superseded cells resolved.
///
/// Implemented as **one k-way merge**, not a pairwise fold: the [`LsmCursor`]
/// already merges all segments newest-wins and applies tombstones, yielding the
/// effective live cells in `(row, col)` order in `O(N log K)` (`K` = segment
/// count); we collect them and `build` one matrix in `O(N)`. (A naive
/// fold-into-a-growing-accumulator would be `O(N·K)` — quadratic in the segment
/// count, catastrophic when compaction was deferred during a bulk load.)
///
/// MVCC-safe: returns a new immutable [`Layers`]; readers pinned to the old
/// snapshot keep their segments until they drop.
pub(crate) fn major_compact_layers<V: LsmCell>(snap: &Snapshot<V>) -> Layers<V> {
    // Already a single tombstone-free base (or empty) — nothing to gain.
    if snap.segs.len() <= 1 && snap.segs.first().is_none_or(|s| s.tombs.is_none()) {
        return Layers {
            segs: snap.segs.clone(),
            _v: PhantomData,
        };
    }
    let version = snap.segs.last().map_or(0, |s| s.version);
    let mut cursor = LsmCursor::new(Arc::clone(snap), 0, DIM - 1);
    let (mut rows, mut cols, mut vals) = (Vec::new(), Vec::new(), Vec::new());
    while let Some((r, c, v)) = cursor.next_cell() {
        rows.push(r);
        cols.push(c);
        vals.push(v);
    }
    let segs = if rows.is_empty() {
        Vec::new()
    } else {
        let weight = rows.len() as u64;
        vec![Segment {
            version,
            adds: Some(Arc::new(V::build_run(&rows, &cols, &vals))),
            tombs: None,
            weight,
        }]
    };
    Layers {
        segs,
        _v: PhantomData,
    }
}

/// One source's current front in the merge heap: the smallest `(row, col)` it
/// has not yet emitted, tagged with the segment's `version` (for newest-wins) and
/// the source index `src` (to advance it after popping). `value` is `Some` for an
/// add run, `None` for a tombstone. Ordered by `(row, col)` only — `version`/
/// `value` are resolved when popping equal cells, so they're excluded from `Ord`.
struct Entry<V> {
    row: u64,
    col: u64,
    version: u64,
    value: Option<V>,
    src: usize,
}

impl<V> PartialEq for Entry<V> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        (self.row, self.col) == (other.row, other.col)
    }
}
impl<V> Eq for Entry<V> {}
impl<V> PartialOrd for Entry<V> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<V> Ord for Entry<V> {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        (self.row, self.col).cmp(&(other.row, other.col))
    }
}

/// Lazy k-way merge over one band's segments for the inclusive row range
/// `[lo, hi]`. For each `(row, col)` the **newest** source wins: an add yields the
/// cell live, a tombstone suppresses it.
///
/// Uses a **binary min-heap** keyed by `(row, col)`: each step pops the smallest
/// cell and re-pushes the advanced source's next front — `O(log K)` per emitted
/// cell (`K` = live segment streams), versus a linear scan of all `K` fronts
/// (`O(K)`). This keeps reads cheap even with many segments, which is what lets
/// compaction be less aggressive. Owns the pinned [`Snapshot`] (so it is `Send`
/// and reclaims by `Arc`-drop).
pub(crate) struct LsmCursor<V: LsmCell> {
    _snap: Snapshot<V>,
    /// One boxed iterator per source (add or tomb run), indexed by `src`.
    iters: Vec<Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send>>,
    /// Segment version of each source (parallel to `iters`).
    versions: Vec<u64>,
    /// Min-heap of each source's current front (`Reverse` flips the max-heap).
    heap: std::collections::BinaryHeap<std::cmp::Reverse<Entry<V>>>,
}

unsafe impl<V: LsmCell> Send for LsmCursor<V> {}

impl<V: LsmCell> LsmCursor<V> {
    pub(crate) fn new(
        snap: Snapshot<V>,
        lo: u64,
        hi: u64,
    ) -> Self {
        let mut iters: Vec<Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send>> = Vec::new();
        let mut versions: Vec<u64> = Vec::new();
        if lo <= hi {
            for seg in &snap.segs {
                if let Some(m) = &seg.adds {
                    iters.push(Box::new(
                        V::scan(m, lo, hi).map(|(r, c, v)| (r, c, Some(v))),
                    ));
                    versions.push(seg.version);
                }
                if let Some(m) = &seg.tombs {
                    iters.push(Box::new(
                        <bool as LsmCell>::scan(m, lo, hi).map(|(r, c, _)| (r, c, None)),
                    ));
                    versions.push(seg.version);
                }
            }
        }
        let mut heap = std::collections::BinaryHeap::with_capacity(iters.len());
        for src in 0..iters.len() {
            if let Some((row, col, value)) = iters[src].next() {
                heap.push(std::cmp::Reverse(Entry {
                    row,
                    col,
                    version: versions[src],
                    value,
                    src,
                }));
            }
        }
        Self {
            _snap: snap,
            iters,
            versions,
            heap,
        }
    }

    /// Pull the next item from source `src` and re-push its front onto the heap.
    fn advance(
        &mut self,
        src: usize,
    ) {
        if let Some((row, col, value)) = self.iters[src].next() {
            self.heap.push(std::cmp::Reverse(Entry {
                row,
                col,
                version: self.versions[src],
                value,
                src,
            }));
        }
    }

    /// The next live cell `(row, col, value)` in range, or `None` when exhausted.
    pub(crate) fn next_cell(&mut self) -> Option<(u64, u64, V)> {
        loop {
            // Pop the globally smallest (row, col).
            let std::cmp::Reverse(first) = self.heap.pop()?;
            let (row, col) = (first.row, first.col);
            let mut best_ver = first.version;
            let mut best_val = first.value;
            self.advance(first.src);

            // Resolve all other sources at the same cell: newest version wins.
            while let Some(&std::cmp::Reverse(Entry {
                row: pr, col: pc, ..
            })) = self.heap.peek()
            {
                if (pr, pc) != (row, col) {
                    break;
                }
                let std::cmp::Reverse(e) = self.heap.pop().unwrap();
                // Two sources at the same cell with the *same* version means one
                // commit both added and tombstoned that cell — the caller must
                // resolve that before commit, or newest-wins here is ambiguous
                // (heap pop order would decide). Guard the invariant.
                debug_assert_ne!(
                    e.version, best_ver,
                    "add+tombstone of the same cell at one version: {row},{col}"
                );
                if e.version >= best_ver {
                    best_ver = e.version;
                    best_val = e.value;
                }
                self.advance(e.src);
            }

            if let Some(val) = best_val {
                return Some((row, col, val));
            }
            // Newest was a tombstone → cell is dead; keep merging.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn init() {
        crate::graph::graphblas::test_init_graphblas();
    }

    /// splitmix64 — deterministic pseudo-randomness, no `rand` dep.
    fn sm64(mut z: u64) -> u64 {
        z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Collect `(row, col, value)` cells in `[lo, hi]` from a snapshot, sorted
    /// lexicographically by `(row, col, value)` so assertions can compare against
    /// a fixed expected order.
    fn scan<V: LsmCell + Ord>(
        snap: &Snapshot<V>,
        lo: u64,
        hi: u64,
    ) -> Vec<(u64, u64, V)> {
        let mut c = LsmCursor::new(Arc::clone(snap), lo, hi);
        let mut out = Vec::new();
        while let Some(cell) = c.next_cell() {
            out.push(cell);
        }
        out.sort_unstable();
        out
    }

    /// The segment matrices must be **hypersparse**. At `DIM = 2^60` any other
    /// format is fatal: sparse needs an `nrows+1` row-pointer array, bitmap/full
    /// need `nrows×ncols` cells — both `≈ 2^60`+. Commit a handful of entries at
    /// rows scattered across the whole band and assert the published matrices
    /// report `GxB_HYPERSPARSE`, with resident bytes `O(entries)` (not `O(2^60)`).
    #[test]
    fn segment_matrices_are_hypersparse() {
        use crate::graph::graphblas::GxB_HYPERSPARSE;
        let hyper = GxB_HYPERSPARSE as i32;
        init();

        // bool (presence) band.
        let m = LsmMatrix::<bool>::new();
        m.commit(
            1,
            &[0, 1 << 30, 1 << 50, DIM - 1],
            &[0, 1, 2, 3],
            &[true, true, true, true],
            &[1 << 40], // a tombstone run too
            &[9],
        );
        for seg in &m.snapshot().segs {
            for mat in [seg.adds.as_ref(), seg.tombs.as_ref()]
                .into_iter()
                .flatten()
            {
                assert_eq!(mat.sparsity_status(), hyper, "bool segment not hypersparse");
                assert!(
                    mat.memory_usage() < 64 * 1024,
                    "resident bytes scale with DIM, not entries"
                );
            }
        }

        // u64 (value-carrying) band.
        let mu = LsmMatrix::<u64>::new();
        mu.commit(
            1,
            &[0, 1 << 45, DIM - 1],
            &[0, 1, 2],
            &[7u64, 9, 11],
            &[],
            &[],
        );
        for seg in &mu.snapshot().segs {
            if let Some(adds) = &seg.adds {
                assert_eq!(adds.sparsity_status(), hyper, "u64 segment not hypersparse");
                assert!(adds.memory_usage() < 64 * 1024);
            }
        }

        // The pin must survive compaction: many equal-weight commits trigger
        // merges (which `dup` and union segments), and a major compaction folds
        // everything into one base via `build_run`. All must stay hypersparse.
        let c = LsmMatrix::<u64>::new();
        for v in 1..=64u64 {
            c.commit(v, &[v << 40], &[v], &[v | 1], &[], &[]);
        }
        c.major_compact();
        for seg in &c.snapshot().segs {
            if let Some(adds) = &seg.adds {
                assert_eq!(
                    adds.sparsity_status(),
                    hyper,
                    "post-compaction not hypersparse"
                );
            }
        }
    }

    #[test]
    fn add_then_scan() {
        init();
        let m = LsmMatrix::<bool>::new();
        m.commit(
            1,
            &[10, 20, 10],
            &[100, 200, 101],
            &[true, true, true],
            &[],
            &[],
        );
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 10), vec![(10, 100, true), (10, 101, true)]);
        assert_eq!(
            scan(&s, 0, 100),
            vec![(10, 100, true), (10, 101, true), (20, 200, true)]
        );
        assert_eq!(scan(&s, 11, 19), vec![]);
    }

    #[test]
    fn tombstone_suppresses_cell() {
        init();
        let m = LsmMatrix::<bool>::new();
        m.commit(1, &[10, 10], &[100, 101], &[true, true], &[], &[]);
        m.commit(2, &[], &[], &[], &[10], &[100]); // delete id 100's cell at row 10
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 10), vec![(10, 101, true)]);
    }

    #[test]
    fn newest_wins_readd_after_tombstone() {
        init();
        let m = LsmMatrix::<u64>::new();
        m.commit(1, &[10], &[5], &[0xAAAA], &[], &[]); // id 5 @ row 10, value AAAA
        m.commit(2, &[], &[], &[], &[10], &[5]); // delete it
        // ...later re-add the *same* cell with a new value (update back).
        m.commit(3, &[10], &[5], &[0xBBBB], &[], &[]);
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 10), vec![(10, 5, 0xBBBB)]); // newest (v3 add) wins
    }

    #[test]
    fn update_moves_to_new_row() {
        init();
        let m = LsmMatrix::<u64>::new();
        m.commit(1, &[10], &[5], &[0x11], &[], &[]); // id 5 at value-row 10
        m.commit(2, &[20], &[5], &[0x11], &[10], &[5]); // update: tomb old, add new
        let s = m.snapshot();
        assert_eq!(scan(&s, 0, 100), vec![(20, 5, 0x11)]);
        assert_eq!(scan(&s, 10, 10), vec![]); // old row vacated
    }

    #[test]
    fn compaction_merges_and_preserves_results() {
        init();
        let m = LsmMatrix::<bool>::new();
        // out-of-order commits; compaction merges them into the base.
        m.commit(1, &[50], &[1], &[true], &[], &[]);
        m.commit(2, &[10], &[2], &[true], &[], &[]);
        m.commit(3, &[30], &[3], &[true], &[], &[]);
        assert_eq!(
            m.seg_count(),
            1,
            "equal-weight commits collapse to one segment"
        );
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 30), vec![(10, 2, true), (30, 3, true)]);
        assert_eq!(
            scan(&s, 0, 100),
            vec![(10, 2, true), (30, 3, true), (50, 1, true)]
        );
    }

    #[test]
    fn snapshot_isolated_from_later_commits_and_deletes() {
        init();
        let m = LsmMatrix::<bool>::new();
        m.commit(1, &[10], &[1], &[true], &[], &[]);
        let old = m.snapshot(); // pin version 1
        m.commit(2, &[20], &[2], &[true], &[], &[]); // add
        m.commit(3, &[], &[], &[], &[10], &[1]); // delete the cell the old snapshot sees
        let new = m.snapshot();
        // Old reader: unaffected by the add or the delete.
        assert_eq!(scan(&old, 0, 100), vec![(10, 1, true)]);
        // New reader: sees the add, not the deleted cell.
        assert_eq!(scan(&new, 0, 100), vec![(20, 2, true)]);
    }

    /// Compaction keeps the segment count logarithmic in the number of commits.
    #[test]
    fn compaction_bounds_segment_count() {
        init();
        let m = LsmMatrix::<bool>::new();
        const N: u64 = 4_096; // 2^12 equal-weight commits
        for v in 1..=N {
            m.commit(v, &[v], &[v], &[true], &[], &[]);
        }
        // 2^12 unit segments → binary-counter merges → ~log2(N) segments.
        assert!(
            m.seg_count() <= 16,
            "segment count {} not bounded ~log2({N})",
            m.seg_count()
        );
        // All cells still present and correct.
        assert_eq!(scan(&m.snapshot(), 1, N).len(), N as usize);
    }

    /// Update churn on a fixed id set: each round tombstones old cells and adds
    /// new ones. Tombstones must be GC'd by bottom-merges, so the resident weight
    /// stays bounded (≈ live set), not growing with the number of rounds.
    #[test]
    fn tombstone_gc_under_churn() {
        init();
        let m = LsmMatrix::<u64>::new();
        const DOCS: u64 = 64;
        let mut version = 0u64;
        // round 0: initial placement at row = id.
        let mut cur_row: Vec<u64> = (0..DOCS).collect();
        let cols: Vec<u64> = (0..DOCS).collect();
        version += 1;
        let vals = vec![version; DOCS as usize];
        m.commit(version, &cur_row, &cols, &vals, &[], &[]);

        for round in 1..200u64 {
            version += 1;
            let new_row: Vec<u64> = (0..DOCS).map(|d| 1000 + round * DOCS + d).collect();
            let vals = vec![version; DOCS as usize];
            // adds at new rows, tombstones at the old rows — same ids (cols).
            m.commit(version, &new_row, &cols, &vals, &cur_row, &cols);
            cur_row = new_row;
        }

        // Live set is exactly DOCS cells regardless of how many rounds ran.
        let live = scan(&m.snapshot(), 0, u64::MAX >> 4);
        assert_eq!(live.len(), DOCS as usize, "live set must equal doc count");
        for d in 0..DOCS {
            assert_eq!(live[d as usize].1, d);
            assert_eq!(live[d as usize].0, cur_row[d as usize]);
        }
        // Resident weight stays small — tombstones GC'd, not accumulated over 200
        // rounds × 64 ids (= 12.8k churned cells). The bound is the live set
        // (`DOCS`) times a slack factor for the few not-yet-bottom-merged
        // segments still holding superseded cells.
        const RESIDENT_SLACK: u64 = 4;
        assert!(
            m.total_weight() < RESIDENT_SLACK * DOCS,
            "resident weight {} should stay ≈ live set, not grow with churn",
            m.total_weight()
        );
    }

    /// A reader pinned before a burst of commits + compactions still scans its
    /// original version correctly — its segments stay `Arc`-alive even as
    /// compaction replaces the live segment list (MVCC reclamation).
    #[test]
    fn reader_survives_compaction() {
        init();
        let m = LsmMatrix::<u64>::new();
        for v in 1..=10u64 {
            m.commit(v, &[v], &[v], &[v], &[], &[]);
        }
        let pinned = m.snapshot();
        let before = scan(&pinned, 0, 1000);
        assert_eq!(before.len(), 10);
        // Heavy churn + compaction after the pin.
        for v in 11..=2_000u64 {
            m.commit(v, &[v], &[v], &[v], &[v - 10], &[v - 10]);
        }
        // The pinned snapshot is byte-for-byte unchanged.
        assert_eq!(scan(&pinned, 0, 1000), before);
    }

    /// Differential fuzz: drive the band with random add/tombstone commits and
    /// check every range scan against a plain reference model, so the LSM's
    /// merge + per-commit compaction must stay bit-exact with newest-wins
    /// semantics across hundreds of merges.
    ///
    /// The model is a `BTreeMap<(row, col) → (version, Option<value>)>`: each
    /// commit overwrites a cell's entry with its newest event — `Some(value)` for
    /// an add, `None` for a tombstone. A cell is *live* in the model iff its
    /// newest entry is `Some`. The key space is deliberately tiny (`ROWS × COLS`)
    /// so cells are revisited constantly — lots of updates, tombstones, and
    /// resurrections, which is what stresses newest-wins and tombstone GC.
    #[test]
    fn differential_fuzz_vs_reference() {
        init();
        let m = LsmMatrix::<u64>::new();
        let mut model: BTreeMap<(u64, u64), (u64, Option<u64>)> = BTreeMap::new();
        const ROWS: u64 = 200;
        const COLS: u64 = 50;

        for step in 1..=400u64 {
            let version = step; // versions are strictly increasing → newest = highest
            // Build this commit's 0..7 random ops, mirroring each into the model.
            let n = (sm64(step) % 8) as usize;
            // A commit can touch the same cell twice; collapse to its last op (a
            // run holds each cell once) in a map, then emit it structure-of-arrays.
            let mut ops: BTreeMap<(u64, u64), Option<u64>> = BTreeMap::new();
            for k in 0..n {
                let h = sm64(step.wrapping_mul(131) ^ k as u64);
                let (row, col) = (h % ROWS, (h >> 16) % COLS);
                // force nonzero so a value is never mistaken for absent
                let op = if h & 1 == 0 { Some(h | 1) } else { None };
                ops.insert((row, col), op);
                model.insert((row, col), (version, op));
            }
            let (mut add_rows, mut add_cols, mut add_vals) = (Vec::new(), Vec::new(), Vec::new());
            let (mut tomb_rows, mut tomb_cols) = (Vec::new(), Vec::new());
            for (&(row, col), op) in &ops {
                match op {
                    Some(v) => {
                        add_rows.push(row);
                        add_cols.push(col);
                        add_vals.push(*v);
                    }
                    None => {
                        tomb_rows.push(row);
                        tomb_cols.push(col);
                    }
                }
            }
            // The store never publishes an empty version (see `commit`'s
            // debug_assert); a no-op step changes nothing, so just skip it.
            if !(add_rows.is_empty() && tomb_rows.is_empty()) {
                m.commit(
                    version, &add_rows, &add_cols, &add_vals, &tomb_rows, &tomb_cols,
                );
            }

            // Periodically verify random sub-ranges scan exactly the model's live
            // cells in that row range.
            if step % 7 == 0 {
                let s = m.snapshot();
                for q in 0..4u64 {
                    let a = sm64(step ^ q) % ROWS;
                    let b = sm64(step ^ q ^ 0xFF) % ROWS;
                    let (lo, hi) = (a.min(b), a.max(b));
                    let got = scan(&s, lo, hi);
                    let mut want: Vec<(u64, u64, u64)> = model
                        .iter()
                        .filter(|((r, _), (_, v))| *r >= lo && *r <= hi && v.is_some())
                        .map(|((r, c), (_, v))| (*r, *c, v.unwrap()))
                        .collect();
                    want.sort_unstable();
                    assert_eq!(got, want, "mismatch at step {step}, range [{lo},{hi}]");
                }
            }
        }
    }

    /// Major compaction collapses many segments into one tombstone-free base
    /// without changing scan results, and a reader pinned before it is unaffected.
    #[test]
    fn major_compaction_collapses_and_preserves() {
        init();
        let m = LsmMatrix::<u64>::new();
        let mut version = 0u64;
        let mut cur_row: Vec<u64> = (0..40).collect();
        let cols: Vec<u64> = (0..40).collect();
        let vals: Vec<u64> = (0..40u64).map(|d| d | 1).collect();
        version += 1;
        m.commit(version, &cur_row, &cols, &vals, &[], &[]);
        for round in 1..60u64 {
            version += 1;
            let new_row: Vec<u64> = (0..40).map(|d| 1000 + round * 40 + d).collect();
            // adds at new rows, tombstones at the old rows — same ids (cols).
            m.commit(version, &new_row, &cols, &vals, &cur_row, &cols);
            cur_row = new_row;
        }
        let before = scan(&m.snapshot(), 0, u64::MAX >> 4);
        assert_eq!(before.len(), 40);
        let pinned = m.snapshot(); // pin pre-compaction
        let pinned_before = scan(&pinned, 0, u64::MAX >> 4);

        m.major_compact();

        assert_eq!(
            m.seg_count(),
            1,
            "major compaction must collapse to one segment"
        );
        assert_eq!(scan(&m.snapshot(), 0, u64::MAX >> 4), before);
        assert_eq!(scan(&pinned, 0, u64::MAX >> 4), pinned_before); // MVCC

        // Idempotent: compacting an already-collapsed base is a no-op — still one
        // segment, same cells.
        m.major_compact();
        assert_eq!(m.seg_count(), 1);
        assert_eq!(scan(&m.snapshot(), 0, u64::MAX >> 4), before);
    }

    /// Concurrent MVCC: many reader threads scan while a writer commits +
    /// compacts. Each commit `v` appends exactly one new cell `(v, v, v)`, so any
    /// consistent snapshot must observe a *contiguous* id prefix `{1..k}` — a gap
    /// would be a torn / non-isolated read. Exercises the lock-free `Arc`
    /// snapshot + compacting-commit path under contention (no UB / panics).
    #[test]
    fn concurrent_readers_during_commits() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::thread;

        init();
        const N: u64 = 2_000;
        let m = Arc::new(LsmMatrix::<u64>::new());
        let done = Arc::new(AtomicBool::new(false));

        let readers: Vec<_> = (0..4)
            .map(|_| {
                let m = Arc::clone(&m);
                let done = Arc::clone(&done);
                thread::spawn(move || {
                    let mut max_seen = 0u64;
                    while !done.load(Ordering::Relaxed) {
                        let snap = m.snapshot();
                        let cells = scan(&snap, 0, N);
                        for (i, &(r, c, v)) in cells.iter().enumerate() {
                            assert_eq!(c, i as u64 + 1, "non-contiguous snapshot → torn read");
                            assert_eq!((r, v), (c, c), "cell row/value inconsistent with commit");
                        }
                        let k = cells.len() as u64;
                        assert!(k >= max_seen, "snapshot regressed: {k} < {max_seen}");
                        max_seen = k;
                    }
                })
            })
            .collect();

        for v in 1..=N {
            m.commit(v, &[v], &[v], &[v], &[], &[]);
        }
        done.store(true, Ordering::Relaxed);
        for r in readers {
            r.join().unwrap();
        }
        assert_eq!(scan(&m.snapshot(), 0, N).len(), N as usize);
    }
}
