//! Attribute storage for graph entities.
//!
//! This module provides [`AttributeStore`], a columnar key-value store backed by
//! [`fjall`] for node and relationship attributes. Each attribute is stored
//! separately using composite keys.
//!
//! ## Design
//!
//! ```text
//! AttributeStore (fjall Keyspace with composite keys)
//!    ├── attrs_name: ["name", "age", "email"]  (property name → index)
//!    └── keyspace:
//!         ├── entity_id || attr_idx(0) → Value("Alice")
//!         ├── entity_id || attr_idx(1) → Value(25)
//!         └── entity_id || attr_idx(2) → Value("alice@example.com")
//! ```
//!
//! **Key format:** `entity_id (8 bytes BE) + attr_idx (2 bytes BE)`

use std::{collections::HashMap, sync::Arc};

use fjall::{Database, Keyspace, KeyspaceCreateOptions, Readable, Snapshot};
use roaring::RoaringTreemap;

use crate::runtime::{
    object_pool::get_object_pool, ordermap::OrderMap, orderset::OrderSet, value::Value,
};

/// A deferred pool operation: acquire or release an interned string ref.
#[derive(Clone)]
enum PoolOp {
    Acquire(Value),
    Release(Value),
}

/// Columnar attribute storage for graph entities backed by fjall.
///
/// Uses composite keys (entity_id + attr_idx) to store each attribute
/// separately, enabling efficient sparse storage and direct attribute access.
#[derive(Clone)]
pub struct AttributeStore {
    database: Database,
    snapshot: Snapshot,
    keyspace: Keyspace,
    /// Attribute names in insertion order (name → column index)
    pub attrs_name: OrderSet<Arc<String>>,
    /// Deferred pool operations, applied only on graph version commit.
    pool_ops: Vec<PoolOp>,
}

/// Create a composite key from entity ID and attribute index.
fn make_key(
    entity_id: u64,
    attr_idx: u16,
) -> [u8; 10] {
    let mut key = [0u8; 10];
    key[..8].copy_from_slice(&entity_id.to_be_bytes());
    key[8..].copy_from_slice(&attr_idx.to_be_bytes());
    key
}

/// Extract attribute index from a composite key.
fn extract_attr_idx(key: &[u8]) -> Option<u16> {
    if key.len() >= 10 {
        Some(u16::from_be_bytes([key[8], key[9]]))
    } else {
        None
    }
}

/// Recursively adjust object pool reference counts for interned strings
/// within a value (including nested lists/maps).
fn release_interned_refs(value: &Value) {
    match value {
        Value::InternedString(s) => {
            get_object_pool().release(s);
        }
        Value::List(items) => {
            for item in items.iter() {
                release_interned_refs(item);
            }
        }
        Value::Map(map) => {
            for v in map.values() {
                release_interned_refs(v);
            }
        }
        _ => {}
    }
}

/// Recursively acquire object pool references for interned strings.
fn acquire_interned_refs(value: &Value) {
    match value {
        Value::InternedString(s) => {
            let _ = get_object_pool().acquire(s);
        }
        Value::List(items) => {
            for item in items.iter() {
                acquire_interned_refs(item);
            }
        }
        Value::Map(map) => {
            for v in map.values() {
                acquire_interned_refs(v);
            }
        }
        _ => {}
    }
}

impl AttributeStore {
    pub fn new(
        database: Database,
        keyspace: &str,
    ) -> Self {
        let exists = database.keyspace_exists(keyspace);
        let keyspace = database
            .keyspace(keyspace, KeyspaceCreateOptions::default)
            .unwrap();
        if exists && keyspace.approximate_len() > 0 {
            // Clear existing data if keyspace already exists (for a fresh start)
            keyspace.clear().unwrap();
        }
        Self {
            snapshot: database.snapshot(),
            database,
            keyspace,
            attrs_name: OrderSet::default(),
            pool_ops: Vec::new(),
        }
    }

    #[must_use]
    pub fn new_version(&self) -> Self {
        Self {
            database: self.database.clone(),
            snapshot: self.database.snapshot(),
            keyspace: self.keyspace.clone(),
            attrs_name: self.attrs_name.clone(),
            pool_ops: Vec::new(),
        }
    }

    pub fn remove(
        &mut self,
        key: u64,
    ) -> Result<(), String> {
        // Collect values to release (deferred until graph version commit)
        let prefix = key.to_be_bytes();
        for entry in self.snapshot.prefix(&self.keyspace, prefix) {
            if let Ok((_, data)) = entry.into_inner()
                && let Some((value, _)) = Value::from_bytes(&data)
            {
                self.pool_ops.push(PoolOp::Release(value));
            }
        }

        // Remove all attributes for this entity using a batch
        let mut batch = self.database.batch();
        for entry in self.keyspace.prefix(prefix) {
            if let Ok(k) = entry.key() {
                batch.remove(&self.keyspace, k);
            }
        }
        batch.durability(None).commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[must_use]
    pub fn get_attr(
        &self,
        key: u64,
        attr: &Arc<String>,
    ) -> Option<Value> {
        let idx = self.attrs_name.get_index_of(attr)? as u16;
        self.get_attr_by_idx(key, idx)
    }

    /// Fetch an attribute value using a pre-resolved attribute index.
    /// This avoids the string-to-index lookup when fetching the same
    /// property for many entities.
    #[must_use]
    pub fn get_attr_by_idx(
        &self,
        key: u64,
        attr_idx: u16,
    ) -> Option<Value> {
        let composite_key = make_key(key, attr_idx);

        match self.snapshot.get(&self.keyspace, composite_key) {
            Ok(Some(data)) => Value::from_bytes(&data).map(|(v, _)| v),
            _ => None,
        }
    }

    #[must_use]
    pub fn has_attributes(
        &self,
        key: u64,
    ) -> bool {
        let prefix = key.to_be_bytes();
        self.snapshot
            .prefix(&self.keyspace, prefix)
            .next()
            .is_some()
    }

    pub fn get_attrs(
        &self,
        key: u64,
    ) -> impl Iterator<Item = Arc<String>> + '_ {
        let prefix = key.to_be_bytes();
        self.snapshot
            .prefix(&self.keyspace, prefix)
            .filter_map(|entry| {
                let k = entry.key().ok()?;
                let idx = extract_attr_idx(&k)?;
                let i = idx as usize;
                if i < self.attrs_name.len() {
                    Some(self.attrs_name[i].clone())
                } else {
                    None
                }
            })
    }

    pub fn get_all_attrs(
        &self,
        key: u64,
    ) -> impl Iterator<Item = (Arc<String>, Value)> + '_ {
        let prefix = key.to_be_bytes();
        self.snapshot
            .prefix(&self.keyspace, prefix)
            .filter_map(|entry| {
                let (k, data) = entry.into_inner().ok()?;
                let idx = extract_attr_idx(&k)?;
                let i = idx as usize;
                if i >= self.attrs_name.len() {
                    return None;
                }
                let (value, _) = Value::from_bytes(&data)?;
                Some((self.attrs_name[i].clone(), value))
            })
    }

    pub fn get_all_attrs_by_id(
        &self,
        key: u64,
    ) -> impl Iterator<Item = (u16, Value)> + '_ {
        let prefix = key.to_be_bytes();
        self.snapshot
            .prefix(&self.keyspace, prefix)
            .filter_map(|entry| {
                let (k, data) = entry.into_inner().ok()?;
                let idx = extract_attr_idx(&k)?;
                let (value, _) = Value::from_bytes(&data)?;
                Some((idx, value))
            })
    }

    pub fn remove_attr(
        &mut self,
        key: u64,
        attr: &Arc<String>,
    ) -> Result<bool, String> {
        if let Some(idx) = self.attrs_name.get_index_of(attr) {
            let composite_key = make_key(key, idx as u16);
            // Defer release until graph version commit
            if let Ok(Some(data)) = self.snapshot.get(&self.keyspace, composite_key)
                && let Some((value, _)) = Value::from_bytes(&data)
            {
                self.pool_ops.push(PoolOp::Release(value));
            }
            self.keyspace
                .remove(composite_key)
                .map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn remove_all(
        &mut self,
        keys: &RoaringTreemap,
    ) -> Result<(), String> {
        // Defer release until graph version commit
        for key in keys {
            let prefix = key.to_be_bytes();
            for entry in self.snapshot.prefix(&self.keyspace, prefix) {
                if let Ok((_, data)) = entry.into_inner()
                    && let Some((value, _)) = Value::from_bytes(&data)
                {
                    self.pool_ops.push(PoolOp::Release(value));
                }
            }
        }

        let mut batch = self.database.batch();
        for key in keys {
            let prefix = key.to_be_bytes();
            for entry in self.keyspace.prefix(prefix) {
                if let Ok(k) = entry.key() {
                    batch.remove(&self.keyspace, k);
                }
            }
        }
        batch.durability(None).commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Batch insert/update multiple attributes for an entity.
    /// Returns the number of attributes that were replaced (vs newly added).
    pub fn insert_attrs(
        &mut self,
        attrs: &HashMap<u64, OrderMap<Arc<String>, Value>>,
    ) -> Result<usize, String> {
        let mut nremoved = 0;
        let mut batch = self.database.batch();

        for (key, attrs) in attrs {
            for (attr, value) in attrs.iter() {
                let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
                    self.attrs_name.insert(attr.clone());
                    self.attrs_name.len() - 1
                }) as u16;

                let composite_key = make_key(*key, idx);

                if *value == Value::Null {
                    // Check snapshot for existence
                    if let Ok(Some(data)) = self.snapshot.get(&self.keyspace, composite_key) {
                        if let Some((old_value, _)) = Value::from_bytes(&data) {
                            self.pool_ops.push(PoolOp::Release(old_value));
                        }
                        batch.remove(&self.keyspace, composite_key);
                        nremoved += 1;
                    }
                } else {
                    // Check if overwriting
                    if let Ok(Some(data)) = self.snapshot.get(&self.keyspace, composite_key) {
                        if let Some((old_value, _)) = Value::from_bytes(&data) {
                            self.pool_ops.push(PoolOp::Release(old_value));
                        }
                        nremoved += 1;
                    }
                    self.pool_ops.push(PoolOp::Acquire(value.clone()));
                    batch.insert(&self.keyspace, composite_key, value.to_bytes());
                }
            }
        }

        batch.durability(None).commit().map_err(|e| e.to_string())?;
        Ok(nremoved)
    }

    #[must_use]
    pub fn get_attr_id(
        &self,
        attr: &Arc<String>,
    ) -> Option<usize> {
        self.attrs_name.get_index_of(attr)
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.keyspace.disk_space() as usize
    }

    pub fn commit(&mut self) {
        self.snapshot = self.database.snapshot();
    }

    /// Apply all deferred pool operations. Called when the graph version is committed.
    pub fn apply_pool_ops(&mut self) {
        for op in self.pool_ops.drain(..) {
            match op {
                PoolOp::Acquire(ref value) => acquire_interned_refs(value),
                PoolOp::Release(ref value) => release_interned_refs(value),
            }
        }
    }

    /// Release interned refs for all values in this attribute store.
    /// Called when the graph is being destroyed (GRAPH.DELETE).
    pub fn release_all_interned_refs(&self) {
        for entry in self.snapshot.prefix(&self.keyspace, []) {
            if let Ok((_, data)) = entry.into_inner()
                && let Some((value, _)) = Value::from_bytes(&data)
            {
                release_interned_refs(&value);
            }
        }
    }
}

unsafe impl Send for AttributeStore {}
unsafe impl Sync for AttributeStore {}
