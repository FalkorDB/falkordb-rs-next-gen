//! FalkorDB MVCC index subsystem — trait seams + skeleton (milestone **M0**).
//!
//! This module is the storage-agnostic, logical-MVCC index that replaces
//! RediSearch. It is gated behind the `index-falkordb` Cargo feature (OFF by
//! default), so the default build is unaffected and RediSearch stays the active
//! path. Nothing in the runtime calls into here yet — M0
//! delivers only the *seams* that later milestones fill in:
//!
//! - [`StorageBackend`] / [`BackendSnapshot`] — the durable-bytes boundary
//!   (enterprise §6.1). OSS
//!   ships [`NullBackend`] (persists nothing → rebuild on restart); the
//!   enterprise repo registers a `FjallBackend`.
//! - [`Residency`] — the RAM↔disk resident-pool controller
//!   (durability §6.3). OSS
//!   ships [`AllHot`] (everything resident, never evicts).
//! - [`Index`] — the per-structure index trait with the single enum-keyed
//!   [`crate::index::IndexQuery`] read entry
//!   (query-api §8).
//! - [`IndexScan`] / [`IndexScanIter`] — the `'q`-scoped pull iterator
//!   (query-api §2, §4).
//!
//! The OSS↔enterprise seam is **runtime injection** (mechanism A): the OSS core
//! exposes [`register_index_backend`] and defaults to `NullBackend`/`AllHot`;
//! the enterprise repo calls it at module init with its disk-backed impls.
//! Everything stays statically linked into the single `.so`.

// --- foundational vocabulary (depended on everywhere) ---
mod error;
mod id;

// --- the storage-agnostic index contract the runtime calls + each kind implements ---
mod api;
// --- the index's per-store durable backend seam + its OSS default (NullBackend).
// The shared registry + residency pool live in `crate::storage`, not here. ---
mod backend;
// --- thin shim over `crate::storage`'s shared registry (StoreKind::Index) ---
mod registry;
// --- the numeric POC kind (the only kind for now) ---
mod numeric;
// --- per-graph registry of FalkorDB index structures (shared across versions) ---
mod manager;

pub use error::{IndexError, Result};
pub use id::{DocKey, ShardId, SnapKey, StructureId};

pub use api::{
    BoundSide, EntityKind, Index, IndexHit, IndexScan, IndexScanIter, IndexSchema, MemoryBreakdown,
    RowEncoder, ScanOptions,
};

pub use backend::wal::{Cell, Keyspace, WalOp, WalRecord};
pub use backend::{BackendSnapshot, BoxIter, NullBackend, StorageBackend};

// The residency pool + its trait are shared infrastructure in `crate::storage`;
// re-exported here for continuity of the `index::falkordb` surface.
pub use crate::storage::{AllHot, Residency, ResidentId, ShardGuard};

pub use registry::{index_backend, index_residency, register_index_backend};

pub use numeric::{NumericEncoder, NumericIndex, NumericScan};

pub use manager::FalkorDbIndexes;

/// Initialize GraphBLAS once for the FalkorDB-index unit tests.
///
/// In production GraphBLAS is initialized on the Redis module-load path
/// ([`crate::graph::graphblas::matrix::init`], wired from `src/module_init.rs`).
/// The FalkorDB-index unit tests, however, construct [`VersionedMatrix`]es
/// directly without that path, so they must initialize GraphBLAS themselves —
/// otherwise the first `GrB_Matrix_new` aborts with `GrB_PANIC`.
///
/// This mirrors the engine's `init`: non-blocking mode with GraphBLAS's
/// built-in ANSI-C allocators (tests don't need the Redis allocators) and the
/// JIT disabled (the JIT's `dlopen` path is both slow and fork-unsafe; the
/// generic kernels are correct and sufficient here). The [`Once`] guard makes
/// it idempotent across the many tests that call it.
///
/// [`VersionedMatrix`]: crate::graph::graphblas::versioned_matrix::VersionedMatrix
#[cfg(test)]
pub(crate) fn test_init_graphblas() {
    use std::sync::Once;

    use crate::graph::graphblas::{
        GrB_GLOBAL, GrB_Global_set_INT32, GrB_Info, GrB_Mode, GrB_init, GxB_JIT_Control,
        GxB_Option_Field,
    };

    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        let info = GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
        assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_init failed: {info:?}");
        let info = GrB_Global_set_INT32(
            GrB_GLOBAL,
            GxB_JIT_Control::GxB_JIT_OFF as i32,
            GxB_Option_Field::GxB_JIT_C_CONTROL as _,
        );
        assert_eq!(
            info,
            GrB_Info::GrB_SUCCESS,
            "GraphBLAS JIT-off failed: {info:?}"
        );
    });
}
