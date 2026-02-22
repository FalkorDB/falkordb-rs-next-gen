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

use std::sync::Arc;

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
        let keyspace = database
            .keyspace(keyspace, KeyspaceCreateOptions::default)
            .unwrap();
        keyspace.clear().unwrap(); // Clear any existing data for a fresh start
        Self {
            database: database.clone(),
            snapshot: database.snapshot(),
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

    #[must_use]
    pub fn get_attrs(
        &self,
        key: u64,
    ) -> Vec<Arc<String>> {
        let prefix = key.to_be_bytes();
        let mut names = vec![];

        for entry in self.snapshot.prefix(&self.keyspace, prefix) {
            if let Ok(k) = entry.key()
                && let Some(idx) = extract_attr_idx(&k)
            {
                let i = idx as usize;
                if i < self.attrs_name.len() {
                    names.push(self.attrs_name[i].clone());
                }
            }
        }

        names
    }

    #[must_use]
    pub fn get_all_attrs(
        &self,
        key: u64,
    ) -> OrderMap<Arc<String>, Value> {
        let prefix = key.to_be_bytes();
        let mut attrs = OrderMap::default();

        for entry in self.snapshot.prefix(&self.keyspace, prefix) {
            if let Ok((k, data)) = entry.into_inner()
                && let Some(idx) = extract_attr_idx(&k)
            {
                let i = idx as usize;
                if i < self.attrs_name.len() {
                    if let Some((value, _)) = Value::from_bytes(&data) {
                        attrs.insert(self.attrs_name[i].clone(), value);
                    }
                }
            }
        }

        attrs
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

    pub fn insert_attr(
        &mut self,
        key: u64,
        attr: &Arc<String>,
        value: Value,
    ) -> Result<bool, String> {
        let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
            self.attrs_name.insert(attr.clone());
            self.attrs_name.len() - 1
        }) as u16;

        let composite_key = make_key(key, idx);

        // Check snapshot for existing value (avoids expensive live keyspace read)
        let replaced = self
            .snapshot
            .contains_key(&self.keyspace, composite_key)
            .map_err(|e| e.to_string())?;

        self.keyspace
            .insert(composite_key, value.to_bytes())
            .map_err(|e| e.to_string())?;

        Ok(replaced)
    }

    /// Batch insert/update multiple attributes for an entity.
    /// Returns the number of attributes that were replaced (vs newly added).
    pub fn insert_attrs(
        &mut self,
        key: u64,
        attrs: &crate::runtime::ordermap::OrderMap<Arc<String>, Value>,
    ) -> Result<usize, String> {
        let mut nremoved = 0;
        let mut batch = self.database.batch();

        for (attr, value) in attrs.iter() {
            let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
                self.attrs_name.insert(attr.clone());
                self.attrs_name.len() - 1
            }) as u16;

            let composite_key = make_key(key, idx);

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

        batch.durability(None).commit().map_err(|e| e.to_string())?;
        Ok(nremoved)
    }

    /// Creates a deep copy of this attribute store with a new keyspace name.
    ///
    /// Copies all data from the current keyspace into a fresh keyspace,
    /// producing an independent attribute store for a copied graph.
    pub fn copy(
        &self,
        name: &str,
    ) -> Self {
        let new_keyspace = self
            .database
            .keyspace(name, KeyspaceCreateOptions::default)
            .unwrap();
        new_keyspace.clear().unwrap();

        let mut batch = self.database.batch();
        for entry in self.snapshot.iter(&self.keyspace) {
            if let Ok((k, v)) = entry.into_inner() {
                batch.insert(&new_keyspace, k, v);
            }
        }
        batch.durability(None).commit().unwrap();

        Self {
            database: self.database.clone(),
            snapshot: self.database.snapshot(),
            keyspace: new_keyspace,
            attrs_name: self.attrs_name.clone(),
        }
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

unsafe impl Send for AttributeStore {}
unsafe impl Sync for AttributeStore {}
