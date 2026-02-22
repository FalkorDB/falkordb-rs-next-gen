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
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use fjall::Database;
use itertools::Itertools;
use lru::LruCache;
use once_cell::sync::OnceCell;
use orx_tree::DynTree;
use roaring::RoaringTreemap;

use crate::{
    ast::ExprIR,
    binder::Binder,
    cypher::Parser,
    graph::{
        attribute_store::AttributeStore,
        matrix::{
            Dup, MaskedElementWiseAdd, MaskedElementWiseMultiply, Matrix, MxM, New, Remove, Set,
            Size,
        },
        tensor::Tensor,
        versioned_matrix::{self, VersionedMatrix},
    },
    indexer::{Document, EntityType, IndexInfo, IndexQuery, IndexType, Indexer},
    optimizer::optimize,
    planner::{IR, Planner},
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
fn populate_index(
    label: Arc<String>,
    lm: versioned_matrix::Iter,
    mut node_indexer: Indexer,
    node_attrs: AttributeStore,
) {
    spawn(
        move || {
            if node_indexer.pending_changes(label.clone()) > 1 {
                node_indexer.enable(label);
                return;
            }
            let attrs = { node_indexer.get_fields(label.clone()) };
            let mut add_docs = vec![];
            for (n, _) in lm {
                let mut doc = Document::new(n);
                for (attr, fields) in &attrs {
                    if let Some(value) = node_attrs.get_attr(n, attr) {
                        for field in fields {
                            doc.set(field.clone(), value.clone());
                        }
                    }
                }
                add_docs.push(doc);
            }
            if node_indexer.pending_changes(label.clone()) > 1 {
                node_indexer.enable(label);
                return;
            }
            let mut index_add_docs = HashMap::new();
            index_add_docs.insert(label.clone(), add_docs);

            node_indexer.commit(&mut index_add_docs, &mut HashMap::new());
            node_indexer.enable(label);
        },
        Some(0),
    );
}

fn drop_index(
    label: Arc<String>,
    mut node_indexer: Indexer,
) {
    spawn(
        move || {
            node_indexer.remove(label);
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

    /// Creates a deep copy of this graph with a new name and independent resources.
    ///
    /// Unlike `new_version()` which creates a CoW version sharing the same keyspace,
    /// `copy()` produces a fully independent graph with its own attribute stores,
    /// cache, and version counter starting at 0.
    #[must_use]
    pub fn copy(
        &self,
        cache_size: usize,
        name: &str,
    ) -> Self {
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
            node_attrs: self.node_attrs.copy(&format!("{name}/nodes")),
            relationship_attrs: self
                .relationship_attrs
                .copy(&format!("{name}/relationships")),
            node_indexer: Indexer::default(),
            node_labels: self.node_labels.clone(),
            relationship_types: self.relationship_types.clone(),
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(cache_size).unwrap(),
            ))),
            version: 0,
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
    pub fn get_labels(&self) -> Vec<Arc<String>> {
        self.node_labels.clone()
    }

    #[must_use]
    pub fn get_label_by_id(
        &self,
        id: LabelId,
    ) -> Arc<String> {
        self.node_labels[id.0].clone()
    }

    #[must_use]
    pub fn get_types(&self) -> Vec<Arc<String>> {
        self.relationship_types.clone()
    }

    #[must_use]
    pub fn get_type(
        &self,
        id: TypeId,
    ) -> Option<Arc<String>> {
        self.relationship_types.get(id.0).cloned()
    }

    #[must_use]
    pub fn get_attrs(&self) -> Vec<Arc<String>> {
        self.node_attrs
            .attrs_name
            .iter()
            .chain(self.relationship_attrs.attrs_name.iter())
            .cloned()
            .collect()
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

        match self.cache.lock() {
            Ok(mut cache) => {
                if let Some(plan) = cache.get(query) {
                    let optimize_plan = optimize(&plan.0, self);
                    Ok(Plan::new(
                        Arc::new(optimize_plan),
                        true,
                        parameters,
                        parse_duration,
                        plan_duration,
                    ))
                } else {
                    drop(cache);
                    let start = Instant::now();
                    let raw_ir = parser.parse()?;
                    let ir = Binder::default().bind(raw_ir)?;
                    ir.validate()?;
                    parse_duration = start.elapsed();

                    let mut planner = Planner::default();
                    let start = Instant::now();
                    let plan = planner.plan(ir);
                    let optimize_plan = optimize(&plan, self);
                    plan_duration = start.elapsed();

                    self.cache
                        .lock()
                        .unwrap()
                        .push(query.to_string(), PlanTree(plan));
                    Ok(Plan::new(
                        Arc::new(optimize_plan),
                        false,
                        parameters,
                        parse_duration,
                        plan_duration,
                    ))
                }
            }
            Err(_) => Err("Failed to acquire read lock on cache".to_string()),
        }
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

    pub fn set_node_attributes(
        &mut self,
        id: NodeId,
        attrs: OrderMap<Arc<String>, Value>,
        index_add_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) -> Result<usize, String> {
        let keys = attrs.keys().cloned().collect::<Vec<_>>();
        let nremoved = self.node_attrs.insert_attrs(id.0, &attrs)?;

        if self.node_indexer.has_indices() {
            for (_, label_id) in self.node_labels_matrix.iter(id.into(), id.into()) {
                for key in &keys {
                    let label = self.node_labels[label_id as usize].clone();
                    if self
                        .node_indexer
                        .is_attr_indexed(label.clone(), key.clone())
                    {
                        index_add_docs
                            .entry(label)
                            .or_default()
                            .insert(u64::from(id));
                    }
                }
            }
        }
        Ok(nremoved)
    }

    pub fn set_nodes_labels(
        &mut self,
        nodes_labels: &mut Matrix,
        index_add_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) {
        self.resize();

        for (id, label_id) in nodes_labels.iter(0, u64::MAX) {
            self.node_labels_matrix.set(id, label_id, true);
            self.labels_matices[label_id as usize].set(id, id, true);
            let label = self.node_labels[label_id as usize].clone();
            if self.node_indexer.is_label_indexed(label.clone())
                && self.node_attrs.has_attributes(id)
            {
                index_add_docs.entry(label.clone()).or_default().insert(id);
            }
        }
    }

    pub fn remove_nodes_labels(
        &mut self,
        nodes_labels: &mut Matrix,
        remove_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) {
        self.resize();

        for (id, label_id) in nodes_labels.iter(0, u64::MAX) {
            self.node_labels_matrix.remove(id, label_id);
            self.labels_matices[label_id as usize].remove(id, id);
            let label = self.node_labels[label_id as usize].clone();
            if self.node_indexer.is_label_indexed(label.clone()) {
                remove_docs.entry(label.clone()).or_default().insert(id);
            }
        }
    }

    pub fn delete_nodes(
        &mut self,
        deleted_nodes: &RoaringTreemap,
        remove_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) -> Result<(), String> {
        self.deleted_nodes |= deleted_nodes;
        self.node_count -= deleted_nodes.len();

        for id in deleted_nodes {
            self.all_nodes_matrix.remove(id, id);

            for (_, label_id) in self.node_labels_matrix.iter(id, id) {
                let label = self.node_labels[label_id as usize].clone();
                self.labels_matices[label_id as usize].remove(id, id);
                if self.node_indexer.is_label_indexed(label.clone()) {
                    for attr in self.node_attrs.get_attrs(id) {
                        if self.node_indexer.is_attr_indexed(label.clone(), attr) {
                            remove_docs.entry(label.clone()).or_default().insert(id);
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

    pub fn set_relationship_attributes(
        &mut self,
        id: RelationshipId,
        attrs: OrderMap<Arc<String>, Value>,
    ) -> Result<usize, String> {
        let nremoved = self.relationship_attrs.insert_attrs(id.0, &attrs)?;
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
            let label = self.relationship_types.get(type_id.0).cloned().unwrap();
            for (id, _, _) in &rels {
                self.relationship_type_matrix.remove(*id, type_id.0 as u64);
                self.relationship_attrs.remove(*id)?;
            }
            self.get_relationship_matrix_mut(&label).remove_all(rels);
        }
        Ok(())
    }

    #[must_use]
    pub fn get_src_dest_relationships(
        &self,
        src: NodeId,
        dest: NodeId,
        types: &[Arc<String>],
    ) -> Vec<RelationshipId> {
        let mut vec = vec![];
        for relationship_type in if types.is_empty() {
            &self.relationship_types
        } else {
            types
        } {
            if let Some(relationship_matrix) = self.get_relationship_matrix(relationship_type) {
                for (_, id) in relationship_matrix.get(src.0, dest.0) {
                    vec.push(RelationshipId(id));
                }
            }
        }
        vec
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
            .filter_map(|label| self.get_label_matrix(label))
            .collect::<Vec<_>>();
        let dest_labels_matrices = dest_labels
            .iter()
            .filter_map(|label| self.get_label_matrix(label))
            .collect::<Vec<_>>();

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
                src_matrix.element_wise_multiply(None, None, Some(&label_matrix.to_matrix()), None);
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

    #[must_use]
    pub fn get_node_attrs(
        &self,
        id: NodeId,
    ) -> Vec<Arc<String>> {
        self.node_attrs.get_attrs(id.0)
    }

    /// Get all attribute names and values for a node in a single storage pass.
    #[must_use]
    pub fn get_node_all_attrs(
        &self,
        id: NodeId,
    ) -> OrderMap<Arc<String>, Value> {
        self.node_attrs.get_all_attrs(id.0)
    }

    #[must_use]
    pub fn get_relationship_attrs(
        &self,
        id: RelationshipId,
    ) -> Vec<Arc<String>> {
        self.relationship_attrs.get_attrs(id.0)
    }

    /// Get all attribute names and values for a relationship in a single storage pass.
    #[must_use]
    pub fn get_relationship_all_attrs(
        &self,
        id: RelationshipId,
    ) -> OrderMap<Arc<String>, Value> {
        self.relationship_attrs.get_all_attrs(id.0)
    }

    pub fn create_index(
        &mut self,
        index_type: &IndexType,
        entity_type: &EntityType,
        label: &Arc<String>,
        attrs: &Vec<Arc<String>>,
    ) -> Result<(), String> {
        match entity_type {
            EntityType::Node => {
                let len = self.get_label_matrix_mut(label).nvals();
                {
                    self.node_indexer
                        .create_index(index_type, label.clone(), attrs, len)?;
                }
                self.populate_index(label);
            }
            EntityType::Relationship => {}
        }
        Ok(())
    }

    fn populate_index(
        &self,
        label: &Arc<String>,
    ) {
        let lm = self.get_label_matrix(label).unwrap();
        let label = label.clone();
        populate_index(
            label,
            lm.iter(0, u64::MAX),
            self.node_indexer.clone(),
            self.node_attrs.clone(),
        );
    }

    pub fn commit_attrs(&mut self) {
        self.node_attrs.commit();
        self.relationship_attrs.commit();
    }

    pub fn commit_index(
        &mut self,
        index_add_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
        remove_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) {
        let mut add_docs = HashMap::new();
        for (label, ids) in index_add_docs.drain() {
            let fields = self.node_indexer.get_fields(label.clone());
            let mut docs = vec![];
            for id in ids {
                let mut doc = Document::new(id);
                for (key, fields) in &fields {
                    if let Some(value) = self.node_attrs.get_attr(id, key) {
                        for field in fields {
                            doc.set(field.clone(), value.clone());
                        }
                    }
                }
                docs.push(doc);
            }
            add_docs.insert(label, docs);
        }

        self.node_indexer.commit(&mut add_docs, remove_docs);
    }

    pub fn drop_index(
        &mut self,
        index_type: &IndexType,
        entity_type: &EntityType,
        label: &Arc<String>,
        attrs: &Vec<Arc<String>>,
    ) {
        match entity_type {
            EntityType::Node => {
                let all_attrs = self
                    .node_indexer
                    .get_fields(label.clone())
                    .iter()
                    .filter_map(|(k, fields)| {
                        if fields.iter().any(|f| f.ty == IndexType::Fulltext) {
                            Some(k.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let total = self.get_label_matrix(label).unwrap().nvals();
                let reindex = self
                    .node_indexer
                    .drop_index(
                        label.clone(),
                        if index_type == &IndexType::Fulltext {
                            &all_attrs
                        } else {
                            attrs
                        },
                        index_type,
                        total,
                    )
                    .is_some();
                self.node_indexer.disable(label.clone());

                if reindex {
                    self.populate_index(label);
                } else {
                    drop_index(label.clone(), self.node_indexer.clone());
                }
            }
            EntityType::Relationship => {}
        }
    }

    #[must_use]
    pub fn is_indexed(
        &self,
        label: &Arc<String>,
        field: &Arc<String>,
    ) -> bool {
        self.node_indexer
            .is_attr_indexed(label.clone(), field.clone())
    }

    pub fn get_indexed_nodes(
        &self,
        label: &Arc<String>,
        query: IndexQuery<Value>,
    ) -> Vec<NodeId> {
        self.node_indexer
            .query(label.clone(), query)
            .into_iter()
            .map(NodeId)
            .collect()
    }

    #[must_use]
    pub fn index_info(&self) -> Vec<IndexInfo> {
        self.node_indexer.index_info()
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
