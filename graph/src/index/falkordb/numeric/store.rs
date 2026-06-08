//! In-RAM logical-MVCC store for the numeric POC, built on the engine's
//! GraphBLAS [`VersionedMatrixT`] — the design's shared "matrix core" (R12,
//! `01-mvcc-core.md` §2) — using an **order-preserving banded layout** that
//! maps the encoder's full-`u64` row keys onto legal matrix indices.
//!
//! # Typed cells
//!
//! The store is generic over the matrix cell type [`CellValue`]:
//!
//! - `MatrixStore<bool>` — node indexes. The cell is mere **presence**; a scan
//!   yields the matching doc (node) id.
//! - `MatrixStore<u64>` — edge indexes. The cell **packs the edge's `(src, dst)`
//!   endpoints** ([`compound_key`]); a value scan yields `(edge_id, endpoints)`
//!   inline, so the index *is* the endpoint store — no second structure, no
//!   resolution hop.
//!
//! # Why banding
//!
//! SuiteSparse GraphBLAS packs row/column indices into 60-bit fields, so every
//! matrix index must be `< 2^60`. The order-preserving `f64 → u64` row keys
//! ([`super::NumericEncoder`]) span the **full** `u64` range and routinely
//! exceed `2^60`, so a key cannot be a matrix row directly. We split the 64-bit
//! key into a high **band** selector and a low 60-bit **row**:
//!
//! ```text
//!   band = key >> BAND_BITS     (the high 4 bits → one of 16 bands)
//!   row  = key &  ROW_MASK      (the low 60 bits → a legal matrix index)
//! ```
//!
//! Each band is its own log-structured matrix ([`Layers`]); band *b* covers the
//! contiguous key interval `[b·2^BAND_BITS, (b+1)·2^BAND_BITS)`. Because the row
//! index *is* the key's low bits, **rows are stored in key order**: a range query
//! is a
//! contiguous matrix row-sweep (`iter(lo_row, hi_row)`), not a per-value
//! dictionary lookup. A range that crosses band boundaries fans out to one
//! contiguous sweep per band it touches (≤ [`NUM_BANDS`] of them).
//!
//! # MVCC
//!
//! - The [`NUM_BANDS`] bands are versioned **together** as one [`Snapshot`]
//!   (`Arc<[Arc<Layers<V>>; NUM_BANDS]>`). A reader pins the whole array
//!   (mechanism A) and never locks during the scan; the writer appends an
//!   immutable segment to each touched band ([`commit_layers`] — build + compact,
//!   no growing copy), `Arc`-shares the untouched bands, and swaps the outer
//!   `Arc`. Old readers keep their `Arc` (and the immutable segments it points
//!   at, even across later compactions); reclamation is `Arc`-drop.
//! - There is **no dictionary**: the key → (band, row) map is pure arithmetic
//!   and identical for every reader, so nothing about ordering is versioned.
//! - The **`reverse_id_row_mapping`** (`doc → encoded keys`) is writer-only. Writes are
//!   serialized (one [`Writer`] mutex), so it always reflects the latest
//!   committed matrices and readers never consult it. It lets a value-less
//!   `remove` (the [`super::Index::commit`] shape) clear every cell of a doc
//!   without scanning or knowing the doc's old value.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

use super::lsm::{Layers, LsmCell, LsmCursor, commit_layers, major_compact_layers};
use crate::index::falkordb::id::DocKey;

/// Low bits of an encoded key used as the matrix row index. `60` keeps each row
/// within GraphBLAS's `< 2^60` index limit while needing only the minimum number
/// of bands to cover the full `u64` key space.
const BAND_BITS: u64 = 60;

/// Number of bands = `2^(64 - BAND_BITS)` — how many `BAND_BITS`-wide intervals
/// tile the full 64-bit key space. `16` at `BAND_BITS = 60`.
const NUM_BANDS: usize = 1 << (u64::BITS as u64 - BAND_BITS);

/// Mask selecting the low [`BAND_BITS`] of a key (the within-band row index).
///
/// Each band matrix is `2^BAND_BITS × 2^BAND_BITS` (rows = a key's low
/// `BAND_BITS`; columns = the doc-id space, capped at GraphBLAS's
/// `GrB_INDEX_MAX`). That dimension is a *logical* bound, not an allocation:
/// GraphBLAS matrices are hypersparse, so memory scales with the stored entries,
/// not with `nrows × ncols`. The LSM owns this dimension internally (its `DIM`).
const ROW_MASK: u64 = (1u64 << BAND_BITS) - 1;

/// The band a key falls in (its high bits).
#[inline]
fn band_of(key: u64) -> usize {
    (key >> BAND_BITS) as usize
}

/// The within-band matrix row of a key (its low [`BAND_BITS`] bits).
#[inline]
fn row_of(key: u64) -> u64 {
    key & ROW_MASK
}

/// The [`NUM_BANDS`] log-structured bands that make up one logical index version.
/// Each band is an `Arc<Layers<V>>` so an untouched band is shared by `Arc` across
/// versions; only touched bands get a fresh [`Layers`] on commit.
type Bands<V> = [Arc<Layers<V>>; NUM_BANDS];

/// One published, immutable index version — what a reader pins (mechanism A).
/// All bands are shared behind a single outer `Arc`, so a snapshot is atomic
/// across bands; each band's segments are immutable, isolating committed state
/// from the writer's next version and from later compactions.
pub(crate) type Snapshot<V> = Arc<Bands<V>>;

/// A fresh, empty set of bands (no segments — hypersparse, no allocation).
fn new_bands<V: LsmCell>() -> Bands<V> {
    std::array::from_fn(|_| Arc::new(Layers::<V>::empty()))
}

/// Writer-only state. Commits are serialized through this mutex, so it always
/// reflects the latest committed matrices.
struct Writer {
    /// Reverse mapping `doc id → the encoded keys it currently occupies` (each
    /// key pins one `(band, row)` cell). Lets a value-less `remove` clear every
    /// cell of a doc without scanning the matrices or knowing the doc's old
    /// value — the matrix iterates by row, so locating a doc's cells the other
    /// way would mean sweeping every band.
    reverse_id_row_mapping: HashMap<DocKey, Vec<u64>>,
    /// Committed logical version (== the graph commit version).
    version: u64,
}

/// The numeric POC's logical-MVCC store: a banded, order-preserving
/// `(row, doc) → cell` matrix set plus writer-only bookkeeping, generic over the
/// cell type `V` (presence for nodes, packed endpoints for edges).
pub(crate) struct MatrixStore<V: LsmCell> {
    /// Latest committed bands. Swapped per commit; readers clone the
    /// `Arc` to pin their version.
    committed: RwLock<Snapshot<V>>,
    /// Serialized writer state.
    ///
    /// A commit is a read-modify-write — snapshot the latest version, apply the
    /// mutation, publish the new version — so two concurrent commits could
    /// `dup()` the same base and the later publish would clobber the earlier one
    /// (a lost update), while both mutate `reverse_id_row_mapping` at once. This
    /// mutex makes the whole commit atomic and is the sole writer of that map;
    /// readers never take it (they only clone the `committed` `Arc`), so reads
    /// stay lock-free.
    ///
    /// The engine already serializes writes upstream, so in practice this lock is
    /// uncontended. We hold it anyway rather than **silently depending on that
    /// external invariant**: `MatrixStore` must be correct on its own terms, not
    /// because of how today's callers happen to be scheduled.
    writer: Mutex<Writer>,
}

impl<V: LsmCell> Default for MatrixStore<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: LsmCell> MatrixStore<V> {
    /// A fresh, empty store at version 0.
    pub(crate) fn new() -> Self {
        Self {
            committed: RwLock::new(Arc::new(new_bands::<V>())),
            writer: Mutex::new(Writer {
                reverse_id_row_mapping: HashMap::new(),
                version: 0,
            }),
        }
    }

    /// An `Arc` share of the latest committed bands — the immutable view a
    /// reader scans (mechanism A). Cheap: one `Arc` clone, no lock held after.
    pub(crate) fn snapshot(&self) -> Snapshot<V> {
        Arc::clone(&self.committed.read())
    }

    /// The committed logical version.
    #[cfg(test)]
    pub(crate) fn version(&self) -> u64 {
        self.writer.lock().version
    }

    /// Approximate resident bytes across all band matrices.
    pub(crate) fn memory_usage(&self) -> usize {
        self.committed
            .read()
            .iter()
            .map(|b| b.memory_usage())
            .sum()
    }

    /// Apply one commit's mutations, publishing a new immutable version at
    /// `version`. Each add is `(doc, encoded row keys, cell value)`: the cell
    /// value is presence (`true`) for node indexes and the doc's packed
    /// endpoints for edge indexes — the same value is written at every one of the
    /// doc's rows. `remove` are docs whose every cell is cleared. A doc in `adds`
    /// is tombstoned first (so re-indexing a changed value moves it to its new
    /// rows rather than accumulating stale ones), making the operation safe
    /// whether or not the caller also listed it in `remove`.
    pub(crate) fn commit(
        &self,
        version: u64,
        adds: &[(DocKey, Vec<u64>, V)],
        remove: &[DocKey],
    ) {
        // Serialize the whole commit (single writer).
        let mut w = self.writer.lock();

        // Group this commit's cells per band: add cells `(row, col, value)` and
        // tombstone cells `(row, col)`. The LSM never mutates published state, so
        // a doc delete/update is resolved here (via `reverse_id_row_mapping`) into
        // the exact old cells to tombstone.
        let mut band_adds: [Vec<(u64, u64, V)>; NUM_BANDS] = std::array::from_fn(|_| Vec::new());
        let mut band_tombs: [Vec<(u64, u64)>; NUM_BANDS] = std::array::from_fn(|_| Vec::new());

        // Value-less removals: tombstone each doc's currently occupied cells.
        for &doc in remove {
            if let Some(keys) = w.reverse_id_row_mapping.remove(&doc) {
                for k in keys {
                    band_tombs[band_of(k)].push((row_of(k), doc));
                }
            }
        }

        // Additions: tombstone the doc's prior placement, then add its new rows.
        for (doc, keys, cell) in adds {
            if let Some(prev) = w.reverse_id_row_mapping.remove(doc) {
                for k in prev {
                    band_tombs[band_of(k)].push((row_of(k), *doc));
                }
            }
            if keys.is_empty() {
                continue;
            }
            // Dedup so an array repeating a value records each row once.
            let mut owned = keys.clone();
            owned.sort_unstable();
            owned.dedup();
            for &k in &owned {
                band_adds[band_of(k)].push((row_of(k), *doc, *cell));
            }
            w.reverse_id_row_mapping.insert(*doc, owned);
        }

        // A cell re-added this commit (e.g. an update that keeps a row, or a
        // remove+re-add of the same doc) must NOT be tombstoned this commit:
        // both runs carry the same version, so drop the superseded tombstone.
        for b in 0..NUM_BANDS {
            if band_tombs[b].is_empty() || band_adds[b].is_empty() {
                continue;
            }
            let added: HashSet<(u64, u64)> =
                band_adds[b].iter().map(|&(r, c, _)| (r, c)).collect();
            band_tombs[b].retain(|rc| !added.contains(rc));
        }

        // Build the next version: untouched bands are shared by `Arc`; touched
        // bands get a fresh `Layers` (append + compact, no growing copy).
        let cur = self.committed.read().clone();
        let next: Bands<V> = std::array::from_fn(|b| {
            if band_adds[b].is_empty() && band_tombs[b].is_empty() {
                Arc::clone(&cur[b])
            } else {
                Arc::new(commit_layers::<V>(
                    &cur[b],
                    version,
                    &band_adds[b],
                    &band_tombs[b],
                ))
            }
        });

        // Publish atomically, then advance the version.
        *self.committed.write() = Arc::new(next);
        w.version = version;
    }

    /// **Major compaction**: collapse every band's segments into a single
    /// tombstone-free base, minimizing resident matrices (fragmentation) and read
    /// amplification. A logical no-op (same data), so it keeps the committed
    /// version; serialized with `commit` via the writer lock and published as one
    /// atomic snapshot swap, so concurrent readers and writers are unaffected.
    pub(crate) fn major_compact(&self) {
        let _w = self.writer.lock();
        let cur = self.committed.read().clone();
        let next: Bands<V> =
            std::array::from_fn(|b| Arc::new(major_compact_layers::<V>(&cur[b])));
        *self.committed.write() = Arc::new(next);
    }
}

/// A lazy, `Send`, self-contained cursor over the docs whose encoded key falls
/// in the inclusive range `[lo, hi]` of one pinned snapshot — the **scan** form
/// (presence only), used by node indexes.
///
/// It **owns** a [`Snapshot`] share (so it is `Send` and reclaims by `Arc`-drop
/// — no borrow of the store, no lock held). Because rows are key-ordered, the
/// range is a contiguous row-sweep per band: the cursor walks bands
/// `band_of(lo) ..= band_of(hi)`, opening one [`MatrixIter`] per band over that
/// band's slice of the range, and yields each `(row, doc)` cell's doc id. A doc
/// indexed under several in-range keys (an array property) is yielded once per
/// occupied row; the scan layer dedups.
pub(crate) struct MatrixRangeCursor<V: LsmCell> {
    bands: Snapshot<V>,
    lo: u64,
    hi: u64,
    /// First and last band the range touches (inclusive).
    band_lo: usize,
    band_hi: usize,
    /// Band whose cursor is currently open (or next to open).
    band: usize,
    /// LSM merge cursor over the current band's in-range rows.
    cursor: Option<LsmCursor<V>>,
    done: bool,
}

impl<V: LsmCell> MatrixRangeCursor<V> {
    /// A cursor over `bands` restricted to the inclusive encoded-key range
    /// `[lo, hi]`. An empty range (`lo > hi`) yields nothing.
    pub(crate) fn new(
        bands: Snapshot<V>,
        lo: u64,
        hi: u64,
    ) -> Self {
        let done = lo > hi;
        let band_lo = band_of(lo);
        let band_hi = band_of(hi);
        Self {
            bands,
            lo,
            hi,
            band_lo,
            band_hi,
            band: band_lo,
            cursor: None,
            done,
        }
    }

    /// The next doc id in range, or `None` when exhausted. Walks each band's
    /// contiguous in-range row slice (one LSM merge cursor per band), advancing to
    /// the next band when one is drained.
    pub(crate) fn next_id(&mut self) -> Option<DocKey> {
        loop {
            if self.done {
                return None;
            }
            match self.cursor.as_mut() {
                // Drain the current band's merge cursor.
                Some(c) => match c.next_cell() {
                    Some((_row, doc, _value)) => return Some(doc),
                    None => {
                        self.cursor = None;
                        self.band += 1;
                    }
                },
                // Open the next band's in-range row sweep. The start row is the
                // key's row only in the first band (0 thereafter); the end row
                // is the key's row only in the last band (ROW_MASK before).
                None => {
                    if self.band > self.band_hi {
                        self.done = true;
                        return None;
                    }
                    let start = if self.band == self.band_lo {
                        row_of(self.lo)
                    } else {
                        0
                    };
                    let end = if self.band == self.band_hi {
                        row_of(self.hi)
                    } else {
                        ROW_MASK
                    };
                    self.cursor = Some(LsmCursor::new(Arc::clone(&self.bands[self.band]), start, end));
                }
            }
        }
    }
}

/// The **value** form of [`MatrixRangeCursor`], used by edge indexes: yields
/// `(doc, cell_value)` so the matched edge's packed `(src, dst)` endpoints come
/// straight from the index cell — no separate resolution structure. Only
/// available for `u64` (value-carrying) stores.
pub(crate) struct ValueRangeCursor {
    bands: Snapshot<u64>,
    lo: u64,
    hi: u64,
    band_lo: usize,
    band_hi: usize,
    band: usize,
    cursor: Option<LsmCursor<u64>>,
    done: bool,
}

impl ValueRangeCursor {
    pub(crate) fn new(
        bands: Snapshot<u64>,
        lo: u64,
        hi: u64,
    ) -> Self {
        let done = lo > hi;
        let band_lo = band_of(lo);
        let band_hi = band_of(hi);
        Self {
            bands,
            lo,
            hi,
            band_lo,
            band_hi,
            band: band_lo,
            cursor: None,
            done,
        }
    }

    /// The next `(doc, value)` in range, or `None` when exhausted.
    pub(crate) fn next_value(&mut self) -> Option<(DocKey, u64)> {
        loop {
            if self.done {
                return None;
            }
            match self.cursor.as_mut() {
                Some(c) => match c.next_cell() {
                    Some((_row, doc, value)) => return Some((doc, value)),
                    None => {
                        self.cursor = None;
                        self.band += 1;
                    }
                },
                None => {
                    if self.band > self.band_hi {
                        self.done = true;
                        return None;
                    }
                    let start = if self.band == self.band_lo {
                        row_of(self.lo)
                    } else {
                        0
                    };
                    let end = if self.band == self.band_hi {
                        row_of(self.hi)
                    } else {
                        ROW_MASK
                    };
                    self.cursor = Some(LsmCursor::new(Arc::clone(&self.bands[self.band]), start, end));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Commit `pairs` (doc → encoded keys) at version 1 into a fresh node store.
    fn store_with(pairs: &[(DocKey, &[u64])]) -> MatrixStore<bool> {
        crate::index::falkordb::test_init_graphblas();
        let s = MatrixStore::<bool>::new();
        let adds: Vec<(DocKey, Vec<u64>, bool)> =
            pairs.iter().map(|(d, ks)| (*d, ks.to_vec(), true)).collect();
        s.commit(1, &adds, &[]);
        s
    }

    /// Collect, sorted, the docs whose key falls in `[lo, hi]` of `snap`.
    fn collect(
        snap: &Snapshot<bool>,
        lo: u64,
        hi: u64,
    ) -> Vec<DocKey> {
        let mut c = MatrixRangeCursor::new(Arc::clone(snap), lo, hi);
        let mut out = Vec::new();
        while let Some(id) = c.next_id() {
            out.push(id);
        }
        out.sort_unstable();
        out
    }

    /// Compose a key in band `b` at within-band row `row` (band-count agnostic).
    fn key(
        b: u64,
        row: u64,
    ) -> u64 {
        (b << BAND_BITS) | row
    }

    #[test]
    fn point_and_range_select_expected_docs() {
        // (doc, keys): docs 200 and 201 share key 20, so a point lookup on 20
        // returns both; docs 100/300 sit at keys 10/30.
        let s = store_with(&[(100, &[10]), (200, &[20]), (201, &[20]), (300, &[30])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 20, 20), vec![200, 201]);
        assert_eq!(collect(&snap, 10, 20), vec![100, 200, 201]);
        assert_eq!(collect(&snap, 0, 1000), vec![100, 200, 201, 300]);
        assert_eq!(collect(&snap, 11, 19), Vec::<DocKey>::new());
    }

    #[test]
    fn empty_range_yields_nothing() {
        let s = store_with(&[(10, &[1])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 20, 10), Vec::<DocKey>::new());
    }

    #[test]
    fn array_doc_occupies_many_rows() {
        // One doc indexed under three keys (an array property).
        let s = store_with(&[(7, &[1, 5, 9])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 1, 1), vec![7]);
        assert_eq!(collect(&snap, 5, 5), vec![7]);
        assert_eq!(collect(&snap, 9, 9), vec![7]);
        // The doc appears once per occupied in-range row.
        assert_eq!(collect(&snap, 1, 9), vec![7, 7, 7]);
    }

    #[test]
    fn remove_tombstones_every_row_of_doc() {
        let s = store_with(&[(7, &[1, 5, 9]), (8, &[5])]);
        s.commit(2, &[], &[7]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 0, 100), vec![8]); // doc 7 gone everywhere
        assert_eq!(collect(&snap, 5, 5), vec![8]); // row 5 still has doc 8
        assert_eq!(collect(&snap, 1, 1), Vec::<DocKey>::new()); // row emptied
    }

    #[test]
    fn update_moves_doc_to_new_rows() {
        let s = store_with(&[(7, &[10])]);
        // Re-index doc 7 under a new key; the old row must be vacated even though
        // the caller did not list 7 in `remove`.
        s.commit(2, &[(7, vec![20], true)], &[]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 10, 10), Vec::<DocKey>::new());
        assert_eq!(collect(&snap, 20, 20), vec![7]);
    }

    #[test]
    fn reader_snapshot_is_isolated_from_later_commit() {
        let s = store_with(&[(7, &[10])]);
        let old = s.snapshot(); // pin version 1
        s.commit(2, &[(8, vec![10], true)], &[]); // add doc 8 at the same key/row
        let new = s.snapshot();
        assert_eq!(collect(&old, 10, 10), vec![7]); // old reader unaffected
        assert_eq!(collect(&new, 10, 10), vec![7, 8]); // new reader sees add
        assert_eq!(s.version(), 2);
    }

    #[test]
    fn interleaved_inserts_preserve_key_order() {
        // Insert out of key order across commits; ranges must still be correct
        // because rows are stored in key order regardless of insertion order.
        crate::index::falkordb::test_init_graphblas();
        let s = MatrixStore::<bool>::new();
        s.commit(1, &[(1, vec![50], true)], &[]);
        s.commit(2, &[(2, vec![10], true)], &[]);
        s.commit(3, &[(3, vec![30], true)], &[]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 10, 30), vec![2, 3]);
        assert_eq!(collect(&snap, 0, 100), vec![1, 2, 3]);
        assert_eq!(collect(&snap, 50, 50), vec![1]);
    }

    #[test]
    fn ranges_span_band_boundaries() {
        // Three docs in three different bands. Exercises the multi-band sweep
        // and the per-band start/end row clamping.
        let s = store_with(&[
            (100, &[key(0, 5)]),
            (200, &[key(1, 7)]),
            (300, &[key(2, 3)]),
        ]);
        let snap = s.snapshot();
        // Full sweep across all three bands.
        assert_eq!(collect(&snap, key(0, 0), key(3, 0)), vec![100, 200, 300]);
        // All of band 1, but only row 0 of band 2 → excludes doc 300 (row 3).
        assert_eq!(collect(&snap, key(1, 0), key(2, 0)), vec![200]);
        // Point lookup inside band 1.
        assert_eq!(collect(&snap, key(1, 7), key(1, 7)), vec![200]);
        // Band 0 only (up to its top row).
        assert_eq!(collect(&snap, key(0, 0), key(0, ROW_MASK)), vec![100]);
    }

    /// Edge (`u64`) store: the cell packs an opaque value that a value scan
    /// returns inline with each matched doc.
    #[test]
    fn value_store_yields_packed_cell() {
        crate::index::falkordb::test_init_graphblas();
        let s = MatrixStore::<u64>::new();
        // (doc, keys, packed-endpoints).
        s.commit(
            1,
            &[(5, vec![10], 0xAAAA_BBBB), (9, vec![20], 0x1111_2222)],
            &[],
        );
        let snap = s.snapshot();
        let mut c = ValueRangeCursor::new(Arc::clone(&snap), 0, 100);
        let mut got = Vec::new();
        while let Some(dv) = c.next_value() {
            got.push(dv);
        }
        got.sort_unstable();
        assert_eq!(got, vec![(5, 0xAAAA_BBBB), (9, 0x1111_2222)]);
        // Point lookup returns the one doc + its packed value.
        let mut c = ValueRangeCursor::new(Arc::clone(&snap), 20, 20);
        assert_eq!(c.next_value(), Some((9, 0x1111_2222)));
        assert_eq!(c.next_value(), None);
    }

    /// Scaling probe (run with `--release --nocapture`): does the banded
    /// `VersionedMatrix` index degrade with scale and/or out-of-order inserts?
    /// Ingests N records in commit-batches (like the engine) with row keys
    /// either ascending (`in_order`) or uniformly random (`random`), then times
    /// a 10% range scan — against a `BTreeMap` baseline (an order-independent
    /// tree, like a classic numeric index). The ingest `us/rec` trend across N
    /// exposes super-linear cost (the per-commit `element_wise_add` fold).
    #[test]
    #[ignore = "scaling probe; run explicitly with --release --nocapture"]
    fn scaling_in_order_vs_random() {
        crate::index::falkordb::test_init_graphblas();
        // splitmix64 — deterministic pseudo-random keys, no rand dep.
        fn sm64(mut z: u64) -> u64 {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
        let batch = 10_000u64;
        let sizes = [1_000_000u64, 2_000_000, 4_000_000, 8_000_000];

        eprintln!("\n=== MatrixStore<u64> — commit batches of {batch} ===");
        for &n in &sizes {
            let keyspace = n;
            for order in ["in_order", "random"] {
                let store = MatrixStore::<u64>::new();
                let t0 = std::time::Instant::now();
                let (mut i, mut ver) = (0u64, 0u64);
                while i < n {
                    ver += 1;
                    let end = (i + batch).min(n);
                    let adds: Vec<(DocKey, Vec<u64>, u64)> = (i..end)
                        .map(|d| {
                            let key = if order == "in_order" { d } else { sm64(d) % keyspace };
                            (d, vec![key], d)
                        })
                        .collect();
                    store.commit(ver, &adds, &[]);
                    i = end;
                }
                let ingest_ms = t0.elapsed().as_secs_f64() * 1000.0;
                let snap = store.snapshot();
                let t1 = std::time::Instant::now();
                let mut c = ValueRangeCursor::new(Arc::clone(&snap), 0, keyspace / 10);
                let mut cnt = 0u64;
                while c.next_value().is_some() {
                    cnt += 1;
                }
                let scan_ms = t1.elapsed().as_secs_f64() * 1000.0;
                eprintln!(
                    "N={n:>9} {order:<9} ingest={ingest_ms:>9.0}ms ({:>6.2} us/rec)  scan10%={scan_ms:>8.2}ms ({cnt} matched)",
                    ingest_ms * 1000.0 / n as f64,
                );
            }
        }

        eprintln!("=== BTreeMap<(key,doc)> baseline (order-independent tree) ===");
        use std::collections::BTreeMap;
        for &n in &sizes {
            let keyspace = n;
            for order in ["in_order", "random"] {
                let mut m: BTreeMap<(u64, u64), ()> = BTreeMap::new();
                let t0 = std::time::Instant::now();
                for d in 0..n {
                    let key = if order == "in_order" { d } else { sm64(d) % keyspace };
                    m.insert((key, d), ());
                }
                let ingest_ms = t0.elapsed().as_secs_f64() * 1000.0;
                let t1 = std::time::Instant::now();
                let cnt = m.range((0, 0)..(keyspace / 10, u64::MAX)).count();
                let scan_ms = t1.elapsed().as_secs_f64() * 1000.0;
                eprintln!(
                    "N={n:>9} {order:<9} ingest={ingest_ms:>9.0}ms ({:>6.2} us/rec)  scan10%={scan_ms:>8.2}ms ({cnt} matched)",
                    ingest_ms * 1000.0 / n as f64,
                );
            }
        }
    }

    /// Amortization probe (run with `--release --nocapture`): at fixed N, sweep
    /// the commit-batch size. Bigger batches = fewer per-commit MVCC copies +
    /// delta assemblies; a single commit is full deferral (one fold). If ingest
    /// falls sharply as the batch grows, the quadratic floor is per-commit
    /// versioned-delta work (copy + assemble), not the data structure itself.
    #[test]
    #[ignore = "amortization probe; run explicitly with --release --nocapture"]
    fn batch_amortization_sweep() {
        crate::index::falkordb::test_init_graphblas();
        fn sm64(mut z: u64) -> u64 {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
        let n = 4_000_000u64;
        let keyspace = n;
        eprintln!("\n=== N={n}: ingest vs commit-batch size (MatrixStore<u64>) ===");
        for &batch in &[10_000u64, 100_000, 1_000_000, 4_000_000] {
            for order in ["in_order", "random"] {
                let store = MatrixStore::<u64>::new();
                let t0 = std::time::Instant::now();
                let (mut i, mut ver) = (0u64, 0u64);
                while i < n {
                    ver += 1;
                    let end = (i + batch).min(n);
                    let adds: Vec<(DocKey, Vec<u64>, u64)> = (i..end)
                        .map(|d| {
                            let key = if order == "in_order" { d } else { sm64(d) % keyspace };
                            (d, vec![key], d)
                        })
                        .collect();
                    store.commit(ver, &adds, &[]);
                    i = end;
                }
                let ms = t0.elapsed().as_secs_f64() * 1000.0;
                eprintln!(
                    "batch={batch:>9} {order:<9} ingest={ms:>8.0}ms ({:>5.2} us/rec, {ver} commits)",
                    ms * 1000.0 / n as f64,
                );
            }
        }
    }
}
