//! Query plan optimization passes.
//!
//! The optimizer transforms the logical execution plan produced by the planner
//! to improve performance. It applies a fixed sequence of rewrite passes, each
//! making local transformations to the IR tree.
//!
//! ## Pass Ordering
//!
//! Passes run in the following order:
//!
//! ```text
//! Input plan (from Planner)
//!       |
//!       v
//! 1. eliminate_true_filters   -- Remove trivial Filter(true) nodes
//!       |
//!       v
//! 2. select_scan_node         -- Pick the best starting node for traversal
//!       |                        chains, possibly reversing chain direction
//!       v
//! 3. push_filters_down        -- Move Filter conjuncts closer to the
//!       |                        operators that produce their variables
//!       v
//! 4. replace_cartesian_       -- Convert CartesianProduct + equality
//!    with_hash_join               Filter into ValueHashJoin
//!       |
//!       v
//! 5. absorb_edge_filters_     -- Fold edge-only filters into
//!    into_vlt                     CondVarLenTraverse's per-hop filter
//!       |
//!       v
//! 6. utilize_index            -- Replace NodeByLabelScan + Filter with
//!       |                        NodeByIndexScan when an index exists
//!       v
//! 7. utilize_node_by_id       -- Replace label scan + id() filter with
//!       |                        NodeByLabelAndIdScan or NodeByIdSeek
//!       v
//! Optimized plan
//! ```
//!
//! ## Implementation Pattern
//!
//! Each pass uses a collect-then-iterate loop: collect candidate node indices
//! via a BFS traversal, attempt one transformation, then restart the traversal
//! if the tree structure changed. This avoids issues with invalidated indices
//! after in-place tree mutations.

mod absorb_edge_filters_into_vlt;
mod eliminate_true_filters;
mod fuse_anonymous_traverse;
mod push_filters_down;
mod reduce_count;
mod reduce_expand_into;
mod reorder_labels;
mod replace_cartesian_with_hash_join;
mod select_scan_node;
mod utilize_index;
mod utilize_node_by_id;

use std::collections::{HashMap, HashSet};

use orx_tree::{Bfs, DynTree, NodeRef};

use crate::{
    graph::graph::Graph,
    index::IndexQuery,
    parser::ast::{ExprIR, QueryExpr, QueryGraph, SetItem, Variable},
    runtime::value::Value,
};

use super::IR;

use absorb_edge_filters_into_vlt::absorb_edge_filters_into_vlt;
use eliminate_true_filters::eliminate_true_filters;
use fuse_anonymous_traverse::fuse_anonymous_traverse;
use push_filters_down::push_filters_down;
use reduce_count::reduce_count;
use reduce_expand_into::reduce_expand_into;
use reorder_labels::reorder_labels;
use replace_cartesian_with_hash_join::replace_cartesian_with_hash_join;
use select_scan_node::select_scan_node;
use utilize_index::utilize_index;
use utilize_node_by_id::utilize_node_by_id;

/// Collects all variable IDs referenced in an expression tree.
pub(crate) fn collect_expr_variables(expr: &DynTree<ExprIR<Variable>>) -> HashSet<u32> {
    let mut vars = HashSet::new();
    for idx in expr.root().indices::<Bfs>() {
        if let ExprIR::Variable(var) = expr.node(idx).data() {
            vars.insert(var.id);
        }
    }
    vars
}

/// Collects all variable IDs provided by a plan subtree.
pub(crate) fn collect_subtree_variables(node: &orx_tree::DynNode<IR>) -> HashSet<u32> {
    use crate::runtime::runtime::GetVariables;
    let mut vars = HashSet::new();
    for var in node.get_variables() {
        vars.insert(var.id);
    }
    vars
}

/// Full optimization pipeline. Pass ordering is load-bearing:
/// `select_scan_node` reverses traversal chains so must precede
/// `push_filters_down`; `eliminate_true_filters` removes the planner's
/// `Bool(true)` filler so must precede `select_scan_node`'s filter scoring.
#[allow(clippy::implicit_hasher)]
pub fn optimize_compiletime(
    optimized_plan: &mut DynTree<IR>,
    graph: &Graph,
    _params: &HashMap<String, Value>,
) {
    reduce_count(optimized_plan);
    reduce_expand_into(optimized_plan);
    // Strip planner-inserted Bool(true) placeholders. Param-dependent
    // elimination is deferred to `optimize_runtime` so the cached plan
    // doesn't bake in a specific parameter value.
    eliminate_true_filters(optimized_plan, &HashMap::new());
    compile_expressions(optimized_plan);
}

/// Runtime-only passes. Re-runs the subset of [`optimize_full`] whose result
/// depends on query parameters or live graph state (counts, schema, indices).
///
/// Run on a cache hit so a cached, fully-optimized plan reflects the
/// current parameters and graph. The structural compile-time passes
/// (`push_filters_down`, `fuse_anonymous_traverse`, etc.) are idempotent on
/// an already-optimized plan, so we skip them — which also preserves the
/// `OnceLock<CompiledExpr>` Arcs populated by [`compile_expressions`].
#[allow(clippy::implicit_hasher)]
pub fn optimize_runtime(
    optimized_plan: &mut DynTree<IR>,
    graph: &Graph,
    params: &HashMap<String, Value>,
) {
    eliminate_true_filters(optimized_plan, params);
    select_scan_node(optimized_plan, graph);
    push_filters_down(optimized_plan);
    fuse_anonymous_traverse(optimized_plan);
    replace_cartesian_with_hash_join(optimized_plan);
    absorb_edge_filters_into_vlt(optimized_plan);
    utilize_index(optimized_plan, graph);
    utilize_node_by_id(optimized_plan);
    reorder_labels(optimized_plan, graph);
}

/// Pre-populate the JIT `OnceLock<CompiledExpr>` for every `QueryExpr`
/// reachable from the plan. Best-effort: expressions not supported by the
/// JIT cache `Some(None)` and fall back to interpreted evaluation at runtime.
pub fn compile_expressions(plan: &DynTree<IR>) {
    let mut compile_one = |expr: &QueryExpr<Variable>| {
        let _ = expr.compiled.get_or_init(|| {
            crate::runtime::jit::try_compile(&expr.tree, expr.tree.root().idx())
                .map(std::sync::Arc::new)
        });
    };
    for idx in plan.root().indices::<Bfs>() {
        for_each_query_expr_in_ir(plan.node(idx).data(), &mut compile_one);
    }
}

fn for_each_query_expr_in_ir(
    ir: &IR,
    f: &mut impl FnMut(&QueryExpr<Variable>),
) {
    match ir {
        IR::ProcedureCall { args, .. } => {
            for e in args {
                f(e);
            }
        }
        IR::Unwind { expr, .. } => f(expr),
        IR::Create(qg) => for_each_query_expr_in_query_graph(qg, f),
        IR::Merge {
            pattern,
            on_create,
            on_match,
        } => {
            for_each_query_expr_in_query_graph(pattern, f);
            for item in on_create {
                for_each_query_expr_in_set_item(item, f);
            }
            for item in on_match {
                for_each_query_expr_in_set_item(item, f);
            }
        }
        IR::Delete { exprs, .. } | IR::Remove(exprs) => {
            for e in exprs {
                f(e);
            }
        }
        IR::Set(items) => {
            for item in items {
                for_each_query_expr_in_set_item(item, f);
            }
        }
        IR::AllNodeScan(node)
        | IR::NodeByLabelScan { node, .. }
        | IR::IncludePending { node, .. } => f(&node.attrs),
        IR::NodeByIndexScan { node, query, .. } => {
            f(&node.attrs);
            for_each_query_expr_in_index_query(query, f);
        }
        IR::EdgeByIndexScan {
            relationship,
            query,
            ..
        } => {
            f(&relationship.attrs);
            f(&relationship.from.attrs);
            f(&relationship.to.attrs);
            for_each_query_expr_in_index_query(query, f);
        }
        IR::NodeByFulltextScan { label, query, .. }
        | IR::EdgeByFulltextScan { label, query, .. } => {
            f(label);
            f(query);
        }
        IR::NodeByVectorScan {
            label,
            attr,
            k,
            vector,
            ..
        }
        | IR::EdgeByVectorScan {
            label,
            attr,
            k,
            vector,
            ..
        } => {
            f(label);
            f(attr);
            f(k);
            f(vector);
        }
        IR::NodeByLabelAndIdScan { node, filter } | IR::NodeByIdSeek { node, filter } => {
            f(&node.attrs);
            for (e, _) in filter {
                f(e);
            }
        }
        IR::CondTraverse { relationship, .. }
        | IR::ExpandInto { relationship, .. }
        | IR::AllShortestPaths(relationship) => {
            f(&relationship.attrs);
            f(&relationship.from.attrs);
            f(&relationship.to.attrs);
        }
        IR::CondVarLenTraverse {
            relationship,
            edge_filter,
        } => {
            f(&relationship.attrs);
            f(&relationship.from.attrs);
            f(&relationship.to.attrs);
            if let Some(ef) = edge_filter {
                f(ef);
            }
        }
        IR::Filter(expr) | IR::Skip(expr) | IR::Limit(expr) => f(expr),
        IR::ValueHashJoin { lhs_exp, rhs_exp } => {
            f(lhs_exp);
            f(rhs_exp);
        }
        IR::LoadCsv {
            file_path,
            delimiter,
            ..
        } => {
            f(file_path);
            f(delimiter);
        }
        IR::Sort(exprs) => {
            for (e, _) in exprs {
                f(e);
            }
        }
        IR::Aggregate {
            keys, aggregations, ..
        } => {
            for (_, e) in keys {
                f(e);
            }
            for (_, e) in aggregations {
                f(e);
            }
        }
        IR::Project { exprs, .. } => {
            for (_, e) in exprs {
                f(e);
            }
        }
        IR::ForEach { list, .. } => f(list),
        IR::CreateIndex {
            options: Some(opts),
            ..
        } => f(opts),
        // No embedded QueryExpr fields:
        IR::Argument
        | IR::Optional(_)
        | IR::PathBuilder(_)
        | IR::CartesianProduct
        | IR::Apply
        | IR::SemiApply
        | IR::AntiSemiApply
        | IR::OrApplyMultiplexer(_)
        | IR::Distinct
        | IR::Union
        | IR::Commit
        | IR::CreateIndex { options: None, .. }
        | IR::DropIndex { .. } => {}
    }
}

fn for_each_query_expr_in_set_item(
    item: &SetItem<std::sync::Arc<String>, Variable>,
    f: &mut impl FnMut(&QueryExpr<Variable>),
) {
    if let SetItem::Attribute { target, value, .. } = item {
        f(target);
        f(value);
    }
}

fn for_each_query_expr_in_query_graph(
    qg: &QueryGraph<std::sync::Arc<String>, std::sync::Arc<String>, Variable>,
    f: &mut impl FnMut(&QueryExpr<Variable>),
) {
    for node in qg.nodes() {
        f(&node.attrs);
    }
    for rel in qg.relationships() {
        f(&rel.attrs);
        f(&rel.from.attrs);
        f(&rel.to.attrs);
    }
}

fn for_each_query_expr_in_index_query(
    query: &IndexQuery<QueryExpr<Variable>>,
    f: &mut impl FnMut(&QueryExpr<Variable>),
) {
    match query {
        IndexQuery::Equal { value, .. } | IndexQuery::ArrayContains { value, .. } => f(value),
        IndexQuery::Range { min, max, .. } => {
            if let Some(e) = min {
                f(e);
            }
            if let Some(e) = max {
                f(e);
            }
        }
        IndexQuery::And(qs) | IndexQuery::Or(qs) => {
            for q in qs {
                for_each_query_expr_in_index_query(q, f);
            }
        }
        IndexQuery::Point { point, radius, .. } => {
            f(point);
            f(radius);
        }
        IndexQuery::InList { list, .. } => f(list),
    }
}
