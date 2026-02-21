//! Multi-Version Concurrency Control for graph access.
//!
//! This module provides [`MvccGraph`], the top-level coordinator for concurrent
//! graph access. It ensures:
//!
//! - Multiple readers can access the graph simultaneously
//! - Only one writer at a time (serialized writes)
//! - Writers work on a versioned copy, committing atomically
//!
//! ## Concurrency Model
//!
//! ```text
//! MvccGraph
//!    ├── graph: Arc<AtomicRefCell<Graph>>  (current committed version)
//!    └── write: AtomicBool  (write lock)
//!
//! read()  → Clone Arc (readers see committed state)
//! write() → Create new version (if lock acquired)
//! commit() → Swap graph pointer, release lock
//! rollback() → Discard version, release lock
//! ```
//!
//! ## Good Practice: Lock-Free Reads
//!
//! Readers never block - they simply clone the Arc to the current graph.
//! The atomic bool only serializes write acquisition, not read access.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use atomic_refcell::AtomicRefCell;

use crate::graph::graph::Graph;

/// MVCC coordinator for concurrent graph access.
///
/// Provides snapshot isolation: readers see a consistent committed state
/// while a writer can make changes that become visible only on commit.
pub struct MvccGraph {
    /// Current committed graph version
    graph: Arc<AtomicRefCell<Graph>>,
    /// Write lock (true = write in progress)
    write: AtomicBool,
}

// SAFETY: MvccGraph is safe to Send/Sync because:
// - AtomicRefCell provides interior mutability with runtime borrow checking
// - Arc provides safe shared ownership across threads
// - AtomicBool ensures atomic synchronization of the write flag
// - Graph itself is Send+Sync, so wrapping it in Arc<AtomicRefCell<>> is safe
// - The MVCC pattern ensures readers get consistent snapshots while writers are isolated
unsafe impl Send for MvccGraph {}
unsafe impl Sync for MvccGraph {}

impl MvccGraph {
    #[must_use]
    pub fn new(
        n: u64,
        e: u64,
        cache_size: usize,
        name: &str,
    ) -> Self {
        Self {
            graph: Arc::new(AtomicRefCell::new(Graph::new(n, e, cache_size, 0, name))),
            write: AtomicBool::new(false),
        }
    }

    #[must_use]
    pub fn read(&self) -> Arc<AtomicRefCell<Graph>> {
        self.graph.clone()
    }

    #[must_use]
    pub fn write(&self) -> Option<Arc<AtomicRefCell<Graph>>> {
        if self
            .write
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(Arc::new(AtomicRefCell::new(
                self.graph.borrow().new_version(),
            )))
        } else {
            None
        }
    }

    pub fn commit(
        &mut self,
        new_graph: Arc<AtomicRefCell<Graph>>,
    ) {
        debug_assert_eq!(self.graph.borrow().version + 1, new_graph.borrow().version);
        self.graph = new_graph;
        self.write.store(false, Ordering::Release);
    }

    pub fn rollback(&self) {
        self.write.store(false, Ordering::Release);
    }
}
