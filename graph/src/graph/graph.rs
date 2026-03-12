//! Core graph data structure and operations.
//!
//! This module contains the main [`Graph`] struct which represents a property graph
//! using sparse matrices for efficient storage and graph operations.
//!
//! ## Graph Model
//!
//! The graph supports:
//! - **Nodes**: Identified by 64-bit IDs, can have multiple labels and properties
//! - **Relationships**: Directed edges with a type and properties
//! - **Properties**: Key-value pairs stored in attribute stores
//! - **Indexes**: Range and full-text indexes on node properties
//!
//! ## Storage Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      Graph Structure                        │
//! ├─────────────────────────────────────────────────────────────┤
//! │ adjacency_matrix     │ Sparse matrix: all relationships    │
//! │ labels_matrices[i]   │ Sparse vector: nodes with label i   │
//! │ relationship_matrices│ Tensor: edges by type (src,dst,id)  │
//! │ node_attrs           │ Properties for each node            │
//! │ relationship_attrs   │ Properties for each relationship    │
//! │ node_indexer         │ Secondary indexes on properties     │
//! │ cache                │ LRU cache for parsed query plans    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Query Plan Caching
//!
//! The graph caches parsed and planned queries in an LRU cache. On cache hit,
//! the plan is returned directly without reparsing. The cache key is the
//! raw query string.

use std::{
    collections::HashMap,
    hash::Hash,
    num::NonZeroUsize,
    sync::Arc,
    time::{Duration, Instant},
};

use atomic_refcell::AtomicRefCell;
use fjall::Database;
use itertools::Itertools;
use lru::LruCache;
use once_cell::sync::OnceCell;
use orx_tree::DynTree;
use parking_lot::Mutex;
use roaring::RoaringTreemap;

use crate::{
    entity_type::EntityType,
    graph::{
        attribute_store::AttributeStore,
        graphblas::{
            matrix::{
                Dup, MaskedElementWiseAdd, MaskedElementWiseMultiply, Matrix, MxM, New, Remove,
                Set, Size,
            },
            tensor::Tensor,
            versioned_matrix::VersionedMatrix,
        },
    },
    index::{
        Field,
        indexer::{Document, IndexInfo, IndexOptions, IndexQuery, IndexType, Indexer},
    },
    parser::{ast::ExprIR, cypher::Parser},
    planner::{IR, Planner, binder::Binder, optimizer::optimize},
    runtime::{ordermap::OrderMap, orderset::OrderSet, pending::PendingRelationship, value::Value},
    threadpool::spawn,
};

/// Result of query parsing and planning.
///
/// Contains the execution plan along with metadata about parsing performance.
pub struct Plan {
    /// The execution plan tree
    pub plan: Arc<DynTree<IR>>,
    /// Whether this plan was retrieved from cache
    pub cached: bool,
    /// Query parameters extracted from CYPHER prefix
    pub parameters: HashMap<String, DynTree<ExprIR<Arc<String>>>>,
    /// Time spent parsing the query
    pub parse_duration: Duration,
    /// Time spent planning/optimizing the query
    pub plan_duration: Duration,
}

/// Opaque identifier for a node label.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LabelId(pub usize);

/// Opaque identifier for a relationship type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeId(usize);

/// Opaque identifier for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u64);

/// Opaque identifier for a relationship (edge).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelationshipId(u64);

impl From<LabelId> for usize {
    fn from(val: LabelId) -> Self {
        val.0
    }
}

impl From<TypeId> for usize {
    fn from(val: TypeId) -> Self {
        val.0
    }
}

impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<NodeId> for u64 {
    fn from(value: NodeId) -> Self {
        value.0
    }
}

impl From<u64> for RelationshipId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<RelationshipId> for u64 {
    fn from(value: RelationshipId) -> Self {
        value.0
    }
}

impl Plan {
    #[must_use]
    pub const fn new(
        plan: Arc<DynTree<IR>>,
        cached: bool,
        parameters: HashMap<String, DynTree<ExprIR<Arc<String>>>>,
        parse_duration: Duration,
        plan_duration: Duration,
    ) -> Self {
        Self {
            plan,
            cached,
            parameters,
            parse_duration,
            plan_duration,
        }
    }
}

/// The main graph data structure.
///
/// Stores nodes, relationships, labels, and properties using sparse matrices
/// for efficient graph operations. Supports:
/// - Node and relationship creation/deletion
/// - Label and property assignment
/// - Index-based lookups
/// - Query plan caching
///
/// # Thread Safety
///
/// The Graph is `Send + Sync` but not internally synchronized. Use [`MvccGraph`]
/// for concurrent access with proper read/write isolation.
pub struct Graph {
    /// Maximum node capacity (for matrix sizing)
    node_cap: u64,
    /// Maximum relationship capacity (for matrix sizing)
    relationship_cap: u64,
    /// Number of node IDs reserved (including deleted)
    reserved_node_count: u64,
    /// Number of relationship IDs reserved (including deleted)
    reserved_relationship_count: u64,
    /// Current count of active nodes
    node_count: u64,
    /// Current count of active relationships
    relationship_count: u64,
    /// Bitmap of deleted node IDs (for ID reuse)
    deleted_nodes: RoaringTreemap,
    /// Bitmap of deleted relationship IDs
    deleted_relationships: RoaringTreemap,
    /// Empty matrix for operations
    zero_matrix: VersionedMatrix,
    /// Combined adjacency matrix (all relationship types)
    adjacancy_matrix: VersionedMatrix,
    /// Matrix mapping nodes to their labels
    node_labels_matrix: VersionedMatrix,
    /// Matrix mapping relationships to their types
    relationship_type_matrix: VersionedMatrix,
    /// Matrix with all nodes (for full scans)
    all_nodes_matrix: VersionedMatrix,
    /// Per-label matrices (label ID → node membership)
    labels_matices: Vec<VersionedMatrix>,
    /// Per-type relationship tensors (type ID → src×dst×edge_id)
    relationship_matrices: Vec<Tensor>,
    /// Node property storage
    node_attrs: AttributeStore,
    /// Relationship property storage
    relationship_attrs: AttributeStore,
    /// Index manager for property indexes
    node_indexer: Indexer,
    /// Label names (ID → name mapping)
    node_labels: Vec<Arc<String>>,
    /// Relationship type names (ID → name mapping)
    relationship_types: Vec<Arc<String>>,
    /// LRU cache for query plans
    cache: Arc<Mutex<LruCache<String, PlanTree>>>,
    /// Version counter (incremented on each write transaction)
    pub version: u64,
}

/// Wrapper for plan trees to implement Send+Sync.
struct PlanTree(DynTree<IR>);

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for PlanTree {}
unsafe impl Sync for PlanTree {}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}

/// Populates an index in the background for existing nodes.
///
/// Each batch borrows the latest committed graph (via the Indexer's shared
/// graph reference) to get a fresh label matrix and attribute store,
/// ensuring nodes added by write transactions between batches are visible.
///
/// The Indexer's serialization lock serializes each batch with write-path
/// `commit_index` calls, so they never run concurrently.  Within a
/// batch the lock is held, preventing writes from committing index
/// changes.  Between batches the lock is released, allowing writes
/// to proceed.
fn populate_index(
    label: Arc<String>,
    node_indexer: Indexer,
) {
    let attrs = node_indexer.get_fields(&label);
    populate_index_batch(label, node_indexer, attrs, 0, 0);
}

/// Processes one batch of index population and spawns the next batch.
fn populate_index_batch(
    label: Arc<String>,
    mut node_indexer: Indexer,
    attrs: HashMap<Arc<String>, Vec<Arc<Field>>>,
    mut progress: u64,
    min_row: u64,
) {
    spawn(
        move || {
            const BATCH_SIZE: usize = 10_000;

            if node_indexer.is_cancelled() || node_indexer.pending_changes(&label) > 1 {
                node_indexer.enable(&label);
                return;
            }

            let exhausted;
            let mut next_min_row = min_row;

            // Hold the Indexer's serialization lock for the entire batch so
            // that write-path `commit_index` calls wait until this batch
            // finishes.  This guarantees no concurrent index mutations.
            {
                let lock = node_indexer.write_lock();
                let guard = lock.lock();

                let mut batch = Vec::with_capacity(BATCH_SIZE);

                if let Some(graph) = node_indexer.get_graph()
                    && let Some(lm) = graph.borrow().get_label_matrix(&label)
                {
                    for (n, _) in lm.iter(min_row, u64::MAX).take(BATCH_SIZE) {
                        let mut doc = Document::new(n);
                        let mut has_fields = false;
                        for (attr, fields) in &attrs {
                            let value = graph.borrow().get_node_attribute(NodeId(n), attr);
                            if let Some(value) = value {
                                for field in fields {
                                    doc.set(field, &value);
                                }
                                has_fields = true;
                            }
                        }
                        if has_fields {
                            batch.push(doc);
                        }
                    }
                    next_min_row = batch.last().map_or(next_min_row, |doc| doc.id() + 1);
                } else {
                    // Graph not yet committed — reschedule this batch.
                    // MvccGraph::commit() will set the indexer's graph
                    // reference, so the next attempt will find it.
                    drop(guard);
                    drop(lock);
                    std::thread::sleep(Duration::from_millis(1));
                    populate_index_batch(label, node_indexer, attrs, progress, min_row);
                    return;
                }

                exhausted = batch.len() < BATCH_SIZE;

                if !batch.is_empty() {
                    progress += batch.len() as u64;
                    let mut add_docs = HashMap::new();
                    add_docs.insert(label.clone(), batch);
                    node_indexer.commit(&mut add_docs, &mut HashMap::new());
                    node_indexer.update_progress(&label, progress);
                }
                // guard dropped here — lock released between batches
            }

            if exhausted {
                node_indexer.enable(&label);
            } else {
                populate_index_batch(label, node_indexer, attrs, progress, next_min_row);
            }
        },
        Some(0),
    );
}

fn drop_index_bg(
    label: Arc<String>,
    mut node_indexer: Indexer,
) {
    spawn(
        move || {
            node_indexer.remove(&label);
        },
        Some(0),
    );
}

static DATABASE: OnceCell<Database> = OnceCell::new();

impl Graph {
    #[must_use]
    pub fn new(
        n: u64,
        e: u64,
        cache_size: usize,
        version: u64,
        name: &str,
    ) -> Self {
        let db = DATABASE.get_or_init(|| {
            Database::builder(format!("./attrs/{}", std::process::id()))
                .temporary(true)
                .manual_journal_persist(true)
                .open()
                .unwrap()
        });
        Self {
            node_cap: n,
            relationship_cap: e,
            reserved_node_count: 0,
            reserved_relationship_count: 0,
            node_count: 0,
            relationship_count: 0,
            deleted_nodes: RoaringTreemap::new(),
            deleted_relationships: RoaringTreemap::new(),
            zero_matrix: VersionedMatrix::new(0, 0),
            adjacancy_matrix: VersionedMatrix::new(n, n),
            node_labels_matrix: VersionedMatrix::new(0, 0),
            relationship_type_matrix: VersionedMatrix::new(0, 0),
            all_nodes_matrix: VersionedMatrix::new(n, n),
            labels_matices: Vec::new(),
            relationship_matrices: Vec::new(),
            node_attrs: AttributeStore::new(db.clone(), &format!("{name}/nodes")),
            relationship_attrs: AttributeStore::new(db.clone(), &format!("{name}/relationships")),
            node_indexer: Indexer::default(),
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(cache_size).unwrap(),
            ))),
            version,
        }
    }

    #[must_use]
    pub fn new_version(&self) -> Self {
        debug_assert_eq!(self.reserved_node_count, 0);
        debug_assert_eq!(self.reserved_relationship_count, 0);
        Self {
            node_cap: self.node_cap,
            relationship_cap: self.relationship_cap,
            reserved_node_count: 0,
            reserved_relationship_count: 0,
            node_count: self.node_count,
            relationship_count: self.relationship_count,
            deleted_nodes: self.deleted_nodes.clone(),
            deleted_relationships: self.deleted_relationships.clone(),
            zero_matrix: self.zero_matrix.dup(),
            adjacancy_matrix: self.adjacancy_matrix.dup(),
            node_labels_matrix: self.node_labels_matrix.dup(),
            relationship_type_matrix: self.relationship_type_matrix.dup(),
            all_nodes_matrix: self.all_nodes_matrix.dup(),
            labels_matices: self
                .labels_matices
                .iter()
                .map(VersionedMatrix::dup)
                .collect(),
            relationship_matrices: self.relationship_matrices.iter().map(Tensor::dup).collect(),
            node_attrs: self.node_attrs.new_version(),
            relationship_attrs: self.relationship_attrs.new_version(),
            node_indexer: self.node_indexer.clone(),
            node_labels: self.node_labels.clone(),
            relationship_types: self.relationship_types.clone(),
            cache: self.cache.clone(),
            version: self.version + 1,
        }
    }

    #[must_use]
    pub const fn node_count(&self) -> u64 {
        self.node_count
    }

    #[must_use]
    pub const fn node_cap(&self) -> u64 {
        self.node_cap
    }

    #[must_use]
    pub const fn labels_count(&self) -> usize {
        self.node_labels.len()
    }

    #[must_use]
    pub fn get_labels(&self) -> &[Arc<String>] {
        &self.node_labels
    }

    #[must_use]
    pub fn get_label_by_id(
        &self,
        id: LabelId,
    ) -> Arc<String> {
        self.node_labels[id.0].clone()
    }

    #[must_use]
    pub fn get_types(&self) -> &[Arc<String>] {
        &self.relationship_types
    }

    #[must_use]
    pub fn get_type(
        &self,
        id: TypeId,
    ) -> Option<Arc<String>> {
        self.relationship_types.get(id.0).cloned()
    }

    pub fn get_attrs(&self) -> impl Iterator<Item = &Arc<String>> + '_ {
        self.node_attrs
            .attrs_name
            .iter()
            .chain(self.relationship_attrs.attrs_name.iter())
    }

    pub fn get_label_id_mut(
        &mut self,
        label: &str,
    ) -> LabelId {
        if let Some(pos) = self
            .node_labels
            .iter()
            .position(|l| l.as_str() == label)
            .map(LabelId)
        {
            return pos;
        }

        self.node_labels.push(Arc::new(label.to_string()));
        self.labels_matices
            .push(VersionedMatrix::new(self.node_cap, self.node_cap));
        LabelId(self.node_labels.len() - 1)
    }

    pub fn get_label_id(
        &self,
        label: &str,
    ) -> Option<LabelId> {
        self.node_labels
            .iter()
            .position(|l| l.as_str() == label)
            .map(LabelId)
    }

    pub fn get_type_id(
        &self,
        relationship_type: &str,
    ) -> Option<TypeId> {
        self.relationship_types
            .iter()
            .position(|t| t.as_str() == relationship_type)
            .map(TypeId)
    }

    pub fn get_plan(
        &self,
        query: &str,
    ) -> Result<Plan, String> {
        let mut parse_duration = Duration::ZERO;
        let mut plan_duration = Duration::ZERO;

        let mut parser = Parser::new(query);
        let (parameters, query) = parser.parse_parameters()?;

        {
            let mut cache = self.cache.lock();
            if let Some(plan) = cache.get(query) {
                let optimize_plan = optimize(&plan.0, self);
                return Ok(Plan::new(
                    Arc::new(optimize_plan),
                    true,
                    parameters,
                    parse_duration,
                    plan_duration,
                ));
            }
        }

        let start = Instant::now();
        let raw_ir = parser.parse()?;
        let (ir, scope_vars) = Binder::default().bind(raw_ir)?;
        ir.validate()?;
        parse_duration = start.elapsed();

        let mut planner = Planner::new(scope_vars);
        let start = Instant::now();
        let plan = planner.plan(ir);
        let optimize_plan = optimize(&plan, self);
        plan_duration = start.elapsed();

        self.cache.lock().push(query.to_string(), PlanTree(plan));
        Ok(Plan::new(
            Arc::new(optimize_plan),
            false,
            parameters,
            parse_duration,
            plan_duration,
        ))
    }

    fn get_label_matrix(
        &self,
        label: &str,
    ) -> Option<&VersionedMatrix> {
        self.node_labels
            .iter()
            .position(|l| l.as_str() == label)
            .map(|i| &self.labels_matices[i])
    }

    fn get_label_matrix_mut(
        &mut self,
        label: &Arc<String>,
    ) -> &mut VersionedMatrix {
        if !self.node_labels.contains(label) {
            self.node_labels.push(label.clone());

            let m = VersionedMatrix::new(self.node_cap, self.node_cap);
            self.labels_matices.insert(self.node_labels.len() - 1, m);
        }

        self.node_labels
            .iter()
            .position(|l| l.as_str() == label.as_str())
            .map(|i| &mut self.labels_matices[i])
            .unwrap()
    }

    fn get_relationship_matrix_mut(
        &mut self,
        relationship_type: &Arc<String>,
    ) -> &mut Tensor {
        if !self.relationship_types.contains(relationship_type) {
            self.relationship_types.push(relationship_type.clone());

            self.relationship_matrices.insert(
                self.relationship_types.len() - 1,
                Tensor::new(self.node_cap, self.node_cap),
            );
        }

        self.relationship_types
            .iter()
            .position(|l| l.as_str() == relationship_type.as_str())
            .map(|i| &mut self.relationship_matrices[i])
            .unwrap()
    }

    fn get_relationship_matrix(
        &self,
        relationship_type: &Arc<String>,
    ) -> Option<&Tensor> {
        if !self.relationship_types.contains(relationship_type) {
            return None;
        }

        self.relationship_types
            .iter()
            .position(|l| l.as_str() == relationship_type.as_str())
            .map(|i| &self.relationship_matrices[i])
    }

    #[must_use]
    pub fn get_node_attribute_id(
        &self,
        attr: &Arc<String>,
    ) -> Option<usize> {
        self.node_attrs.get_attr_id(attr)
    }

    #[must_use]
    pub fn get_relationship_attribute_id(
        &self,
        attr: &Arc<String>,
    ) -> Option<usize> {
        self.relationship_attrs.get_attr_id(attr)
    }

    pub fn reserve_node(&mut self) -> NodeId {
        if self.reserved_node_count < self.deleted_nodes.len() {
            let id = self.deleted_nodes.select(self.reserved_node_count).unwrap();
            self.reserved_node_count += 1;
            return NodeId(id);
        }
        self.reserved_node_count += 1;
        NodeId(self.node_count + self.reserved_node_count - 1)
    }

    pub fn create_nodes(
        &mut self,
        nodes: &RoaringTreemap,
    ) {
        self.node_count += nodes.len();
        self.reserved_node_count -= nodes.len();
        self.deleted_nodes -= nodes;

        self.resize();

        for id in nodes {
            self.all_nodes_matrix.set(id, id, true);
        }
    }

    #[must_use]
    pub fn max_node_id(&self) -> u64 {
        if self.node_count == 0 {
            return 0;
        }
        self.node_count + self.deleted_nodes.len() - 1
    }

    pub fn set_nodes_attributes(
        &mut self,
        attrs: &HashMap<u64, OrderMap<Arc<String>, Value>>,
        index_add_docs: &mut HashMap<u64, RoaringTreemap>,
    ) -> Result<usize, String> {
        let nremoved = self.node_attrs.insert_attrs(attrs)?;

        if self.node_indexer.has_indices() {
            for (id, attrs) in attrs {
                for (_, label_id) in self.node_labels_matrix.iter(*id, *id) {
                    let label = &self.node_labels[label_id as usize];
                    for key in attrs.keys() {
                        if self.node_indexer.has_indexed_attr(label, key) {
                            index_add_docs.entry(label_id).or_default().insert(*id);
                        }
                    }
                }
            }
        }
        Ok(nremoved)
    }

    pub fn set_nodes_labels(
        &mut self,
        nodes_labels: &mut Matrix,
        index_add_docs: &mut HashMap<u64, RoaringTreemap>,
    ) {
        self.resize();

        for (id, label_id) in nodes_labels.iter(0, u64::MAX) {
            self.node_labels_matrix.set(id, label_id, true);
            self.labels_matices[label_id as usize].set(id, id, true);
            let label = &self.node_labels[label_id as usize];
            if self.node_indexer.has_index(label) && self.node_attrs.has_attributes(id) {
                index_add_docs.entry(label_id).or_default().insert(id);
            }
        }
    }

    pub fn remove_nodes_labels(
        &mut self,
        nodes_labels: &mut Matrix,
        remove_docs: &mut HashMap<u64, RoaringTreemap>,
    ) {
        self.resize();

        for (id, label_id) in nodes_labels.iter(0, u64::MAX) {
            self.node_labels_matrix.remove(id, label_id);
            self.labels_matices[label_id as usize].remove(id, id);
            let label = &self.node_labels[label_id as usize];
            if self.node_indexer.has_index(label) {
                remove_docs.entry(label_id).or_default().insert(id);
            }
        }
    }

    pub fn delete_nodes(
        &mut self,
        deleted_nodes: &RoaringTreemap,
        remove_docs: &mut HashMap<u64, RoaringTreemap>,
    ) -> Result<(), String> {
        self.deleted_nodes |= deleted_nodes;
        self.node_count -= deleted_nodes.len();

        for id in deleted_nodes {
            self.all_nodes_matrix.remove(id, id);

            for (_, label_id) in self.node_labels_matrix.iter(id, id) {
                let label = &self.node_labels[label_id as usize];
                self.labels_matices[label_id as usize].remove(id, id);
                if self.node_indexer.has_index(label) {
                    for attr in self.node_attrs.get_attrs(id) {
                        if self.node_indexer.has_indexed_attr(label, &attr) {
                            remove_docs.entry(label_id).or_default().insert(id);
                            break;
                        }
                    }
                }
            }

            for label_id in 0..self.labels_matices.len() {
                self.node_labels_matrix.remove(id, label_id as _);
            }
        }
        self.node_attrs.remove_all(deleted_nodes)?;
        Ok(())
    }

    pub fn get_node_relationships(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (NodeId, NodeId, RelationshipId)> + '_ {
        self.relationship_matrices
            .iter()
            .flat_map(move |m| m.iter(id.0, id.0, false).chain(m.iter(id.0, id.0, true)))
            .map(|(src, dest, id)| {
                let src_node = NodeId(src);
                let dest_node = NodeId(dest);
                (src_node, dest_node, RelationshipId(id))
            })
    }

    #[must_use]
    pub fn get_nodes(
        &self,
        labels: &OrderSet<Arc<String>>,
        min_row: u64,
    ) -> Box<dyn Iterator<Item = NodeId>> {
        if labels.is_empty() {
            return Box::new(
                self.all_nodes_matrix
                    .iter(min_row, u64::MAX)
                    .map(|(id, _)| NodeId(id)),
            );
        }
        if labels.len() == 1 {
            if let Some(label_matrix) = self.get_label_matrix(&labels[0]) {
                return Box::new(
                    label_matrix
                        .iter(min_row, u64::MAX)
                        .map(|(id, _)| NodeId(id)),
                );
            }
            return Box::new(std::iter::empty());
        }
        let matrices = labels
            .iter()
            .map(|label| self.get_label_matrix(label))
            .collect::<Option<Vec<_>>>();
        Box::new(
            matrices
                .map_or_else(
                    || self.zero_matrix.to_matrix().iter(min_row, u64::MAX),
                    |mut matrices| {
                        let mut iter = matrices.iter_mut();
                        let mut m = iter.next().unwrap().to_matrix();
                        for label_matrix in iter {
                            m.element_wise_multiply(
                                None,
                                None,
                                Some(&label_matrix.to_matrix()),
                                None,
                            );
                        }
                        m.iter(min_row, u64::MAX)
                    },
                )
                .map(|(id, _)| NodeId(id)),
        )
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn get_node_label_ids(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = LabelId> {
        self.node_labels_matrix
            .iter(id.0, id.0)
            .map(|(_, l)| LabelId(l as usize))
    }

    pub fn get_node_labels(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = Arc<String>> {
        self.get_node_label_ids(id)
            .map(move |label_id| self.node_labels[label_id.0].clone())
    }

    #[must_use]
    pub fn get_node_attribute(
        &self,
        id: NodeId,
        attr: &Arc<String>,
    ) -> Option<Value> {
        self.node_attrs.get_attr(id.0, attr)
    }

    pub fn reserve_relationship(&mut self) -> RelationshipId {
        if self.reserved_relationship_count < self.deleted_relationships.len() {
            let id = self
                .deleted_relationships
                .select(self.reserved_relationship_count)
                .unwrap();
            self.reserved_relationship_count += 1;
            return RelationshipId(id);
        }
        self.reserved_relationship_count += 1;
        RelationshipId(self.relationship_count + self.reserved_relationship_count - 1)
    }

    pub fn create_relationships(
        &mut self,
        relationships: &HashMap<RelationshipId, PendingRelationship>,
    ) {
        self.relationship_count += relationships.len() as u64;
        self.reserved_relationship_count -= relationships.len() as u64;

        for id in relationships.keys() {
            if self.deleted_relationships.is_empty() {
                break;
            }
            self.deleted_relationships.remove(id.0);
        }

        for (
            id,
            PendingRelationship {
                type_name,
                from: start,
                to: end,
                ..
            },
        ) in relationships
        {
            self.get_relationship_matrix_mut(type_name)
                .set(start.0, end.0, id.0);
        }

        self.resize();

        for (
            id,
            PendingRelationship {
                type_name,
                from: start,
                to: end,
            },
        ) in relationships
        {
            self.adjacancy_matrix.set(start.0, end.0, true);
            self.relationship_type_matrix.set(
                id.0,
                self.relationship_types
                    .iter()
                    .position(|p| p.as_str() == type_name.as_str())
                    .unwrap() as u64,
                true,
            );
        }
    }

    pub fn set_relationships_attributes(
        &mut self,
        attrs: &HashMap<u64, OrderMap<Arc<String>, Value>>,
    ) -> Result<usize, String> {
        let nremoved = self.relationship_attrs.insert_attrs(attrs)?;
        Ok(nremoved)
    }

    #[must_use]
    pub fn is_node_deleted(
        &self,
        id: NodeId,
    ) -> bool {
        self.deleted_nodes.contains(id.0)
    }

    #[must_use]
    pub fn is_relationship_deleted(
        &self,
        id: RelationshipId,
    ) -> bool {
        self.deleted_relationships.contains(id.0)
    }

    pub fn delete_relationships(
        &mut self,
        rels: OrderSet<(RelationshipId, NodeId, NodeId)>,
    ) -> Result<(), String> {
        self.deleted_relationships
            .extend(rels.iter().map(|(id, _, _)| id.0));
        self.relationship_count -= rels.len() as u64;
        let mut r = vec![];
        for (type_id, rels) in &rels
            .into_iter()
            .chunk_by(|(id, _, _)| self.get_relationship_type_id(*id))
        {
            r.push((
                type_id,
                rels.map(|(id, src, dest)| (id.0, src.0, dest.0))
                    .collect::<Vec<_>>(),
            ));
        }

        for (type_id, rels) in r {
            for (id, _, _) in &rels {
                self.relationship_type_matrix.remove(*id, type_id.0 as u64);
                self.relationship_attrs.remove(*id)?;
            }
            let label = self.relationship_types[type_id.0].clone();
            self.get_relationship_matrix_mut(&label).remove_all(rels);
        }
        Ok(())
    }

    pub fn get_src_dest_relationships(
        &self,
        src: NodeId,
        dest: NodeId,
        types: &[Arc<String>],
    ) -> impl Iterator<Item = RelationshipId> + use<> {
        let iters: Vec<_> = if types.is_empty() {
            &self.relationship_types
        } else {
            types
        }
        .iter()
        .filter_map(|relationship_type| self.get_relationship_matrix(relationship_type))
        .map(|relationship_matrix| relationship_matrix.get(src.0, dest.0))
        .collect();

        iters
            .into_iter()
            .flat_map(|iter| iter.map(|(_, id)| RelationshipId(id)))
    }

    pub fn get_relationships(
        &self,
        types: &[Arc<String>],
        src_lables: &OrderSet<Arc<String>>,
        dest_labels: &OrderSet<Arc<String>>,
    ) -> impl Iterator<Item = (NodeId, NodeId)> + use<> {
        let matrices = types
            .iter()
            .filter_map(|relationship_type| self.get_relationship_matrix(relationship_type))
            .collect::<Vec<_>>();
        let src_labels_matrices = src_lables
            .iter()
            .map(|label| self.get_label_matrix(label))
            .collect::<Option<Vec<_>>>();
        let dest_labels_matrices = dest_labels
            .iter()
            .map(|label| self.get_label_matrix(label))
            .collect::<Option<Vec<_>>>();
        // If labels/types were requested but none exist in the graph,
        // no results can match.
        let no_match = (!types.is_empty() && matrices.is_empty())
            || src_labels_matrices.is_none()
            || dest_labels_matrices.is_none();

        let src_labels_matrices = src_labels_matrices.unwrap_or_default();
        let dest_labels_matrices = dest_labels_matrices.unwrap_or_default();

        let m = if no_match {
            // If labels/types were requested but none exist in the graph,
            // no results can match - clear the matrix to return empty.
            self.zero_matrix.to_matrix()
        } else {
            let mut iter = matrices.into_iter();
            let mut m = iter.next().map_or_else(
                || self.adjacancy_matrix.to_matrix(),
                |t| t.matrix().to_matrix(),
            );
            for relationship_matrix in iter {
                m.element_wise_add(
                    None,
                    None,
                    Some(&relationship_matrix.matrix().to_matrix()),
                    None,
                );
            }

            if !src_labels_matrices.is_empty() {
                let mut iter = src_labels_matrices.iter();
                let mut src_matrix = iter.next().unwrap().to_matrix();
                for label_matrix in iter {
                    src_matrix.element_wise_multiply(
                        None,
                        None,
                        Some(&label_matrix.to_matrix()),
                        None,
                    );
                }
                m.rmxm(&src_matrix);
            }
            if !dest_labels_matrices.is_empty() {
                let mut iter = dest_labels_matrices.iter();
                let mut dest_matrix = iter.next().unwrap().to_matrix();
                for label_matrix in iter {
                    dest_matrix.element_wise_multiply(
                        None,
                        None,
                        Some(&label_matrix.to_matrix()),
                        None,
                    );
                }
                m.lmxm(&dest_matrix);
            }
            m
        };
        m.iter(0, u64::MAX)
            .map(|(src, dest)| (NodeId(src), NodeId(dest)))
    }

    #[must_use]
    pub fn get_relationship_type_id(
        &self,
        id: RelationshipId,
    ) -> TypeId {
        #[allow(clippy::cast_possible_truncation)]
        self.relationship_type_matrix
            .iter(id.0, id.0)
            .map(|(_, l)| TypeId(l as usize))
            .next()
            .unwrap()
    }

    #[must_use]
    pub fn get_relationship_attribute(
        &self,
        id: RelationshipId,
        attr: &Arc<String>,
    ) -> Option<Value> {
        self.relationship_attrs.get_attr(id.0, attr)
    }

    fn resize(&mut self) {
        if self.node_count > self.node_cap {
            while self.node_count > self.node_cap {
                self.node_cap *= 2;
            }
            self.adjacancy_matrix.resize(self.node_cap, self.node_cap);
            self.node_labels_matrix
                .resize(self.node_cap, self.labels_matices.len() as u64);
            self.all_nodes_matrix.resize(self.node_cap, self.node_cap);
            for label_matrix in &mut self.labels_matices {
                label_matrix.resize(self.node_cap, self.node_cap);
            }
            for relationship_matrix in &mut self.relationship_matrices {
                relationship_matrix.resize(self.node_cap, self.node_cap);
            }
        }

        if self.labels_matices.len() as u64 > self.node_labels_matrix.ncols() {
            self.node_labels_matrix
                .resize(self.node_cap, self.labels_matices.len() as u64);
        }

        if self.relationship_count > self.relationship_cap {
            while self.relationship_count > self.relationship_cap {
                self.relationship_cap *= 2;
            }
            self.relationship_type_matrix
                .resize(self.relationship_cap, self.relationship_types.len() as u64);
        }

        if self.relationship_types.len() as u64 > self.relationship_type_matrix.ncols() {
            self.relationship_type_matrix
                .resize(self.relationship_cap, self.relationship_types.len() as u64);
        }
    }

    pub fn get_node_attrs(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = Arc<String>> + '_ {
        self.node_attrs.get_attrs(id.0)
    }

    /// Get all attribute names and values for a node in a single storage pass.
    pub fn get_node_all_attrs(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (Arc<String>, Value)> + '_ {
        self.node_attrs.get_all_attrs(id.0)
    }

    pub fn get_node_all_attrs_by_id(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (u16, Value)> + '_ {
        self.node_attrs.get_all_attrs_by_id(id.0)
    }

    pub fn get_relationship_attrs(
        &self,
        id: RelationshipId,
    ) -> impl Iterator<Item = Arc<String>> + '_ {
        self.relationship_attrs.get_attrs(id.0)
    }

    /// Get all attribute names and values for a relationship in a single storage pass.
    pub fn get_relationship_all_attrs(
        &self,
        id: RelationshipId,
    ) -> impl Iterator<Item = (Arc<String>, Value)> + '_ {
        self.relationship_attrs.get_all_attrs(id.0)
    }

    pub fn get_relationship_all_attrs_by_id(
        &self,
        id: RelationshipId,
    ) -> impl Iterator<Item = (u16, Value)> + '_ {
        self.relationship_attrs.get_all_attrs_by_id(id.0)
    }

    pub fn create_index(
        &mut self,
        index_type: &IndexType,
        entity_type: &EntityType,
        label: &Arc<String>,
        attrs: &Vec<Arc<String>>,
        options: Option<IndexOptions>,
    ) -> Result<(), String> {
        match entity_type {
            EntityType::Node => {
                let len = self.get_label_matrix_mut(label).nvals();
                self.node_indexer
                    .create_index(index_type, label, attrs, len, options)?;
                self.start_populate_index(label);
            }
            EntityType::Relationship => {}
        }
        Ok(())
    }

    fn start_populate_index(
        &self,
        label: &Arc<String>,
    ) {
        populate_index(label.clone(), self.node_indexer.clone());
    }

    pub fn commit_attrs(&mut self) {
        self.node_attrs.commit();
        self.relationship_attrs.commit();
    }

    /// Apply all deferred pool operations from attribute stores.
    /// Called when the graph version is promoted via MvccGraph::commit.
    pub fn apply_pool_ops(&mut self) {
        self.node_attrs.apply_pool_ops();
        self.relationship_attrs.apply_pool_ops();
    }

    /// Release all interned string refs held by this graph's attribute stores.
    /// Called when the graph is being destroyed.
    pub fn release_all_interned_refs(&self) {
        self.node_attrs.release_all_interned_refs();
        self.relationship_attrs.release_all_interned_refs();
    }

    pub fn commit_index(
        &mut self,
        index_add_docs: &mut HashMap<u64, RoaringTreemap>,
        remove_docs: &mut HashMap<u64, RoaringTreemap>,
    ) {
        let lock = self.node_indexer.write_lock();
        let _guard = lock.lock();

        let mut add_docs = HashMap::new();
        for (label_id, ids) in index_add_docs.drain() {
            let label = &self.node_labels[label_id as usize];
            let fields = self.node_indexer.get_fields(label);
            let mut docs = vec![];
            for id in ids {
                let mut doc = Document::new(id);
                for (key, fields) in &fields {
                    if let Some(value) = self.node_attrs.get_attr(id, key) {
                        for field in fields {
                            doc.set(field, &value);
                        }
                    }
                }
                docs.push(doc);
            }
            add_docs.insert(label.clone(), docs);
        }

        let mut remove = HashMap::new();
        for (label_id, ids) in remove_docs.drain() {
            let label = &self.node_labels[label_id as usize];
            remove.insert(label.clone(), ids);
        }

        self.node_indexer.commit(&mut add_docs, &mut remove);
    }

    pub fn drop_index(
        &mut self,
        index_type: &IndexType,
        entity_type: &EntityType,
        label: &Arc<String>,
        attrs: &Vec<Arc<String>>,
    ) -> Result<usize, String> {
        match entity_type {
            EntityType::Node => {
                let total = self.get_label_matrix(label).unwrap().nvals();
                let reindex = self
                    .node_indexer
                    .drop_index(label, attrs, index_type, total);

                if let Some((dropped, remaining)) = reindex {
                    if dropped > 0 {
                        if remaining > 0 {
                            self.node_indexer.recreate_index(label)?;
                            self.start_populate_index(label);
                        } else {
                            drop_index_bg(label.clone(), self.node_indexer.clone());
                        }
                    }
                    return Ok(dropped);
                }
            }
            EntityType::Relationship => {}
        }
        Ok(0)
    }

    #[must_use]
    pub fn is_indexed(
        &self,
        label: &Arc<String>,
        field: &Arc<String>,
    ) -> bool {
        self.node_indexer.is_attr_indexed(label, field)
    }

    pub fn get_indexed_nodes(
        &self,
        label: &Arc<String>,
        query: IndexQuery<Value>,
    ) -> impl Iterator<Item = NodeId> + use<> {
        self.node_indexer.query(label, query).map(NodeId)
    }

    pub fn fulltext_query_nodes(
        &self,
        label: &Arc<String>,
        query: &str,
    ) -> Result<impl Iterator<Item = (NodeId, f64)> + use<>, String> {
        self.node_indexer
            .fulltext_query(label, query)
            .map(|r| r.map(|(id, score)| (NodeId(id), score)))
    }

    #[must_use]
    pub fn index_info(&self) -> Vec<IndexInfo> {
        self.node_indexer.index_info()
    }

    pub fn cancel_indexing(&self) {
        self.node_indexer.cancel();
    }

    pub fn set_indexer_graph(
        &mut self,
        graph: Arc<AtomicRefCell<Self>>,
    ) {
        self.node_indexer.set_graph(graph);
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        let mut size = 0usize;
        size += self.adjacancy_matrix.memory_usage();
        size += self.node_labels_matrix.memory_usage();
        size += self.relationship_type_matrix.memory_usage();
        size += self.all_nodes_matrix.memory_usage();
        for label_matrix in &self.labels_matices {
            size += label_matrix.memory_usage();
        }
        for relationship_matrix in &self.relationship_matrices {
            size += relationship_matrix.memory_usage();
        }
        size += self.node_attrs.memory_usage();
        // size += self.relationship_attrs.memory_usage();
        // size += self.node_indexer.memory_usage();
        size
    }
}
