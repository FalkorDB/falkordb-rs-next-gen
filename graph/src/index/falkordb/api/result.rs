//! The read surface's value types: one [`IndexHit`] and the per-scan
//! [`ScanOptions`] (query-api §1, §3).

use std::sync::{Arc, atomic::AtomicBool};

use crate::index::falkordb::id::DocKey;

/// One read result (query-api §1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexHit {
    /// The matched document (node id, or edge id alone).
    pub id: DocKey,
    /// `Some` for ranked kinds (BM25, KNN); `None` for boolean predicates
    /// (numeric / exact / array — the POC).
    pub score: Option<f32>,
}

/// Per-scan tuning (query-api §3).
#[derive(Clone, Default)]
pub struct ScanOptions {
    /// Hard cap — guardrail for unbounded predicates (e.g. open-ended range).
    pub max_results: Option<usize>,
    /// Cooperative cancellation, polled at coarse granularity (per shard
    /// fan-out / per fault-in), not per hit.
    pub cancel: Option<Arc<AtomicBool>>,
    /// Candidate breadth for ranked kinds (HNSW `ef` is one instance); unused
    /// by the numeric POC.
    pub search_candidates: Option<usize>,
}
