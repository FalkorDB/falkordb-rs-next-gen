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

use crate::runtime::{ordermap::OrderMap, orderset::OrderSet, value::Value};

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
        }
    }

    #[must_use]
    pub fn new_version(&self) -> Self {
        Self {
            database: self.database.clone(),
            snapshot: self.database.snapshot(),
            keyspace: self.keyspace.clone(),
            attrs_name: self.attrs_name.clone(),
        }
    }

    pub fn remove(
        &mut self,
        key: u64,
    ) -> Result<(), String> {
        // Remove all attributes for this entity using a batch
        let prefix = key.to_be_bytes();
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
        let composite_key = make_key(key, idx);

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

        for (key, attrs) in attrs.iter() {
            for (attr, value) in attrs.iter() {
                let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
                    self.attrs_name.insert(attr.clone());
                    self.attrs_name.len() - 1
                }) as u16;

                let composite_key = make_key(u64::from(*key), idx);

                if *value == Value::Null {
                    // Check snapshot for existence
                    if self
                        .snapshot
                        .contains_key(&self.keyspace, composite_key)
                        .map_err(|e| e.to_string())?
                    {
                        batch.remove(&self.keyspace, composite_key);
                        nremoved += 1;
                    }
                } else {
                    // Check snapshot for replaced count
                    if self
                        .snapshot
                        .contains_key(&self.keyspace, composite_key)
                        .map_err(|e| e.to_string())?
                    {
                        nremoved += 1;
                    }
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
}

// SAFETY: AttributeStore wraps fjall Database/Keyspace/Snapshot, all of which are
// designed for concurrent access. The attrs_name OrderSet is only mutated during
// write transactions (which are serialized by MvccGraph). Each reader gets its own
// snapshot, so there is no shared mutable state across threads.
unsafe impl Send for AttributeStore {}
unsafe impl Sync for AttributeStore {}
