//! Runtime injection (mechanism A) for the shared storage seam.
//!
//! The OSS core exposes [`register_backend`] / [`register_residency`] and
//! **defaults** to the OSS impls (each store falls back to its own default
//! backend; residency defaults to [`AllHot`]). The enterprise crate calls these
//! once at module init to swap in its disk-backed impls. A Cargo feature
//! (`index-falkordb`, and later the attribute-store equivalent) only decides
//! whether a *consumer* is compiled in; it does not name the enterprise impls.
//! Everything stays statically linked into the single `.so`.
//!
//! Backends are **per-store** and therefore stored type-erased: each store
//! registers and retrieves its own backend trait object — the index registers an
//! `Arc<dyn StorageBackend>` under [`StoreKind::Index`]; the attribute store will
//! register its own under [`StoreKind::Attrs`]. The residency pool is **shared**:
//! a single `Arc<dyn Residency>` for the whole process.
//!
//! Type erasure note: an `Arc<dyn SomeBackend>` is itself a `'static + Sized`
//! value, so it implements [`Any`] and round-trips through `Box<dyn Any>` —
//! [`backend`] downcasts back to the exact `Arc<B>` the store registered. A store
//! must read with the same trait `B` it wrote.

#![allow(dead_code)] // Consumed by index/falkordb (feature-gated) and, later, the attribute store.

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use parking_lot::RwLock;

use super::StoreKind;
use super::residency::{AllHot, Residency};

/// The injected impls: per-store backends (type-erased) + one shared residency.
struct Registry {
    /// Per-store backend trait objects, type-erased as `Box<dyn Any>` wrapping an
    /// `Arc<dyn …Backend>`. Each store downcasts back to its own backend trait.
    backends: HashMap<StoreKind, Box<dyn Any + Send + Sync>>,
    /// The one process-wide residency pool, shared by every store.
    residency: Arc<dyn Residency>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            backends: HashMap::new(),
            residency: Arc::new(AllHot),
        }
    }
}

fn registry() -> &'static RwLock<Registry> {
    static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(Registry::default()))
}

/// Register `store`'s durable backend. Call once at startup, before any of that
/// store's structures are created. `B` is the store's own backend trait (e.g.
/// `dyn StorageBackend` for the index); it is stored type-erased and handed back
/// by [`backend`] to the same store, which knows its concrete `B`.
pub fn register_backend<B>(
    store: StoreKind,
    backend: Arc<B>,
) where
    B: ?Sized + Send + Sync + 'static,
{
    registry().write().backends.insert(store, Box::new(backend));
}

/// The backend registered for `store`, as that store's backend trait object
/// `Arc<B>`, or `None` if nothing was registered (the store then uses its own
/// default — e.g. the index falls back to `NullBackend`). `B` must be the exact
/// trait the store registered under `store`.
#[must_use]
pub fn backend<B>(store: StoreKind) -> Option<Arc<B>>
where
    B: ?Sized + Send + Sync + 'static,
{
    registry()
        .read()
        .backends
        .get(&store)
        .and_then(|b| b.downcast_ref::<Arc<B>>())
        .cloned()
}

/// Install the one shared residency controller. Call once at startup. In OSS
/// this is never called and the default ([`AllHot`]) stands; the enterprise
/// crate calls it with its tiered, budgeted impl.
pub fn register_residency(residency: Arc<dyn Residency>) {
    registry().write().residency = residency;
}

/// The shared residency controller (an `Arc` clone). Defaults to [`AllHot`]
/// until [`register_residency`] runs.
#[must_use]
pub fn residency() -> Arc<dyn Residency> {
    registry().read().residency.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    // A throwaway backend trait to prove the type-erased round-trip without
    // depending on any real store's backend.
    trait DummyBackend: Send + Sync {
        fn tag(&self) -> u32;
    }
    struct Dummy(u32);
    impl DummyBackend for Dummy {
        fn tag(&self) -> u32 {
            self.0
        }
    }

    #[test]
    fn backend_round_trips_per_store() {
        // Nothing registered under Attrs by default.
        assert!(backend::<dyn DummyBackend>(StoreKind::Attrs).is_none());

        register_backend(
            StoreKind::Attrs,
            Arc::new(Dummy(7)) as Arc<dyn DummyBackend>,
        );
        let got = backend::<dyn DummyBackend>(StoreKind::Attrs).expect("registered");
        assert_eq!(got.tag(), 7);
    }

    #[test]
    fn residency_defaults_to_all_hot() {
        // The default pool is unbounded (AllHot reports 0 resident bytes).
        assert_eq!(residency().resident_bytes(), 0);
    }
}
