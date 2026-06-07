//! The OSS default [`Residency`]: everything is hot, forever.

use crate::index::native::backend::BackendSnapshot;
use crate::index::native::error::Result;
use crate::index::native::id::ShardId;

use super::{Residency, ShardGuard};

/// The OSS residency: everything is hot, forever. The pool is effectively
/// unbounded — `on_access` never faults (nothing is cold) and nothing is ever
/// evicted (durability §6.3).
#[derive(Debug, Default, Clone, Copy)]
pub struct AllHot;

impl Residency for AllHot {
    fn on_access(
        &self,
        shard: ShardId,
        _snap: &dyn BackendSnapshot,
    ) -> Result<ShardGuard> {
        // Nothing is ever cold under AllHot, so this is a pure pin.
        Ok(ShardGuard::new(shard))
    }

    fn on_commit(
        &self,
        _dirty: &[ShardId],
    ) -> Result<()> {
        Ok(())
    }

    fn resident_bytes(&self) -> usize {
        // AllHot does not track a budget.
        0
    }
}
