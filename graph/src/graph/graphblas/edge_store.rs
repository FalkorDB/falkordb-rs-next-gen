//! Copy-on-write edge-id store for relationship tensors.
//!
//! Maps a `compound_key(src, dst)` pair to the sorted set of edge ids
//! connecting that pair. This replaces the GraphBLAS UINT64 inline values and
//! the hypersparse `me` overflow matrix with a plain Rust hash map that
//! participates in the graph's MVCC model:
//!
//! ```text
//!   EdgeStore
//!     |
//!     |-- base   Arc<FxHashMap<key, EdgeIds>>   committed, shared across versions
//!     |-- delta  FxHashMap<key, Option<EdgeIds>>  pair-level replacement overlay
//!
//!   Effective view: delta entry wins (Some = replacement, None = pair deleted),
//!   otherwise base.
//! ```
//!
//! `dup()` clones the `Arc` and the (bounded) delta, mirroring how
//! `Cow<Matrix>` gives `VersionedMatrix` cheap per-version duplication.
//! `fold()` drains the delta into the base — in place when the base is
//! uniquely owned, or via a full clone once the delta exceeds
//! [`FOLD_THRESHOLD`] (the analog of `VersionedMatrix::flush`). The base is
//! never mutated while shared; snapshot isolation depends on this.
//!
//! Edge ids are stored as `u32` (hard cap, asserted on insert); node ids are
//! already capped at `u32` by the compound-key encoding.

use std::sync::Arc;

use rustc_hash::FxHashMap;
use thin_vec::ThinVec;

const FOLD_THRESHOLD: usize = 10_000;

/// Sorted (ascending), duplicate-free edge ids of one `(src, dst)` pair.
/// `Many` always holds at least two ids.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeIds {
    One(u32),
    Many(ThinVec<u32>),
}

impl EdgeIds {
    // No `is_empty`: an `EdgeIds` always holds at least one id (empty pairs
    // are removed from the store entirely).
    #[allow(clippy::len_without_is_empty)]
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(v) => v.len(),
        }
    }

    /// Ascending edge ids, widened to `u64`.
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        let (one, many): (Option<u32>, &[u32]) = match self {
            Self::One(id) => (Some(*id), &[]),
            Self::Many(v) => (None, v),
        };
        one.into_iter().chain(many.iter().copied()).map(u64::from)
    }
}

/// Outcome of [`EdgeStore::remove`].
#[derive(Debug, PartialEq, Eq)]
pub enum PairState {
    /// The id was removed; the pair still has at least one edge.
    Removed,
    /// The id was removed and the pair now has no edges.
    Emptied,
    /// The id was not present for this pair.
    NotFound,
}

pub struct EdgeStore {
    base: Arc<FxHashMap<u64, EdgeIds>>,
    delta: FxHashMap<u64, Option<EdgeIds>>,
    /// Effective total edge count, maintained incrementally.
    edge_count: u64,
    /// Effective number of pairs with more than one edge.
    multi_pairs: u64,
}

fn to_u32(id: u64) -> u32 {
    u32::try_from(id).expect("EdgeStore edge id overflow: edge ids must fit in u32")
}

impl EdgeStore {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: Arc::new(FxHashMap::default()),
            delta: FxHashMap::default(),
            edge_count: 0,
            multi_pairs: 0,
        }
    }

    /// Cheap per-version duplicate: shares the committed base, copies the
    /// (bounded) delta and counters.
    #[must_use]
    pub fn dup(&self) -> Self {
        Self {
            base: Arc::clone(&self.base),
            delta: self.delta.clone(),
            edge_count: self.edge_count,
            multi_pairs: self.multi_pairs,
        }
    }

    /// Effective ids of a pair: the delta overlay wins over the base.
    #[must_use]
    pub fn ids(
        &self,
        key: u64,
    ) -> Option<&EdgeIds> {
        match self.delta.get(&key) {
            Some(overlay) => overlay.as_ref(),
            None => self.base.get(&key),
        }
    }

    #[must_use]
    pub fn contains_pair(
        &self,
        key: u64,
    ) -> bool {
        self.ids(key).is_some()
    }

    /// Add one edge id to a pair. Returns `true` when the pair had no edges
    /// before (the caller must then set the pair bit in the adjacency
    /// matrices). Adding an id that is already present is a no-op returning
    /// `false`.
    pub fn add(
        &mut self,
        key: u64,
        id: u64,
    ) -> bool {
        let id = to_u32(id);
        let was_empty = match self.ids(key) {
            None => {
                self.delta.insert(key, Some(EdgeIds::One(id)));
                self.edge_count += 1;
                true
            }
            Some(ids) => {
                let mut v: ThinVec<u32> = match ids {
                    EdgeIds::One(existing) => {
                        if *existing == id {
                            return false;
                        }
                        let mut v = ThinVec::with_capacity(2);
                        v.push(*existing);
                        v
                    }
                    EdgeIds::Many(v) => v.clone(),
                };
                let Err(pos) = v.binary_search(&id) else {
                    return false;
                };
                v.insert(pos, id);
                if v.len() == 2 {
                    self.multi_pairs += 1;
                }
                self.edge_count += 1;
                self.delta.insert(key, Some(EdgeIds::Many(v)));
                false
            }
        };
        self.maybe_fold();
        was_empty
    }

    /// Merge a batch of ids into a pair (deduplicating against existing ids).
    /// Returns `true` when the pair had no edges before.
    pub fn add_many(
        &mut self,
        key: u64,
        new_ids: impl Iterator<Item = u64>,
    ) -> bool {
        let mut v: ThinVec<u32> = match self.ids(key) {
            None => ThinVec::new(),
            Some(EdgeIds::One(id)) => {
                let mut v = ThinVec::new();
                v.push(*id);
                v
            }
            Some(EdgeIds::Many(m)) => m.clone(),
        };
        let was_empty = v.is_empty();
        let old_len = v.len();
        v.extend(new_ids.map(to_u32));
        v.sort_unstable();
        v.dedup();
        if v.len() == old_len {
            return false;
        }
        self.edge_count += (v.len() - old_len) as u64;
        if old_len <= 1 && v.len() > 1 {
            self.multi_pairs += 1;
        }
        let ids = if v.len() == 1 {
            EdgeIds::One(v[0])
        } else {
            EdgeIds::Many(v)
        };
        self.delta.insert(key, Some(ids));
        self.maybe_fold();
        was_empty
    }

    /// Remove one edge id from a pair.
    pub fn remove(
        &mut self,
        key: u64,
        id: u64,
    ) -> PairState {
        let Ok(id) = u32::try_from(id) else {
            return PairState::NotFound;
        };
        let state = match self.ids(key) {
            None => return PairState::NotFound,
            Some(EdgeIds::One(existing)) => {
                if *existing != id {
                    return PairState::NotFound;
                }
                self.delta.insert(key, None);
                self.edge_count -= 1;
                PairState::Emptied
            }
            Some(EdgeIds::Many(v)) => {
                let Ok(pos) = v.binary_search(&id) else {
                    return PairState::NotFound;
                };
                let mut v = v.clone();
                v.remove(pos);
                self.edge_count -= 1;
                if v.len() == 1 {
                    self.multi_pairs -= 1;
                    self.delta.insert(key, Some(EdgeIds::One(v[0])));
                } else {
                    self.delta.insert(key, Some(EdgeIds::Many(v)));
                }
                PairState::Removed
            }
        };
        self.maybe_fold();
        state
    }

    /// Every effective `(src, dst, edge_id)` triple. Pair order is arbitrary
    /// (hash order); ids within a pair are ascending.
    pub fn iter_edges(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.iter_pairs()
            .flat_map(|(key, ids)| ids.iter().map(move |id| (key >> 32, key & 0xFFFF_FFFF, id)))
    }

    /// Every effective `(compound_key, &EdgeIds)` pair, in arbitrary order.
    pub fn iter_pairs(&self) -> impl Iterator<Item = (u64, &EdgeIds)> + '_ {
        let base = self
            .base
            .iter()
            .filter(|(k, _)| !self.delta.contains_key(*k))
            .map(|(k, ids)| (*k, ids));
        let delta = self
            .delta
            .iter()
            .filter_map(|(k, overlay)| overlay.as_ref().map(|ids| (*k, ids)));
        base.chain(delta)
    }

    #[must_use]
    pub const fn edge_count(&self) -> u64 {
        self.edge_count
    }

    #[must_use]
    pub const fn multi_pairs(&self) -> u64 {
        self.multi_pairs
    }

    /// Fold the delta into the base when the base is uniquely owned. Called
    /// opportunistically at commit/sync points; a no-op while the base is
    /// still shared with readers of an older version.
    pub fn fold(&mut self) {
        if self.delta.is_empty() {
            return;
        }
        if let Some(base) = Arc::get_mut(&mut self.base) {
            for (k, overlay) in self.delta.drain() {
                match overlay {
                    Some(ids) => {
                        base.insert(k, ids);
                    }
                    None => {
                        base.remove(&k);
                    }
                }
            }
            self.debug_assert_counters();
        }
    }

    /// Bound the delta: once it exceeds the threshold and the base cannot be
    /// folded in place, pay one full base clone (the analog of a `Cow` deep
    /// copy) so `dup()` stays cheap.
    fn maybe_fold(&mut self) {
        if self.delta.len() < FOLD_THRESHOLD {
            return;
        }
        self.fold();
        if self.delta.is_empty() {
            return;
        }
        let mut base: FxHashMap<u64, EdgeIds> = (*self.base).clone();
        for (k, overlay) in self.delta.drain() {
            match overlay {
                Some(ids) => {
                    base.insert(k, ids);
                }
                None => {
                    base.remove(&k);
                }
            }
        }
        self.base = Arc::new(base);
        self.debug_assert_counters();
    }

    fn debug_assert_counters(&self) {
        #[cfg(debug_assertions)]
        {
            let mut edges = 0u64;
            let mut multi = 0u64;
            for (_, ids) in self.iter_pairs() {
                edges += ids.len() as u64;
                if ids.len() > 1 {
                    multi += 1;
                }
            }
            debug_assert_eq!(edges, self.edge_count, "EdgeStore edge_count drift");
            debug_assert_eq!(multi, self.multi_pairs, "EdgeStore multi_pairs drift");
        }
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        const ENTRY: usize = size_of::<u64>() + size_of::<EdgeIds>() + 1;
        let heap = |ids: &EdgeIds| match ids {
            EdgeIds::One(_) => 0,
            EdgeIds::Many(v) => v.capacity() * size_of::<u32>() + 16,
        };
        self.base.capacity() * ENTRY
            + self.base.values().map(heap).sum::<usize>()
            + self.delta.capacity() * (ENTRY + size_of::<usize>())
            + self.delta.values().flatten().map(heap).sum::<usize>()
    }
}

impl Default for EdgeStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(
        src: u64,
        dst: u64,
    ) -> u64 {
        (src << 32) | dst
    }

    #[test]
    fn add_get_remove_multi_edge() {
        let mut e = EdgeStore::new();
        assert!(e.add(key(0, 1), 9));
        assert!(!e.add(key(0, 1), 5));
        assert!(e.add(key(2, 1), 7));

        assert_eq!(e.edge_count(), 3);
        assert_eq!(e.multi_pairs(), 1);

        let ids: Vec<u64> = e.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![5, 9]);

        assert_eq!(e.remove(key(0, 1), 5), PairState::Removed);
        assert_eq!(e.multi_pairs(), 0);
        assert_eq!(e.remove(key(0, 1), 5), PairState::NotFound);
        assert_eq!(e.remove(key(0, 1), 9), PairState::Emptied);
        assert!(!e.contains_pair(key(0, 1)));
        assert_eq!(e.edge_count(), 1);
    }

    #[test]
    fn duplicate_add_is_noop() {
        let mut e = EdgeStore::new();
        assert!(e.add(key(1, 2), 3));
        assert!(!e.add(key(1, 2), 3));
        assert_eq!(e.edge_count(), 1);
        assert_eq!(e.multi_pairs(), 0);
    }

    #[test]
    fn cow_isolation() {
        let mut parent = EdgeStore::new();
        parent.add(key(0, 1), 5);
        parent.fold();

        let mut child = parent.dup();
        child.add(key(0, 1), 6);
        child.remove(key(0, 1), 5);
        child.add(key(3, 4), 7);

        // Parent unchanged.
        let ids: Vec<u64> = parent.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![5]);
        assert!(!parent.contains_pair(key(3, 4)));
        assert_eq!(parent.edge_count(), 1);

        // Child sees its own state.
        let ids: Vec<u64> = child.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![6]);
        assert_eq!(child.edge_count(), 2);

        // Fold is a no-op while the base is shared...
        child.fold();
        let ids: Vec<u64> = parent.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![5]);

        // ...and drains once the parent is gone.
        drop(parent);
        child.fold();
        assert!(child.delta.is_empty());
        let ids: Vec<u64> = child.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![6]);
        assert_eq!(child.edge_count(), 2);
    }

    #[test]
    fn add_many_merges_and_dedups() {
        let mut e = EdgeStore::new();
        e.add(key(0, 1), 4);
        assert!(!e.add_many(key(0, 1), [9, 2, 4].into_iter()));
        let ids: Vec<u64> = e.ids(key(0, 1)).unwrap().iter().collect();
        assert_eq!(ids, vec![2, 4, 9]);
        assert_eq!(e.edge_count(), 3);
        assert_eq!(e.multi_pairs(), 1);

        assert!(e.add_many(key(5, 5), [1].into_iter()));
        assert_eq!(e.ids(key(5, 5)), Some(&EdgeIds::One(1)));
    }

    #[test]
    fn iter_edges_effective_view() {
        let mut e = EdgeStore::new();
        e.add(key(0, 1), 5);
        e.add(key(2, 3), 6);
        e.fold();
        e.add(key(0, 1), 7); // overlay on base pair
        e.add(key(4, 4), 8); // delta-only pair

        let mut triples: Vec<(u64, u64, u64)> = e.iter_edges().collect();
        triples.sort_unstable();
        assert_eq!(triples, vec![(0, 1, 5), (0, 1, 7), (2, 3, 6), (4, 4, 8)]);
    }

    #[test]
    #[should_panic(expected = "edge id overflow")]
    fn edge_id_overflow_asserts() {
        let mut e = EdgeStore::new();
        e.add(key(0, 1), u64::from(u32::MAX) + 1);
    }
}
