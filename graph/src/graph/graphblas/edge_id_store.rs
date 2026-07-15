//! Native copy-on-write edge-id store, backed by the index's [`CowBTree`].
//!
//! This is the source of truth for `compound_key(src,dst) → {edge ids}` in the
//! decoupled `tensor`: the relationship adjacency matrices `m`/`mt` are
//! structure-only `bool`, and all edge ids live here as `(compound_key,
//! edge_id)` tuples.
//!
//! ## Why a B-tree, not base/delta
//!
//! A [`CowBTree`] stores exactly `(key: u64, doc: u64)` sorted by `(key, doc)`
//! with **page-level** copy-on-write: a snapshot is an `O(1)` root `Arc` bump,
//! and a write path-copies only the root→leaf path (sharing every untouched
//! page). Because writes mutate in place *within a copied leaf*, there is **no
//! tombstone backlog and no flush threshold** — a delete+reinsert workload stays
//! compact without a background rebuild. This is what a flat `Arc<[..]>` base +
//! delta-map + tombstone set could not do (whole-array COW forces either
//! unbounded delta growth or an `O(n)` flush).
//!
//! ## Count invariant
//!
//! [`EdgeIdStore::count`] is maintained exactly for single [`set`]/[`remove`]
//! (the tree reports whether the tuple actually changed). [`insert_batch`]
//! assumes its entries are **new** (disjoint from the tree) — which the tensor
//! guarantees, since edge ids are globally unique and never re-inserted.

use crate::index::falkordb::data_structures::cow_btree::CowBTree;

/// Native copy-on-write `(compound_key → {edge id})` multimap with page-level
/// snapshot isolation.
#[derive(Clone, Default)]
pub struct EdgeIdStore {
    /// `(compound_key, edge_id)` tuples, sorted by `(key, id)`.
    /// `DOC_BYTES = 4`: edge ids are u32-ranged (reused, `Vec`-indexed), so the
    /// doc packs into 4 bytes — 12 B/entry (8 B compound key + 4 B id) instead
    /// of 16. A `> u32` edge id panics loudly (see `cow_btree::doc_le_bytes`).
    tree: CowBTree<256, 256, 4>,
    /// Live tuple count (exact; maintained on every mutation).
    count: u64,
}

impl EdgeIdStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a committed store directly from `(key, id)` pairs (bulk load).
    /// Sorts and de-duplicates, then packs B-tree pages bottom-up.
    #[must_use]
    pub fn from_pairs(mut pairs: Vec<(u64, u64)>) -> Self {
        pairs.sort_unstable();
        pairs.dedup();
        let count = pairs.len() as u64;
        Self {
            tree: CowBTree::from_sorted(&pairs),
            count,
        }
    }

    /// New MVCC version: an `O(1)` root `Arc` bump; the first write in the new
    /// version path-copies only the touched pages.
    #[must_use]
    pub fn dup(&self) -> Self {
        self.clone()
    }

    /// Add edge `id` under `key`. Idempotent (re-adding an existing tuple is a
    /// no-op and leaves the count unchanged).
    pub fn set(
        &mut self,
        key: u64,
        id: u64,
    ) {
        if self.tree.insert(key, id) {
            self.count += 1;
        }
    }

    /// Remove edge `id` under `key`. No-op if absent.
    pub fn remove(
        &mut self,
        key: u64,
        id: u64,
    ) {
        if self.tree.remove(key, id) {
            self.count -= 1;
        }
    }

    /// Batch-insert `(key, id)` pairs (sorts once, packs touched pages in one
    /// pass). Assumes the entries are new — see the count invariant above.
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
        self.tree.insert_batch(&sorted);
        self.count += sorted.len() as u64;
    }

    /// Batch-remove `(key, id)` pairs. Sorted first so consecutive removals
    /// route through adjacent leaves/shared upper branches — with the tree's
    /// copy-on-write (`Arc::make_mut`), each touched page is then copied once
    /// per version instead of once per removal. Exact count (each removal is
    /// reported by the tree).
    pub fn remove_batch(
        &mut self,
        pairs: &[(u64, u64)],
    ) {
        let mut sorted: Vec<(u64, u64)> = pairs.to_vec();
        sorted.sort_unstable();
        for (key, id) in sorted {
            if self.tree.remove(key, id) {
                self.count -= 1;
            }
        }
    }

    /// Live edge ids for `key`, ascending — a lazy cursor owning a snapshot
    /// (droppable mid-scan, no allocation).
    #[must_use]
    pub fn ids_iter(
        &self,
        key: u64,
    ) -> impl Iterator<Item = u64> {
        self.tree.point(key)
    }

    /// Live edge ids for `key`, ascending, as an owned `Vec`.
    #[must_use]
    pub fn get(
        &self,
        key: u64,
    ) -> Vec<u64> {
        self.tree.point(key).collect()
    }

    /// Whether `key` has at least one live edge id.
    #[must_use]
    pub fn pair_nonempty(
        &self,
        key: u64,
    ) -> bool {
        self.tree.point(key).next().is_some()
    }

    /// Lazily stream all live `(key, id)` pairs whose key is in `[min_key,
    /// max_key]`, ascending by `(key, id)`. Owns a snapshot; no allocation.
    #[must_use]
    pub fn range_iter(
        &self,
        min_key: u64,
        max_key: u64,
    ) -> impl Iterator<Item = (u64, u64)> {
        self.tree.range_tuples(min_key, max_key)
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

    /// Call `f(key, id)` for every live pair, ascending by `(key, id)`. The tree
    /// walk (`for_each_tuple`) matches each leaf's format once and runs a tight
    /// inner loop — the fast, allocation-free primitive consumers should prefer
    /// when they process every pair (e.g. the weighted-MSF ephemeral rebuild
    /// filling its `(row, col, val)` build arrays).
    pub fn for_each_pair<F: FnMut(u64, u64)>(
        &self,
        f: F,
    ) {
        self.tree.for_each_tuple(f);
    }

    /// Every live `(key, id)` pair, ascending, as an owned `Vec`. Prefer
    /// [`for_each_pair`](Self::for_each_pair) on hot paths — this allocates.
    #[must_use]
    pub fn all_pairs(&self) -> Vec<(u64, u64)> {
        let mut v = Vec::with_capacity(self.count as usize);
        self.for_each_pair(|k, d| v.push((k, d)));
        v
    }

    /// Total live edge count.
    #[must_use]
    pub fn nvals(&self) -> u64 {
        self.count
    }

    /// Whether any key has more than one live id (multi-edge present). Scans the
    /// tree for two consecutive tuples sharing a key, short-circuiting on the
    /// first. Only weighted MSF / `COUNT` optimization call it, off hot paths.
    #[must_use]
    pub fn has_multi_edge(&self) -> bool {
        let mut prev: Option<u64> = None;
        for (k, _) in self.tree.range_tuples(0, u64::MAX) {
            if prev == Some(k) {
                return true;
            }
            prev = Some(k);
        }
        false
    }

    /// Approximate resident bytes of the backing tree plus this wrapper.
    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.tree.heap_bytes() + std::mem::size_of::<Self>()
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
    fn set_is_idempotent_for_count() {
        let mut s = EdgeIdStore::from_pairs(vec![(7, 42)]);
        s.set(7, 42); // already present → no-op, count unchanged
        assert_eq!(s.nvals(), 1);
        assert_eq!(s.get(7), vec![42]);
    }

    #[test]
    fn remove_then_readd() {
        let mut s = EdgeIdStore::from_pairs(vec![(1, 10), (1, 11), (2, 20)]);
        s.remove(1, 10);
        assert_eq!(s.get(1), vec![11]);
        assert_eq!(s.nvals(), 2);
        s.remove(9, 99); // absent → no-op
        assert_eq!(s.nvals(), 2);
        s.set(1, 10); // re-add
        assert_eq!(s.get(1), vec![10, 11]);
        assert_eq!(s.nvals(), 3);
        s.remove(2, 20);
        assert!(!s.pair_nonempty(2));
    }

    #[test]
    fn batch_insert_remove() {
        let mut s = EdgeIdStore::from_pairs(vec![(2, 20), (7, 70)]);
        // Disjoint batch (fresh ids) — the tensor invariant.
        s.insert_batch(&[(5, 50), (5, 40), (9, 90), (5, 60)]);
        assert_eq!(
            s.all_pairs(),
            vec![(2, 20), (5, 40), (5, 50), (5, 60), (7, 70), (9, 90)]
        );
        assert_eq!(s.nvals(), 6);
        assert_eq!(s.get(5), vec![40, 50, 60]);
        s.remove_batch(&[(5, 40), (7, 70), (5, 60)]);
        assert_eq!(s.get(5), vec![50]);
        assert!(!s.pair_nonempty(7));
        assert_eq!(s.nvals(), 3);
    }

    #[test]
    fn streaming_iters_match_owned() {
        let mut s = EdgeIdStore::from_pairs(vec![(1, 10), (3, 30), (3, 31), (5, 50)]);
        s.set(2, 20);
        s.set(3, 32);
        s.remove(5, 50);
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
        // Old snapshot is untouched (page-level COW).
        assert_eq!(base.get(1), vec![10]);
        assert_eq!(base.all_pairs(), vec![(1, 10), (2, 20)]);
        assert_eq!(base.nvals(), 2);
        // New version sees its writes.
        assert!(v2.get(1).is_empty());
        assert_eq!(v2.get(3), vec![30]);
        assert_eq!(v2.all_pairs(), vec![(2, 20), (3, 30)]);
        assert_eq!(v2.nvals(), 2);
    }

    #[test]
    fn repro_insert_batch_empty() {
        let mut s = EdgeIdStore::new();
        s.insert_batch(&[(1, 100), (1, 101), (4294967298, 102), (2, 103)]);
        assert_eq!(s.get(1), vec![100, 101]);
        assert_eq!(s.get(2), vec![103]);
        assert_eq!(s.nvals(), 4);
        assert_eq!(s.all_pairs().len(), 4);
    }

    #[test]
    fn repro_mvcc_dup_then_batch() {
        let mut v1 = EdgeIdStore::new();
        v1.insert_batch(&[(1, 100)]);
        let mut v2 = v1.dup();
        v2.insert_batch(&[(1, 101), (2, 102)]);
        assert_eq!(v1.all_pairs(), vec![(1, 100)], "v1 must be isolated");
        assert_eq!(v2.all_pairs(), vec![(1, 100), (1, 101), (2, 102)]);
        assert_eq!(v1.nvals(), 1);
        assert_eq!(v2.nvals(), 3);
    }

    #[test]
    fn repro_chain_of_versions() {
        // Mimic the runtime: each "write" dups the previous committed version
        // and batch-inserts fresh edges.
        let mut committed = EdgeIdStore::new();
        for round in 0..5u64 {
            let mut next = committed.dup();
            next.insert_batch(&[(round, 1000 + round)]);
            committed = next;
        }
        assert_eq!(committed.nvals(), 5);
        assert_eq!(committed.all_pairs().len(), 5);
    }

    #[test]
    fn larger_multi_edge_and_count() {
        let mut s = EdgeIdStore::new();
        let mut expected = 0u64;
        for pair in 0..1000u64 {
            let key = (pair << 32) | (pair + 1);
            for e in 0..(pair % 3 + 1) {
                s.set(key, 10_000 + pair * 10 + e);
                expected += 1;
            }
        }
        assert_eq!(s.nvals(), expected);
        assert!(s.has_multi_edge());
        // Round-trip through all_pairs preserves everything.
        let pairs = s.all_pairs();
        assert_eq!(pairs.len() as u64, expected);
        assert!(pairs.windows(2).all(|w| w[0] < w[1]));
    }
}

/// A/B performance comparison: the CowBTree-backed [`EdgeIdStore`] vs an inline
/// reimplementation of the previous `base/add/del` design (the milestone). Run:
/// `cargo test -p graph --release -- edge_id_store::perf --nocapture --include-ignored`.
#[cfg(test)]
mod perf {
    use std::collections::BTreeMap;
    use std::hint::black_box;
    use std::sync::Arc;
    use std::time::Instant;

    use rustc_hash::FxHashSet;
    use smallvec::SmallVec;

    use super::EdgeIdStore;

    // ---- Baseline: the previous flat base + delta-map + tombstone store. ----
    type Ids = SmallVec<[u64; 1]>;

    #[derive(Clone, Default)]
    struct Baseline {
        base: Arc<[(u64, u64)]>,
        add: Arc<BTreeMap<u64, Ids>>,
        del: Arc<FxHashSet<(u64, u64)>>,
    }

    impl Baseline {
        fn from_pairs(mut p: Vec<(u64, u64)>) -> Self {
            p.sort_unstable();
            p.dedup();
            Self {
                base: p.into(),
                add: Default::default(),
                del: Default::default(),
            }
        }
        fn base_contains(
            &self,
            k: u64,
            id: u64,
        ) -> bool {
            self.base.binary_search(&(k, id)).is_ok()
        }
        fn set(
            &mut self,
            k: u64,
            id: u64,
        ) {
            if self.base_contains(k, id) {
                if self.del.contains(&(k, id)) {
                    Arc::make_mut(&mut self.del).remove(&(k, id));
                }
                return;
            }
            let a = Arc::make_mut(&mut self.add);
            let v = a.entry(k).or_default();
            if let Err(p) = v.binary_search(&id) {
                v.insert(p, id);
            }
        }
        fn remove(
            &mut self,
            k: u64,
            id: u64,
        ) {
            if self.base_contains(k, id) {
                Arc::make_mut(&mut self.del).insert((k, id));
                return;
            }
            let a = Arc::make_mut(&mut self.add);
            if let Some(v) = a.get_mut(&k)
                && let Ok(p) = v.binary_search(&id)
            {
                v.remove(p);
                if v.is_empty() {
                    a.remove(&k);
                }
            }
        }
        fn insert_batch(
            &mut self,
            pairs: &[(u64, u64)],
        ) {
            for &(k, id) in pairs {
                self.set(k, id);
            }
        }
        fn get(
            &self,
            k: u64,
        ) -> Vec<u64> {
            let lo = self.base.partition_point(|&(x, _)| x < k);
            let hi = self.base.partition_point(|&(x, _)| x <= k);
            let mut o: Vec<u64> = self.base[lo..hi]
                .iter()
                .filter(|&&(x, id)| !self.del.contains(&(x, id)))
                .map(|&(_, id)| id)
                .collect();
            if let Some(v) = self.add.get(&k) {
                o.extend_from_slice(v);
                o.sort_unstable();
            }
            o
        }
        /// Effective full-scan (base∖del ∪ add), reading every tuple.
        fn iter_xor(&self) -> u64 {
            let mut s = 0u64;
            for &(k, id) in self.base.iter() {
                if !self.del.contains(&(k, id)) {
                    s ^= k ^ id;
                }
            }
            for (&k, ids) in self.add.iter() {
                for &id in ids {
                    s ^= k ^ id;
                }
            }
            s
        }
        fn dup(&self) -> Self {
            self.clone()
        }
        fn memory(&self) -> usize {
            let base = self.base.len() * 16;
            let del = self.del.len() * (16 + 1) * 8 / 7;
            let add = self.add.len() * (8 + std::mem::size_of::<Ids>());
            base + del + add
        }
    }

    const E: u64 = 200_000;
    /// distinct single-edge keys 0..E, doc = 1_000_000 + i
    fn pairs() -> Vec<(u64, u64)> {
        (0..E)
            .map(|i| ((i << 32) | (i + 1), 1_000_000 + i))
            .collect()
    }

    #[inline]
    fn lcg(s: &mut u64) -> u64 {
        *s = s.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        *s >> 16
    }

    #[test]
    #[ignore = "benchmark; run with --release --nocapture --include-ignored"]
    fn perf_ab() {
        let all = pairs();
        println!("\n===== EdgeIdStore: CowBTree vs base/add/del (E={E}) =====");

        // ---- INGEST (batch): fresh store, one insert_batch of all edges ----
        let t = Instant::now();
        let cow = EdgeIdStore::from_pairs(all.clone()); // bulk load (from_sorted)
        let cow_bulk = t.elapsed();
        let t = Instant::now();
        let mut cow_b = EdgeIdStore::new();
        cow_b.insert_batch(&all);
        let cow_batch = t.elapsed();
        let t = Instant::now();
        let mut base_b = Baseline::default();
        base_b.insert_batch(&all);
        let base_batch = t.elapsed();
        println!(
            "[ingest ] cow from_sorted {:5.0} ns/e | cow insert_batch {:5.0} ns/e | base insert_batch {:5.0} ns/e",
            cow_bulk.as_nanos() as f64 / E as f64,
            cow_batch.as_nanos() as f64 / E as f64,
            base_batch.as_nanos() as f64 / E as f64,
        );

        // ---- INGEST (single set loop) ----
        let t = Instant::now();
        let mut cow_s = EdgeIdStore::new();
        for &(k, id) in &all {
            cow_s.set(k, id);
        }
        let cow_set = t.elapsed();
        let t = Instant::now();
        let mut base_s = Baseline::default();
        for &(k, id) in &all {
            base_s.set(k, id);
        }
        let base_set = t.elapsed();
        println!(
            "[ingest1] cow set-loop {:5.0} ns/e | base set-loop {:5.0} ns/e",
            cow_set.as_nanos() as f64 / E as f64,
            base_set.as_nanos() as f64 / E as f64,
        );

        // Runtime-state base: everything lives in the `add` BTreeMap because the
        // tensor never flushes. `base_s` (built above via the set loop) IS that
        // state — use it for the read/mvcc benches. (`cow` has no flushed vs
        // unflushed distinction; its tree is always compact.)
        let base = base_s;
        let iters = 500_000usize;

        // ---- POINT SEEK (get) ----
        let mut st = 1u64;
        let mut acc = 0u64;
        let t = Instant::now();
        for _ in 0..iters {
            let i = lcg(&mut st) % E;
            let k = (i << 32) | (i + 1);
            acc += cow.get(k).iter().sum::<u64>();
        }
        let cow_get = t.elapsed();
        let mut st = 1u64;
        let t = Instant::now();
        for _ in 0..iters {
            let i = lcg(&mut st) % E;
            let k = (i << 32) | (i + 1);
            acc += base.get(k).iter().sum::<u64>();
        }
        let base_get = t.elapsed();
        black_box(acc);
        println!(
            "[seek   ] cow {:5.0} ns/op | base {:5.0} ns/op",
            cow_get.as_nanos() as f64 / iters as f64,
            base_get.as_nanos() as f64 / iters as f64,
        );

        // ---- ITERATION (full scan, materialized — the iter_edges / MSF path) ----
        let reps = 30u32;
        let t = Instant::now();
        let mut c = 0u64;
        for _ in 0..reps {
            c += cow.all_pairs().len() as u64; // bulk for_each_tuple
        }
        let cow_it = t.elapsed();
        // old lazy cursor path, for reference
        let t2 = Instant::now();
        let mut c_lazy = 0u64;
        for _ in 0..reps {
            c_lazy += cow.range_iter(0, u64::MAX).count() as u64;
        }
        let cow_lazy = t2.elapsed();
        println!(
            "[iter*  ] cow all_pairs(bulk) {:6.1} M/s | cow range_iter(lazy) {:6.1} M/s",
            (c as f64 / cow_it.as_secs_f64()) / 1e6,
            (c_lazy as f64 / cow_lazy.as_secs_f64()) / 1e6,
        );
        // committed base has empty add/del → full scan reads the base slice;
        // read both fields so the loop isn't optimized to `len()`.
        let t = Instant::now();
        let mut sink = 0u64;
        for _ in 0..reps {
            sink ^= base.iter_xor();
        }
        let base_it = t.elapsed();
        assert_eq!(c / u64::from(reps), E);
        black_box(sink);
        let base_n = E * u64::from(reps);
        println!(
            "[iter   ] cow {:6.1} M/s | base {:6.1} M/s",
            (c as f64 / cow_it.as_secs_f64()) / 1e6,
            (base_n as f64 / base_it.as_secs_f64()) / 1e6,
        );

        // ---- DELETE (single remove loop on a committed snapshot) ----
        let mut cow_d = cow.dup();
        let t = Instant::now();
        for &(k, id) in all.iter().take(50_000) {
            cow_d.remove(k, id);
        }
        let cow_del = t.elapsed();
        let mut base_d = base.dup();
        let t = Instant::now();
        for &(k, id) in all.iter().take(50_000) {
            base_d.remove(k, id);
        }
        let base_del = t.elapsed();
        println!(
            "[delete ] cow {:5.0} ns/e | base {:5.0} ns/e",
            cow_del.as_nanos() as f64 / 50_000.0,
            base_del.as_nanos() as f64 / 50_000.0,
        );

        // ---- MVCC dup()+1 insert ----
        let reps = 100_000u32;
        let t = Instant::now();
        for i in 0..u64::from(reps) {
            let mut v = cow.dup();
            v.set((E + i) << 32, 5_000_000 + i);
            black_box(&v);
        }
        let cow_mv = t.elapsed();
        let t = Instant::now();
        for i in 0..u64::from(reps) {
            let mut v = base.dup();
            v.set((E + i) << 32, 5_000_000 + i);
            black_box(&v);
        }
        let base_mv = t.elapsed();
        println!(
            "[mvcc   ] cow dup+1 {:5.0} ns | base dup+1 {:5.0} ns",
            cow_mv.as_nanos() as f64 / f64::from(reps),
            base_mv.as_nanos() as f64 / f64::from(reps),
        );

        // ---- MEMORY (runtime-built via set loop: the no-flush reality) ----
        println!(
            "[memory ] cow runtime {:.1} B/e | base runtime {:.1} B/e  (committed cow {:.1} B/e)",
            cow_s.memory_usage() as f64 / E as f64,
            base.memory() as f64 / E as f64,
            cow.memory_usage() as f64 / E as f64,
        );
        println!("==========================================================\n");
    }
}
