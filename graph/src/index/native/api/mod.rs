//! The storage-agnostic **index contract**: the [`Index`] trait every kind
//! implements, plus the shared value types the runtime exchanges with it
//! (integration §3,
//! query-api).
//!
//! Submodules carve the contract into single-concept files:
//! - [`scan`] — the `'q`-scoped pull iterator ([`IndexScan`] / [`IndexScanIter`]).
//! - [`schema`] — what describes a structure ([`IndexSchema`] / [`EntityKind`]).
//! - [`result`] — a yielded hit and per-scan options ([`IndexHit`] / [`ScanOptions`]).
//! - [`encoder`] — the per-kind sortable row-key seam ([`RowEncoder`] / [`BoundSide`]).
//! - [`memory`] — accounting for `CALL db.indexes()` ([`MemoryBreakdown`]).
//!
//! One [`Index`] instance == one **structure** (index × field × entity-kind),
//! fixed at create time as node *or* edge. The read surface is the **single
//! enum-keyed** [`query`](Index::query) over [`crate::index::IndexQuery`]
//! (query-api §8) — not a
//! method per kind. A new kind adds an `IndexQuery` variant + a match arm,
//! never a new public method.

mod encoder;
mod memory;
mod result;
mod scan;
mod schema;

pub use encoder::{BoundSide, RowEncoder};
pub use memory::MemoryBreakdown;
pub use result::{IndexHit, ScanOptions};
pub use scan::{IndexScan, IndexScanIter};
pub use schema::{EntityKind, IndexSchema};

use std::sync::Arc;

use crate::index::{IndexInfo, IndexQuery};
use crate::runtime::value::Value;

use super::backend::StorageBackend;
use super::error::Result;
use super::id::DocKey;

/// A single index structure over the logical-MVCC matrix core.
pub trait Index: Send + Sync {
    // --- lifecycle ---

    /// Build a fresh index for `schema`, durably backed by `backend`
    /// (`NullBackend` in OSS). The injected `Arc<dyn StorageBackend>` is the
    /// OSS↔enterprise seam.
    fn create(
        schema: &IndexSchema,
        backend: Arc<dyn StorageBackend>,
    ) -> Self
    where
        Self: Sized;

    /// Declared kinds/fields/options for `CALL db.indexes()` and the RDB schema.
    fn info(&self) -> IndexInfo;

    // --- write (rides the graph commit) ---

    /// Apply one commit's mutations at logical `version` (== the graph commit
    /// version). `add` are `(doc, value)` postings; `remove` are docs whose
    /// rows are tombstoned. A list-valued `value` indexes as many rows.
    fn commit(
        &self,
        version: u64,
        add: &[(DocKey, Value)],
        remove: &[DocKey],
    ) -> Result<()>;

    // --- read: one entry, keyed by the query enum ---

    /// Evaluate `q` against this structure, returning a `'q`-scoped scan. The
    /// scan borrows the committed share (mechanism A); it does **not** deep-dup
    /// the matrix.
    fn query<'q>(
        &'q self,
        q: &IndexQuery<Value>,
        opts: ScanOptions,
    ) -> Result<IndexScanIter<'q>>;

    // --- accounting / population progress (CALL db.indexes()) ---

    /// Resident matrix + payload bytes.
    fn memory_usage(&self) -> MemoryBreakdown;

    /// Report population progress (`done` of `total`).
    fn update_progress(
        &self,
        done: u64,
        total: u64,
    );

    /// Request cooperative cancellation of an in-flight population.
    fn cancel(&self);

    /// Whether cancellation was requested.
    fn is_cancelled(&self) -> bool;

    // --- recovery: adopt durable state iff caught up, else signal rebuild ---

    /// Try to adopt persisted state for graph version `v_graph`
    /// (durability §8).
    /// Returns `Ok(true)` if adopted (register shards cold, fault in lazily,
    /// trim WAL tails to `<= v_graph`); `Ok(false)` if the index is behind and
    /// the caller must run `populate_indexes_sync`.
    fn try_adopt(
        &self,
        v_graph: u64,
    ) -> Result<bool>;
}
