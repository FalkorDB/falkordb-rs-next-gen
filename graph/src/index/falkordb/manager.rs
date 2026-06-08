//! Per-graph registry of FalkorDB index structures.
//!
//! Mirrors the engine [`crate::index::indexer::Indexer`]: a cheaply-cloneable
//! handle whose `Arc`-inner is shared across the graph's copy-on-write versions
//! ([`crate::graph::Graph::new_version`] clones it). So every graph version sees
//! the same set of [`NumericIndex`]es, and each index MVCC-versions its own
//! matrix internally — the index is a *shared subsystem that adopts the graph
//! commit version*, not folded into the per-commit `Cow`.
//!
//! Keyed per `(entity, label, field)`: one [`NumericIndex`] per numeric field,
//! because a single matrix store maps one `Value → doc` and two fields on a
//! label would collide in one matrix.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::RwLock;

use super::{EntityKind, Index, IndexSchema, NumericIndex, index_backend};

/// Registry key: one numeric index per `(entity, label, field)`.
type Key = (EntityKind, Arc<String>, Arc<String>);

/// The FalkorDB index structures owned by a graph. Cloning shares the inner state
/// (the per-version [`crate::graph::Graph::new_version`] clone), so all graph
/// versions observe the same indexes.
#[derive(Default, Clone)]
pub struct FalkorDbIndexes {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    indexes: RwLock<HashMap<Key, Arc<NumericIndex>>>,
    /// Hands out stable ids to new index structures.
    next_id: AtomicU64,
}

impl FalkorDbIndexes {
    /// Register one numeric index per field (idempotent — a field that already
    /// has an index keeps it). Called from `CREATE INDEX` when the index type
    /// is `Range`.
    pub fn create_numeric(
        &self,
        entity: EntityKind,
        label: &Arc<String>,
        fields: &[Arc<String>],
    ) {
        let mut map = self.inner.indexes.write();
        for field in fields {
            let key = (entity, Arc::clone(label), Arc::clone(field));
            map.entry(key).or_insert_with(|| {
                let schema = IndexSchema {
                    index_id: self.inner.next_id.fetch_add(1, Ordering::Relaxed),
                    entity,
                    label: Arc::clone(label),
                    fields: vec![Arc::clone(field)],
                };
                Arc::new(NumericIndex::create(&schema, index_backend()))
            });
        }
    }

    /// Drop the numeric indexes for `fields` (or every field of `label` when
    /// `fields` is empty), matching the engine `drop_index` expansion.
    pub fn drop_fields(
        &self,
        entity: EntityKind,
        label: &Arc<String>,
        fields: &[Arc<String>],
    ) {
        let mut map = self.inner.indexes.write();
        if fields.is_empty() {
            map.retain(|(e, l, _), _| !(*e == entity && l == label));
        } else {
            for field in fields {
                map.remove(&(entity, Arc::clone(label), Arc::clone(field)));
            }
        }
    }

    /// The FalkorDB-indexed field names for `(entity, label)` — used by the
    /// commit hook to know which fields to feed, and the backfill on create.
    #[must_use]
    pub fn fields(
        &self,
        entity: EntityKind,
        label: &Arc<String>,
    ) -> Vec<Arc<String>> {
        self.inner
            .indexes
            .read()
            .keys()
            .filter(|(e, l, _)| *e == entity && l == label)
            .map(|(_, _, field)| Arc::clone(field))
            .collect()
    }

    /// The numeric index for `(entity, label, field)`, if one exists.
    #[must_use]
    pub fn get(
        &self,
        entity: EntityKind,
        label: &Arc<String>,
        field: &Arc<String>,
    ) -> Option<Arc<NumericIndex>> {
        self.inner
            .indexes
            .read()
            .get(&(entity, Arc::clone(label), Arc::clone(field)))
            .map(Arc::clone)
    }

    /// Major-compact every index (collapse each band's segments into one base).
    /// Returns the number of indexes compacted. An on-demand maintenance op — each
    /// index publishes its own atomic snapshot, so reads/writes are unaffected.
    pub fn compact_all(&self) -> usize {
        // Snapshot the handles under a short read lock, then compact without
        // holding it (each index is internally synchronized).
        let indexes: Vec<Arc<NumericIndex>> =
            self.inner.indexes.read().values().map(Arc::clone).collect();
        for idx in &indexes {
            idx.major_compact();
        }
        indexes.len()
    }
}
