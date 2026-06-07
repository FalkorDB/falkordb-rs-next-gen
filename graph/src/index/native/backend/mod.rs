//! The durable-bytes boundary: [`StorageBackend`] + [`BackendSnapshot`].
//!
//! This is the **OSS↔enterprise seam**
//! (durability §6.1). The
//! trait is **object-safe on purpose** — no associated types, `snapshot()`
//! returns `Box<dyn BackendSnapshot>` — so the whole backend injects as
//! `Arc<dyn StorageBackend>` (see [`super::register_index_backend`]). OSS
//! registers [`NullBackend`]; the enterprise repo registers a `FjallBackend`.
//!
//! [`wal`] holds the version-tagged WAL record types the backend persists.

mod null;
pub mod wal;

pub use null::NullBackend;

use super::error::Result;
use super::id::{ShardId, SnapKey};

use self::wal::{Keyspace, WalRecord};

/// A boxed, `Send` owning iterator — what the backend hands back for WAL/KV
/// scans (object-safe; no borrowed lifetime in the trait surface).
pub type BoxIter<T> = Box<dyn Iterator<Item = T> + Send>;

/// Durable storage for the index: per-shard WAL, base-snapshot blobs, a
/// read/reclaim snapshot, and a plain-KV metadata area.
///
/// All methods take `&self` and the trait is `Send + Sync`, so one backend is
/// shared as `Arc<dyn StorageBackend>` across the writer and all read threads.
pub trait StorageBackend: Send + Sync {
    // --- per-shard WAL (write / durability path) ---

    /// Append version-tagged records to a shard's WAL.
    fn wal_append(
        &self,
        shard: ShardId,
        records: &[WalRecord],
    ) -> Result<()>;

    /// Scan a shard's WAL forward from `from_version` (tail replay / digest).
    fn wal_scan(
        &self,
        shard: ShardId,
        from_version: u64,
    ) -> BoxIter<WalRecord>;

    /// Drop folded WAL records up to `up_to_version` (after a digest).
    fn wal_truncate(
        &self,
        shard: ShardId,
        up_to_version: u64,
    ) -> Result<()>;

    // --- base-snapshot blobs (matrix serialized as-is) ---

    /// Store a base blob for a shard at a version.
    fn put_blob(
        &self,
        key: SnapKey,
        blob: &[u8],
    ) -> Result<()>;

    /// Read a base blob, if present.
    fn get_blob(
        &self,
        key: SnapKey,
    ) -> Result<Option<Vec<u8>>>;

    /// Delete a base blob (DROP INDEX / explicit GC).
    fn delete_blob(
        &self,
        key: SnapKey,
    ) -> Result<()>;

    // --- backend snapshot = disk read + reclaim view ---

    /// Open a consistent on-disk view. A cold reader reads base+WAL *through*
    /// this so the digest thread can rewrite/truncate freely underneath it.
    /// It provides read-consistency and reclamation, **not** version selection
    /// (the logical graph version selects what a reader sees).
    fn snapshot(&self) -> Result<Box<dyn BackendSnapshot>>;

    /// One durability barrier per commit (batched across all dirty shards).
    fn flush(&self) -> Result<()>;

    // --- plain KV (metadata) ---

    /// Store a metadata key.
    fn put(
        &self,
        ks: Keyspace,
        k: &[u8],
        v: &[u8],
    ) -> Result<()>;

    /// Read a metadata key.
    fn get(
        &self,
        ks: Keyspace,
        k: &[u8],
    ) -> Result<Option<Vec<u8>>>;

    /// Range-scan a metadata keyspace `[lo, hi)`.
    fn range(
        &self,
        ks: Keyspace,
        lo: &[u8],
        hi: &[u8],
    ) -> BoxIter<(Vec<u8>, Vec<u8>)>;
}

/// A pinned, consistent on-disk view. Reclamation of the bytes it references is
/// deferred until the last snapshot referencing them drops — the disk analogue
/// of the in-RAM `Arc` drop.
pub trait BackendSnapshot: Send + Sync {
    /// Read a base blob as of this snapshot.
    fn get_blob(
        &self,
        key: SnapKey,
    ) -> Result<Option<Vec<u8>>>;

    /// Scan a shard's WAL forward from `from_version`, as of this snapshot.
    fn wal_scan(
        &self,
        shard: ShardId,
        from_version: u64,
    ) -> BoxIter<WalRecord>;

    /// The graph/index version pinned by this snapshot.
    fn version(&self) -> u64;
}
