//! Native copy-on-write edge-id store (POC for the decoupled tensor).
//!
//! This is the source of truth for `compound_key(src,dst) → {edge ids}` in the
//! "decouple" alternative, where the relationship adjacency matrices `m`/`mt`
//! go back to pure `VersionedMatrix<bool>` (structure only) and all edge ids
//! live here instead of inside a GraphBLAS matrix.
//!
//! ## MVCC shape (mirrors `VersionedMatrix`)
//!
//! ```text
//!   EdgeIdStore
//!     |-- base  Arc<[(key,id)]>   committed, sorted by (key,id)   ~ m
//!     |-- add   Arc<BTreeMap>     per-version pending additions   ~ dp
//!     |-- del   Arc<FxHashSet>    per-version tombstones (⊆ base)  ~ dm
//!
//!   Effective content = (base ∖ del) ∪ add   (a disjoint union)
//! ```
//!
//! `base` is sorted by `(compound_key, edge_id)` — which is exactly forward
//! row-major `(src, dst, id)` adjacency order — so a forward `src` range is a
//! contiguous slice and iteration needs no per-pair lookup. Versioning is the
//! same hybrid the codebase already uses for `attribute_store::DataBlock`:
//! clone three `Arc`s on `dup()` (O(1)); `Arc::make_mut` deep-copies only the
//! small `add`/`del` delta on first write per version; `base` is shared
//! read-only across snapshots and rebuilt only at `flush`/load.
//!
//! Invariants maintained by [`EdgeIdStore::set`] / [`EdgeIdStore::remove`]:
//! - `del ⊆ base` (only committed entries get tombstoned)
//! - `add ∩ (base ∖ del) = ∅` (an id is in `base` XOR `add`, never both live)
//! - every `add[key]` is sorted ascending and non-empty

use std::collections::BTreeMap;
use std::sync::Arc;

use itertools::merge;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

/// Pending ids for one key. Inline for the common single-overflow case.
type Ids = SmallVec<[u64; 1]>;

/// Native COW `(compound_key → {edge id})` multimap with snapshot isolation.
#[derive(Clone, Default)]
pub struct EdgeIdStore {
    /// Committed entries, sorted ascending by `(key, id)`. Immutable at runtime.
    base: Arc<[(u64, u64)]>,
    /// Per-version additions, `key → ascending ids`, disjoint from live `base`.
    add: Arc<BTreeMap<u64, Ids>>,
    /// Per-version tombstones over `base` entries.
    del: Arc<FxHashSet<(u64, u64)>>,
}

impl EdgeIdStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a committed store directly from `(key, id)` pairs (bulk load).
    /// Sorts and de-duplicates into `base`; `add`/`del` start empty.
    #[must_use]
    pub fn from_pairs(mut pairs: Vec<(u64, u64)>) -> Self {
        pairs.sort_unstable();
        pairs.dedup();
        Self {
            base: pairs.into(),
            add: Arc::new(BTreeMap::new()),
            del: Arc::new(FxHashSet::default()),
        }
    }

    /// New MVCC version: share all three layers via `Arc` (O(1)); the first
    /// write in the new version copies only the touched delta.
    #[must_use]
    pub fn dup(&self) -> Self {
        self.clone()
    }

    /// Half-open `base` index range `[lo, hi)` of the run for `key`.
    #[inline]
    fn base_range(
        &self,
        key: u64,
    ) -> (usize, usize) {
        let lo = self.base.partition_point(|&(k, _)| k < key);
        let hi = self.base.partition_point(|&(k, _)| k <= key);
        (lo, hi)
    }

    #[inline]
    fn base_contains(
        &self,
        key: u64,
        id: u64,
    ) -> bool {
        self.base.binary_search(&(key, id)).is_ok()
    }

    /// Add edge `id` under `key`. Idempotent.
    pub fn set(
        &mut self,
        key: u64,
        id: u64,
    ) {
        if self.base_contains(key, id) {
            // Committed already: un-delete if it was tombstoned, else no-op.
            if self.del.contains(&(key, id)) {
                Arc::make_mut(&mut self.del).remove(&(key, id));
            }
            return;
        }
        let add = Arc::make_mut(&mut self.add);
        let ids = add.entry(key).or_default();
        if let Err(pos) = ids.binary_search(&id) {
            ids.insert(pos, id);
        }
    }

    /// Remove edge `id` under `key`. No-op if absent.
    pub fn remove(
        &mut self,
        key: u64,
        id: u64,
    ) {
        if self.base_contains(key, id) {
            Arc::make_mut(&mut self.del).insert((key, id));
            return;
        }
        let add = Arc::make_mut(&mut self.add);
        if let Some(ids) = add.get_mut(&key)
            && let Ok(pos) = ids.binary_search(&id)
        {
            ids.remove(pos);
            if ids.is_empty() {
                add.remove(&key);
            }
        }
    }

    /// Batch-insert `(key, id)` pairs. Adopts the sorted batch-apply pattern
    /// (cf. `CowBTree::insert_batch` / `attribute_store` `merge_span`): classify
    /// the whole batch against the base once, then perform a single copy-on-write
    /// of each touched delta and group the additions by key — one `BTreeMap`
    /// entry per distinct key instead of per edge.
    pub fn insert_batch(
        &mut self,
        pairs: &[(u64, u64)],
    ) {
        if pairs.is_empty() {
            return;
        }
        let mut sorted: Vec<(u64, u64)> = pairs.to_vec();
        sorted.sort_unstable();
        sorted.dedup();

        // Phase 1 (read-only): split into committed-but-tombstoned (un-delete)
        // and genuinely-new (add).
        let mut undeletes: Vec<(u64, u64)> = Vec::new();
        let mut fresh: Vec<(u64, u64)> = Vec::new();
        for &kv in &sorted {
            if self.base_contains(kv.0, kv.1) {
                if self.del.contains(&kv) {
                    undeletes.push(kv);
                }
            } else {
                fresh.push(kv);
            }
        }

        // Phase 2 (batched mutation): one make_mut per touched delta.
        if !undeletes.is_empty() {
            let del = Arc::make_mut(&mut self.del);
            for kv in &undeletes {
                del.remove(kv);
            }
        }
        if !fresh.is_empty() {
            let add = Arc::make_mut(&mut self.add);
            let mut i = 0;
            while i < fresh.len() {
                let key = fresh[i].0;
                let ids = add.entry(key).or_default();
                while i < fresh.len() && fresh[i].0 == key {
                    let id = fresh[i].1;
                    if let Err(pos) = ids.binary_search(&id) {
                        ids.insert(pos, id);
                    }
                    i += 1;
                }
            }
        }
    }

    /// Batch-remove `(key, id)` pairs: tombstone committed entries, drop pending
    /// ones — a single copy-on-write of each touched delta.
    pub fn remove_batch(
        &mut self,
        pairs: &[(u64, u64)],
    ) {
        if pairs.is_empty() {
            return;
        }
        let mut sorted: Vec<(u64, u64)> = pairs.to_vec();
        sorted.sort_unstable();
        sorted.dedup();

        let mut tombstones: Vec<(u64, u64)> = Vec::new();
        let mut add_removes: Vec<(u64, u64)> = Vec::new();
        for &kv in &sorted {
            if self.base_contains(kv.0, kv.1) {
                tombstones.push(kv);
            } else {
                add_removes.push(kv);
            }
        }

        if !tombstones.is_empty() {
            let del = Arc::make_mut(&mut self.del);
            for kv in tombstones {
                del.insert(kv);
            }
        }
        if !add_removes.is_empty() {
            let add = Arc::make_mut(&mut self.add);
            for (key, id) in add_removes {
                if let Some(ids) = add.get_mut(&key)
                    && let Ok(pos) = ids.binary_search(&id)
                {
                    ids.remove(pos);
                    if ids.is_empty() {
                        add.remove(&key);
                    }
                }
            }
        }
    }

    /// Live edge ids for `key`, ascending — a lazy streaming merge of the base
    /// run (skipping tombstones) with the pending `add` ids for this key. Both
    /// sources are ascending and disjoint, so no buffering or sort is needed.
    #[must_use]
    pub fn ids_iter(
        &self,
        key: u64,
    ) -> impl Iterator<Item = u64> + '_ {
        let (lo, hi) = self.base_range(key);
        let del = &self.del;
        let base = self.base[lo..hi]
            .iter()
            .filter(move |kv| !del.contains(kv))
            .map(|&(_, id)| id);
        let add = self.add.get(&key).into_iter().flatten().copied();
        merge(base, add)
    }

    /// Live edge ids for `key`, ascending, as an owned `Vec`.
    #[must_use]
    pub fn get(
        &self,
        key: u64,
    ) -> Vec<u64> {
        self.ids_iter(key).collect()
    }

    /// Whether `key` has at least one live edge id.
    #[must_use]
    pub fn pair_nonempty(
        &self,
        key: u64,
    ) -> bool {
        let (lo, hi) = self.base_range(key);
        if self.base[lo..hi]
            .iter()
            .any(|&(k, id)| !self.del.contains(&(k, id)))
        {
            return true;
        }
        self.add.get(&key).is_some_and(|ids| !ids.is_empty())
    }

    /// Lazily stream all live `(key, id)` pairs whose key is in
    /// `[min_key, max_key]`, ascending by `(key, id)` — a merge of the base
    /// slice (skipping tombstones) with the pending `add` range. No allocation,
    /// droppable mid-scan. Forward tensor iteration uses this over a
    /// `src`-derived key range.
    #[must_use]
    pub fn range_iter(
        &self,
        min_key: u64,
        max_key: u64,
    ) -> impl Iterator<Item = (u64, u64)> + '_ {
        let lo = self.base.partition_point(|&(k, _)| k < min_key);
        let hi = self.base.partition_point(|&(k, _)| k <= max_key);
        let del = &self.del;
        let base = self.base[lo..hi]
            .iter()
            .copied()
            .filter(move |kv| !del.contains(kv));
        let add = self
            .add
            .range(min_key..=max_key)
            .flat_map(|(&k, ids)| ids.iter().map(move |&id| (k, id)));
        merge(base, add)
    }

    /// All live `(key, id)` pairs whose key is in `[min_key, max_key]`,
    /// ascending, as an owned `Vec`.
    #[must_use]
    pub fn range_pairs(
        &self,
        min_key: u64,
        max_key: u64,
    ) -> Vec<(u64, u64)> {
        self.range_iter(min_key, max_key).collect()
    }

    /// Every live `(key, id)` pair, ascending, as an owned `Vec`.
    #[must_use]
    pub fn all_pairs(&self) -> Vec<(u64, u64)> {
        self.range_pairs(0, u64::MAX)
    }

    /// Total live edge count: `|base| − |del| + |add|`.
    #[must_use]
    pub fn nvals(&self) -> u64 {
        let add_total: usize = self.add.values().map(SmallVec::len).sum();
        (self.base.len() - self.del.len() + add_total) as u64
    }

    /// Whether any key has more than one live id (multi-edge present).
    #[must_use]
    pub fn has_multi_edge(&self) -> bool {
        // Cheap-ish: scan add first (usually the source of multiplicity at
        // runtime), then base runs. For the POC this is not on a hot path.
        if self.add.values().any(|ids| ids.len() > 1) {
            // A key with ≥2 pending ids, or 1 pending + ≥1 live base, is multi.
            return true;
        }
        // add has ≤1 id per key here; a multi pair then needs a live base id
        // for a key that also has a pending id, or ≥2 live base ids per key.
        let mut i = 0;
        while i < self.base.len() {
            let key = self.base[i].0;
            let mut live = 0u32;
            while i < self.base.len() && self.base[i].0 == key {
                if !self.del.contains(&self.base[i]) {
                    live += 1;
                }
                i += 1;
            }
            if self.add.contains_key(&key) {
                live += 1;
            }
            if live > 1 {
                return true;
            }
        }
        false
    }

    /// Merge `add`/`del` into `base` and clear the deltas. Rebuilds the sorted
    /// committed snapshot (analogue of `VersionedMatrix::flush`).
    pub fn flush(&mut self) {
        if self.add.is_empty() && self.del.is_empty() {
            return;
        }
        let add_total: usize = self.add.values().map(SmallVec::len).sum();
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(self.base.len() + add_total);
        merged.extend(
            self.base
                .iter()
                .copied()
                .filter(|kv| !self.del.contains(kv)),
        );
        for (&k, ids) in self.add.iter() {
            merged.extend(ids.iter().map(|&id| (k, id)));
        }
        merged.sort_unstable();
        self.base = merged.into();
        self.add = Arc::new(BTreeMap::new());
        self.del = Arc::new(FxHashSet::default());
    }

    /// Approximate resident bytes. `base` (16 B/edge) dominates at steady
    /// state; the small deltas are estimated with modest per-entry overhead.
    #[must_use]
    pub fn memory_usage(&self) -> usize {
        let base = self.base.len() * std::mem::size_of::<(u64, u64)>();
        // FxHashSet: ~1.14× slots of (key,id) plus 1 control byte each.
        let del = self.del.len() * (std::mem::size_of::<(u64, u64)>() + 1) * 8 / 7;
        // BTreeMap: per-entry key + SmallVec header; spilled ids add 8 B each.
        let add_headers =
            self.add.len() * (std::mem::size_of::<u64>() + std::mem::size_of::<Ids>());
        let add_spilled: usize = self
            .add
            .values()
            .map(|ids| ids.len().saturating_sub(1) * std::mem::size_of::<u64>())
            .sum();
        base + del + add_headers + add_spilled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_edge_roundtrip() {
        let mut s = EdgeIdStore::new();
        s.set(10, 100);
        assert_eq!(s.get(10), vec![100]);
        assert!(s.pair_nonempty(10));
        assert!(!s.pair_nonempty(11));
        assert_eq!(s.nvals(), 1);
        assert!(!s.has_multi_edge());
    }

    #[test]
    fn multi_edge_ascending() {
        let mut s = EdgeIdStore::new();
        s.set(10, 300);
        s.set(10, 100);
        s.set(10, 200);
        assert_eq!(s.get(10), vec![100, 200, 300]);
        assert_eq!(s.nvals(), 3);
        assert!(s.has_multi_edge());
    }

    #[test]
    fn remove_from_base_and_add() {
        // base holds (10,100),(10,200); add holds (10,300)
        let mut s = EdgeIdStore::from_pairs(vec![(10, 100), (10, 200)]);
        s.set(10, 300);
        assert_eq!(s.get(10), vec![100, 200, 300]);

        s.remove(10, 200); // tombstone a base entry
        assert_eq!(s.get(10), vec![100, 300]);
        s.remove(10, 300); // drop a pending add
        assert_eq!(s.get(10), vec![100]);
        s.remove(10, 100); // tombstone last base entry -> empty
        assert!(s.get(10).is_empty());
        assert!(!s.pair_nonempty(10));
        assert_eq!(s.nvals(), 0);
    }

    #[test]
    fn re_add_after_tombstone_is_undelete() {
        let mut s = EdgeIdStore::from_pairs(vec![(7, 42)]);
        s.remove(7, 42);
        assert!(s.get(7).is_empty());
        s.set(7, 42); // must un-delete, not create a shadow in add
        assert_eq!(s.get(7), vec![42]);
        assert_eq!(s.nvals(), 1);
        assert!(s.del.is_empty());
    }

    #[test]
    fn range_pairs_is_sorted_and_scoped() {
        let mut s = EdgeIdStore::from_pairs(vec![(1, 5), (3, 9), (3, 1)]);
        s.set(2, 7);
        assert_eq!(s.all_pairs(), vec![(1, 5), (2, 7), (3, 1), (3, 9)]);
        assert_eq!(s.range_pairs(2, 3), vec![(2, 7), (3, 1), (3, 9)]);
        assert_eq!(s.range_pairs(4, 100), vec![]);
    }

    #[test]
    fn flush_equals_effective() {
        let mut s = EdgeIdStore::from_pairs(vec![(1, 10), (2, 20), (2, 21)]);
        s.set(3, 30);
        s.remove(2, 20);
        let before = s.all_pairs();
        let nvals = s.nvals();
        s.flush();
        assert!(s.add.is_empty() && s.del.is_empty());
        assert_eq!(s.all_pairs(), before);
        assert_eq!(s.nvals(), nvals);
        assert_eq!(s.base.len() as u64, nvals);
    }

    #[test]
    fn insert_batch_equals_per_element() {
        // Same ops via batch vs per-element `set` must yield identical state.
        let ops = [(5, 50), (5, 40), (2, 20), (9, 90), (5, 60), (2, 20)];
        let mut a = EdgeIdStore::from_pairs(vec![(2, 20), (7, 70)]);
        let mut b = a.clone();
        for &(k, id) in &ops {
            a.set(k, id);
        }
        b.insert_batch(&ops);
        assert_eq!(a.all_pairs(), b.all_pairs());
        assert_eq!(a.nvals(), b.nvals());
        // Includes a re-add of the tombstone-free base entry (2,20): idempotent.
        assert_eq!(b.get(2), vec![20]);
        assert_eq!(b.get(5), vec![40, 50, 60]);
    }

    #[test]
    fn remove_batch_equals_per_element() {
        let base = EdgeIdStore::from_pairs(vec![(1, 10), (1, 11), (2, 20), (3, 30)]);
        let extra = [(1, 12), (4, 40)];
        let removals = [(1, 10), (4, 40), (3, 30), (1, 12)];
        let mut a = base.clone();
        let mut b = base.clone();
        a.insert_batch(&extra);
        b.insert_batch(&extra);
        for &(k, id) in &removals {
            a.remove(k, id);
        }
        b.remove_batch(&removals);
        assert_eq!(a.all_pairs(), b.all_pairs());
        assert_eq!(a.nvals(), b.nvals());
    }

    #[test]
    fn streaming_iters_match_owned() {
        let mut s = EdgeIdStore::from_pairs(vec![(1, 10), (3, 30), (3, 31), (5, 50)]);
        s.set(2, 20);
        s.set(3, 32);
        s.remove(5, 50);
        // range_iter == range_pairs; ids_iter == get.
        assert_eq!(s.range_iter(0, u64::MAX).collect::<Vec<_>>(), s.all_pairs());
        assert_eq!(s.range_iter(2, 3).collect::<Vec<_>>(), s.range_pairs(2, 3));
        for k in 0..7 {
            assert_eq!(s.ids_iter(k).collect::<Vec<_>>(), s.get(k));
        }
        assert_eq!(s.get(3), vec![30, 31, 32]);
        assert!(s.get(5).is_empty());
    }

    #[test]
    fn mvcc_snapshot_isolation() {
        let base = EdgeIdStore::from_pairs(vec![(1, 10), (2, 20)]);
        let mut v2 = base.dup();
        v2.set(3, 30);
        v2.remove(1, 10);
        // Old snapshot is untouched.
        assert_eq!(base.get(1), vec![10]);
        assert_eq!(base.all_pairs(), vec![(1, 10), (2, 20)]);
        // New version sees its writes.
        assert!(v2.get(1).is_empty());
        assert_eq!(v2.get(3), vec![30]);
        assert_eq!(v2.all_pairs(), vec![(2, 20), (3, 30)]);
    }
}
