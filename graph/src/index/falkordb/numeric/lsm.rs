//! Log-structured matrix store — the LSM core + compaction (milestones **M1/M2**).
//!
//! Replaces the single copy-on-write delta of [`super::store`] (which re-copies a
//! growing delta on every commit → super-linear ingest) with an **append-only
//! list of immutable segments**. A commit *appends* one small immutable segment;
//! it never mutates or copies anything that grows. MVCC isolation comes from
//! immutability: published segments are frozen and `Arc`-shared, so a reader's
//! pinned snapshot is stable while later commits add (or compact) segments.
//!
//! ```text
//!   one band (rows = encoded-key low 60 bits, cols = doc id), newest segment last
//!     Segment { adds: Matrix<V>,  tombs: Matrix<bool>,  version, weight }   ...
//!
//!   effective = ⋃ seg.adds  −  ⋃ seg.tombs            (newest version wins per cell)
//! ```
//!
//! The oldest segment (`segs[0]`) is the "base": tombstones there suppress nothing
//! older, so a bottom-merge **drops** them (tombstone GC).
//!
//! - **Reads** (M1) — a lazy k-way merge over the segments with newest-wins per
//!   cell (see [`LsmCursor`]); this is `(⋃ adds) − (⋃ tombs)`, the generalization
//!   of the old `(m ∪ dp) − dm`.
//! - **Compaction** (M2) — after each append, adjacent segments closer than a
//!   geometric factor `F` are merged ([`compact`]). This keeps the segment count
//!   `O(log N)` (bounded read amplification) and gives amortized `O(N log N)`
//!   ingest, insert-order-independent.

use std::iter::Peekable;
use std::marker::PhantomData;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::graph::graphblas::matrix::{Dup, MaskedElementWiseAdd, Matrix, New, Size};

/// One band's matrix dimension: `2^60`. Rows are an encoded key's low 60 bits,
/// columns are doc ids — both `< 2^60` (GraphBLAS's index ceiling). Banding (the
/// high 4 bits) is handled one level up, in the store (M3); a single
/// [`LsmMatrix`] is one band.
const DIM: u64 = 1 << 60;

/// Compaction fan-out: adjacent segments are merged while the older is no more
/// than `F×` the newer (i.e. they are within the same size tier). Stable state
/// therefore has each segment `> F×` the next-newer one, so weights grow
/// geometrically and the segment count is `O(log_F N)`. `F = 2` balances read
/// amplification (segment count) against write amplification (merge work);
/// tunable in M4.
const F: u64 = 2;

/// Cell type of an LSM matrix: presence (`bool`, node indexes) or value-carrying
/// (`u64`, edge indexes packing endpoints). Provides the type-specific GraphBLAS
/// primitives the LSM needs — build a run, scan it, an empty matrix, and a
/// value-preserving union. Local to the index (the engine's `CellValue` is
/// untouched).
pub(crate) trait LsmCell: Copy + Send + Sync + 'static {
    /// Bulk-build an immutable run matrix from `(rows, cols, vals)`. Entries are
    /// unique per run (a doc is added once per row in a commit), so no dup
    /// resolution is needed. GraphBLAS sorts internally — hence ingest is
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

    /// An empty matrix of this cell type, full band dimensions.
    fn empty() -> Matrix;

    /// `dst = dst ∪ src`, preserving values; on a cell conflict `src` (the newer
    /// run) wins. Used to fold a newer segment's adds into an older one.
    fn union_into(
        dst: &mut Matrix,
        src: &Matrix,
    );
}

impl LsmCell for bool {
    fn build_run(
        rows: &[u64],
        cols: &[u64],
        _vals: &[Self],
    ) -> Matrix {
        let mut m = Matrix::new(DIM, DIM);
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

    fn empty() -> Matrix {
        Matrix::new(DIM, DIM)
    }

    fn union_into(
        dst: &mut Matrix,
        src: &Matrix,
    ) {
        // PAIR/BOOL presence union — value is always `true`, so newer-wins is moot.
        dst.element_wise_add(None, None, Some(src), None);
    }
}

impl LsmCell for u64 {
    fn build_run(
        rows: &[u64],
        cols: &[u64],
        vals: &[Self],
    ) -> Matrix {
        let mut m = Matrix::new_uint64(DIM, DIM);
        if !rows.is_empty() {
            m.build_uint64(rows, cols, vals);
        }
        m
    }

    fn scan(
        m: &Matrix,
        lo: u64,
        hi: u64,
    ) -> Box<dyn Iterator<Item = (u64, u64, Self)> + Send> {
        Box::new(m.iter_values(lo, hi))
    }

    fn empty() -> Matrix {
        Matrix::new_uint64(DIM, DIM)
    }

    fn union_into(
        dst: &mut Matrix,
        src: &Matrix,
    ) {
        // ANY/SECOND over UINT64: `src` is the second operand, so it wins on a
        // conflicting cell — i.e. the newer run's value is kept.
        dst.element_wise_add_uint64(src);
    }
}

/// One immutable segment: a commit's (or a merge's) net add/tombstone runs.
/// `adds`/`tombs` are `None` when empty. `weight ≈ nnz(adds) + nnz(tombs)` drives
/// the compaction tiering. `version` orders segments for newest-wins reads.
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
// immutable once published and only read concurrently, so sharing is safe — same
// posture as the engine's `VersionedMatrix`.
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
pub(crate) fn commit_layers<V: LsmCell>(
    cur: &Layers<V>,
    version: u64,
    adds: &[(u64, u64, V)],
    tombs: &[(u64, u64)],
) -> Layers<V> {
    let adds_m = (!adds.is_empty()).then(|| {
        let mut rows = Vec::with_capacity(adds.len());
        let mut cols = Vec::with_capacity(adds.len());
        let mut vals = Vec::with_capacity(adds.len());
        for &(r, c, v) in adds {
            rows.push(r);
            cols.push(c);
            vals.push(v);
        }
        Arc::new(V::build_run(&rows, &cols, &vals))
    });
    let tombs_m = (!tombs.is_empty()).then(|| {
        let mut rows = Vec::with_capacity(tombs.len());
        let mut cols = Vec::with_capacity(tombs.len());
        for &(r, c) in tombs {
            rows.push(r);
            cols.push(c);
        }
        Arc::new(<bool as LsmCell>::build_run(&rows, &cols, &[]))
    });

    let mut segs = clone_segs(&cur.segs);
    if adds_m.is_some() || tombs_m.is_some() {
        segs.push(Segment {
            version,
            adds: adds_m,
            tombs: tombs_m,
            weight: adds.len() as u64 + tombs.len() as u64,
        });
        compact::<V>(&mut segs);
    }
    Layers {
        segs,
        _v: PhantomData,
    }
}

/// A published snapshot of one band — what a reader pins (mechanism A).
pub(crate) type Snapshot<V> = Arc<Layers<V>>;

/// One band's log-structured matrix: a single published [`Layers`] swapped under
/// a lock on commit. Readers clone the `Arc` (lock-free after); the writer builds
/// new immutable segments and publishes a new `Layers` that shares the untouched
/// old segments by `Arc`.
pub(crate) struct LsmMatrix<V: LsmCell> {
    committed: RwLock<Snapshot<V>>,
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
            committed: RwLock::new(Arc::new(Layers {
                segs: Vec::new(),
                _v: PhantomData,
            })),
        }
    }

    /// Pin the latest published version — the immutable view a reader scans.
    pub(crate) fn snapshot(&self) -> Snapshot<V> {
        Arc::clone(&self.committed.read())
    }

    /// Append one commit's changes as a new immutable segment, run compaction,
    /// and publish. `adds` are `(row, col, value)` cells; `tombs` are
    /// `(row, col)` cells to suppress (the caller — the store — resolves doc
    /// deletes/updates into the exact old cells). The new version shares all
    /// untouched old segments by `Arc`; only the small segment list is cloned, so
    /// a quiet commit is `O(this commit's changes)` and a compacting commit pays
    /// `O(merged nnz)` (amortized `O(log N)` per element).
    pub(crate) fn commit(
        &self,
        version: u64,
        adds: &[(u64, u64, V)],
        tombs: &[(u64, u64)],
    ) {
        if adds.is_empty() && tombs.is_empty() {
            return; // empty commit — nothing to publish
        }
        let cur = self.committed.read().clone();
        let next = commit_layers::<V>(&cur, version, adds, tombs);
        *self.committed.write() = Arc::new(next);
    }

    /// Number of live segments. Bounded to `O(log N)` by compaction.
    #[cfg(test)]
    pub(crate) fn seg_count(&self) -> usize {
        self.committed.read().segs.len()
    }

    /// Sum of segment weights — a proxy for resident cells (memory). Used by the
    /// tombstone-GC test to confirm churn doesn't grow the structure unboundedly.
    #[cfg(test)]
    pub(crate) fn total_weight(&self) -> u64 {
        self.committed.read().segs.iter().map(|s| s.weight).sum()
    }
}

/// Shallow-clone the segment list (each `Segment` shares its matrices by `Arc`).
fn clone_segs(segs: &[Segment]) -> Vec<Segment> {
    segs.iter()
        .map(|s| Segment {
            version: s.version,
            adds: s.adds.clone(),
            tombs: s.tombs.clone(),
            weight: s.weight,
        })
        .collect()
}

/// Merge two adjacent segments `a` (older) and `b` (newer) into one, applying
/// newest-wins:
///
/// ```text
///   adds  = (a.adds ∪ b.adds[newer wins]) − b.tombs
///   tombs = (a.tombs ∪ b.tombs)           − b.adds      (dropped if `bottom`)
/// ```
///
/// `a.adds`/`a.tombs` are `Arc`-shared and never mutated — we `dup` `a` and fold
/// `b` into the copy, leaving the originals intact for older readers. When `a` is
/// the bottom segment (`segs[0]`), its merged tombstones suppress nothing older,
/// so they are GC'd (`bottom = true`).
fn merge_segments<V: LsmCell>(
    a: &Segment,
    b: &Segment,
    bottom: bool,
) -> Segment {
    // adds = (a.adds ∪ b.adds) − b.tombs
    let mut add = a.adds.as_ref().map_or_else(V::empty, |m| m.dup());
    if let Some(ba) = &b.adds {
        V::union_into(&mut add, ba);
    }
    if let Some(bt) = &b.tombs {
        add.remove_all(bt);
    }
    let add_n = add.nvals();
    let adds = (add_n > 0).then(|| Arc::new(add));

    // tombs = (a.tombs ∪ b.tombs) − b.adds ; dropped at the bottom (nothing older).
    let (tombs, tomb_n) = if bottom {
        (None, 0)
    } else {
        let mut t = a
            .tombs
            .as_ref()
            .map_or_else(|| Matrix::new(DIM, DIM), |m| m.dup());
        if let Some(bt) = &b.tombs {
            t.element_wise_add(None, None, Some(bt), None);
        }
        if let Some(ba) = &b.adds {
            t.remove_all(ba);
        }
        let n = t.nvals();
        ((n > 0).then(|| Arc::new(t)), n)
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

/// One merge source: a peekable stream of `(row, col, Option<value>)` —
/// `Some(value)` for an add run, `None` for a tombstone — tagged with the
/// segment's version for newest-wins resolution.
struct Src<V: LsmCell> {
    it: Peekable<Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send>>,
    version: u64,
}

/// Build the merge sources for a `[lo, hi]` row scan over a pinned snapshot: one
/// add-source and/or one tomb-source per segment.
fn sources<V: LsmCell>(
    snap: &Snapshot<V>,
    lo: u64,
    hi: u64,
) -> Vec<Src<V>> {
    let mut srcs: Vec<Src<V>> = Vec::with_capacity(snap.segs.len());
    for seg in &snap.segs {
        if let Some(m) = &seg.adds {
            let it: Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send> =
                Box::new(V::scan(m, lo, hi).map(|(r, c, v)| (r, c, Some(v))));
            srcs.push(Src {
                it: it.peekable(),
                version: seg.version,
            });
        }
        if let Some(m) = &seg.tombs {
            let it: Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send> =
                Box::new(<bool as LsmCell>::scan(m, lo, hi).map(|(r, c, _)| (r, c, None)));
            srcs.push(Src {
                it: it.peekable(),
                version: seg.version,
            });
        }
    }
    srcs
}

/// Lazy k-way merge over one band's segments for the inclusive row range
/// `[lo, hi]`. For each `(row, col)`, the **newest** source wins: an add-source
/// yields the cell live, a tombstone source suppresses it. Owns the pinned
/// [`Snapshot`] (so it is `Send` and reclaims by `Arc`-drop).
pub(crate) struct LsmCursor<V: LsmCell> {
    _snap: Snapshot<V>,
    srcs: Vec<Src<V>>,
}

unsafe impl<V: LsmCell> Send for LsmCursor<V> {}

impl<V: LsmCell> LsmCursor<V> {
    pub(crate) fn new(
        snap: Snapshot<V>,
        lo: u64,
        hi: u64,
    ) -> Self {
        let srcs = if lo > hi {
            Vec::new()
        } else {
            sources(&snap, lo, hi)
        };
        Self { _snap: snap, srcs }
    }

    /// The next live cell `(row, col, value)` in range, or `None` when exhausted.
    pub(crate) fn next_cell(&mut self) -> Option<(u64, u64, V)> {
        loop {
            // Pass 1: smallest (row, col) currently peeked across all sources.
            let mut min_rc: Option<(u64, u64)> = None;
            for s in &mut self.srcs {
                if let Some(&(r, c, _)) = s.it.peek() {
                    if min_rc.is_none_or(|m| (r, c) < m) {
                        min_rc = Some((r, c));
                    }
                }
            }
            let rc = min_rc?;

            // Pass 2: among sources at `rc`, the newest version wins; advance them.
            let mut best: Option<(u64, Option<V>)> = None;
            for s in &mut self.srcs {
                if let Some(&(r, c, v)) = s.it.peek() {
                    if (r, c) == rc {
                        if best.is_none_or(|(bv, _)| s.version >= bv) {
                            best = Some((s.version, v));
                        }
                        s.it.next();
                    }
                }
            }
            // `best` is Some because `rc` came from a peeked source.
            if let Some((_, Some(val))) = best {
                return Some((rc.0, rc.1, val));
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
        crate::index::falkordb::test_init_graphblas();
    }

    /// splitmix64 — deterministic pseudo-randomness, no `rand` dep.
    fn sm64(mut z: u64) -> u64 {
        z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Collect `(row, col, value)` cells in `[lo, hi]`, sorted, from a snapshot.
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

    #[test]
    fn add_then_scan() {
        init();
        let m = LsmMatrix::<bool>::new();
        m.commit(1, &[(10, 100, true), (20, 200, true), (10, 101, true)], &[]);
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
        m.commit(1, &[(10, 100, true), (10, 101, true)], &[]);
        m.commit(2, &[], &[(10, 100)]); // delete doc 100's cell at row 10
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 10), vec![(10, 101, true)]);
    }

    #[test]
    fn newest_wins_readd_after_tombstone() {
        init();
        let m = LsmMatrix::<u64>::new();
        m.commit(1, &[(10, 5, 0xAAAA)], &[]); // doc 5 @ row 10, value AAAA
        m.commit(2, &[], &[(10, 5)]); // delete it
        // ...later re-add the *same* cell with a new value (update back).
        m.commit(3, &[(10, 5, 0xBBBB)], &[]);
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 10), vec![(10, 5, 0xBBBB)]); // newest (v3 add) wins
    }

    #[test]
    fn update_moves_to_new_row() {
        init();
        let m = LsmMatrix::<u64>::new();
        m.commit(1, &[(10, 5, 0x11)], &[]); // doc 5 at value-row 10
        m.commit(2, &[(20, 5, 0x11)], &[(10, 5)]); // update: tomb old row, add new row
        let s = m.snapshot();
        assert_eq!(scan(&s, 0, 100), vec![(20, 5, 0x11)]);
        assert_eq!(scan(&s, 10, 10), vec![]); // old row vacated
    }

    #[test]
    fn compaction_merges_and_preserves_results() {
        init();
        let m = LsmMatrix::<bool>::new();
        // out-of-order commits; compaction merges them into the base.
        m.commit(1, &[(50, 1, true)], &[]);
        m.commit(2, &[(10, 2, true)], &[]);
        m.commit(3, &[(30, 3, true)], &[]);
        assert_eq!(m.seg_count(), 1, "equal-weight commits collapse to one segment");
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
        m.commit(1, &[(10, 1, true)], &[]);
        let old = m.snapshot(); // pin version 1
        m.commit(2, &[(20, 2, true)], &[]); // add
        m.commit(3, &[], &[(10, 1)]); // delete the cell the old snapshot sees
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
            m.commit(v, &[(v, v, true)], &[]);
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

    /// Update churn on a fixed doc set: each round tombstones old cells and adds
    /// new ones. Tombstones must be GC'd by bottom-merges, so the resident weight
    /// stays bounded (≈ live set), not growing with the number of rounds.
    #[test]
    fn tombstone_gc_under_churn() {
        init();
        let m = LsmMatrix::<u64>::new();
        const DOCS: u64 = 64;
        let mut version = 0u64;
        // round 0: initial placement at row = doc.
        let mut cur_row: Vec<u64> = (0..DOCS).collect();
        version += 1;
        let adds: Vec<_> = (0..DOCS).map(|d| (cur_row[d as usize], d, version)).collect();
        m.commit(version, &adds, &[]);

        for round in 1..200u64 {
            version += 1;
            let new_row: Vec<u64> = (0..DOCS).map(|d| 1000 + round * DOCS + d).collect();
            let adds: Vec<_> = (0..DOCS).map(|d| (new_row[d as usize], d, version)).collect();
            let tombs: Vec<_> = (0..DOCS).map(|d| (cur_row[d as usize], d)).collect();
            m.commit(version, &adds, &tombs);
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
        // rounds × 64 docs (= 12.8k churned cells).
        assert!(
            m.total_weight() < 4 * DOCS,
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
            m.commit(v, &[(v, v, v)], &[]);
        }
        let pinned = m.snapshot();
        let before = scan(&pinned, 0, 1000);
        assert_eq!(before.len(), 10);
        // Heavy churn + compaction after the pin.
        for v in 11..=2_000u64 {
            m.commit(v, &[(v, v, v)], &[(v - 10, v - 10)]);
        }
        // The pinned snapshot is byte-for-byte unchanged.
        assert_eq!(scan(&pinned, 0, 1000), before);
    }

    /// Differential fuzz: random add/tomb commits vs a reference model
    /// (`(row,col) → (version, Option<value>)`, newest wins). Compaction runs on
    /// every commit, so this also validates that compaction is semantics-
    /// preserving across hundreds of merges.
    #[test]
    fn differential_fuzz_vs_reference() {
        init();
        let m = LsmMatrix::<u64>::new();
        let mut model: BTreeMap<(u64, u64), (u64, Option<u64>)> = BTreeMap::new();
        const ROWS: u64 = 200; // small key space → lots of collisions/updates
        const COLS: u64 = 50;

        for step in 1..=400u64 {
            let version = step;
            let n = (sm64(step) % 8) as usize; // 0..7 ops this commit
            let mut adds: Vec<(u64, u64, u64)> = Vec::new();
            let mut tombs: Vec<(u64, u64)> = Vec::new();
            for k in 0..n {
                let h = sm64(step.wrapping_mul(131) ^ k as u64);
                let row = h % ROWS;
                let col = (h >> 16) % COLS;
                if h & 1 == 0 {
                    let val = h | 1; // nonzero value
                    adds.push((row, col, val));
                    model.insert((row, col), (version, Some(val)));
                } else {
                    tombs.push((row, col));
                    model.insert((row, col), (version, None));
                }
            }
            dedup_last(&mut adds, &mut tombs);
            m.commit(version, &adds, &tombs);

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

    /// Concurrent MVCC: many reader threads scan while a writer commits +
    /// compacts. Each commit `v` appends exactly one new cell `(v, v, v)`, so any
    /// consistent snapshot must observe a *contiguous* doc prefix `{1..k}` — a gap
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
            m.commit(v, &[(v, v, v)], &[]);
        }
        done.store(true, Ordering::Relaxed);
        for r in readers {
            r.join().unwrap();
        }
        assert_eq!(scan(&m.snapshot(), 0, N).len(), N as usize);
    }

    /// Within one commit a cell can appear at most once (runs hold unique cells);
    /// collapse to the last op per cell so the run build and the reference agree.
    fn dedup_last(
        adds: &mut Vec<(u64, u64, u64)>,
        tombs: &mut Vec<(u64, u64)>,
    ) {
        use std::collections::HashMap;
        let mut last: HashMap<(u64, u64), Option<u64>> = HashMap::new();
        for &(r, c, v) in adds.iter() {
            last.insert((r, c), Some(v));
        }
        for &(r, c) in tombs.iter() {
            last.insert((r, c), None);
        }
        adds.clear();
        tombs.clear();
        for ((r, c), v) in last {
            match v {
                Some(v) => adds.push((r, c, v)),
                None => tombs.push((r, c)),
            }
        }
    }
}
