//! Global string interning pool for deduplicating string values.
//!
//! The [`ObjectPool`] is a singleton shared across all graphs in the database.
//! When a string is interned via `intern()`, a single canonical copy is stored
//! here. Reference counts track how many attribute store entries reference
//! each interned string; when the count drops to zero the entry is removed.

use parking_lot::RwLock;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

static OBJECT_POOL: OnceLock<ObjectPool> = OnceLock::new();

pub struct ObjectPool {
    inner: RwLock<HashMap<Arc<String>, u64>>,
}

impl ObjectPool {
    fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// Increment the reference count for the given string.
    /// If the string is not in the pool, insert it with count 1.
    /// Returns the canonical `Arc<String>` from the pool.
    pub fn acquire(
        &self,
        s: &Arc<String>,
    ) -> Arc<String> {
        let mut map = self.inner.write();
        match map.entry(s.clone()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
                entry.key().clone()
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let canonical = entry.key().clone();
                entry.insert(1);
                canonical
            }
        }
    }

    /// Decrement the reference count for the given string.
    /// Removes the entry when the count reaches zero.
    pub fn release(
        &self,
        s: &Arc<String>,
    ) {
        let mut map = self.inner.write();
        if let Some(count) = map.get_mut(s) {
            match count.checked_sub(1) {
                Some(0) => {
                    map.remove(s);
                }
                Some(new_count) => {
                    *count = new_count;
                }
                None => {
                    debug_assert!(false, "ObjectPool::release underflow for {:?}", s);
                    map.remove(s);
                }
            }
        }
    }

    /// Returns (unique_object_count, average_refs_per_object).
    pub fn stats(&self) -> (usize, f64) {
        let map = self.inner.read();
        let count = map.len();
        if count == 0 {
            return (0, 0.0);
        }
        let total: u64 = map.values().sum();
        (count, total as f64 / count as f64)
    }

    /// Clear the pool (called on FlushDB).
    pub fn clear(&self) {
        self.inner.write().clear();
    }
}

/// Initialize the global object pool. Call once at module startup.
pub fn init_object_pool() {
    OBJECT_POOL.get_or_init(ObjectPool::new);
}

/// Get a reference to the global object pool.
/// Lazily initializes the pool if it hasn't been initialized yet.
pub fn get_object_pool() -> &'static ObjectPool {
    OBJECT_POOL.get_or_init(ObjectPool::new)
}
