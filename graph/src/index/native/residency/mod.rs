//! The RAM↔disk resident-pool controller: the [`Residency`] trait and its
//! [`ShardGuard`] pin.
//!
//! Tiering is governed by **one process-wide pool of resident matrices with a
//! configurable byte size** (the memory budget)
//! (durability §6.3). A
//! shard's matrix occupies the pool only while hot; everything else is cold (on
//! disk, with a tiny in-RAM descriptor). `Residency` is the controller for that
//! pool. OSS ships [`AllHot`]: the pool is effectively unbounded — nothing is
//! ever faulted in (nothing is cold) and nothing is ever evicted.

mod all_hot;

pub use all_hot::AllHot;

use super::backend::BackendSnapshot;
use super::error::Result;
use super::id::ShardId;

/// A pin that keeps a shard **resident** for the duration of use. While any
/// `ShardGuard` for a shard is alive, the controller will not evict it. Dropping
/// the guard releases the pin (the disk analogue of an `Arc` drop).
///
/// M0 carries no state; later milestones make it a refcount handle into the
/// resident pool.
#[derive(Debug)]
#[must_use = "dropping the guard immediately unpins the shard"]
pub struct ShardGuard {
    shard: ShardId,
}

impl ShardGuard {
    /// Pin `shard`. Constructed by [`Residency`] implementations.
    pub(crate) const fn new(shard: ShardId) -> Self {
        Self { shard }
    }

    /// The shard this guard pins.
    #[must_use]
    pub const fn shard(&self) -> ShardId {
        self.shard
    }
}

/// Controls the process-wide resident pool: faults shards in on access, evicts
/// under budget, and (on the enterprise build) enqueues digests.
pub trait Residency: Send + Sync {
    /// A reader/writer is about to touch `shard`.
    ///
    /// - *Already resident:* bump LRU recency and return a pin.
    /// - *Cold:* fault it in (`Decode(base)` + apply WAL tail under `snap`,
    ///   evicting LRU unpinned shards first if admission would exceed the pool
    ///   size), then return the pin.
    fn on_access(
        &self,
        shard: ShardId,
        snap: &dyn BackendSnapshot,
    ) -> Result<ShardGuard>;

    /// After a commit mutated `dirty`: re-account hot shards, evict down to the
    /// budget if over, and (enterprise) enqueue a digest for any shard whose
    /// WAL crossed its cap.
    fn on_commit(
        &self,
        dirty: &[ShardId],
    ) -> Result<()>;

    /// Current pool occupancy in bytes (Σ resident matrix + payload bytes).
    fn resident_bytes(&self) -> usize;
}
