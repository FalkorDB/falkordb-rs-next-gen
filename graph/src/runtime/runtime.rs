//! Query execution engine.
//!
//! This module contains the [`Runtime`] struct which executes query plans against
//! the graph. The runtime implements a pull-based iterator model where each
//! operator requests tuples from its children.
//!
//! ## Execution Model
//!
//! ```text
//! Plan Tree (IR)          Runtime Execution
//!     Return               → Iterator yielding Env tuples
//!       │                        │
//!     Filter               → Filters tuples via predicate
//!       │                        │
//!     Expand               → Traverses relationships
//!       │                        │
//!     NodeScan             → Scans nodes by label
//! ```
//!
//! ## Key Types
//!
//! - [`Runtime`]: Main execution context
//! - [`ResultSummary`]: Query result with statistics
//! - [`QueryStatistics`]: Mutation counts and timing
//! - [`Env`]: Tuple of variable bindings during execution
//!
//! ## Write Operations
//!
//! Write operations (CREATE, DELETE, SET) are batched in [`Pending`] and applied
//! at the end of the query. This allows reads to see a consistent snapshot.

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
use crate::{
    graph::graph::{Graph, NodeId, RelationshipId},
    index::indexer::{IndexOptions, IndexType, TextIndexOptions},
    parser::ast::{ExprIR, QuantifierType, QueryExpr, Variable},
    planner::IR,
    runtime::{
        env::Env,
        functions::{FnType, apply_pow},
        ops::{
            AggregateOp, ApplyOp, ArgumentOp, CartesianProductOp, CommitOp, CondTraverseOp,
            CondVarLenTraverseOp, CreateOp, DeleteOp, DistinctOp, EmptyOp, ExpandIntoOp, FilterOp,
            LimitOp, LoadCsvOp, MergeOp, NodeByFulltextScanOp, NodeByIdSeekOp, NodeByIndexScanOp,
            NodeByLabelAndIdScanOp, NodeByLabelScanOp, OpIter, OptionalOp, OrApplyMultiplexerOp,
            PathBuilderOp, ProcedureCallOp, ProjectOp, RemoveOp, SemiApplyOp, SetOp, SkipOp,
            SortOp, UnionOp, UnwindOp,
        },
        ordermap::OrderMap,
        orderset::OrderSet,
        pending::Pending,
        value::{
            CompareValue, Contains, DeletedNode, DeletedRelationship, DisjointOrNull, Value,
            ValuesDeduper,
        },
    },
};
use atomic_refcell::AtomicRefCell;
use once_cell::unsync::Lazy;
use orx_tree::{Bfs, Dyn, DynNode, DynTree, MemoryPolicy, NodeIdx, NodeRef};
use roaring::RoaringTreemap;
use std::{
    cell::RefCell, cmp::Ordering, collections::HashMap, fmt::Debug, sync::Arc, time::Instant,
};
use thin_vec::{ThinVec, thin_vec};

pub enum ValueIter {
    Empty,
    Once(Option<Value>),
    RangeUp { current: i64, end: i64, step: usize },
    RangeDown { current: i64, end: i64, step: usize },
    List(thin_vec::IntoIter<Value>),
}

impl Iterator for ValueIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Once(v) => v.take(),
            Self::RangeUp { current, end, step } => {
                if *current > *end {
                    return None;
                }
                let val = *current;
                *current += *step as i64;
                Some(Value::Int(val))
            }
            Self::RangeDown { current, end, step } => {
                if *current < *end {
                    return None;
                }
                let val = *current;
                *current -= *step as i64;
                Some(Value::Int(val))
            }
            Self::List(iter) => iter.next(),
        }
    }
}

/// Query result containing statistics and returned tuples.
pub struct ResultSummary {
    /// Mutation statistics (nodes created, etc.)
    pub stats: QueryStatistics,
    /// Result tuples, each Env contains variable bindings
    pub result: Vec<Env>,
}

/// Statistics about query execution and mutations performed.
#[derive(Default)]
pub struct QueryStatistics {
    pub labels_added: usize,
    pub labels_removed: usize,
    pub nodes_created: u64,
    pub relationships_created: usize,
    pub nodes_deleted: u64,
    pub relationships_deleted: usize,
    pub properties_set: usize,
    pub properties_removed: usize,
    pub indexes_created: usize,
    pub indexes_dropped: usize,
    /// Total execution time in milliseconds
    pub execution_time: f64,
    /// Whether the query plan was retrieved from cache
    pub cached: bool,
}

/// The query execution context.
///
/// Runtime holds all state needed to execute a query plan:
/// - Graph reference and parameters
/// - Pending mutations (for deferred writes)
/// - Statistics tracking
/// - Variable bindings cache
///
/// # Lifecycle
/// 1. Create Runtime with graph, parameters, and plan
/// 2. Call `run()` to execute
/// 3. Pending mutations applied at end of execution
/// 4. Return `ResultSummary` with results and stats
pub struct Runtime {
    /// Query parameters ($param syntax)
    pub parameters: HashMap<String, Value>,
    /// Graph being queried (shared, thread-safe reference)
    pub g: Arc<AtomicRefCell<Graph>>,
    /// Whether this is a write query
    pub write: bool,
    /// Batched mutations (lazy-initialized)
    pub pending: Lazy<RefCell<Pending>>,
    /// Execution statistics
    pub stats: RefCell<QueryStatistics>,
    /// Query execution plan tree
    pub plan: Arc<DynTree<IR>>,
    /// Deduplication state for DISTINCT operations
    pub value_dedupers: RefCell<HashMap<String, ValuesDeduper>>,
    /// Variables to return in query results
    pub return_names: Vec<Variable>,
    /// Debug mode: record operator execution
    pub inspect: bool,
    /// Debug records of operator execution
    pub record: RefCell<Vec<(NodeIdx<Dyn<IR>>, Result<Env, String>)>>,
    /// Folder for LOAD CSV operations
    pub import_folder: String,
    /// Cache of deleted nodes for result consistency
    pub deleted_nodes: RefCell<HashMap<NodeId, DeletedNode>>,
    /// Cache of deleted relationships for result consistency
    pub deleted_relationships: RefCell<HashMap<RelationshipId, DeletedRelationship>>,
    /// Cache for MERGE pattern matching
    pub merge_pattern_cache: RefCell<HashMap<u64, Env>>,
}

pub trait GetVariables {
    fn get_variables(&self) -> Vec<Variable>;
}

impl<T: MemoryPolicy> GetVariables for DynNode<'_, IR, T> {
    fn get_variables(&self) -> Vec<Variable> {
        let mut vars = vec![];
        for node in self.walk::<Bfs>() {
            match node {
                IR::Optional(variables) => vars.extend(variables.iter().cloned()),
                IR::ProcedureCall(_, _, named_outputs) => {
                    vars.extend(named_outputs.clone());
                }
                IR::Unwind(_, variable) => vars.push(variable.clone()),
                IR::Create(query_graph) | IR::Merge(query_graph, _, _) => {
                    for node in query_graph.nodes() {
                        vars.push(node.alias.clone());
                    }
                    for relationship in query_graph.relationships() {
                        vars.push(relationship.alias.clone());
                    }
                    for path in query_graph.paths() {
                        vars.push(path.var.clone());
                    }
                }
                IR::Delete(_, _)
                | IR::Argument
                | IR::Set(_)
                | IR::Remove(_)
                | IR::Filter(_)
                | IR::CartesianProduct
                | IR::Union
                | IR::Apply
                | IR::SemiApply
                | IR::AntiSemiApply
                | IR::OrApplyMultiplexer(_)
                | IR::Sort(_)
                | IR::Skip(_)
                | IR::Limit(_)
                | IR::Distinct
                | IR::Commit
                | IR::CreateIndex { .. }
                | IR::DropIndex { .. } => {}
                IR::NodeByLabelScan(node)
                | IR::AllNodeScan(node)
                | IR::NodeByIndexScan { node, .. }
                | IR::NodeByLabelAndIdScan { node, .. }
                | IR::NodeByIdSeek { node, .. } => {
                    vars.push(node.alias.clone());
                }
                IR::NodeByFulltextScan { node, score, .. } => {
                    vars.push(node.clone());
                    if let Some(score) = score {
                        vars.push(score.clone());
                    }
                }
                IR::CondTraverse(query_relationship)
                | IR::CondVarLenTraverse(query_relationship) => {
                    vars.push(query_relationship.alias.clone());
                }
                IR::ExpandInto(query_relationship) => vars.push(query_relationship.alias.clone()),
                IR::PathBuilder(query_paths) => {
                    for path in query_paths {
                        vars.push(path.var.clone());
                    }
                }
                IR::LoadCsv { var, .. } => {
                    vars.push(var.clone());
                }
                IR::Aggregate(variables, _, _) => {
                    vars.extend(variables.iter().cloned());
                }
                IR::Project(items, _) => {
                    vars.extend(items.iter().map(|v| v.0.clone()));
                    break;
                }
            }
        }
        vars
    }
}

trait ReturnNames {
    fn get_return_names(&self) -> Vec<Variable>;
}

impl ReturnNames for DynNode<'_, IR> {
    fn get_return_names(&self) -> Vec<Variable> {
        match self.data() {
            IR::Project(trees, _) => trees.iter().map(|v| v.0.clone()).collect(),
            IR::Commit => self
                .get_child(0)
                .map_or(vec![], |child| child.get_return_names()),
            IR::ProcedureCall(_, _, named_outputs) => named_outputs.clone(),
            IR::NodeByFulltextScan { node, score, .. } => {
                let mut v = vec![node.clone()];
                if let Some(score) = score {
                    v.push(score.clone());
                }
                v
            }
            IR::Sort(_) | IR::Skip(_) | IR::Limit(_) | IR::Distinct => {
                self.child(0).get_return_names()
            }
            IR::Union => self.child(0).get_return_names(),
            IR::Aggregate(names, _, _) => names.clone(),
            _ => vec![],
        }
    }
}

impl Debug for Env {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_list().entries(self.as_ref().iter()).finish()
    }
}

impl Runtime {
    #[inline]
    pub fn inspect_result(
        &self,
        idx: NodeIdx<Dyn<IR>>,
        result: &Result<Env, String>,
    ) {
        if self.inspect {
            self.record.borrow_mut().push((idx, result.clone()));
        }
    }

    #[must_use]
    pub fn new(
        g: Arc<AtomicRefCell<Graph>>,
        parameters: HashMap<String, Value>,
        write: bool,
        plan: Arc<DynTree<IR>>,
        inspect: bool,
        import_folder: String,
    ) -> Self {
        let return_names = plan.root().get_return_names();
        Self {
            parameters,
            g,
            write,
            pending: Lazy::new(|| RefCell::new(Pending::new())),
            stats: RefCell::new(QueryStatistics::default()),
            plan,
            return_names,
            value_dedupers: RefCell::new(HashMap::new()),
            inspect,
            record: RefCell::new(vec![]),
            import_folder,
            deleted_nodes: RefCell::new(HashMap::new()),
            deleted_relationships: RefCell::new(HashMap::new()),
            merge_pattern_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn query(&mut self) -> Result<ResultSummary, String> {
        let start = Instant::now();
        let idx = self.plan.root().idx();
        let labels_count = self.g.borrow().labels_count();
        let mut result = vec![];
        for env in self.run(idx)? {
            let env = env?;
            result.push(env);
        }
        let run_duration = start.elapsed();

        self.stats.borrow_mut().labels_added += self.g.borrow().labels_count() - labels_count;
        self.stats.borrow_mut().execution_time = run_duration.as_secs_f64() * 1000.0;
        Ok(ResultSummary {
            stats: self.stats.take(),
            result,
        })
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    pub fn run_expr(
        &self,
        ir: &DynTree<ExprIR<Variable>>,
        idx: NodeIdx<Dyn<ExprIR<Variable>>>,
        env: &Env,
        agg_group_key: Option<u64>,
    ) -> Result<Value, String> {
        match ir.node(idx).data() {
            ExprIR::Null => return Ok(Value::Null),
            ExprIR::Bool(x) => return Ok(Value::Bool(*x)),
            ExprIR::Integer(x) => return Ok(Value::Int(*x)),
            ExprIR::Float(x) => return Ok(Value::Float(*x)),
            ExprIR::String(x) => return Ok(Value::String(x.clone())),
            ExprIR::Variable(x) => {
                return env
                    .get(x)
                    .ok_or_else(|| format!("Variable {} not found", x.as_str()))
                    .cloned();
            }

            ExprIR::Parameter(x) => {
                return self.parameters.get(x).map_or_else(
                    || Err(format!("Parameter {x} not found")),
                    |v| Ok(v.clone()),
                );
            }
            ExprIR::Map => {
                return Ok(Value::Map(Arc::new(
                    ir.node(idx)
                        .children()
                        .map(|child| {
                            Ok((
                                if let ExprIR::String(key) = child.data() {
                                    key.clone()
                                } else {
                                    todo!();
                                },
                                self.run_expr(ir, child.child(0).idx(), env, agg_group_key)?,
                            ))
                        })
                        .collect::<Result<_, String>>()?,
                )));
            }
            ExprIR::MapProjection => {
                return self.eval_map_projection(ir, idx, env, agg_group_key);
            }
            _ => {}
        }
        let mut res: Vec<Value> = vec![];
        let mut stack = thin_vec![(idx, false)];
        while let Some((idx, reenter)) = stack.pop() {
            let node = ir.node(idx);
            match node.data() {
                ExprIR::Null => res.push(Value::Null),
                ExprIR::Bool(x) => res.push(Value::Bool(*x)),
                ExprIR::Integer(x) => res.push(Value::Int(*x)),
                ExprIR::Float(x) => res.push(Value::Float(*x)),
                ExprIR::String(x) => res.push(Value::String(x.clone())),
                ExprIR::Variable(x) => res.push(
                    env.get(x)
                        .ok_or_else(|| format!("Variable {} not found", x.as_str()))?
                        .clone(),
                ),
                ExprIR::Parameter(x) => res.push(self.parameters.get(x).map_or_else(
                    || Err(format!("Parameter {x} not found")),
                    |v| Ok(v.clone()),
                )?),
                ExprIR::List => {
                    if reenter {
                        let mut list = thin_vec![];
                        for _ in 0..node.num_children() {
                            list.push(res.pop().unwrap());
                        }
                        res.push(Value::List(Arc::new(list)));
                    } else if node.num_children() > 0 {
                        stack.push((idx, true));
                        for idx in node.children().map(|c| c.idx()) {
                            stack.push((idx, false));
                        }
                    } else {
                        res.push(Value::List(Arc::new(thin_vec![])));
                    }
                }
                ExprIR::Length => {
                    match self.run_expr(ir, node.child(0).idx(), env, agg_group_key)? {
                        Value::List(arr) => res.push(Value::Int(arr.len() as _)),
                        _ => return Err(String::from("Length operator requires a list")),
                    }
                }
                ExprIR::GetElement => {
                    let arr = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    let i = self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?;
                    match (arr, i) {
                        (Value::List(values), Value::Int(i)) => {
                            // Handle negative indexing:  -1 is last element, -2 is second to last, etc.
                            let len = values.len() as i64;
                            let normalized_index = if i < 0 {
                                // Negative index: convert to positive offset
                                len + i
                            } else {
                                i
                            };

                            // Check bounds:  valid range is [0, len)
                            if normalized_index >= 0 && normalized_index < len {
                                res.push(values[normalized_index as usize].clone());
                            } else {
                                res.push(Value::Null);
                            }
                        }
                        (Value::List(_), v) => {
                            return Err(format!(
                                "Type mismatch: expected Integer but was {}",
                                v.name()
                            ));
                        }
                        (Value::Node(id), Value::String(key) | Value::InternedString(key)) => {
                            res.push(self.get_node_attribute(id, &key).unwrap_or(Value::Null));
                        }
                        (
                            Value::Relationship(rel),
                            Value::String(key) | Value::InternedString(key),
                        ) => {
                            res.push(
                                self.get_relationship_attribute(rel.0, &key)
                                    .unwrap_or(Value::Null),
                            );
                        }
                        (Value::Map(map), Value::String(key) | Value::InternedString(key)) => {
                            res.push(map.get(&key).map_or(Value::Null, std::clone::Clone::clone));
                        }
                        (Value::Map(_), Value::Null) | (Value::Null, _) => res.push(Value::Null),
                        v => return Err(format!("Type mismatch: unexpected types {v:?}")),
                    }
                }
                ExprIR::GetElements => {
                    let arr = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    let a = self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?;
                    let b = self.run_expr(ir, node.child(2).idx(), env, agg_group_key)?;
                    res.push(get_elements(&arr, &a, &b)?);
                }
                ExprIR::IsNode => {
                    match self.run_expr(ir, node.child(0).idx(), env, agg_group_key)? {
                        Value::Node(_) => res.push(Value::Bool(true)),
                        _ => res.push(Value::Bool(false)),
                    }
                }
                ExprIR::IsRelationship => {
                    match self.run_expr(ir, node.child(0).idx(), env, agg_group_key)? {
                        Value::Relationship(_) => res.push(Value::Bool(true)),
                        _ => res.push(Value::Bool(false)),
                    }
                }
                ExprIR::Or => {
                    let mut is_null = false;
                    let mut found = false;
                    for child in node.children() {
                        match self.run_expr(ir, child.idx(), env, agg_group_key)? {
                            Value::Bool(true) => {
                                found = true;
                                res.push(Value::Bool(true));
                                break;
                            }
                            Value::Bool(false) => {}
                            Value::Null => is_null = true,
                            ir => {
                                return Err(format!("Type mismatch: expected Bool but was {ir:?}"));
                            }
                        }
                    }
                    if !found {
                        if is_null {
                            res.push(Value::Null);
                        } else {
                            res.push(Value::Bool(false));
                        }
                    }
                }
                ExprIR::Xor => {
                    let mut last = None;
                    let mut found = false;
                    for child in node.children() {
                        match self.run_expr(ir, child.idx(), env, agg_group_key)? {
                            Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                            Value::Null => {
                                found = true;
                                res.push(Value::Null);
                                break;
                            }
                            ir => {
                                return Err(format!("Type mismatch: expected Bool but was {ir:?}"));
                            }
                        }
                    }
                    if !found {
                        res.push(Value::Bool(last.unwrap_or(false)));
                    }
                }
                ExprIR::And => {
                    let mut is_null = false;
                    let mut found = false;
                    for child in node.children() {
                        match self.run_expr(ir, child.idx(), env, agg_group_key)? {
                            Value::Bool(false) => {
                                found = true;
                                res.push(Value::Bool(false));
                                break;
                            }
                            Value::Bool(true) => {}
                            Value::Null => is_null = true,
                            ir => {
                                return Err(format!("Type mismatch: expected Bool but was {ir:?}"));
                            }
                        }
                    }
                    if !found {
                        if is_null {
                            res.push(Value::Null);
                        } else {
                            res.push(Value::Bool(true));
                        }
                    }
                }
                ExprIR::Not => match self.run_expr(ir, node.child(0).idx(), env, agg_group_key)? {
                    Value::Bool(b) => res.push(Value::Bool(!b)),
                    Value::Null => res.push(Value::Null),
                    v => {
                        return Err(format!(
                            "Type mismatch: expected Boolean or Null but was {}",
                            v.name()
                        ));
                    }
                },
                ExprIR::Negate => {
                    match self.run_expr(ir, node.child(0).idx(), env, agg_group_key)? {
                        Value::Int(i) => res.push(Value::Int(-i)),
                        Value::Float(f) => res.push(Value::Float(-f)),
                        Value::Null => res.push(Value::Null),
                        v => {
                            return Err(format!(
                                "Type mismatch: expected Integer, Float, or Null but was {}",
                                v.name()
                            ));
                        }
                    }
                }
                ExprIR::Eq => res
                    .push(all_equals(node.children().map(|child| {
                        self.run_expr(ir, child.idx(), env, agg_group_key)
                    }))?),
                ExprIR::Neq => res
                    .push(all_not_equals(node.children().map(|child| {
                        self.run_expr(ir, child.idx(), env, agg_group_key)
                    }))?),
                ExprIR::Lt => match self
                    .run_expr(ir, node.child(0).idx(), env, agg_group_key)?
                    .compare_value(&self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?)
                {
                    (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => {
                        res.push(Value::Null);
                    }
                    (_, DisjointOrNull::NaN) => res.push(Value::Bool(false)),
                    (Ordering::Less, _) => res.push(Value::Bool(true)),
                    _ => res.push(Value::Bool(false)),
                },
                ExprIR::Gt => match self
                    .run_expr(ir, node.child(0).idx(), env, agg_group_key)?
                    .compare_value(&self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?)
                {
                    (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => {
                        res.push(Value::Null);
                    }
                    (_, DisjointOrNull::NaN) => res.push(Value::Bool(false)),
                    (Ordering::Greater, _) => res.push(Value::Bool(true)),
                    _ => res.push(Value::Bool(false)),
                },
                ExprIR::Le => match self
                    .run_expr(ir, node.child(0).idx(), env, agg_group_key)?
                    .compare_value(&self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?)
                {
                    (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => {
                        res.push(Value::Null);
                    }
                    (_, DisjointOrNull::NaN) => res.push(Value::Bool(false)),
                    (Ordering::Less | Ordering::Equal, _) => res.push(Value::Bool(true)),
                    _ => res.push(Value::Bool(false)),
                },
                ExprIR::Ge => match self
                    .run_expr(ir, node.child(0).idx(), env, agg_group_key)?
                    .compare_value(&self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?)
                {
                    (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => {
                        res.push(Value::Null);
                    }
                    (_, DisjointOrNull::NaN) => res.push(Value::Bool(false)),
                    (Ordering::Greater | Ordering::Equal, _) => res.push(Value::Bool(true)),
                    _ => res.push(Value::Bool(false)),
                },
                ExprIR::In => {
                    let value = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    let list = self.run_expr(ir, node.child(1).idx(), env, agg_group_key)?;
                    res.push(list_contains(&list, value)?);
                }
                ExprIR::Add => res.push(
                    node.children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(|acc, value| acc? + value?)
                        .ok_or_else(|| {
                            String::from("Add operator requires at least one operand")
                        })??,
                ),
                ExprIR::Sub => res.push(
                    node.children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(|acc, value| acc? - value?)
                        .ok_or_else(|| {
                            String::from("Sub operator requires at least one argument")
                        })??,
                ),
                ExprIR::Mul => res.push(
                    node.children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(|acc, value| acc? * value?)
                        .ok_or_else(|| {
                            String::from("Mul operator requires at least one argument")
                        })??,
                ),
                ExprIR::Div => res.push(
                    node.children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(|acc, value| acc? / value?)
                        .ok_or_else(|| {
                            String::from("Div operator requires at least one argument")
                        })??,
                ),
                ExprIR::Modulo => res.push(
                    node.children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(|acc, value| acc? % value?)
                        .ok_or_else(|| {
                            String::from("Modulo operator requires at least one argument")
                        })??,
                ),
                ExprIR::Pow => res.push(
                    node.children()
                        .flat_map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .reduce(apply_pow)
                        .ok_or_else(|| {
                            String::from("Pow operator requires at least one argument")
                        })?,
                ),
                ExprIR::Distinct => {
                    let group_id = agg_group_key.unwrap();
                    let values = node
                        .children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .collect::<Result<ThinVec<_>, _>>()?;
                    let mut value_dedupers = self.value_dedupers.borrow_mut();
                    let value_deduper = value_dedupers
                        .entry(format!("{idx:?}_{group_id}"))
                        .or_default();
                    if value_deduper.is_seen(&values) {
                        res.push(Value::List(Arc::new(thin_vec![Value::Null])));
                    } else {
                        res.push(Value::List(Arc::new(values)));
                    }
                }
                ExprIR::Property(attr) => {
                    let obj = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    match obj {
                        Value::Node(id) => {
                            res.push(self.get_node_attribute(id, attr).unwrap_or(Value::Null));
                        }
                        Value::Relationship(rel) => {
                            res.push(
                                self.get_relationship_attribute(rel.0, attr)
                                    .unwrap_or(Value::Null),
                            );
                        }
                        other => {
                            res.push(other.get_attr(attr)?);
                        }
                    }
                }
                ExprIR::FuncInvocation(func) => {
                    if agg_group_key.is_none()
                        && let FnType::Aggregation(_, finalize) = &func.fn_type
                        && let ExprIR::Variable(key) = node.child(node.num_children() - 1).data()
                    {
                        let acc = env.get(key).unwrap().clone();

                        return match finalize {
                            Some(func) => Ok((func)(acc)),
                            None => Ok(acc),
                        };
                    }
                    let mut args = node
                        .children()
                        .map(|child| self.run_expr(ir, child.idx(), env, agg_group_key))
                        .collect::<Result<ThinVec<_>, _>>()?;
                    if node.num_children() == 2 && matches!(node.child(0).data(), ExprIR::Distinct)
                    {
                        match args.remove(0) {
                            Value::List(values) => {
                                let mut values = Arc::unwrap_or_clone(values);
                                values.append(&mut args);
                                args = values;
                            }
                            _ => unreachable!(),
                        }
                    }

                    func.validate_args_type(&args)?;
                    if !self.write && func.write {
                        return Err(String::from(
                            "graph.RO_QUERY is to be executed only on read-only queries",
                        ));
                    }

                    res.push((func.func)(self, args)?);
                }
                ExprIR::Map => res.push(Value::Map(Arc::new(
                    node.children()
                        .map(|child| {
                            Ok((
                                if let ExprIR::String(key) = child.data() {
                                    key.clone()
                                } else {
                                    todo!();
                                },
                                self.run_expr(ir, child.child(0).idx(), env, agg_group_key)?,
                            ))
                        })
                        .collect::<Result<_, String>>()?,
                ))),
                ExprIR::MapProjection => {
                    res.push(self.eval_map_projection(ir, idx, env, agg_group_key)?);
                }
                ExprIR::Quantifier(quantifier, var) => {
                    let list = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    match list {
                        Value::List(values) => {
                            let mut env = env.clone();
                            let mut t = 0;
                            let mut f = 0;
                            let mut n = 0;
                            for value in values.iter().cloned() {
                                env.insert(var, value);

                                match self.run_expr(ir, node.child(1).idx(), &env, agg_group_key)? {
                                    Value::Bool(true) => t += 1,
                                    Value::Bool(false) => f += 1,
                                    Value::Null => n += 1,
                                    value => {
                                        return Err(format!(
                                            "Type mismatch: expected Boolean but was {}",
                                            value.name()
                                        ));
                                    }
                                }
                            }

                            res.push(Self::eval_quantifier(quantifier, t, f, n));
                        }
                        Value::Null => res.push(Value::Null),
                        value => {
                            return Err(format!(
                                "Type mismatch: expected List but was {}",
                                value.name()
                            ));
                        }
                    }
                }
                ExprIR::ListComprehension(var) => {
                    let iter = self.run_iter_expr(ir, node.child(0).idx(), env)?;
                    let mut env = env.clone();
                    let mut acc = thin_vec![];
                    for value in iter {
                        env.insert(var, value);
                        match self.run_expr(ir, node.child(1).idx(), &env, agg_group_key)? {
                            Value::Bool(true) => {}
                            _ => continue,
                        }
                        acc.push(self.run_expr(ir, node.child(2).idx(), &env, agg_group_key)?);
                    }

                    res.push(Value::List(Arc::new(acc)));
                }
                ExprIR::PatternComprehension(graph) => {
                    let children: Vec<_> = node.children().map(|c| c.idx()).collect();
                    let where_idx = children[0];
                    let expr_idx = children[1];
                    let mut results = ThinVec::new();

                    // Iterate each relationship hop in the pattern.
                    // For single-hop patterns (the common case), there's
                    // exactly one relationship; multi-hop chains expand
                    // intermediate envs through each hop.
                    let mut envs = vec![env.clone()];
                    for rel_pattern in graph.relationships() {
                        let mut next_envs = Vec::new();
                        for current_env in &envs {
                            let from_id =
                                current_env
                                    .get(&rel_pattern.from.alias)
                                    .and_then(|v| match v {
                                        Value::Node(id) => Some(*id),
                                        _ => None,
                                    });
                            let to_id =
                                current_env
                                    .get(&rel_pattern.to.alias)
                                    .and_then(|v| match v {
                                        Value::Node(id) => Some(*id),
                                        _ => None,
                                    });

                            // Phase 1: collect matches while holding graph borrow.
                            let matches: Vec<(NodeId, NodeId, RelationshipId)> = {
                                let g = self.g.borrow();
                                g.get_relationships(
                                    &rel_pattern.types,
                                    &rel_pattern.from.labels,
                                    &rel_pattern.to.labels,
                                )
                                .filter(|(src, dst)| {
                                    from_id.is_none_or(|fid| fid == *src)
                                        && to_id.is_none_or(|tid| tid == *dst)
                                })
                                .flat_map(|(src, dst)| {
                                    g.get_src_dest_relationships(src, dst, &rel_pattern.types)
                                        .map(move |rid| (src, dst, rid))
                                })
                                .collect()
                            };

                            // Phase 2: build envs per match (graph borrow released).
                            for (src, dst, rel_id) in matches {
                                let mut local_env = current_env.clone();
                                local_env.insert(
                                    &rel_pattern.alias,
                                    Value::Relationship(Box::new((rel_id, src, dst))),
                                );
                                local_env.insert(&rel_pattern.from.alias, Value::Node(src));
                                local_env.insert(&rel_pattern.to.alias, Value::Node(dst));

                                // Check inline property predicates on nodes and relationship.
                                if !self.check_inline_node_attrs(
                                    &rel_pattern.from.attrs,
                                    src,
                                    &local_env,
                                    agg_group_key,
                                )? || !self.check_inline_node_attrs(
                                    &rel_pattern.to.attrs,
                                    dst,
                                    &local_env,
                                    agg_group_key,
                                )? || !self.check_inline_rel_attrs(
                                    &rel_pattern.attrs,
                                    rel_id,
                                    &local_env,
                                    agg_group_key,
                                )? {
                                    continue;
                                }

                                next_envs.push(local_env);
                            }

                            // Handle bidirectional: also check reverse direction.
                            if rel_pattern.bidirectional {
                                let rev_matches: Vec<(NodeId, NodeId, RelationshipId)> = {
                                    let g = self.g.borrow();
                                    g.get_relationships(
                                        &rel_pattern.types,
                                        &rel_pattern.to.labels,
                                        &rel_pattern.from.labels,
                                    )
                                    .filter(|(src, dst)| {
                                        // Reversed: to_id matches src, from_id matches dst
                                        to_id.is_none_or(|tid| tid == *src)
                                            && from_id.is_none_or(|fid| fid == *dst)
                                    })
                                    .flat_map(|(src, dst)| {
                                        g.get_src_dest_relationships(src, dst, &rel_pattern.types)
                                            .map(move |rid| (src, dst, rid))
                                    })
                                    .collect()
                                };
                                for (src, dst, rel_id) in rev_matches {
                                    let mut local_env = current_env.clone();
                                    local_env.insert(
                                        &rel_pattern.alias,
                                        Value::Relationship(Box::new((rel_id, src, dst))),
                                    );
                                    // Swap from/to for bidirectional reverse
                                    local_env.insert(&rel_pattern.from.alias, Value::Node(dst));
                                    local_env.insert(&rel_pattern.to.alias, Value::Node(src));

                                    // Check inline property predicates (swapped nodes).
                                    if !self.check_inline_node_attrs(
                                        &rel_pattern.from.attrs,
                                        dst,
                                        &local_env,
                                        agg_group_key,
                                    )? || !self.check_inline_node_attrs(
                                        &rel_pattern.to.attrs,
                                        src,
                                        &local_env,
                                        agg_group_key,
                                    )? || !self.check_inline_rel_attrs(
                                        &rel_pattern.attrs,
                                        rel_id,
                                        &local_env,
                                        agg_group_key,
                                    )? {
                                        continue;
                                    }

                                    next_envs.push(local_env);
                                }
                            }
                        }
                        envs = next_envs;
                    }

                    // Build path values for named paths in the pattern.
                    for matched_env in &mut envs {
                        for path in graph.paths() {
                            let p = path
                                .vars
                                .iter()
                                .map(|v| {
                                    matched_env
                                        .get(v)
                                        .ok_or_else(|| format!("Variable {} not found", v.as_str()))
                                        .cloned()
                                })
                                .collect::<Result<ThinVec<_>, String>>()?;
                            matched_env.insert(&path.var, Value::Path(Arc::new(p)));
                        }
                    }

                    // Evaluate WHERE + result expression per matched env.
                    for matched_env in &envs {
                        match self.run_expr(ir, where_idx, matched_env, agg_group_key)? {
                            Value::Bool(true) => {}
                            _ => continue,
                        }
                        results.push(self.run_expr(ir, expr_idx, matched_env, agg_group_key)?);
                    }
                    res.push(Value::List(Arc::new(results)));
                }
                ExprIR::Paren => {
                    res.push(self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?);
                }
                ExprIR::Pattern(_) => {
                    unreachable!("Pattern expressions should have been rewritten in the planner");
                }
            }
        }
        debug_assert_eq!(res.len(), 1);
        Ok(res.pop().unwrap())
    }

    pub fn run_iter_expr(
        &self,
        ir: &DynTree<ExprIR<Variable>>,
        idx: NodeIdx<Dyn<ExprIR<Variable>>>,
        env: &Env,
    ) -> Result<ValueIter, String> {
        match ir.node(idx).data() {
            ExprIR::FuncInvocation(func) if func.name == "range" => {
                let start = self.run_expr(ir, ir.node(idx).child(0).idx(), env, None)?;
                let end = self.run_expr(ir, ir.node(idx).child(1).idx(), env, None)?;
                let step = ir.node(idx).get_child(2).map_or_else(
                    || Ok(Value::Int(1)),
                    |c| self.run_expr(ir, c.idx(), env, None),
                )?;
                func.validate_args_type(&[&start, &end, &step])?;
                match (start, end, step) {
                    (Value::Int(start), Value::Int(end), Value::Int(step)) => {
                        if step == 0 {
                            return Err(String::from(
                                "ArgumentError: step argument to range() can't be 0",
                            ));
                        }
                        if (start > end && step > 0) || (start < end && step < 0) {
                            return Ok(ValueIter::Empty);
                        }
                        let length = (end - start) / step + 1;
                        #[allow(clippy::cast_lossless)]
                        if length > u32::MAX as i64 {
                            return Err(String::from("Range too large"));
                        }

                        if step > 0 {
                            return Ok(ValueIter::RangeUp {
                                current: start,
                                end,
                                step: step as usize,
                            });
                        }
                        Ok(ValueIter::RangeDown {
                            current: start,
                            end,
                            step: (-step) as usize,
                        })
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            _ => {
                let res = self.run_expr(ir, idx, env, None)?;
                match res {
                    Value::List(arr) => Ok(ValueIter::List(Arc::unwrap_or_clone(arr).into_iter())),
                    Value::Null => Ok(ValueIter::Empty),
                    _ => Ok(ValueIter::Once(Some(res))),
                }
            }
        }
    }

    const fn eval_quantifier(
        quantifier_type: &QuantifierType,
        true_count: usize,
        false_count: usize,
        null_count: usize,
    ) -> Value {
        match quantifier_type {
            QuantifierType::All => {
                if false_count > 0 {
                    Value::Bool(false)
                } else if null_count > 0 {
                    Value::Null
                } else {
                    Value::Bool(true)
                }
            }
            QuantifierType::Any => {
                if true_count > 0 {
                    Value::Bool(true)
                } else if null_count > 0 {
                    Value::Null
                } else {
                    Value::Bool(false)
                }
            }
            QuantifierType::None => {
                if true_count > 0 {
                    Value::Bool(false)
                } else if null_count > 0 {
                    Value::Null
                } else {
                    Value::Bool(true)
                }
            }
            QuantifierType::Single => {
                if true_count == 1 && null_count == 0 {
                    Value::Bool(true)
                } else if true_count > 1 {
                    Value::Bool(false)
                } else if null_count > 0 {
                    Value::Null
                } else {
                    Value::Bool(false)
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn run(
        &self,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Result<OpIter<'_>, String> {
        let child0_idx = self.plan.node(idx).get_child(0).map(|n| n.idx());
        let iter = if matches!(
            self.plan.node(idx).data(),
            IR::Optional(_) | IR::Merge(_, _, _)
        ) {
            if let Some(child_idx) = child0_idx
                && self.plan.node(idx).num_children() > 1
            {
                self.run(child_idx)?
            } else {
                OpIter::OnceOk(Some(Env::default()))
            }
        } else if matches!(self.plan.node(idx).data(), IR::Delete(_, _))
            && self.plan.node(idx).num_children() == 0
        {
            return Err(String::from(
                "DELETE can only be called on nodes, paths and relationships",
            ));
        } else if matches!(
            self.plan.node(idx).data(),
            IR::Set(_)
                | IR::PathBuilder(_)
                | IR::Filter(_)
                | IR::Sort(_)
                | IR::Skip(_)
                | IR::Limit(_)
                | IR::Distinct
                | IR::Commit
        ) && self.plan.node(idx).num_children() == 0
        {
            println!(
                "Runtime error: {:?} node has no children",
                self.plan.node(idx).data()
            );
            unreachable!();
        } else if matches!(self.plan.node(idx).data(), IR::Union) {
            // Union handles all children in its match arm.
            OpIter::Empty(EmptyOp)
        } else if let Some(child_idx) = child0_idx {
            self.run(child_idx)?
        } else {
            OpIter::OnceOk(Some(Env::default()))
        };
        let iter = Box::new(iter);
        match self.plan.node(idx).data() {
            IR::Argument => Ok(OpIter::Argument(ArgumentOp::new())),
            IR::Optional(vars) => Ok(OpIter::Optional(OptionalOp::new(self, iter, vars, idx))),
            IR::ProcedureCall(func, trees, name_outputs) => Ok(OpIter::ProcedureCall(
                ProcedureCallOp::new(self, func, trees, name_outputs, idx)?,
            )),
            IR::Unwind(list, name) => {
                Ok(OpIter::Unwind(UnwindOp::new(self, iter, list, name, idx)))
            }
            IR::Create(pattern) => Ok(OpIter::Create(CreateOp::new(self, iter, pattern, idx))),
            IR::Merge(pattern, on_create_set_items, on_match_set_items) => {
                Ok(OpIter::Merge(MergeOp::new(
                    self,
                    iter,
                    pattern,
                    on_create_set_items,
                    on_match_set_items,
                    idx,
                )))
            }
            IR::Delete(trees, _) => Ok(OpIter::Delete(DeleteOp::new(self, iter, trees, idx))),
            IR::Set(items) => Ok(OpIter::Set(SetOp::new(self, iter, items, idx))),
            IR::Remove(items) => Ok(OpIter::Remove(RemoveOp::new(self, iter, items, idx))),
            IR::NodeByLabelScan(node) | IR::AllNodeScan(node) => Ok(OpIter::NodeByLabelScan(
                NodeByLabelScanOp::new(self, iter, node, idx),
            )),
            IR::NodeByLabelAndIdScan { node, filter } => Ok(OpIter::NodeByLabelAndIdScan(
                NodeByLabelAndIdScanOp::new(self, iter, node, filter, idx),
            )),
            IR::NodeByIdSeek { node, filter } => Ok(OpIter::NodeByIdSeek(NodeByIdSeekOp::new(
                self, iter, node, filter, idx,
            ))),
            IR::NodeByIndexScan { node, index, query } => Ok(OpIter::NodeByIndexScan(
                NodeByIndexScanOp::new(self, iter, node, index, query, idx),
            )),
            IR::NodeByFulltextScan {
                node,
                label,
                query,
                score,
            } => Ok(OpIter::NodeByFulltextScan(NodeByFulltextScanOp::new(
                self, iter, node, label, query, score, idx,
            ))),
            IR::CondTraverse(relationship_pattern) => Ok(OpIter::CondTraverse(
                CondTraverseOp::new(self, iter, relationship_pattern, idx),
            )),
            IR::CondVarLenTraverse(relationship_pattern) => Ok(OpIter::CondVarLenTraverse(
                CondVarLenTraverseOp::new(self, iter, relationship_pattern, idx),
            )),
            IR::ExpandInto(relationship_pattern) => Ok(OpIter::ExpandInto(ExpandIntoOp::new(
                self,
                iter,
                relationship_pattern,
                idx,
            ))),
            IR::PathBuilder(paths) => Ok(OpIter::PathBuilder(PathBuilderOp::new(
                self, iter, paths, idx,
            ))),
            IR::Filter(tree) => Ok(OpIter::Filter(FilterOp::new(self, iter, tree, idx))),
            IR::CartesianProduct => Ok(OpIter::CartesianProduct(CartesianProductOp::new(
                self, iter, idx,
            ))),
            IR::Union => Ok(OpIter::Union(UnionOp::new(self, idx))),
            IR::Apply => Ok(OpIter::Apply(ApplyOp::new(self, iter, idx))),
            IR::SemiApply | IR::AntiSemiApply => {
                let is_anti = matches!(self.plan.node(idx).data(), IR::AntiSemiApply);
                Ok(OpIter::SemiApply(SemiApplyOp::new(
                    self, iter, is_anti, idx,
                )))
            }
            IR::OrApplyMultiplexer(anti_flags) => Ok(OpIter::OrApplyMultiplexer(
                OrApplyMultiplexerOp::new(self, iter, anti_flags, idx),
            )),
            IR::LoadCsv {
                file_path,
                headers,
                delimiter,
                var,
            } => Ok(OpIter::LoadCsv(LoadCsvOp::new(
                self, iter, file_path, headers, delimiter, var, idx,
            ))),
            IR::Sort(trees) => Ok(OpIter::Sort(SortOp::new(self, iter, trees, idx))),
            IR::Skip(skip) => {
                let Value::Int(skip) =
                    self.run_expr(skip, skip.root().idx(), &Env::default(), None)?
                else {
                    return Err(String::from("Skip operator requires an integer argument"));
                };
                Ok(OpIter::Skip(SkipOp::new(self, iter, skip as usize, idx)))
            }
            IR::Limit(limit) => {
                let Value::Int(limit) =
                    self.run_expr(limit, limit.root().idx(), &Env::default(), None)?
                else {
                    return Err(String::from("Limit operator requires an integer argument"));
                };
                Ok(OpIter::Limit(LimitOp::new(self, iter, limit as usize, idx)))
            }
            IR::Aggregate(_, keys, agg) => Ok(OpIter::Aggregate(AggregateOp::new(
                self, iter, keys, agg, idx,
            ))),
            IR::Project(trees, copy_from_parent) => Ok(OpIter::Project(ProjectOp::new(
                self,
                iter,
                trees,
                copy_from_parent,
                idx,
            ))),
            IR::Distinct => Ok(OpIter::Distinct(DistinctOp::new(self, iter, idx))),
            IR::Commit => Ok(OpIter::Commit(CommitOp::new(self, iter, idx)?)),
            IR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options,
            } => {
                if !self.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                let index_options = match options {
                    Some(expr) => {
                        let val = self.run_expr(expr, expr.root().idx(), &Env::default(), None)?;
                        match val {
                            Value::Map(map) => map_to_index_options(index_type, &map)?,
                            _ => return Err("Index options must be a map".into()),
                        }
                    }
                    None => None,
                };
                self.g.borrow_mut().create_index(
                    index_type,
                    entity_type,
                    label,
                    attrs,
                    index_options,
                )?;
                self.stats.borrow_mut().indexes_created += attrs.len();
                Ok(OpIter::Empty(EmptyOp))
            }
            IR::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            } => {
                if !self.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }

                let dropped =
                    self.g
                        .borrow_mut()
                        .drop_index(index_type, entity_type, label, attrs)?;
                self.stats.borrow_mut().indexes_dropped += dropped;
                Ok(OpIter::Empty(EmptyOp))
            }
        }
    }

    pub fn evaluate_id_filter(
        &self,
        filter: &Vec<(QueryExpr<Variable>, ExprIR<Variable>)>,
        vars: &Env,
    ) -> Result<Option<RoaringTreemap>, String> {
        let mut min = 0u64;
        let mut max = self.g.borrow().max_node_id();
        for (expr, op) in filter {
            let id = match self.run_expr(expr, expr.root().idx(), vars, None)? {
                Value::Int(id) => id as u64,
                _ => {
                    return Err(String::from("Node ID must be an integer"));
                }
            };
            match op {
                ExprIR::Eq => {
                    if id < min || id > max {
                        return Ok(None);
                    }
                    min = id;
                    max = id;
                }
                ExprIR::Gt => {
                    if id >= max {
                        return Ok(None);
                    }
                    min = std::cmp::max(min, id + 1);
                }
                ExprIR::Ge => {
                    if id > max {
                        return Ok(None);
                    }
                    min = std::cmp::max(min, id);
                }
                ExprIR::Lt => {
                    if id <= min {
                        return Ok(None);
                    }
                    max = std::cmp::min(max, id - 1);
                }
                ExprIR::Le => {
                    if id < min {
                        return Ok(None);
                    }
                    max = std::cmp::min(max, id);
                }
                _ => {
                    unreachable!()
                }
            }
        }
        let mut result = RoaringTreemap::new();
        result.insert_range(min..=max);
        Ok(Some(result))
    }

    pub fn get_node_attribute(
        &self,
        id: NodeId,
        attr: &Arc<String>,
    ) -> Option<Value> {
        if let Some(dn) = self.deleted_nodes.borrow().get(&id) {
            if let Some(value) = dn.attrs.get(attr) {
                return Some(value.clone());
            }
            return None;
        }
        if let Some(value) = self.pending.borrow().get_node_attribute(id, attr) {
            return Some(value.clone());
        }
        self.g.borrow().get_node_attribute(id, attr)
    }

    pub fn get_relationship_attribute(
        &self,
        id: RelationshipId,
        attr: &Arc<String>,
    ) -> Option<Value> {
        if let Some(dn) = self.deleted_relationships.borrow().get(&id) {
            if let Some(value) = dn.attrs.get(attr) {
                return Some(value.clone());
            }
            return None;
        }
        if let Some(value) = self.pending.borrow().get_relationship_attribute(id, attr) {
            return Some(value.clone());
        }
        self.g.borrow().get_relationship_attribute(id, attr)
    }

    /// Checks inline property predicates on a node.
    /// Returns `true` if all properties match (or if there are no predicates).
    fn check_inline_node_attrs(
        &self,
        attrs: &QueryExpr<Variable>,
        node_id: NodeId,
        env: &Env,
        agg_group_key: Option<u64>,
    ) -> Result<bool, String> {
        if attrs.root().children().next().is_none() {
            return Ok(true);
        }
        let filter = self.run_expr(attrs, attrs.root().idx(), env, agg_group_key)?;
        if let Value::Map(filter_attrs) = &filter
            && !filter_attrs.is_empty()
        {
            for (attr, avalue) in filter_attrs.iter() {
                match self.get_node_attribute(node_id, attr) {
                    Some(pvalue) if *avalue == pvalue => {}
                    _ => return Ok(false),
                }
            }
        }
        Ok(true)
    }

    /// Checks inline property predicates on a relationship.
    /// Returns `true` if all properties match (or if there are no predicates).
    fn check_inline_rel_attrs(
        &self,
        attrs: &QueryExpr<Variable>,
        rel_id: RelationshipId,
        env: &Env,
        agg_group_key: Option<u64>,
    ) -> Result<bool, String> {
        if attrs.root().children().next().is_none() {
            return Ok(true);
        }
        let filter = self.run_expr(attrs, attrs.root().idx(), env, agg_group_key)?;
        if let Value::Map(filter_attrs) = &filter
            && !filter_attrs.is_empty()
        {
            for (attr, avalue) in filter_attrs.iter() {
                match self.get_relationship_attribute(rel_id, attr) {
                    Some(pvalue) if *avalue == pvalue => {}
                    _ => return Ok(false),
                }
            }
        }
        Ok(true)
    }

    pub fn get_node_labels(
        &self,
        id: NodeId,
    ) -> OrderSet<Arc<String>> {
        if let Some(dn) = self.deleted_nodes.borrow().get(&id) {
            return dn
                .labels
                .iter()
                .map(|l| self.g.borrow().get_label_by_id(*l))
                .collect();
        }
        let mut labels = self
            .g
            .borrow()
            .get_node_label_ids(id)
            .collect::<OrderSet<_>>();
        self.pending.borrow().update_node_labels(id, &mut labels);

        labels
            .iter()
            .map(|l| self.g.borrow().get_label_by_id(*l))
            .collect()
    }

    fn eval_map_projection(
        &self,
        ir: &DynTree<ExprIR<Variable>>,
        idx: NodeIdx<Dyn<ExprIR<Variable>>>,
        env: &Env,
        agg_group_key: Option<u64>,
    ) -> Result<Value, String> {
        let node = ir.node(idx);
        // child 0 is the base expression
        let base = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;

        if matches!(base, Value::Null) {
            return Ok(Value::Null);
        }

        // Validate base type
        if !matches!(
            &base,
            Value::Node(_) | Value::Relationship(_) | Value::Map(_)
        ) {
            return Err("Encountered unhandled type evaluating map projection".to_string());
        }

        let mut result = OrderMap::default();

        // children 1..N are projection items
        for i in 1..node.num_children() {
            let item = node.child(i);
            match item.data() {
                ExprIR::MapProjection => {
                    // .* - include all properties from base
                    match &base {
                        Value::Node(id) => {
                            for (k, v) in self.get_node_attrs(*id) {
                                result.insert(k, v);
                            }
                        }
                        Value::Relationship(rel) => {
                            for (k, v) in self.get_relationship_attrs(rel.0) {
                                result.insert(k, v);
                            }
                        }
                        Value::Map(map) => {
                            for (k, v) in map.iter() {
                                result.insert(k.clone(), v.clone());
                            }
                        }
                        _ => {
                            return Err(
                                "Encountered unhandled type evaluating map projection".to_string()
                            );
                        }
                    }
                }
                ExprIR::Property(prop_name) => {
                    // .prop - property shorthand
                    let value = match &base {
                        Value::Node(id) => self
                            .get_node_attribute(*id, prop_name)
                            .unwrap_or(Value::Null),
                        Value::Relationship(rel) => self
                            .get_relationship_attribute(rel.0, prop_name)
                            .unwrap_or(Value::Null),
                        Value::Map(map) => map.get(prop_name).cloned().unwrap_or(Value::Null),
                        _ => {
                            return Err(
                                "Encountered unhandled type evaluating map projection".to_string()
                            );
                        }
                    };
                    result.insert(prop_name.clone(), value);
                }
                ExprIR::String(_) => {
                    // key: expr - named projection
                    let key = if let ExprIR::String(k) = item.data() {
                        k.clone()
                    } else {
                        unreachable!();
                    };
                    let value = self.run_expr(ir, item.child(0).idx(), env, agg_group_key)?;
                    result.insert(key, value);
                }
                _ => {
                    return Err("Encountered unhandled type evaluating map projection".to_string());
                }
            }
        }

        Ok(Value::Map(Arc::new(result)))
    }

    pub fn get_node_attrs(
        &self,
        id: NodeId,
    ) -> impl Iterator<Item = (Arc<String>, Value)> {
        if let Some(dn) = self.deleted_nodes.borrow().get(&id) {
            let attrs: OrderMap<Arc<String>, Value> = dn
                .attrs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            return attrs.into_iter();
        }
        let mut actual = self.g.borrow().get_node_all_attrs(id).collect();
        self.pending.borrow().update_node_attrs(id, &mut actual);
        actual.into_iter()
    }

    pub fn get_relationship_attrs(
        &self,
        id: RelationshipId,
    ) -> impl Iterator<Item = (Arc<String>, Value)> {
        if let Some(dr) = self.deleted_relationships.borrow().get(&id) {
            let attrs: OrderMap<Arc<String>, Value> = dr
                .attrs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            return attrs.into_iter();
        }
        let mut actual = self.g.borrow().get_relationship_all_attrs(id).collect();
        self.pending
            .borrow()
            .update_relationship_attrs(id, &mut actual);
        actual.into_iter()
    }

    pub fn get_relationship_type(
        &self,
        id: RelationshipId,
    ) -> Option<Arc<String>> {
        if let Some(dr) = self.deleted_relationships.borrow().get(&id) {
            return Some(self.g.borrow().get_type(dr.type_id).unwrap());
        }
        if let Some(type_name) = self.pending.borrow().get_relationship_type(id) {
            return Some(type_name);
        }
        self.g
            .borrow()
            .get_type(self.g.borrow().get_relationship_type_id(id))
    }
}

fn map_to_index_options(
    index_type: &IndexType,
    kv_map: &OrderMap<Arc<String>, Value>,
) -> Result<Option<IndexOptions>, String> {
    let get = |key: &str| -> Option<&Value> {
        kv_map
            .iter()
            .find_map(|(k, v)| if k.as_str() == key { Some(v) } else { None })
    };
    match index_type {
        IndexType::Fulltext => {
            let weight = match get("weight") {
                Some(Value::Float(f)) => Some(*f),
                Some(Value::Int(i)) => Some(*i as f64),
                None => None,
                _ => return Err("Weight must be numeric".into()),
            };
            let nostem = match get("nostem") {
                Some(Value::Bool(b)) => Some(*b),
                None => None,
                _ => return Err("Nostem must be bool".into()),
            };
            let phonetic = match get("phonetic") {
                Some(Value::Bool(b)) => Some(*b),
                None => None,
                _ => return Err("Phonetic must be bool".into()),
            };
            let language = match get("language") {
                Some(Value::String(s) | Value::InternedString(s)) => Some(s.clone()),
                None => None,
                _ => return Err("Language must be string".into()),
            };
            let stopwords = match get("stopwords") {
                Some(Value::List(list)) => {
                    let mut words = Vec::with_capacity(list.len());
                    for v in list.iter() {
                        match v {
                            Value::String(s) | Value::InternedString(s) => words.push(s.clone()),
                            _ => {
                                return Err("Stopwords must be an array of strings".into());
                            }
                        }
                    }
                    Some(words)
                }
                None => None,
                _ => return Err("Stopwords must be array".into()),
            };
            let options = IndexOptions::Text(TextIndexOptions {
                weight,
                nostem,
                phonetic,
                language,
                stopwords,
            });
            Ok(Some(options))
        }
        _ => Ok(None),
    }
}

pub fn evaluate_param(expr: &DynNode<ExprIR<Arc<String>>>) -> Result<Value, String> {
    match expr.data() {
        ExprIR::Null => Ok(Value::Null),
        ExprIR::Bool(x) => Ok(Value::Bool(*x)),
        ExprIR::Integer(x) => Ok(Value::Int(*x)),
        ExprIR::Float(x) => Ok(Value::Float(*x)),
        ExprIR::String(x) => Ok(Value::String(x.clone())),
        ExprIR::List => Ok(Value::List(Arc::new(
            expr.children()
                .map(|c| evaluate_param(&c))
                .collect::<Result<ThinVec<_>, _>>()?,
        ))),
        ExprIR::Map => Ok(Value::Map(Arc::new(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::String(key) => {
                        Ok::<_, String>((key.clone(), evaluate_param(&ir.child(0))?))
                    }
                    _ => todo!(),
                })
                .collect::<Result<OrderMap<_, _>, _>>()?,
        ))),
        ExprIR::Negate => {
            let v = evaluate_param(&expr.child(0))?;
            match v {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Ok(Value::Null),
            }
        }
        _ => Err(String::from("Invalid parameter expression.")),
    }
}

fn get_elements(
    arr: &Value,
    start: &Value,
    end: &Value,
) -> Result<Value, String> {
    match (arr, start, end) {
        (Value::List(values), Value::Int(start), Value::Int(end)) => {
            let mut start = *start;
            let mut end = *end;
            if start < 0 {
                start = (values.len() as i64 + start).max(0);
            }
            if end < 0 {
                end = (values.len() as i64 + end).max(0);
            } else {
                end = end.min(values.len() as i64);
            }
            if start > end {
                return Ok(Value::List(Arc::new(thin_vec![])));
            }
            Ok(Value::List(Arc::new(
                values[start as usize..end as usize]
                    .iter()
                    .cloned()
                    .collect::<ThinVec<_>>(),
            )))
        }
        (_, Value::Null, _) | (_, _, Value::Null) => Ok(Value::Null),
        _ => Err(String::from("Invalid array range parameters.")),
    }
}

fn list_contains(
    list: &Value,
    value: Value,
) -> Result<Value, String> {
    match list {
        Value::List(l) => Ok(Contains::contains(l.as_ref(), value)),
        Value::Null => Ok(Value::Null),
        _ => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            list.name()
        )),
    }
}

// the semantic of Eq [1, 2, 3] is: 1 EQ 2 AND 2 EQ 3
fn all_equals<I>(mut iter: I) -> Result<Value, String>
where
    I: Iterator<Item = Result<Value, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.compare_value(&next) {
                (_, DisjointOrNull::ComparedNull) => return Ok(Value::Null),
                (_, DisjointOrNull::NaN | DisjointOrNull::Disjoint) => {
                    return Ok(Value::Bool(false));
                }
                (Ordering::Equal, _) => {}
                _ => return Ok(Value::Bool(false)),
            }
        }
        Ok(Value::Bool(true))
    } else {
        Err(String::from("Eq operator requires at least two arguments"))
    }
}

fn all_not_equals<I>(mut iter: I) -> Result<Value, String>
where
    I: Iterator<Item = Result<Value, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(Value::Null),
                Some(Ordering::Less | Ordering::Greater) => {}
                Some(Ordering::Equal) => return Ok(Value::Bool(false)),
            }
        }
        Ok(Value::Bool(true))
    } else {
        Err(String::from("Eq operator requires at least two arguments"))
    }
}

#[inline]
const fn logical_xor(
    a: bool,
    b: bool,
) -> bool {
    (a && !b) || (!a && b)
}
