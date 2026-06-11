//! The index subsystem's view of the shared storage seam
//! ([`crate::storage`]).
//!
//! The registry itself, the [`Residency`] trait, and the one shared residency
//! pool now live in [`crate::storage`] so the index and the (future) attribute
//! store share them. These thin wrappers preserve the index's existing
//! `index_backend()` / `register_index_backend()` API over that shared, per-store
//! registry: the index registers its [`StorageBackend`] under [`StoreKind::Index`]
//! and supplies the OSS default ([`NullBackend`]) when the enterprise crate has
//! not registered a disk-backed backend.

use std::sync::Arc;

use crate::storage::{self, Residency, StoreKind};

use super::backend::{NullBackend, StorageBackend};

/// Install the index's durable backend + the shared residency pool. OSS never
/// calls this (the defaults stand: `NullBackend` + `AllHot`); the enterprise repo
/// calls it once at module init with its `FjallBackend` + tiered residency.
pub fn register_index_backend(
    backend: Arc<dyn StorageBackend>,
    residency: Arc<dyn Residency>,
) {
    storage::register_backend(StoreKind::Index, backend);
    storage::register_residency(residency);
}

/// The index's [`StorageBackend`] (an `Arc` clone). Falls back to [`NullBackend`]
/// until [`register_index_backend`] runs.
#[must_use]
pub fn index_backend() -> Arc<dyn StorageBackend> {
    storage::backend::<dyn StorageBackend>(StoreKind::Index)
        .unwrap_or_else(|| Arc::new(NullBackend))
}

/// The shared [`Residency`] controller (an `Arc` clone). Defaults to `AllHot`
/// until [`register_index_backend`] runs.
#[must_use]
pub fn index_residency() -> Arc<dyn Residency> {
    storage::residency()
}
