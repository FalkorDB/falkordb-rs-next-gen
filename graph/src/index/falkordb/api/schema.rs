//! The index declaration: what entity an index targets and the fields it
//! covers. An [`super::Index`] is created from an [`IndexSchema`].

use std::sync::Arc;

/// Whether an index targets nodes or edges (fixed at create time).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    /// A node index — `DocKey` is the node id.
    Node,
    /// An edge index — `DocKey` is the edge id alone
    /// (mvcc-core §5).
    Edge,
}

/// The declaration an [`super::Index`] is created from. Minimal for M0; later
/// milestones extend it (analyzer, vector dim/metric, geo cell-resolution).
#[derive(Debug, Clone)]
pub struct IndexSchema {
    /// Stable id for this index.
    pub index_id: u64,
    /// Node vs. edge.
    pub entity: EntityKind,
    /// The label (node) or relationship type (edge) being indexed.
    pub label: Arc<String>,
    /// The indexed field names, in declaration order.
    pub fields: Vec<Arc<String>>,
}
