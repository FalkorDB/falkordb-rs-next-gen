//! An ordered `(value → id)` store on a log-structured GraphBLAS matrix
//! ([`super`]) — an **order-preserving banded layout** that maps full-`u64` row
//! keys onto legal matrix indices.
//!
//! # Typed cells
//!
//! Generic over the matrix cell type [`LsmCell`]:
//!
//! - `BandedLsmStore<bool>` — the cell is mere **presence**; a scan yields the
//!   matching id.
//! - `BandedLsmStore<u64>` — the cell carries a caller-defined `u64` payload; a
//!   value scan yields `(id, payload)` inline, so a caller that packs related
//!   data into the cell needs no second structure and no resolution hop.
//!
//! # Why banding
//!
//! SuiteSparse GraphBLAS packs row/column indices into 60-bit fields, so every
//! matrix index must be `< 2^60`. Order-preserving row keys span the **full**
//! `u64` range and routinely exceed `2^60`, so a key cannot be a matrix row
//! directly. We split the 64-bit key into a high **band** selector and a low
//! 60-bit **row**:
//!
//! ```text
//!   band = key >> BAND_BITS     (the high 4 bits → one of 16 bands)
//!   row  = key &  ROW_MASK      (the low 60 bits → a legal matrix index)
//! ```
//!
//! Each band is its own log-structured matrix ([`Layers`]); band *b* covers the
//! contiguous key interval `[b·2^BAND_BITS, (b+1)·2^BAND_BITS)`. Because the row
//! index *is* the key's low bits, **rows are stored in key order**: a range query
//! is a contiguous matrix row-sweep (`iter(lo_row, hi_row)`), not a per-value
//! dictionary lookup. A range that crosses band boundaries fans out to one
//! contiguous sweep per band it touches (≤ [`NUM_BANDS`] of them).
//!
//! # MVCC
//!
//! - The [`NUM_BANDS`] bands are versioned **together** as one [`Snapshot`]
//!   (`Arc<[Arc<Layers<V>>; NUM_BANDS]>`). A reader pins the whole array and never
//!   locks during the scan; the writer appends an immutable segment to each
//!   touched band ([`commit_layers`] — build + compact, no growing copy),
//!   `Arc`-shares the untouched bands, and swaps the outer `Arc`. Old readers keep
//!   their `Arc` (and the immutable segments it points at, even across later
//!   compactions); reclamation is `Arc`-drop.
//! - There is **no dictionary**: the key → (band, row) map is pure arithmetic
//!   and identical for every reader, so nothing about ordering is versioned.
//! - The **`reverse_id_row_mapping`** (`id → row keys`) is writer-only. Writes
//!   are serialized (one [`Writer`] mutex), so it always reflects the latest
//!   committed matrices and readers never consult it. It lets a value-less
//!   `remove` clear every cell of an id without scanning or knowing its old
//!   value.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

use super::{Layers, LsmCell, LsmCursor, commit_layers, major_compact_layers};

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
/// `BAND_BITS`; columns = the id space, capped at GraphBLAS's
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

/// One published, immutable version — what a reader pins.
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
    /// Reverse mapping `id → the encoded keys it currently occupies` (each
    /// key pins one `(band, row)` cell). Lets a value-less `remove` clear every
    /// cell of an id without scanning the matrices or knowing the id's old
    /// value — the matrix iterates by row, so locating an id's cells the other
    /// way would mean sweeping every band.
    reverse_id_row_mapping: HashMap<u64, Vec<u64>>,
    /// Committed logical version (== the graph commit version).
    version: u64,
}

/// The logical-MVCC store: a banded, order-preserving
/// `(row, id) → cell` matrix set plus writer-only bookkeeping, generic over the
/// cell type `V` (a `bool` presence cell, or a `u64` caller payload cell).
pub(crate) struct BandedLsmStore<V: LsmCell> {
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
    /// external invariant**: `BandedLsmStore` must be correct on its own terms, not
    /// because of how today's callers happen to be scheduled.
    writer: Mutex<Writer>,
}

impl<V: LsmCell> Default for BandedLsmStore<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: LsmCell> BandedLsmStore<V> {
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
    /// reader scans. Cheap: one `Arc` clone, no lock held after.
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
        self.committed.read().iter().map(|b| b.memory_usage()).sum()
    }

    /// Apply one commit's mutations, publishing a new immutable version at
    /// `version`. Each add is `(id, encoded row keys, cell value)`: the cell
    /// value is presence (`true`) for a `bool` store and the caller's packed
    /// `u64` payload for a `u64` store — the same value is written at every one
    /// of the id's rows. `remove` are ids whose every cell is cleared. An id in
    /// `adds` is tombstoned first (so re-inserting a changed value moves it to
    /// its new rows rather than accumulating stale ones), making the operation
    /// safe whether or not the caller also listed it in `remove`.
    pub(crate) fn commit(
        &self,
        version: u64,
        adds: &[(u64, Vec<u64>, V)],
        remove: &[u64],
    ) {
        // Serialize the whole commit (single writer).
        let mut w = self.writer.lock();

        // Group this commit's cells per band: add cells `(row, col, value)` and
        // tombstone cells `(row, col)`. The LSM never mutates published state, so
        // an id delete/update is resolved here (via `reverse_id_row_mapping`) into
        // the exact old cells to tombstone.
        let mut band_adds: [Vec<(u64, u64, V)>; NUM_BANDS] = std::array::from_fn(|_| Vec::new());
        let mut band_tombs: [Vec<(u64, u64)>; NUM_BANDS] = std::array::from_fn(|_| Vec::new());

        // Value-less removals: tombstone each id's currently occupied cells.
        for &doc in remove {
            if let Some(keys) = w.reverse_id_row_mapping.remove(&doc) {
                for k in keys {
                    band_tombs[band_of(k)].push((row_of(k), doc));
                }
            }
        }

        // Additions: tombstone the id's prior placement, then add its new rows.
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
        // remove+re-add of the same id) must NOT be tombstoned this commit:
        // both runs carry the same version, so drop the superseded tombstone.
        for b in 0..NUM_BANDS {
            if band_tombs[b].is_empty() || band_adds[b].is_empty() {
                continue;
            }
            let added: HashSet<(u64, u64)> = band_adds[b].iter().map(|&(r, c, _)| (r, c)).collect();
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
        let next: Bands<V> = std::array::from_fn(|b| Arc::new(major_compact_layers::<V>(&cur[b])));
        *self.committed.write() = Arc::new(next);
    }
}

/// A lazy, `Send`, self-contained cursor over the ids whose encoded key falls
/// in the inclusive range `[lo, hi]` of one pinned snapshot — the **scan** form
/// (presence only), used by a `bool` store.
///
/// It **owns** a [`Snapshot`] share (so it is `Send` and reclaims by `Arc`-drop
/// — no borrow of the store, no lock held). Because rows are key-ordered, the
/// range is a contiguous row-sweep per band: the cursor walks bands
/// `band_of(lo) ..= band_of(hi)`, opening one [`MatrixIter`] per band over that
/// band's slice of the range, and yields each `(row, id)` cell's id. An id
/// stored under several in-range keys is yielded once per occupied row; the scan
/// layer dedups.
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

    /// The next id in range, or `None` when exhausted. Walks each band's
    /// contiguous in-range row slice (one LSM merge cursor per band), advancing to
    /// the next band when one is drained.
    pub(crate) fn next_col(&mut self) -> Option<u64> {
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
                    self.cursor = Some(LsmCursor::new(
                        Arc::clone(&self.bands[self.band]),
                        start,
                        end,
                    ));
                }
            }
        }
    }
}

/// The **value** form of [`MatrixRangeCursor`], used by a `u64` store: yields
/// `(id, cell_value)` so the caller's packed `u64` payload comes straight from
/// the cell — no separate resolution structure. Only available for `u64`
/// (value-carrying) stores.
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

    /// The next `(id, value)` in range, or `None` when exhausted.
    pub(crate) fn next_value(&mut self) -> Option<(u64, u64)> {
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
                    self.cursor = Some(LsmCursor::new(
                        Arc::clone(&self.bands[self.band]),
                        start,
                        end,
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Commit `pairs` (id → encoded keys) at version 1 into a fresh `bool` store.
    fn store_with(pairs: &[(u64, &[u64])]) -> BandedLsmStore<bool> {
        crate::graph::graphblas::test_init_graphblas();
        let s = BandedLsmStore::<bool>::new();
        let adds: Vec<(u64, Vec<u64>, bool)> = pairs
            .iter()
            .map(|(d, ks)| (*d, ks.to_vec(), true))
            .collect();
        s.commit(1, &adds, &[]);
        s
    }

    /// Collect, sorted, the ids whose key falls in `[lo, hi]` of `snap`.
    fn collect(
        snap: &Snapshot<bool>,
        lo: u64,
        hi: u64,
    ) -> Vec<u64> {
        let mut c = MatrixRangeCursor::new(Arc::clone(snap), lo, hi);
        let mut out = Vec::new();
        while let Some(id) = c.next_col() {
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
        // (id, keys): ids 200 and 201 share key 20, so a point lookup on 20
        // returns both; ids 100/300 sit at keys 10/30.
        let s = store_with(&[(100, &[10]), (200, &[20]), (201, &[20]), (300, &[30])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 20, 20), vec![200, 201]);
        assert_eq!(collect(&snap, 10, 20), vec![100, 200, 201]);
        assert_eq!(collect(&snap, 0, 1000), vec![100, 200, 201, 300]);
        assert_eq!(collect(&snap, 11, 19), Vec::<u64>::new());
    }

    #[test]
    fn empty_range_yields_nothing() {
        let s = store_with(&[(10, &[1])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 20, 10), Vec::<u64>::new());
    }

    #[test]
    fn array_doc_occupies_many_rows() {
        // One id stored under three keys.
        let s = store_with(&[(7, &[1, 5, 9])]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 1, 1), vec![7]);
        assert_eq!(collect(&snap, 5, 5), vec![7]);
        assert_eq!(collect(&snap, 9, 9), vec![7]);
        // The id appears once per occupied in-range row.
        assert_eq!(collect(&snap, 1, 9), vec![7, 7, 7]);
    }

    #[test]
    fn remove_tombstones_every_row_of_doc() {
        let s = store_with(&[(7, &[1, 5, 9]), (8, &[5])]);
        s.commit(2, &[], &[7]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 0, 100), vec![8]); // id 7 gone everywhere
        assert_eq!(collect(&snap, 5, 5), vec![8]); // row 5 still has id 8
        assert_eq!(collect(&snap, 1, 1), Vec::<u64>::new()); // row emptied
    }

    #[test]
    fn update_moves_doc_to_new_rows() {
        let s = store_with(&[(7, &[10])]);
        // Re-insert id 7 under a new key; the old row must be vacated even though
        // the caller did not list 7 in `remove`.
        s.commit(2, &[(7, vec![20], true)], &[]);
        let snap = s.snapshot();
        assert_eq!(collect(&snap, 10, 10), Vec::<u64>::new());
        assert_eq!(collect(&snap, 20, 20), vec![7]);
    }

    #[test]
    fn reader_snapshot_is_isolated_from_later_commit() {
        let s = store_with(&[(7, &[10])]);
        let old = s.snapshot(); // pin version 1
        s.commit(2, &[(8, vec![10], true)], &[]); // add id 8 at the same key/row
        let new = s.snapshot();
        assert_eq!(collect(&old, 10, 10), vec![7]); // old reader unaffected
        assert_eq!(collect(&new, 10, 10), vec![7, 8]); // new reader sees add
        assert_eq!(s.version(), 2);
    }

    #[test]
    fn interleaved_inserts_preserve_key_order() {
        // Insert out of key order across commits; ranges must still be correct
        // because rows are stored in key order regardless of insertion order.
        crate::graph::graphblas::test_init_graphblas();
        let s = BandedLsmStore::<bool>::new();
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
        // Three ids in three different bands. Exercises the multi-band sweep
        // and the per-band start/end row clamping.
        let s = store_with(&[
            (100, &[key(0, 5)]),
            (200, &[key(1, 7)]),
            (300, &[key(2, 3)]),
        ]);
        let snap = s.snapshot();
        // Full sweep across all three bands.
        assert_eq!(collect(&snap, key(0, 0), key(3, 0)), vec![100, 200, 300]);
        // All of band 1, but only row 0 of band 2 → excludes id 300 (row 3).
        assert_eq!(collect(&snap, key(1, 0), key(2, 0)), vec![200]);
        // Point lookup inside band 1.
        assert_eq!(collect(&snap, key(1, 7), key(1, 7)), vec![200]);
        // Band 0 only (up to its top row).
        assert_eq!(collect(&snap, key(0, 0), key(0, ROW_MASK)), vec![100]);
    }

    /// `u64` store: the cell packs an opaque caller payload that a value scan
    /// returns inline with each matched id.
    #[test]
    fn value_store_yields_packed_cell() {
        crate::graph::graphblas::test_init_graphblas();
        let s = BandedLsmStore::<u64>::new();
        // (id, keys, packed payload).
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
        // Point lookup returns the one id + its packed value.
        let mut c = ValueRangeCursor::new(Arc::clone(&snap), 20, 20);
        assert_eq!(c.next_value(), Some((9, 0x1111_2222)));
        assert_eq!(c.next_value(), None);
    }
}
