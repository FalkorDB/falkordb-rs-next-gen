//! Log-structured matrix store — the LSM core (milestone **M1**).
//!
//! Replaces the single copy-on-write delta of [`super::store`] (which re-copies a
//! growing delta on every commit → super-linear ingest) with an **append-only
//! list of immutable run matrices**. A commit *appends* a small immutable run; it
//! never mutates or copies anything that grows. MVCC isolation comes from
//! immutability: published runs/base are frozen and `Arc`-shared, so a reader's
//! pinned snapshot is stable while later commits add new runs.
//!
//! ```text
//!   one band (rows = encoded-key low 60 bits, cols = doc id)
//!     base   : Arc<Matrix>            folded live cells (cell type V)
//!     runs   : [(ver, Arc<Matrix>)]   immutable add-runs, ascending version
//!     tombs  : [(ver, Arc<Matrix>)]   immutable tombstone runs (BOOL)
//!
//!   effective = (base ∪ ⋃ runs) − ⋃ tombs        (newest version wins per cell)
//! ```
//!
//! Reads are a k-way merge over `base + runs − tombs` (see [`LsmCursor`]).
//! Compaction (bounding the run count) lands in M2; this milestone only appends,
//! so the run list grows — enough to prove correctness + MVCC in isolation.

use std::iter::Peekable;
use std::marker::PhantomData;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::graph::graphblas::matrix::{Matrix, New};

/// One band's matrix dimension: `2^60`. Rows are an encoded key's low 60 bits,
/// columns are doc ids — both `< 2^60` (GraphBLAS's index ceiling). Banding (the
/// high 4 bits) is handled one level up, in the store (M3); a single
/// [`LsmMatrix`] is one band.
const DIM: u64 = 1 << 60;

/// Cell type of an LSM matrix: presence (`bool`, node indexes) or value-carrying
/// (`u64`, edge indexes packing endpoints). Provides the two type-specific
/// primitives the LSM needs — bulk-build an immutable run, and scan a run's row
/// range yielding the cell value. Local to the index (the engine's `CellValue`
/// is untouched).
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
}

/// One published, immutable version of a band: a base plus version-tagged add and
/// tombstone runs. Everything is `Arc`-shared and never mutated after publish, so
/// a reader holding this is fully isolated from later commits.
pub(crate) struct Layers<V: LsmCell> {
    base: Arc<Matrix>,
    /// Add-runs, ascending version (newest last). `base` is logically version 0.
    runs: Vec<(u64, Arc<Matrix>)>,
    /// Tombstone runs (always `BOOL`), ascending version.
    tombs: Vec<(u64, Arc<Matrix>)>,
    _v: PhantomData<V>,
}

// `Layers`/`LsmMatrix`/`LsmCursor` hold GraphBLAS matrices behind `Arc`; they are
// immutable once published and only read concurrently, so sharing is safe — same
// posture as the engine's `VersionedMatrix`.
unsafe impl<V: LsmCell> Send for Layers<V> {}
unsafe impl<V: LsmCell> Sync for Layers<V> {}

/// A published snapshot of one band — what a reader pins (mechanism A).
pub(crate) type Snapshot<V> = Arc<Layers<V>>;

/// One band's log-structured matrix: a single published [`Layers`] swapped under
/// a lock on commit. Readers clone the `Arc` (lock-free after); the writer builds
/// a new immutable run and publishes a new `Layers` that shares the old base +
/// runs by `Arc`.
pub(crate) struct LsmMatrix<V: LsmCell> {
    committed: RwLock<Snapshot<V>>,
}

impl<V: LsmCell> Default for LsmMatrix<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: LsmCell> LsmMatrix<V> {
    /// An empty band at version 0.
    pub(crate) fn new() -> Self {
        Self {
            committed: RwLock::new(Arc::new(Layers {
                base: Arc::new(V::build_run(&[], &[], &[])),
                runs: Vec::new(),
                tombs: Vec::new(),
                _v: PhantomData,
            })),
        }
    }

    /// Pin the latest published version — the immutable view a reader scans.
    pub(crate) fn snapshot(&self) -> Snapshot<V> {
        Arc::clone(&self.committed.read())
    }

    /// Append one commit's changes as immutable runs and publish a new version.
    /// `adds` are `(row, col, value)` cells; `tombs` are `(row, col)` cells to
    /// suppress (the caller — the store — resolves doc deletes/updates into the
    /// exact old cells). The new version shares the old base + existing runs by
    /// `Arc`; only the (small) run lists are cloned. No growing structure is
    /// copied, so a commit is `O(this commit's changes)`.
    pub(crate) fn commit(
        &self,
        version: u64,
        adds: &[(u64, u64, V)],
        tombs: &[(u64, u64)],
    ) {
        let add_run = (!adds.is_empty()).then(|| {
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
        let tomb_run = (!tombs.is_empty()).then(|| {
            let mut rows = Vec::with_capacity(tombs.len());
            let mut cols = Vec::with_capacity(tombs.len());
            for &(r, c) in tombs {
                rows.push(r);
                cols.push(c);
            }
            Arc::new(<bool as LsmCell>::build_run(&rows, &cols, &[]))
        });

        let cur = self.committed.read().clone();
        let mut runs = cur.runs.clone();
        let mut tombs_v = cur.tombs.clone();
        if let Some(r) = add_run {
            runs.push((version, r));
        }
        if let Some(t) = tomb_run {
            tombs_v.push((version, t));
        }
        let next = Layers {
            base: Arc::clone(&cur.base),
            runs,
            tombs: tombs_v,
            _v: PhantomData,
        };
        *self.committed.write() = Arc::new(next);
    }

    /// Number of live runs (excludes the base). Used by tests / future compaction.
    #[cfg(test)]
    pub(crate) fn run_count(&self) -> usize {
        self.committed.read().runs.len()
    }
}

/// Build the source iterators for a `[lo, hi]` row scan over a pinned snapshot:
/// base (version 0) + each add-run (`Some(value)`) + each tomb-run (`None`),
/// each peekable and version-tagged.
fn sources<V: LsmCell>(
    snap: &Snapshot<V>,
    lo: u64,
    hi: u64,
) -> Vec<Src<V>> {
    let mut srcs: Vec<Src<V>> = Vec::with_capacity(1 + snap.runs.len() + snap.tombs.len());
    let add = |m: &Matrix| -> Peekable<Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send>> {
        let it: Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send> =
            Box::new(V::scan(m, lo, hi).map(|(r, c, v)| (r, c, Some(v))));
        it.peekable()
    };
    srcs.push(Src {
        it: add(&snap.base),
        version: 0,
    });
    for (ver, m) in &snap.runs {
        srcs.push(Src {
            it: add(m),
            version: *ver,
        });
    }
    for (ver, m) in &snap.tombs {
        let it: Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send> =
            Box::new(<bool as LsmCell>::scan(m, lo, hi).map(|(r, c, _)| (r, c, None)));
        srcs.push(Src {
            it: it.peekable(),
            version: *ver,
        });
    }
    srcs
}

struct Src<V: LsmCell> {
    it: Peekable<Box<dyn Iterator<Item = (u64, u64, Option<V>)> + Send>>,
    version: u64,
}

/// Lazy k-way merge over one band's `base + runs − tombs` for the inclusive row
/// range `[lo, hi]`. For each `(row, col)`, the **newest** source wins: an
/// add-source yields the cell live, a tombstone source suppresses it. Owns the
/// pinned [`Snapshot`] (so it is `Send` and reclaims by `Arc`-drop).
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
        let srcs = if lo > hi { Vec::new() } else { sources(&snap, lo, hi) };
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
        assert_eq!(scan(&s, 0, 100), vec![(10, 100, true), (10, 101, true), (20, 200, true)]);
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
    fn many_runs_merge() {
        init();
        let m = LsmMatrix::<bool>::new();
        // out-of-order commits across many runs (no compaction yet).
        m.commit(1, &[(50, 1, true)], &[]);
        m.commit(2, &[(10, 2, true)], &[]);
        m.commit(3, &[(30, 3, true)], &[]);
        assert_eq!(m.run_count(), 3);
        let s = m.snapshot();
        assert_eq!(scan(&s, 10, 30), vec![(10, 2, true), (30, 3, true)]);
        assert_eq!(scan(&s, 0, 100), vec![(10, 2, true), (30, 3, true), (50, 1, true)]);
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

    /// Differential fuzz: random add/tomb commits vs a reference model
    /// (`(row,col) → (version, Option<value>)`, newest wins). After each commit,
    /// random range scans must match the reference's live cells.
    #[test]
    fn differential_fuzz_vs_reference() {
        init();
        let m = LsmMatrix::<u64>::new();
        // reference: latest (version, value-or-tombstone) per cell.
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
            // Dedup within-commit cell collisions (a run can't hold a dup cell);
            // keep the last op for each cell this commit, matching `model`.
            dedup_last(&mut adds, &mut tombs);
            m.commit(version, &adds, &tombs);

            // Check a few random ranges against the reference.
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

    /// Concurrent MVCC: many reader threads scan while a writer commits. Each
    /// commit `v` appends exactly one new cell `(v, v, v)`, so any consistent
    /// snapshot must observe a *contiguous* doc prefix `{1..k}` — a gap would be
    /// a torn / non-isolated read. Also exercises the lock-free `Arc` snapshot +
    /// `Arc`-swap commit path under contention (no UB / panics).
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
                        // Docs must be exactly the contiguous prefix {1..k}.
                        for (i, &(r, c, v)) in cells.iter().enumerate() {
                            assert_eq!(c, i as u64 + 1, "non-contiguous snapshot → torn read");
                            assert_eq!((r, v), (c, c), "cell row/value inconsistent with commit");
                        }
                        // A reader's observed version only moves forward.
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
