//! The **one process-wide RAM↔disk resident pool** and its controller.
//!
//! Tiering is governed by a single pool of resident bytes under a configurable
//! memory budget. Every pluggable store ([`StoreKind`]) shares this *one* pool,
//! so the index and the attribute store compete for a single budget rather than
//! each carrying its own — which is the whole point of a budget.
//!
//! [`Residency`] is **store-agnostic**: it accounts bytes, pins, and evicts,
//! keyed by an opaque [`ResidentId`]. It deliberately does **not** know how to
//! decode a cold shard — that is store-specific (the index decodes a matrix base
//! + WAL tail; the attribute store decodes its own structure). A store faults
//! its own bytes in, then calls [`Residency::admit`] to charge the pool
//! (evicting LRU unpinned entries first) and obtain a [`ShardGuard`] pin. This
//! keeps each store's durable record types (e.g. the index's `BackendSnapshot` /
//! `WalRecord`) out of this shared, ungated module — which is exactly why
//! `Residency` can live here while the per-store `StorageBackend` cannot.
//!
//! OSS ships [`AllHot`]: the pool is effectively unbounded — nothing is ever
//! cold and nothing is ever evicted.

mod all_hot;

pub use all_hot::AllHot;

use super::StoreKind;
use super::error::Result;

/// A store-agnostic resident-pool key: which store, and an opaque shard handle
/// within it. The index maps its `ShardId` (structure + ordinal) onto the
/// `shard` field; other stores choose their own packing. Two different stores
/// never collide because [`StoreKind`] discriminates them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResidentId {
    /// The store that owns the resident bytes.
    pub store: StoreKind,
    /// An opaque, store-chosen shard handle.
    pub shard: u64,
}

impl ResidentId {
    /// Construct a resident-pool key for `store`'s `shard`.
    #[must_use]
    pub const fn new(
        store: StoreKind,
        shard: u64,
    ) -> Self {
        Self { store, shard }
    }
}

/// A pin that keeps a resident entry in the pool for the duration of use. While
/// any `ShardGuard` for an entry is alive, the controller will not evict it.
/// Dropping the guard releases the pin (the disk analogue of an `Arc` drop).
///
/// Carries no refcount state under [`AllHot`]; the enterprise controller makes
/// it a handle into the resident pool.
#[derive(Debug)]
#[must_use = "dropping the guard immediately unpins the entry"]
pub struct ShardGuard {
    id: ResidentId,
}

impl ShardGuard {
    /// Pin `id`. Constructed by [`Residency`] implementations.
    pub(crate) const fn new(id: ResidentId) -> Self {
        Self { id }
    }

    /// The resident entry this guard pins.
    #[must_use]
    pub const fn id(&self) -> ResidentId {
        self.id
    }
}

/// Controls the one process-wide resident pool: pins entries on access, charges
/// admitted bytes, evicts under budget, and (on the enterprise build) enqueues
/// digests. Shared as `Arc<dyn Residency>` across the writer and all readers of
/// every store.
pub trait Residency: Send + Sync {
    /// A reader/writer is about to touch an **already-resident** `id`: bump LRU
    /// recency and return a pin. If the entry is cold the caller must first
    /// fault its bytes in (store-specific decode) and call
    /// [`admit`](Residency::admit) instead.
    fn on_access(
        &self,
        id: ResidentId,
    ) -> Result<ShardGuard>;

    /// Charge `bytes` for a freshly faulted-in `id`, evicting LRU unpinned
    /// entries first if admission would otherwise exceed the budget, then return
    /// a pin. The store performs the decode; `admit` only does pool accounting.
    fn admit(
        &self,
        id: ResidentId,
        bytes: usize,
    ) -> Result<ShardGuard>;

    /// After a commit touched `dirty`: re-account those entries, evict down to
    /// the budget if over, and (enterprise) enqueue a digest for any entry whose
    /// WAL crossed its cap.
    fn on_commit(
        &self,
        dirty: &[ResidentId],
    ) -> Result<()>;

    /// Current pool occupancy in bytes (Σ resident bytes across all stores).
    fn resident_bytes(&self) -> usize;
}
