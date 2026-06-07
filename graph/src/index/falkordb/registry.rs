//! The OSS↔enterprise injection point (mechanism A,
//! poc-plan §2).
//!
//! The OSS core exposes [`register_index_backend`] and **defaults** to
//! [`NullBackend`] + [`AllHot`]. The enterprise repo calls it once at module
//! init with its disk-backed `FjallBackend` + `TieredResidency` — so the OSS
//! manifest never names the closed-source crate, and everything stays
//! statically linked into the single `.so`. The `index-falkordb` Cargo feature
//! only decides whether this seam is compiled in; it does not name the impls.

use std::sync::{Arc, OnceLock};

use parking_lot::RwLock;

use super::backend::{NullBackend, StorageBackend};
use super::residency::{AllHot, Residency};

/// The currently-installed backend + residency. Replaced wholesale by
/// [`register_index_backend`]; read (as cheap `Arc` clones) by the index core.
struct Registry {
    backend: Arc<dyn StorageBackend>,
    residency: Arc<dyn Residency>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            backend: Arc::new(NullBackend),
            residency: Arc::new(AllHot),
        }
    }
}

fn registry() -> &'static RwLock<Registry> {
    static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(Registry::default()))
}

/// Install the durable backend + residency controller. Call once at startup,
/// before any index is created. In OSS this is never called and the defaults
/// (`NullBackend` + `AllHot`) stand; the enterprise repo calls it with its
/// disk-backed impls.
pub fn register_index_backend(
    backend: Arc<dyn StorageBackend>,
    residency: Arc<dyn Residency>,
) {
    let mut reg = registry().write();
    reg.backend = backend;
    reg.residency = residency;
}

/// The installed [`StorageBackend`] (an `Arc` clone). Defaults to
/// [`NullBackend`] until [`register_index_backend`] runs.
#[must_use]
pub fn index_backend() -> Arc<dyn StorageBackend> {
    registry().read().backend.clone()
}

/// The installed [`Residency`] controller (an `Arc` clone). Defaults to
/// [`AllHot`] until [`register_index_backend`] runs.
#[must_use]
pub fn index_residency() -> Arc<dyn Residency> {
    registry().read().residency.clone()
}
