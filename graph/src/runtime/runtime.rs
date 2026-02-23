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
    ast::{
        ExprIR, QuantifierType, QueryExpr, QueryGraph, QueryNode, QueryRelationship, SetItem,
        Variable,
    },
    graph::graph::{Graph, LabelId, NodeId, RelationshipId},
    indexer::IndexQuery,
    planner::IR,
    runtime::{
        functions::{FnType, apply_pow},
        iter::{Aggregate, CondInspectIter, LazyReplace, TryFlatMap, TryMap},
        ordermap::OrderMap,
        orderset::OrderSet,
        pending::Pending,
        value::{
            CompareValue, Contains, DeletedNode, DeletedRelationship, DisjointOrNull, Env, Value,
            ValuesDeduper,
        },
    },
};
use atomic_refcell::AtomicRefCell;
use once_cell::{sync::OnceCell, unsync::Lazy};
use orx_tree::{Bfs, Dyn, DynNode, DynTree, MemoryPolicy, NodeIdx, NodeRef};
use reqwest::blocking::get;
use roaring::RoaringTreemap;
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    iter::{empty, once},
    path::Path,
    sync::Arc,
    time::Instant,
};
use thin_vec::{ThinVec, thin_vec};

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
    parameters: HashMap<String, Value>,
    /// Graph being queried (shared, thread-safe reference)
    pub g: Arc<AtomicRefCell<Graph>>,
    /// Whether this is a write query
    write: bool,
    /// Batched mutations (lazy-initialized)
    pending: Lazy<RefCell<Pending>>,
    /// Execution statistics
    stats: RefCell<QueryStatistics>,
    /// Query execution plan tree
    plan: Arc<DynTree<IR>>,
    /// Deduplication state for DISTINCT operations
    value_dedupers: RefCell<HashMap<String, ValuesDeduper>>,
    /// Variables to return in query results
    pub return_names: Vec<Variable>,
    /// Debug mode: record operator execution
    inspect: bool,
    /// Debug records of operator execution
    pub record: RefCell<Vec<(NodeIdx<Dyn<IR>>, Result<Env, String>)>>,
    /// Folder for LOAD CSV operations
    import_folder: String,
    /// Cache of deleted nodes for result consistency
    pub deleted_nodes: RefCell<HashMap<NodeId, DeletedNode>>,
    /// Cache of deleted relationships for result consistency
    pub deleted_relationships: RefCell<HashMap<RelationshipId, DeletedRelationship>>,
    /// Cached environments for CALL {} IN TRANSACTIONS
    argument_envs: RefCell<HashMap<NodeIdx<Dyn<IR>>, Env>>,
    /// Cache for MERGE pattern matching
    merge_pattern_cache: RefCell<HashMap<u64, Env>>,
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
                | IR::Empty
                | IR::Argument
                | IR::Set(_)
                | IR::Remove(_)
                | IR::Filter(_)
                | IR::CartesianProduct
                | IR::Sort(_)
                | IR::Skip(_)
                | IR::Limit(_)
                | IR::Distinct
                | IR::Commit
                | IR::CreateIndex { .. }
                | IR::DropIndex { .. } => {}
                IR::NodeByLabelScan(node)
                | IR::NodeByIndexScan { node, .. }
                | IR::NodeByLabelAndIdScan { node, .. }
                | IR::NodeByIdSeek { node, .. } => {
                    vars.push(node.alias.clone());
                }
                IR::CondTraverse(query_relationship) => {
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
            IR::Sort(_) | IR::Skip(_) | IR::Limit(_) | IR::Distinct => {
                self.child(0).get_return_names()
            }
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

impl<'a> Runtime {
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
            argument_envs: RefCell::new(HashMap::new()),
            merge_pattern_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn query(&mut self) -> Result<ResultSummary, String> {
        let labels_count = self.g.borrow().labels_count();
        let start = Instant::now();
        let idx = self.plan.root().idx();
        let mut result = vec![];
        for env in self.run(idx)? {
            let env = env?;
            result.push(env);
        }
        let run_duration = start.elapsed();

        self.stats.borrow_mut().labels_added = self.g.borrow().labels_count() - labels_count;
        self.stats.borrow_mut().execution_time = run_duration.as_secs_f64() * 1000.0;
        Ok(ResultSummary {
            stats: self.stats.take(),
            result,
        })
    }

    fn set_agg_expr_zero(
        ir: &DynNode<ExprIR<Variable>>,
        env: &mut Env,
    ) {
        match ir.data() {
            ExprIR::FuncInvocation(func) if func.is_aggregate() => {
                if let FnType::Aggregation(zero, _) = &func.fn_type {
                    let ExprIR::Variable(key) = ir.child(ir.num_children() - 1).data() else {
                        unreachable!();
                    };
                    env.insert(key, zero.clone());
                }
            }
            _ => {
                for child in ir.children() {
                    Self::set_agg_expr_zero(&child, env);
                }
            }
        }
    }

    fn run_agg_expr(
        &self,
        ir: &DynTree<ExprIR<Variable>>,
        idx: NodeIdx<Dyn<ExprIR<Variable>>>,
        curr: &mut Env,
        acc: &mut Env,
        agg_group_key: u64,
    ) -> Result<(), String> {
        match ir.node(idx).data() {
            ExprIR::FuncInvocation(func) if func.is_aggregate() => {
                let num_children = ir.node(idx).num_children();

                // The last child is always the accumulator key variable
                // Minimum valid structure: [arg, accumulator_key]
                if num_children < 2 {
                    return Err(String::from(
                        "Aggregation function must have at least one argument",
                    ));
                }

                let ExprIR::Variable(key) = ir.node(idx).child(num_children - 1).data() else {
                    return Err(String::from(
                        "Aggregation function must end with a variable",
                    ));
                };

                // Take ownership of accumulator (moves value, no clone)
                let prev_value = acc.take(key).unwrap_or(Value::Null);

                // PHASE 1:  Evaluate all arguments
                let arg_results: Result<ThinVec<Value>, String> = (0..num_children - 1)
                    .map(|i| {
                        let child = ir.node(idx).child(i);
                        self.run_expr(ir, child.idx(), curr, Some(agg_group_key))
                    })
                    .collect();

                let mut args = match arg_results {
                    Ok(a) => a,
                    Err(e) => {
                        // Restore accumulator before returning error
                        acc.insert(key, prev_value);
                        return Err(e);
                    }
                };

                // PHASE 2: Handle DISTINCT unpacking (if present)
                if num_children == 2 && matches!(ir.node(idx).child(0).data(), ExprIR::Distinct) {
                    let arg = args.remove(0);
                    if let Value::List(values) = arg {
                        args = values;
                    } else {
                        // Restore accumulator before returning error
                        acc.insert(key, prev_value);
                        return Err(String::from("DISTINCT should return a list"));
                    }
                }

                // PHASE 3: Validate argument types
                if let Err(e) = func.validate_args_type(&args) {
                    // Restore accumulator before returning error
                    acc.insert(key, prev_value);
                    return Err(e);
                }

                // PHASE 4: Validate domain constraints
                // This catches things like percentile out of [0.0, 1.0]
                if let Err(e) = func.validate_args_domain(&args) {
                    // Restore accumulator before returning error
                    acc.insert(key, prev_value);
                    return Err(e);
                }

                // PHASE 5: Push the accumulator as the last argument (moved, not cloned!)
                args.push(prev_value);

                // PHASE 6: Call the aggregation function
                // At this point, all validation is complete
                let new_value = (func.func)(self, args)?;

                // Store result back in accumulator
                acc.insert(key, new_value);
            }
            _ => {
                for child in ir.node(idx).children() {
                    self.run_agg_expr(ir, child.idx(), curr, acc, agg_group_key)?;
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn run_expr(
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
                    .ok_or_else(|| format!("Variable {} not found", x.as_str()));
            }

            ExprIR::Parameter(x) => {
                return self.parameters.get(x).map_or_else(
                    || Err(format!("Parameter {x} not found")),
                    |v| Ok(v.clone()),
                );
            }
            ExprIR::Map => {
                return Ok(Value::Map(
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
                ));
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
                        .ok_or_else(|| format!("Variable {} not found", x.as_str()))?,
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
                        res.push(Value::List(list));
                    } else if node.num_children() > 0 {
                        stack.push((idx, true));
                        for idx in node.children().map(|c| c.idx()) {
                            stack.push((idx, false));
                        }
                    } else {
                        res.push(Value::List(thin_vec![]));
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
                        (Value::Node(id), Value::String(key)) => {
                            res.push(self.get_node_attribute(id, &key).unwrap_or(Value::Null));
                        }
                        (Value::Relationship(rel), Value::String(key)) => {
                            res.push(
                                self.get_relationship_attribute(rel.0, &key)
                                    .unwrap_or(Value::Null),
                            );
                        }
                        (Value::Map(map), Value::String(key)) => {
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
                        res.push(Value::List(thin_vec![Value::Null]));
                    } else {
                        res.push(Value::List(values));
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
                        let acc = env.get(key).unwrap();

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
                        let arg = &args[0];
                        if let Value::List(values) = arg {
                            let mut values = values.clone();
                            args.remove(0);
                            values.append(&mut args);
                            args = values;
                        } else {
                            unreachable!();
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
                ExprIR::Map => res.push(Value::Map(
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
                )),
                ExprIR::Quantifier(quantifier, var) => {
                    let list = self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?;
                    match list {
                        Value::List(values) => {
                            let mut env = env.clone();
                            let mut t = 0;
                            let mut f = 0;
                            let mut n = 0;
                            for value in values {
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

                    res.push(Value::List(acc));
                }
                ExprIR::Paren => {
                    res.push(self.run_expr(ir, node.child(0).idx(), env, agg_group_key)?);
                }
            }
        }
        debug_assert_eq!(res.len(), 1);
        Ok(res.pop().unwrap())
    }

    fn run_iter_expr(
        &self,
        ir: &DynTree<ExprIR<Variable>>,
        idx: NodeIdx<Dyn<ExprIR<Variable>>>,
        env: &Env,
    ) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        match ir.node(idx).data() {
            ExprIR::FuncInvocation(func) if func.name == "range" => {
                let start = self.run_expr(ir, ir.node(idx).child(0).idx(), env, None)?;
                let end = self.run_expr(ir, ir.node(idx).child(1).idx(), env, None)?;
                let step = ir.node(idx).get_child(2).map_or_else(
                    || Ok(Value::Int(1)),
                    |c| self.run_expr(ir, c.idx(), env, None),
                )?;
                func.validate_args_type(&[start.clone(), end.clone(), step.clone()])?;
                match (start, end, step) {
                    (Value::Int(start), Value::Int(end), Value::Int(step)) => {
                        if step == 0 {
                            return Err(String::from(
                                "ArgumentError: step argument to range() can't be 0",
                            ));
                        }
                        if (start > end && step > 0) || (start < end && step < 0) {
                            return Ok(Box::new(empty()));
                        }
                        let length = (end - start) / step + 1;
                        #[allow(clippy::cast_lossless)]
                        if length > u32::MAX as i64 {
                            return Err(String::from("Range too large"));
                        }

                        if step > 0 {
                            return Ok(Box::new(
                                (start..=end).step_by(step as usize).map(Value::Int),
                            ));
                        }
                        Ok(Box::new(
                            (end..=start)
                                .rev()
                                .step_by((-step) as usize)
                                .map(Value::Int),
                        ))
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            _ => {
                let res = self.run_expr(ir, idx, env, None)?;
                match res {
                    Value::List(arr) => Ok(Box::new(arr.into_iter())),
                    Value::Null => Ok(Box::new(empty())),
                    _ => Ok(Box::new(once(res))),
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
    fn run(
        &self,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + '_>, String> {
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
                Box::new(once(Ok(Env::default())))
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
        } else if let Some(child_idx) = child0_idx {
            self.run(child_idx)?
        } else {
            Box::new(once(Ok(Env::default())))
        };
        match self.plan.node(idx).data() {
            IR::Empty => Ok(Box::new(empty())),
            IR::Argument => {
                let env = self.argument_envs.borrow_mut().remove(&idx).unwrap();

                Ok(Box::new(once(Ok(env))))
            }
            IR::Optional(vars) => {
                let optional_child_idx = if self.plan.node(idx).num_children() == 1 {
                    self.plan.node(idx).child(0).idx()
                } else {
                    self.plan.node(idx).child(1).idx()
                };
                Ok(iter
                    .try_flat_map(move |mut env| {
                        for v in vars {
                            env.insert(v, Value::Null);
                        }
                        Ok(self
                            .run(optional_child_idx)?
                            .lazy_replace(move || once(Ok(env))))
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::ProcedureCall(func, trees, name_outputs) => {
                let args = trees
                    .iter()
                    .map(|ir| self.run_expr(ir, ir.root().idx(), &Env::default(), None))
                    .collect::<Result<ThinVec<_>, _>>()?;
                if !self.write && func.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                let res = (func.func)(self, args)?;
                match res {
                    Value::List(arr) => Ok(arr
                        .into_iter()
                        .map(move |v| {
                            let mut env = Env::default();
                            if let Value::Map(map) = v {
                                for output in name_outputs {
                                    env.insert(
                                        output,
                                        map.get(output.name.as_ref().unwrap()).unwrap().clone(),
                                    );
                                }
                            }
                            Ok(env)
                        })
                        .cond_inspect(self.inspect, move |res| {
                            self.record.borrow_mut().push((idx, res.clone()));
                        })),
                    _ => unreachable!(),
                }
            }
            IR::Unwind(list, name) => Ok(iter
                .try_flat_map(move |vars| {
                    let value = self.run_iter_expr(list, list.root().idx(), &vars)?;
                    Ok(value.map(move |v| {
                        let mut vars = vars.clone();
                        vars.insert(name, v);
                        Ok(vars)
                    }))
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::Create(pattern) => {
                let parent_commit = if let Some(parent) = self.plan.node(idx).parent()
                    && matches!(parent.data(), IR::Commit)
                    && parent.parent().is_none()
                {
                    true
                } else {
                    false
                };

                let resolved_pattern = self.resolve_pattern(pattern);
                self.pending
                    .borrow_mut()
                    .resize(self.g.borrow().node_cap(), self.g.borrow().labels_count());
                Ok(iter
                    .try_flat_map(move |mut vars| {
                        self.create(&resolved_pattern, &mut vars)?;

                        if parent_commit {
                            return Ok(
                                Box::new(empty()) as Box<dyn Iterator<Item = Result<Env, String>>>
                            );
                        }

                        Ok(Box::new(once(Ok(vars)))
                            as Box<dyn Iterator<Item = Result<Env, String>>>)
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Merge(pattern, on_create_set_items, on_match_set_items) => {
                let merge_child_idx = if self.plan.node(idx).num_children() == 1 {
                    self.plan.node(idx).child(0).idx()
                } else {
                    self.plan.node(idx).child(1).idx()
                };

                // Find all Argument nodes in the child tree
                let argument_indices: Vec<NodeIdx<Dyn<IR>>> = self
                    .plan
                    .node(merge_child_idx)
                    .indices::<Bfs>()
                    .filter(|&i| matches!(self.plan.node(i).data(), IR::Argument))
                    .collect();

                let resolved_pattern = self.resolve_pattern(pattern);
                let resolved_on_match_set_items = OnceCell::new();
                let resolved_on_create_set_items = OnceCell::new();
                self.pending
                    .borrow_mut()
                    .resize(self.g.borrow().node_cap(), self.g.borrow().labels_count());

                Ok(iter
                    .try_flat_map(move |vars| {
                        let cvars = vars.clone();

                        // Check if all nodes in the pattern are already bound
                        // If so, MERGE should only check existence (return one result)
                        // If not, MERGE may need to return all matching nodes
                        let all_nodes_bound = resolved_pattern
                            .nodes()
                            .iter()
                            .all(|node| vars.get(&node.alias).is_some());

                        // Set the environment for all Argument nodes in this subtree
                        for arg_idx in &argument_indices {
                            self.argument_envs
                                .borrow_mut()
                                .insert(*arg_idx, vars.clone());
                        }

                        let resolved_pattern = resolved_pattern.clone();
                        let resolved_on_match_set_items = resolved_on_match_set_items.clone();
                        let resolved_on_create_set_items = resolved_on_create_set_items.clone();
                        let resolved_on_match_set_items_for_lazy =
                            resolved_on_match_set_items.clone();

                        // When all nodes are bound, we only need to check if the pattern exists
                        // (take 1 match), otherwise we return all matches
                        let child_iter = self.run(merge_child_idx)?;
                        let child_iter: Box<dyn Iterator<Item = Result<Env, String>> + '_> =
                            if all_nodes_bound {
                                Box::new(child_iter.take(1))
                            } else {
                                Box::new(child_iter)
                            };

                        let iter = child_iter
                            .try_map(move |v| {
                                let mut vars = vars.clone();
                                vars.merge(v);
                                self.set(
                                    resolved_on_match_set_items.get_or_init(|| {
                                        let res = self.resolve_set_items(on_match_set_items);
                                        self.pending.borrow_mut().resize(
                                            self.g.borrow().node_cap(),
                                            self.g.borrow().labels_count(),
                                        );
                                        res
                                    }),
                                    &vars,
                                )?;
                                Ok(vars)
                            })
                            .lazy_replace(move || {
                                let mut vars = cvars.clone();

                                // Compute hash for the pattern to check if it's already been created
                                match self.compute_merge_pattern_hash(&resolved_pattern, &vars) {
                                    Ok(pattern_hash) => {
                                        let merge_cache = self.merge_pattern_cache.borrow_mut();

                                        // Check if this pattern was already created in this query
                                        if let Some(cached_vars) = merge_cache.get(&pattern_hash) {
                                            // Pattern already created, apply ON MATCH and return cached vars
                                            let mut result_vars = vars.clone();
                                            result_vars.merge(cached_vars.clone());
                                            drop(merge_cache);

                                            match self.set(
                                                resolved_on_match_set_items_for_lazy.get_or_init(
                                                    || {
                                                        let res = self
                                                            .resolve_set_items(on_match_set_items);
                                                        self.pending.borrow_mut().resize(
                                                            self.g.borrow().node_cap(),
                                                            self.g.borrow().labels_count(),
                                                        );
                                                        res
                                                    },
                                                ),
                                                &result_vars,
                                            ) {
                                                Ok(()) => once(Ok(result_vars)),
                                                Err(e) => once(Err(e)),
                                            }
                                        } else {
                                            // Pattern not yet created, create it
                                            drop(merge_cache);

                                            match self.create(&resolved_pattern, &mut vars) {
                                                Ok(()) => {
                                                    // Cache the created pattern
                                                    self.merge_pattern_cache
                                                        .borrow_mut()
                                                        .insert(pattern_hash, vars.clone());

                                                    match self.set(
                                                        resolved_on_create_set_items.get_or_init(
                                                            || {
                                                                let res = self.resolve_set_items(
                                                                    on_create_set_items,
                                                                );
                                                                self.pending.borrow_mut().resize(
                                                                    self.g.borrow().node_cap(),
                                                                    self.g.borrow().labels_count(),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                        &vars,
                                                    ) {
                                                        Ok(()) => once(Ok(vars)),
                                                        Err(e) => once(Err(e)),
                                                    }
                                                }
                                                Err(e) => once(Err(e)),
                                            }
                                        }
                                    }
                                    Err(e) => once(Err(e)),
                                }
                            });
                        Ok(iter)
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Delete(trees, _) => Ok(iter
                .try_map(move |vars| {
                    self.delete(trees, &vars)?;
                    Ok(vars)
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::Set(items) => {
                let resolved_items = self.resolve_set_items(items);
                self.pending
                    .borrow_mut()
                    .resize(self.g.borrow().node_cap(), self.g.borrow().labels_count());
                Ok(iter
                    .try_map(move |vars| {
                        self.set(&resolved_items, &vars)?;
                        Ok(vars)
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Remove(items) => {
                self.pending
                    .borrow_mut()
                    .resize(self.g.borrow().node_cap(), self.g.borrow().labels_count());
                Ok(iter
                    .try_map(move |vars| {
                        self.remove(items, &vars)?;
                        Ok(vars)
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::NodeByLabelScan(node) => Ok(iter
                .try_flat_map(move |vars| self.node_by_label_scan(node, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::NodeByLabelAndIdScan { node, filter } => Ok(iter
                .try_flat_map(move |vars| self.node_by_label_and_id_scan(node, filter, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::NodeByIdSeek { node, filter } => Ok(iter
                .try_flat_map(move |vars| self.node_by_id_seek(node, filter, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::NodeByIndexScan { node, index, query } => Ok(iter
                .try_flat_map(move |vars| self.node_by_index_scan(node, index, query, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::CondTraverse(relationship_pattern) => Ok(iter
                .try_flat_map(move |vars| self.relationship_scan(relationship_pattern, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::ExpandInto(relationship_pattern) => Ok(iter
                .try_flat_map(move |vars| self.expand_into(relationship_pattern, vars))
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::PathBuilder(paths) => Ok(iter
                .try_map(move |mut vars| {
                    let mut paths = paths.clone();
                    for path in &mut paths {
                        let p = path
                            .vars
                            .iter()
                            .map(|v| {
                                vars.get(v).map_or_else(
                                    || Err(format!("Variable {} not found", v.as_str())),
                                    Ok,
                                )
                            })
                            .collect::<Result<_, String>>()?;
                        vars.insert(&path.var, Value::Path(p));
                    }
                    Ok(vars)
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::Filter(tree) => Ok(iter
                .filter_map(move |vars| match vars {
                    Ok(vars) => match self.run_expr(tree, tree.root().idx(), &vars, None) {
                        Ok(Value::Bool(true)) => Some(Ok(vars)),
                        Ok(Value::Bool(false) | Value::Null) => None,
                        Err(e) => Some(Err(e)),
                        _ => Some(Err(String::from("Expected boolean predicate."))),
                    },
                    Err(e) => Some(Err(e)),
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::CartesianProduct => {
                let mut iter = iter;
                let node = self.plan.node(idx);
                for child in node.children().skip(1) {
                    let idx = child.idx();
                    iter = Box::new(iter.try_flat_map(move |vars1| {
                        Ok(self.run(idx)?.try_map(move |vars2| {
                            let mut vars = vars1.clone();
                            vars.merge(vars2);
                            Ok(vars)
                        }))
                    }));
                }
                Ok(iter.cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                }))
            }
            IR::LoadCsv {
                file_path,
                headers,
                delimiter,
                var,
            } => Ok(iter
                .try_flat_map(move |vars| {
                    let path = self.run_expr(file_path, file_path.root().idx(), &vars, None)?;
                    let Value::String(delimiter) =
                        self.run_expr(delimiter, delimiter.root().idx(), &vars, None)?
                    else {
                        return Err(String::from("Delimiter must be a string"));
                    };
                    if delimiter.len() != 1 {
                        return Err(String::from("Delimiter must be a single character"));
                    }
                    let Value::String(path) = path else {
                        return Err(String::from("File path must be a string"));
                    };
                    let path = if let Some(path) = path.strip_prefix("file://") {
                        let path = self.import_folder.clone() + path;
                        let import_folder =
                            Path::new(&self.import_folder).canonicalize().map_err(|e| {
                                format!(
                                    "Failed to canonicalize import folder path '{}': {e}",
                                    self.import_folder
                                )
                            })?;
                        let cpath = Path::new(&path).canonicalize().map_err(|e| {
                            format!("Failed to canonicalize file path '{path}': {e}")
                        })?;
                        if !cpath.starts_with(&import_folder) {
                            return Err(format!(
                                "File path '{path}' is not within the import folder '{}'",
                                self.import_folder
                            ));
                        }
                        path
                    } else if path.starts_with("https://") {
                        String::from(path.as_str())
                    } else {
                        return Err(String::from("File path must start with 'file://' prefix"));
                    };

                    if path.starts_with("https://") {
                        Self::load_csv_url(&path, *headers, delimiter, var, &vars)
                    } else {
                        Self::load_csv_file(&path, *headers, delimiter, var, &vars)
                    }
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::Sort(trees) => {
                let mut items = iter
                    .try_map(|env| {
                        Ok((
                            env.clone(),
                            trees
                                .iter()
                                .map(|(tree, desc)| {
                                    Ok((self.run_expr(tree, tree.root().idx(), &env, None)?, desc))
                                })
                                .collect::<Result<Vec<_>, String>>()?,
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                items.sort_by(|(_, a), (_, b)| {
                    a.iter()
                        .zip(b)
                        .fold(Ordering::Equal, |acc, ((a, desc), (b, _))| {
                            if acc != Ordering::Equal {
                                return acc;
                            }

                            let (ordering, _) = a.compare_value(b);
                            if **desc { ordering.reverse() } else { ordering }
                        })
                });
                Ok(items.into_iter().map(|(env, _)| Ok(env)).cond_inspect(
                    self.inspect,
                    move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    },
                ))
            }
            IR::Skip(skip) => {
                let Value::Int(skip) =
                    self.run_expr(skip, skip.root().idx(), &Env::default(), None)?
                else {
                    return Err(String::from("Skip operator requires an integer argument"));
                };
                Ok(iter
                    .skip(skip as usize)
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Limit(limit) => {
                let Value::Int(limit) =
                    self.run_expr(limit, limit.root().idx(), &Env::default(), None)?
                else {
                    return Err(String::from("Limit operator requires an integer argument"));
                };
                Ok(iter
                    .take(limit as usize)
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Aggregate(_, keys, agg) => {
                let mut cache = HashMap::new();
                let mut env = Env::default();
                for (_var, t) in agg {
                    Self::set_agg_expr_zero(&t.root(), &mut env);
                }
                // in case there are no aggregation keys the aggregator will return
                // default value for empty iterator
                if keys.is_empty() {
                    let key = Ok(Env::default());
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    let k = hasher.finish();
                    cache.insert(k, (key, Ok(env.clone())));
                }
                Ok(iter
                    .aggregate(
                        move |vars| {
                            let vars = vars.clone()?;
                            let mut return_vars = Env::default();
                            for (name, tree) in keys {
                                let value = self.run_expr(tree, tree.root().idx(), &vars, None)?;
                                return_vars.insert(name, value);
                            }
                            Ok::<Env, String>(return_vars)
                        },
                        Ok(env),
                        move |group_key, x, acc| {
                            let mut x = x?;
                            let mut acc: Env = acc?;
                            for (_, tree) in agg {
                                self.run_agg_expr(
                                    tree,
                                    tree.root().idx(),
                                    &mut x,
                                    &mut acc,
                                    group_key,
                                )?;
                            }
                            Ok(acc)
                        },
                        cache,
                    )
                    .map(move |(key, v)| {
                        let mut vars = v?;
                        vars.merge(key?);
                        for (name, tree) in agg {
                            vars.insert(name, self.run_expr(tree, tree.root().idx(), &vars, None)?);
                        }
                        Ok(vars)
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Project(trees, copy_from_parent) => Ok(iter
                .try_map(move |vars| {
                    let mut return_vars = Env::default();
                    for (old_var, new_var) in copy_from_parent {
                        if let Some(value) = vars.get(old_var) {
                            return_vars.insert(new_var, value);
                        }
                    }
                    for (name, tree) in trees {
                        let value = self.run_expr(tree, tree.root().idx(), &vars, None)?;
                        return_vars.insert(name, value);
                    }
                    Ok(return_vars)
                })
                .cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                })),
            IR::Distinct => {
                let deduper = ValuesDeduper::default();
                Ok(iter
                    .filter_map(move |item| {
                        // Propagate errors immediately
                        let vars = match item {
                            Err(e) => return Some(Err(e)),
                            Ok(vars) => vars,
                        };

                        // compute the hash of all the values in return_names
                        // by order
                        let mut hasher = DefaultHasher::new();
                        for name in &self.return_names {
                            vars.get(name)
                                .unwrap_or_else(|| {
                                    unreachable!("Variable {} not found", name.as_str())
                                })
                                .hash(&mut hasher);
                        }
                        if deduper.has_hash(hasher.finish()) {
                            None
                        } else {
                            Some(Ok(vars))
                        }
                    })
                    .cond_inspect(self.inspect, move |res| {
                        self.record.borrow_mut().push((idx, res.clone()));
                    }))
            }
            IR::Commit => {
                if !self.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                let iter = iter
                    .collect::<Result<Vec<_>, String>>()?
                    .into_iter()
                    .map(Ok);
                self.pending.borrow_mut().commit(&self.g, &self.stats)?;
                Ok(iter.cond_inspect(self.inspect, move |res| {
                    self.record.borrow_mut().push((idx, res.clone()));
                }))
            }
            IR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options: _,
            } => {
                if !self.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                self.stats.borrow_mut().indexes_created += attrs.len();
                self.g
                    .borrow_mut()
                    .create_index(index_type, entity_type, label, attrs)?;
                Ok(Box::new(empty()))
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
                self.stats.borrow_mut().indexes_dropped += attrs.len();
                self.g
                    .borrow_mut()
                    .drop_index(index_type, entity_type, label, attrs);
                Ok(Box::new(empty()))
            }
        }
    }

    fn resolve_pattern(
        &self,
        pattern: &QueryGraph<Arc<String>, Arc<String>, Variable>,
    ) -> QueryGraph<Arc<String>, LabelId, Variable> {
        let mut resolved_pattern = QueryGraph::default();
        for node in pattern.nodes() {
            resolved_pattern.add_node(Arc::new(QueryNode::new(
                node.alias.clone(),
                node.labels
                    .iter()
                    .map(|l| self.g.borrow_mut().get_label_id_mut(l.as_str()))
                    .collect(),
                node.attrs.clone(),
            )));
        }
        for rel in pattern.relationships() {
            resolved_pattern.add_relationship(Arc::new(QueryRelationship::new(
                rel.alias.clone(),
                rel.types.clone(),
                rel.attrs.clone(),
                Arc::new(QueryNode::new(
                    rel.from.alias.clone(),
                    rel.from
                        .labels
                        .iter()
                        .map(|l| self.g.borrow_mut().get_label_id_mut(l.as_str()))
                        .collect(),
                    rel.from.attrs.clone(),
                )),
                Arc::new(QueryNode::new(
                    rel.to.alias.clone(),
                    rel.to
                        .labels
                        .iter()
                        .map(|l| self.g.borrow_mut().get_label_id_mut(l.as_str()))
                        .collect(),
                    rel.to.attrs.clone(),
                )),
                rel.bidirectional,
            )));
        }
        for path in pattern.paths() {
            resolved_pattern.add_path(path.clone());
        }
        resolved_pattern
    }

    fn resolve_set_items(
        &self,
        items: &[SetItem<Arc<String>, Variable>],
    ) -> Vec<SetItem<LabelId, Variable>> {
        items
            .iter()
            .map(|item| match item {
                SetItem::Label(var, labels) => SetItem::Label(
                    var.clone(),
                    labels
                        .iter()
                        .map(|l| self.g.borrow_mut().get_label_id_mut(l.as_str()))
                        .collect(),
                ),
                SetItem::Attribute(entity, value, replace) => {
                    SetItem::Attribute(entity.clone(), value.clone(), *replace)
                }
            })
            .collect()
    }

    fn load_csv_file(
        path: &str,
        headers: bool,
        delimiter: Arc<String>,
        var: &'a Variable,
        vars: &Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(headers)
            .delimiter(delimiter.as_bytes()[0])
            .from_path(path)
            .map_err(|e| format!("Failed to read CSV file: {e}"))?;

        let vars = vars.clone();
        if headers {
            let headers = reader
                .headers()
                .map_err(|e| format!("Failed to read CSV headers: {e}"))?
                .iter()
                .map(|s| Arc::new(String::from(s)))
                .collect::<Vec<_>>();
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::Map(
                                record
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, field)| {
                                        if field.is_empty() {
                                            None
                                        } else {
                                            Some((
                                                headers.get(i).cloned().unwrap_or_else(|| {
                                                    Arc::new(format!("col_{i}"))
                                                }),
                                                Value::String(Arc::new(String::from(field))),
                                            ))
                                        }
                                    })
                                    .collect::<OrderMap<_, _>>(),
                            ),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        } else {
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::List(
                                record
                                    .iter()
                                    .map(|field| {
                                        if field.is_empty() {
                                            Value::Null
                                        } else {
                                            Value::String(Arc::new(String::from(field)))
                                        }
                                    })
                                    .collect(),
                            ),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        }
    }

    fn load_csv_url(
        path: &str,
        headers: bool,
        delimiter: Arc<String>,
        var: &'a Variable,
        vars: &Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let response = get(path).map_err(|e| format!("Failed to fetch CSV file: {e}"))?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(headers)
            .delimiter(delimiter.as_bytes()[0])
            .from_reader(response);

        let vars = vars.clone();
        if headers {
            let headers = reader
                .headers()
                .map_err(|e| format!("Failed to read CSV headers: {e}"))?
                .iter()
                .map(|s| Arc::new(String::from(s)))
                .collect::<Vec<_>>();
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::Map(
                                record
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, field)| {
                                        if field.is_empty() {
                                            None
                                        } else {
                                            Some((
                                                headers.get(i).cloned().unwrap_or_else(|| {
                                                    Arc::new(format!("col_{i}"))
                                                }),
                                                Value::String(Arc::new(String::from(field))),
                                            ))
                                        }
                                    })
                                    .collect::<OrderMap<_, _>>(),
                            ),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        } else {
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::List(
                                record
                                    .iter()
                                    .map(|field| {
                                        if field.is_empty() {
                                            Value::Null
                                        } else {
                                            Value::String(Arc::new(String::from(field)))
                                        }
                                    })
                                    .collect(),
                            ),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        }
    }

    fn remove(
        &self,
        items: &Vec<QueryExpr<Variable>>,
        vars: &Env,
    ) -> Result<(), String> {
        for item in items {
            let (entity, property, labels) = match item.root().data() {
                ExprIR::Property(property) => (
                    self.run_expr(item, item.root().child(0).idx(), vars, None)?,
                    Some(property),
                    None,
                ),
                ExprIR::FuncInvocation(func) if func.name == "hasLabels" => {
                    let labels = item
                        .root()
                        .child(1)
                        .children()
                        .filter_map(|c| match c.data() {
                            ExprIR::String(label) => Some(label.clone()),
                            _ => None,
                        })
                        .collect::<OrderSet<_>>();

                    (
                        self.run_expr(item, item.root().child(0).idx(), vars, None)?,
                        None,
                        Some(labels),
                    )
                }
                _ => {
                    unreachable!();
                }
            };
            match entity {
                Value::Node(node) => {
                    if (self.g.borrow().is_node_deleted(node)
                        && !self.pending.borrow().is_node_created(node))
                        || self.pending.borrow().is_node_deleted(node)
                    {
                        continue;
                    }
                    if let Some(property) = property {
                        self.pending.borrow_mut().set_node_attribute(
                            node,
                            property.clone(),
                            Value::Null,
                        )?;
                    }
                    if let Some(labels) = labels {
                        let mut current_labels = self
                            .g
                            .borrow()
                            .get_node_label_ids(node)
                            .collect::<OrderSet<_>>();
                        self.pending
                            .borrow()
                            .update_node_labels(node, &mut current_labels);
                        let labels = labels
                            .iter()
                            .filter_map(|l| self.g.borrow_mut().get_label_id(l.as_str()))
                            .filter(|l| current_labels.contains(l))
                            .collect();
                        self.pending.borrow_mut().remove_node_labels(node, labels);
                    }
                }
                Value::Relationship(rel) => {
                    if let Some(property) = property {
                        self.pending.borrow_mut().set_relationship_attribute(
                            rel.0,
                            property.clone(),
                            Value::Null,
                        )?;
                    }
                    if labels.is_some() {
                        return Err(String::from(
                            "Type mismatch: expected Node but was Relationship",
                        ));
                    }
                }
                Value::Null => {}
                _ => {
                    return Err(format!(
                        "Type mismatch: expected Node or Relationship but was {}",
                        entity.name()
                    ));
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn set(
        &self,
        items: &Vec<SetItem<LabelId, Variable>>,
        vars: &Env,
    ) -> Result<(), String> {
        for item in items {
            match item {
                SetItem::Attribute(entity, value, replace) => {
                    let run_expr = self.run_expr(value, value.root().idx(), vars, None)?;
                    let (entity, attr) = match entity.root().data() {
                        ExprIR::Variable(name) => {
                            let entity = vars
                                .get(name)
                                .ok_or_else(|| format!("Variable {} not found", name.as_str()))?;
                            (entity, None)
                        }
                        ExprIR::Property(property) => (
                            self.run_expr(entity, entity.root().child(0).idx(), vars, None)?,
                            Some(property),
                        ),
                        _ => {
                            unreachable!();
                        }
                    };
                    match entity {
                        Value::Node(id) => {
                            if (self.g.borrow().is_node_deleted(id)
                                && !self.pending.borrow().is_node_created(id))
                                || self.pending.borrow().is_node_deleted(id)
                            {
                                continue;
                            }
                            if let Some(attr) = attr {
                                if let Some(v) = self.get_node_attribute(id, attr)
                                    && v == run_expr
                                {
                                    continue;
                                }

                                self.pending.borrow_mut().set_node_attribute(
                                    id,
                                    attr.clone(),
                                    run_expr,
                                )?;
                            } else {
                                match run_expr {
                                    Value::Map(map) => {
                                        if *replace {
                                            self.pending.borrow_mut().clear_node_attributes(id);
                                            for key in self.g.borrow().get_node_attrs(id) {
                                                self.pending.borrow_mut().set_node_attribute(
                                                    id,
                                                    key,
                                                    Value::Null,
                                                )?;
                                            }
                                        }
                                        for (key, value) in map.iter() {
                                            self.pending.borrow_mut().set_node_attribute(
                                                id,
                                                key.clone(),
                                                value.clone(),
                                            )?;
                                        }
                                    }
                                    Value::Node(tid) => {
                                        let g = self.g.borrow();
                                        let attrs = self.get_node_attrs(tid);
                                        if *replace {
                                            for key in g.get_node_attrs(id) {
                                                self.pending.borrow_mut().set_node_attribute(
                                                    id,
                                                    key,
                                                    Value::Null,
                                                )?;
                                            }
                                        }
                                        for (key, value) in attrs.iter() {
                                            self.pending.borrow_mut().set_node_attribute(
                                                id,
                                                key.clone(),
                                                value.clone(),
                                            )?;
                                        }
                                    }
                                    Value::Relationship(rel) => {
                                        let g = self.g.borrow();
                                        let attrs = self.get_relationship_attrs(rel.0);
                                        if *replace {
                                            for key in g.get_node_attrs(id) {
                                                self.pending.borrow_mut().set_node_attribute(
                                                    id,
                                                    key,
                                                    Value::Null,
                                                )?;
                                            }
                                        }
                                        for (key, value) in attrs.iter() {
                                            self.pending.borrow_mut().set_node_attribute(
                                                id,
                                                key.clone(),
                                                value.clone(),
                                            )?;
                                        }
                                    }
                                    _ => {
                                        return Err("Property values can only be of primitive types or arrays of primitive types".to_string());
                                    }
                                }
                            }
                        }
                        Value::Relationship(target_rel) => {
                            if self.g.borrow().is_relationship_deleted(target_rel.0)
                                || self.pending.borrow().is_relationship_deleted(
                                    target_rel.0,
                                    target_rel.1,
                                    target_rel.2,
                                )
                            {
                                continue;
                            }
                            if let Some(attr) = attr {
                                if let Some(v) = self.get_relationship_attribute(target_rel.0, attr)
                                    && v == run_expr
                                {
                                    continue;
                                }

                                self.pending.borrow_mut().set_relationship_attribute(
                                    target_rel.0,
                                    attr.clone(),
                                    run_expr,
                                )?;
                            } else {
                                match run_expr {
                                    Value::Node(sid) => {
                                        let g = self.g.borrow();
                                        let attrs = self.get_node_attrs(sid);
                                        if *replace {
                                            for key in g.get_relationship_attrs(target_rel.0) {
                                                self.pending
                                                    .borrow_mut()
                                                    .set_relationship_attribute(
                                                        target_rel.0,
                                                        key,
                                                        Value::Null,
                                                    )?;
                                            }
                                        }
                                        for (key, value) in attrs.iter() {
                                            self.pending.borrow_mut().set_relationship_attribute(
                                                target_rel.0,
                                                key.clone(),
                                                value.clone(),
                                            )?;
                                        }
                                    }
                                    Value::Relationship(source_rel) => {
                                        let g = self.g.borrow();
                                        let attrs = self.get_relationship_attrs(source_rel.0);
                                        if *replace {
                                            for key in g.get_relationship_attrs(target_rel.0) {
                                                self.pending
                                                    .borrow_mut()
                                                    .set_relationship_attribute(
                                                        target_rel.0,
                                                        key,
                                                        Value::Null,
                                                    )?;
                                            }
                                        }
                                        for (key, value) in attrs.iter() {
                                            self.pending.borrow_mut().set_relationship_attribute(
                                                target_rel.0,
                                                key.clone(),
                                                value.clone(),
                                            )?;
                                        }
                                    }
                                    _ => {
                                        return Err("Property values can only be of primitive types or arrays of primitive types".to_string());
                                    }
                                }
                            }
                        }
                        // Silently ignore SET on Null and non-entity types
                        // (e.g. Path), matching C `FalkorDB` behavior.
                        _ => {}
                    }
                }
                SetItem::Label(entity, labels) => {
                    let run_expr = vars.get(entity);
                    match run_expr {
                        Some(Value::Node(id)) => {
                            if (self.g.borrow().is_node_deleted(id)
                                && !self.pending.borrow().is_node_created(id))
                                || self.pending.borrow().is_node_deleted(id)
                            {
                                continue;
                            }
                            self.pending.borrow_mut().set_node_labels(id, labels);
                        }
                        Some(Value::Null) => {}
                        _ => {
                            return Err(format!(
                                "Type mismatch: expected Node but was {}",
                                run_expr.map_or_else(|| "undefined".to_string(), |v| v.name())
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn relationship_scan(
        &'a self,
        relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let filter_attrs = self.run_expr(
            &relationship_pattern.attrs,
            relationship_pattern.attrs.root().idx(),
            &vars,
            None,
        )?;
        let from_id = vars
            .get(&relationship_pattern.from.alias)
            .and_then(|v| match v {
                Value::Node(id) => Some(id),
                _ => None,
            });
        let to_id = vars
            .get(&relationship_pattern.to.alias)
            .and_then(|v| match v {
                Value::Node(id) => Some(id),
                _ => None,
            });
        let iter = self.g.borrow().get_relationships(
            &relationship_pattern.types,
            &relationship_pattern.from.labels,
            &relationship_pattern.to.labels,
        );
        Ok(Box::new(iter.flat_map(move |(src, dst)| {
            if from_id.is_some() && from_id.unwrap() != src {
                return Box::new(empty()) as Box<dyn Iterator<Item = Result<Env, String>>>;
            }
            if to_id.is_some() && to_id.unwrap() != dst {
                return Box::new(empty()) as Box<dyn Iterator<Item = Result<Env, String>>>;
            }
            let vars = vars.clone();
            let filter_attrs = filter_attrs.clone();
            Box::new(
                self.g
                    .borrow()
                    .get_src_dest_relationships(src, dst, &relationship_pattern.types)
                    .into_iter()
                    .filter(move |v| {
                        if let Value::Map(filter_attrs) = &filter_attrs
                            && !filter_attrs.is_empty()
                        {
                            let g = self.g.borrow();
                            for (attr, avalue) in filter_attrs.iter() {
                                if let Some(pvalue) = g.get_relationship_attribute(*v, attr) {
                                    if *avalue == pvalue {
                                        continue;
                                    }
                                    return false;
                                }
                                return false;
                            }
                        }
                        true
                    })
                    .map(move |id| {
                        let mut vars = vars.clone();
                        vars.insert(
                            &relationship_pattern.alias,
                            Value::Relationship(Box::new((id, src, dst))),
                        );
                        vars.insert(&relationship_pattern.from.alias, Value::Node(src));
                        vars.insert(&relationship_pattern.to.alias, Value::Node(dst));
                        Ok(vars)
                    }),
            ) as Box<dyn Iterator<Item = Result<Env, String>>>
        })))
    }

    fn expand_into(
        &'a self,
        relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let src = vars.get(&relationship_pattern.from.alias).map_or_else(
            || Err(String::from("Node not found")),
            |v| match v {
                Value::Node(id) => Ok(id),
                _ => Err(String::from(
                    "Invalid node id for 'from' in relationship pattern",
                )),
            },
        )?;
        let dst = vars.get(&relationship_pattern.to.alias).map_or_else(
            || Err(String::from("Node not found")),
            |v| match v {
                Value::Node(id) => Ok(id),
                _ => Err(String::from(
                    "Invalid node id for 'from' in relationship pattern",
                )),
            },
        )?;
        Ok(Box::new(
            self.g
                .borrow()
                .get_src_dest_relationships(src, dst, &relationship_pattern.types)
                .into_iter()
                .map(move |id| {
                    let mut vars = vars.clone();
                    vars.insert(
                        &relationship_pattern.alias,
                        Value::Relationship(Box::new((id, src, dst))),
                    );
                    vars.insert(&relationship_pattern.from.alias, Value::Node(src));
                    vars.insert(&relationship_pattern.to.alias, Value::Node(dst));
                    Ok(vars)
                }),
        ))
    }

    fn node_by_label_scan(
        &'a self,
        node_pattern: &'a QueryNode<Arc<String>, Variable>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let attrs = self.run_expr(
            &node_pattern.attrs,
            node_pattern.attrs.root().idx(),
            &vars,
            None,
        )?;
        let iter = self.g.borrow().get_nodes(&node_pattern.labels, 0);
        Ok(Box::new(iter.filter_map(move |v| {
            if let Value::Map(attrs) = &attrs
                && !attrs.is_empty()
            {
                let g = self.g.borrow();
                for (attr, avalue) in attrs.iter() {
                    if let Some(pvalue) = g.get_node_attribute(v, attr) {
                        if *avalue == pvalue {
                            continue;
                        }
                        return None;
                    }
                    return None;
                }
            }
            let mut vars = vars.clone();
            vars.insert(&node_pattern.alias, Value::Node(v));
            Some(Ok(vars))
        })))
    }

    fn evaluate_index_query(
        &self,
        query: &IndexQuery<QueryExpr<Variable>>,
        vars: &Env,
    ) -> Result<IndexQuery<Value>, String> {
        match query {
            IndexQuery::Equal(key, value) => {
                let value = self.run_expr(value, value.root().idx(), vars, None)?;
                Ok(IndexQuery::Equal(key.clone(), value))
            }
            IndexQuery::Range(key, min, max) => {
                let (min, max) = match (min, max) {
                    (Some(min), Some(max)) => {
                        let min = self.run_expr(min, min.root().idx(), vars, None)?;
                        let max = self.run_expr(max, max.root().idx(), vars, None)?;
                        (Some(min), Some(max))
                    }
                    (Some(min), None) => {
                        let min = self.run_expr(min, min.root().idx(), vars, None)?;
                        (Some(min), None)
                    }
                    (None, Some(max)) => {
                        let max = self.run_expr(max, max.root().idx(), vars, None)?;
                        (None, Some(max))
                    }
                    (None, None) => (None, None),
                };
                Ok(IndexQuery::Range(key.clone(), min, max))
            }
            IndexQuery::Point { key, point, radius } => {
                let point = self.run_expr(point, point.root().idx(), vars, None)?;
                let radius = self.run_expr(radius, radius.root().idx(), vars, None)?;
                Ok(IndexQuery::Point {
                    key: key.clone(),
                    point,
                    radius,
                })
            }
            _ => todo!(),
        }
    }

    fn node_by_index_scan(
        &'a self,
        node_pattern: &'a QueryNode<Arc<String>, Variable>,
        index: &Arc<String>,
        query: &IndexQuery<QueryExpr<Variable>>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let attrs = self.run_expr(
            &node_pattern.attrs,
            node_pattern.attrs.root().idx(),
            &vars,
            None,
        )?;

        let q = self.evaluate_index_query(query, &vars)?;

        Ok(Box::new(
            self.g
                .borrow()
                .get_indexed_nodes(index, q)
                .into_iter()
                .filter_map(move |v| {
                    let mut vars = vars.clone();
                    if let Value::Map(attrs) = &attrs
                        && !attrs.is_empty()
                    {
                        let g = self.g.borrow();
                        for (attr, avalue) in attrs.iter() {
                            if let Some(pvalue) = g.get_node_attribute(v, attr) {
                                if *avalue == pvalue {
                                    continue;
                                }
                                return None;
                            }
                            return None;
                        }
                    }
                    vars.insert(&node_pattern.alias, Value::Node(v));
                    Some(Ok(vars))
                }),
        ))
    }

    fn evaluate_id_filter(
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

    fn node_by_label_and_id_scan(
        &'a self,
        node_pattern: &'a QueryNode<Arc<String>, Variable>,
        filter: &Vec<(QueryExpr<Variable>, ExprIR<Variable>)>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        match self.evaluate_id_filter(filter, &vars)? {
            Some(range) => {
                let g = self.g.borrow();
                Ok(Box::new(
                    g.get_nodes(&node_pattern.labels, range.min().unwrap())
                        .filter_map(move |nid| {
                            if range.contains(u64::from(nid)) {
                                let mut vars = vars.clone();
                                vars.insert(&node_pattern.alias, Value::Node(nid));
                                Some(Ok(vars))
                            } else {
                                None
                            }
                        }),
                ))
            }
            None => {
                return Ok(Box::new(std::iter::empty()));
            }
        }
    }

    fn node_by_id_seek(
        &'a self,
        node_pattern: &'a QueryNode<Arc<String>, Variable>,
        filter: &Vec<(QueryExpr<Variable>, ExprIR<Variable>)>,
        vars: Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        match self.evaluate_id_filter(filter, &vars)? {
            Some(range) => {
                let g = self.g.borrow();
                Ok(Box::new(range.into_iter().filter_map(move |nid| {
                    if g.is_node_deleted(NodeId::from(nid)) {
                        None
                    } else {
                        let mut vars = vars.clone();
                        vars.insert(&node_pattern.alias, Value::Node(NodeId::from(nid)));
                        Some(Ok(vars))
                    }
                })))
            }
            None => {
                return Ok(Box::new(std::iter::empty()));
            }
        }
    }

    fn delete(
        &self,
        trees: &Vec<QueryExpr<Variable>>,
        vars: &Env,
    ) -> Result<(), String> {
        for tree in trees {
            let value = self.run_expr(tree, tree.root().idx(), vars, None)?;
            self.delete_entity(value)?;
        }
        Ok(())
    }

    fn delete_entity(
        &self,
        value: Value,
    ) -> Result<(), String> {
        match value {
            Value::Node(id) => {
                if !self.g.borrow().is_node_deleted(id) {
                    for (src, dest, id) in self.g.borrow().get_node_relationships(id) {
                        self.pending
                            .borrow_mut()
                            .deleted_relationship(id, src, dest);
                    }
                    self.pending.borrow_mut().deleted_node(id);
                    let labels = self.g.borrow().get_node_label_ids(id).collect();
                    let attrs = self.get_node_attrs(id);
                    self.deleted_nodes
                        .borrow_mut()
                        .insert(id, DeletedNode::new(labels, attrs));
                }
            }
            Value::Relationship(rel) => {
                if !self.g.borrow().is_relationship_deleted(rel.0) {
                    self.pending
                        .borrow_mut()
                        .deleted_relationship(rel.0, rel.1, rel.2);
                    let type_id = self.g.borrow().get_relationship_type_id(rel.0);
                    let attrs = self.get_relationship_attrs(rel.0);
                    self.deleted_relationships
                        .borrow_mut()
                        .insert(rel.0, DeletedRelationship::new(type_id, attrs));
                }
            }
            Value::Path(values) => {
                for value in values {
                    self.delete_entity(value)?;
                }
            }
            Value::Null => {}
            _ => {
                return Err(String::from(
                    "Delete type mismatch, expecting either Node or Relationship.",
                ));
            }
        }
        Ok(())
    }

    fn create(
        &self,
        pattern: &QueryGraph<Arc<String>, LabelId, Variable>,
        vars: &mut Env,
    ) -> Result<(), String> {
        for node in pattern.nodes() {
            let id = self.g.borrow_mut().reserve_node();
            {
                let mut pending = self.pending.borrow_mut();
                pending.created_node(id);
                pending.set_node_labels(id, &node.labels);
            }

            let attrs = self.run_expr(&node.attrs, node.attrs.root().idx(), vars, None)?;
            match attrs {
                Value::Map(attrs) => {
                    self.pending.borrow_mut().set_node_attributes(id, attrs)?;
                }
                _ => unreachable!(),
            }
            vars.insert(&node.alias, Value::Node(id));
        }
        for rel in pattern.relationships() {
            let (from_id, to_id) = {
                let Value::Node(from_id) = vars
                    .get(&rel.from.alias)
                    .ok_or_else(|| format!("Variable {} not found", rel.from.alias.as_str()))?
                else {
                    return Err(String::from("Invalid node id"));
                };
                let Value::Node(to_id) = vars
                    .get(&rel.to.alias)
                    .ok_or_else(|| format!("Variable {} not found", rel.to.alias.as_str()))?
                else {
                    return Err(String::from("Invalid node id"));
                };
                (from_id, to_id)
            };

            {
                let g = self.g.borrow();
                let pending = self.pending.borrow();
                if (g.is_node_deleted(from_id) && !pending.is_node_created(from_id))
                    || pending.is_node_deleted(from_id)
                    || (g.is_node_deleted(to_id) && !pending.is_node_created(to_id))
                    || pending.is_node_deleted(to_id)
                {
                    return Err(String::from(
                        "Failed to create relationship; endpoint was not found.",
                    ));
                }
            }
            let id = self.g.borrow_mut().reserve_relationship();
            self.pending.borrow_mut().created_relationship(
                id,
                from_id,
                to_id,
                rel.types.first().unwrap().clone(),
            );
            let attrs = self.run_expr(&rel.attrs, rel.attrs.root().idx(), vars, None)?;
            match attrs {
                Value::Map(attrs) => {
                    self.pending
                        .borrow_mut()
                        .set_relationship_attributes(id, attrs)?;
                }
                _ => {
                    return Err(String::from("Invalid relationship properties"));
                }
            }
            vars.insert(
                &rel.alias,
                Value::Relationship(Box::new((id, from_id, to_id))),
            );
        }
        Ok(())
    }

    fn compute_merge_pattern_hash(
        &self,
        pattern: &QueryGraph<Arc<String>, LabelId, Variable>,
        vars: &Env,
    ) -> Result<u64, String> {
        let mut hasher = DefaultHasher::new();

        // Hash nodes in the pattern
        for node in pattern.nodes() {
            // If the node variable exists in vars, hash its ID
            if let Some(value) = vars.get(&node.alias) {
                value.hash(&mut hasher);
            } else {
                // Hash the node structure (labels and attributes)
                for label in node.labels.iter() {
                    label.hash(&mut hasher);
                }
                let attrs = self.run_expr(&node.attrs, node.attrs.root().idx(), vars, None)?;

                // Validate that no attributes are NULL
                if let Value::Map(ref map) = attrs {
                    for (key, value) in map.iter() {
                        if *value == Value::Null {
                            return Err(format!(
                                "Cannot merge node using null property value for key '{key}'"
                            ));
                        }
                    }
                }

                attrs.hash(&mut hasher);
            }
        }

        // Hash relationships in the pattern
        for rel in pattern.relationships() {
            // Hash relationship type
            rel.types.hash(&mut hasher);

            // Hash from/to node references
            if let Some(value) = vars.get(&rel.from.alias) {
                value.hash(&mut hasher);
            }
            if let Some(value) = vars.get(&rel.to.alias) {
                value.hash(&mut hasher);
            }

            // Hash relationship attributes
            let attrs = self.run_expr(&rel.attrs, rel.attrs.root().idx(), vars, None)?;

            // Validate that no attributes are NULL
            if let Value::Map(ref map) = attrs {
                for (key, value) in map.iter() {
                    if *value == Value::Null {
                        return Err(format!(
                            "Cannot merge relationship using null property value for key '{key}'"
                        ));
                    }
                }
            }

            attrs.hash(&mut hasher);
        }

        Ok(hasher.finish())
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

    pub fn get_node_attrs(
        &self,
        id: NodeId,
    ) -> OrderMap<Arc<String>, Value> {
        if let Some(dn) = self.deleted_nodes.borrow().get(&id) {
            return dn.attrs.clone();
        }
        let mut actual = self.g.borrow().get_node_all_attrs(id);
        self.pending.borrow().update_node_attrs(id, &mut actual);
        actual
    }

    pub fn get_relationship_attrs(
        &self,
        id: RelationshipId,
    ) -> OrderMap<Arc<String>, Value> {
        if let Some(dr) = self.deleted_relationships.borrow().get(&id) {
            return dr.attrs.clone();
        }
        let mut actual = self.g.borrow().get_relationship_all_attrs(id);
        self.pending
            .borrow()
            .update_relationship_attrs(id, &mut actual);
        actual
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

pub fn evaluate_param(expr: &DynNode<ExprIR<Arc<String>>>) -> Result<Value, String> {
    match expr.data() {
        ExprIR::Null => Ok(Value::Null),
        ExprIR::Bool(x) => Ok(Value::Bool(*x)),
        ExprIR::Integer(x) => Ok(Value::Int(*x)),
        ExprIR::Float(x) => Ok(Value::Float(*x)),
        ExprIR::String(x) => Ok(Value::String(x.clone())),
        ExprIR::List => Ok(Value::List(
            expr.children()
                .map(|c| evaluate_param(&c))
                .collect::<Result<ThinVec<_>, _>>()?,
        )),
        ExprIR::Map => Ok(Value::Map(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::String(key) => {
                        Ok::<_, String>((key.clone(), evaluate_param(&ir.child(0))?))
                    }
                    _ => todo!(),
                })
                .collect::<Result<OrderMap<_, _>, _>>()?,
        )),
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
                return Ok(Value::List(thin_vec![]));
            }
            Ok(Value::List(
                values[start as usize..end as usize]
                    .iter()
                    .cloned()
                    .collect::<ThinVec<_>>(),
            ))
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
        Value::List(l) => Ok(Contains::contains(l, value)),
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
