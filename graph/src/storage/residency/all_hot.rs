//! The OSS default [`Residency`]: everything is hot, forever.

use super::super::error::Result;
use super::{Residency, ResidentId, ShardGuard};

/// The OSS residency: everything is hot, forever. The pool is effectively
/// unbounded — `on_access` / `admit` never evict (nothing is cold) and
/// `resident_bytes` is untracked (returns 0).
#[derive(Debug, Default, Clone, Copy)]
pub struct AllHot;

impl Residency for AllHot {
    fn on_access(
        &self,
        id: ResidentId,
    ) -> Result<ShardGuard> {
        // Nothing is ever cold under AllHot, so this is a pure pin.
        Ok(ShardGuard::new(id))
    }

    fn admit(
        &self,
        id: ResidentId,
        _bytes: usize,
    ) -> Result<ShardGuard> {
        // AllHot tracks no budget, so admission always succeeds without eviction.
        Ok(ShardGuard::new(id))
    }

    fn on_commit(
        &self,
        _dirty: &[ResidentId],
    ) -> Result<()> {
        Ok(())
    }

    fn resident_bytes(&self) -> usize {
        0
    }
}
