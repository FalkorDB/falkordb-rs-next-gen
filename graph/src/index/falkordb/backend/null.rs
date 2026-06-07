//! The OSS default [`StorageBackend`]: persists nothing, so restart rebuilds.

use crate::index::falkordb::error::Result;
use crate::index::falkordb::id::{ShardId, SnapKey};

use super::wal::{Keyspace, WalRecord};
use super::{BackendSnapshot, BoxIter, StorageBackend};

/// The OSS backend: a no-op that persists nothing. With `NullBackend`
/// installed, restart always rebuilds via `populate_indexes_sync`
/// (durability §1, §8).
#[derive(Debug, Default, Clone, Copy)]
pub struct NullBackend;

impl StorageBackend for NullBackend {
    fn wal_append(
        &self,
        _shard: ShardId,
        _records: &[WalRecord],
    ) -> Result<()> {
        Ok(())
    }

    fn wal_scan(
        &self,
        _shard: ShardId,
        _from_version: u64,
    ) -> BoxIter<WalRecord> {
        Box::new(std::iter::empty())
    }

    fn wal_truncate(
        &self,
        _shard: ShardId,
        _up_to_version: u64,
    ) -> Result<()> {
        Ok(())
    }

    fn put_blob(
        &self,
        _key: SnapKey,
        _blob: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn get_blob(
        &self,
        _key: SnapKey,
    ) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn delete_blob(
        &self,
        _key: SnapKey,
    ) -> Result<()> {
        Ok(())
    }

    fn snapshot(&self) -> Result<Box<dyn BackendSnapshot>> {
        Ok(Box::new(NullSnapshot))
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }

    fn put(
        &self,
        _ks: Keyspace,
        _k: &[u8],
        _v: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn get(
        &self,
        _ks: Keyspace,
        _k: &[u8],
    ) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn range(
        &self,
        _ks: Keyspace,
        _lo: &[u8],
        _hi: &[u8],
    ) -> BoxIter<(Vec<u8>, Vec<u8>)> {
        Box::new(std::iter::empty())
    }
}

/// The empty snapshot handed out by [`NullBackend`]; pinned at version 0.
#[derive(Debug, Default, Clone, Copy)]
struct NullSnapshot;

impl BackendSnapshot for NullSnapshot {
    fn get_blob(
        &self,
        _key: SnapKey,
    ) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn wal_scan(
        &self,
        _shard: ShardId,
        _from_version: u64,
    ) -> BoxIter<WalRecord> {
        Box::new(std::iter::empty())
    }

    fn version(&self) -> u64 {
        0
    }
}
