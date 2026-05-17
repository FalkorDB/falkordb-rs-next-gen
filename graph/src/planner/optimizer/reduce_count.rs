//! Count reduction optimization pass.
//!
//! Replaces scan + aggregate patterns that simply count nodes or relationships
//! with a single `Project` node that reads the count directly from graph
//! metadata. This avoids scanning and aggregating all entities.
//!
//! ## Transformations
//!
//! **Node count (unlabeled):**
//!
//! ```text
//! Before:                     After:
//!
//! Aggregate(COUNT(n))         Project(node_count)
//!   AllNodeScan(n)
//! ```
//!
//! **Node count (labeled):**
//!
//! ```text
//! Before:                        After:
//!
//! Aggregate(COUNT(n))            Project(label_node_count)
//!   NodeByLabelScan(n:Label)
//! ```
//!
//! **Edge count:**
//!
//! ```text
//! Before:                                After:
//!
//! Aggregate(COUNT(r))                    Project(edge_count)
//!   CondTraverse(()-[r]->())
//! ```

use std::sync::Arc;

use orx_tree::{DynTree, NodeRef};

use crate::{
    parser::ast::{CountKind, ExprIR, Variable},
    tree,
};

use super::super::IR;

/// Attempts to reduce a count aggregation to a direct graph metadata lookup.
///
/// Detects patterns where the entire query is a simple `MATCH ... RETURN COUNT(x)`
/// with no filters, and replaces the Aggregate + scan subtree with a Project
/// that emits an `ExprIR::GraphCount`. The actual count is read from the live
/// graph at runtime, so this rewrite stays valid across cache hits as the
/// graph mutates.
pub(super) fn reduce_count(optimized_plan: &mut DynTree<IR>) {
    // Walk the plan looking for Aggregate nodes.
    let indices = optimized_plan
        .root()
        .indices::<orx_tree::Bfs>()
        .collect::<Vec<_>>();

    for idx in indices {
        let IR::Aggregate {
            keys,
            aggregations,
            projections,
            ..
        } = optimized_plan.node(idx).data()
        else {
            continue;
        };

        // Must have zero group-by keys, exactly one aggregation, zero projections.
        if !keys.is_empty() || aggregations.len() != 1 || !projections.is_empty() {
            continue;
        }

        let (agg_var, agg_expr) = &aggregations[0];

        // The aggregation expression must be a single count() function call
        // on a variable.
        let Some(count_var_id) = extract_count_variable(agg_expr) else {
            continue;
        };

        // Examine the child of the Aggregate node.
        let agg_node = optimized_plan.node(idx);
        if agg_node.num_children() != 1 {
            continue;
        }
        let child = agg_node.child(0);

        let kind = match child.data() {
            // MATCH (n) RETURN COUNT(n)
            IR::AllNodeScan(node) if node.alias.id == count_var_id && child.num_children() == 0 => {
                Some(CountKind::AllNodes)
            }
            // MATCH (n:Label) RETURN COUNT(n)
            IR::NodeByLabelScan { node, .. }
                if node.alias.id == count_var_id
                    && child.num_children() == 0
                    && node.labels.len() == 1 =>
            {
                let label = node.labels.iter().next().unwrap();
                Some(CountKind::NodesWithLabel(label.clone()))
            }
            // MATCH ()-[r]->() RETURN COUNT(r) or MATCH ()-[r:Type]->() RETURN COUNT(r)
            // Plan shape: CondTraverse { rel } (leaf, before select_scan_node adds the scan child)
            // Only safe when both endpoints are unlabeled and direction is not reversed.
            IR::CondTraverse {
                relationship,
                transposed,
                ..
            } if relationship.alias.id == count_var_id && !transposed => {
                // The CondTraverse must be a leaf (no children) — this means
                // the pattern is a simple scan, not part of a longer chain.
                if child.num_children() != 0 {
                    continue;
                }
                // Skip when endpoints have labels — the graph-level count
                // doesn't account for endpoint label filtering.
                if !relationship.from.labels.is_empty() || !relationship.to.labels.is_empty() {
                    continue;
                }
                // Skip bidirectional patterns — the graph-level count is directional.
                if relationship.bidirectional {
                    continue;
                }
                if relationship.types.is_empty() {
                    Some(CountKind::AllRelationships)
                } else {
                    Some(CountKind::RelationshipsByTypes(
                        relationship.types.iter().cloned().collect(),
                    ))
                }
            }
            _ => None,
        };

        let Some(kind) = kind else {
            continue;
        };

        // Replace the Aggregate subtree with a Project that emits the count
        // via a runtime GraphCount lookup.
        let agg_var = agg_var.clone();
        let count_expr: crate::parser::ast::QueryExpr<Variable> = Arc::new(
            crate::parser::ast::QueryExprInner::from(tree!(ExprIR::GraphCount(kind))),
        );

        // First prune all children.
        while optimized_plan.node(idx).num_children() > 0 {
            let child_idx = optimized_plan.node(idx).child(0).idx();
            optimized_plan.node_mut(child_idx).prune();
        }

        // Then replace data.
        *optimized_plan.node_mut(idx).data_mut() = IR::Project {
            exprs: vec![(agg_var, count_expr)],
            copies: vec![],
        };
        // Only handle one reduction per plan (the plan shouldn't have multiple
        // such patterns, but breaking is safer after tree mutation).
        break;
    }
}

/// If the expression is `count(var)`, returns `Some(var.id)`.
/// Returns `None` for any other expression shape.
fn extract_count_variable(expr: &DynTree<ExprIR<Variable>>) -> Option<u32> {
    let ExprIR::FuncInvocation(func) = expr.root().data() else {
        return None;
    };
    if func.name != "count" {
        return None;
    }
    // count(var) has the counted variable as the first child.
    // There may be an additional __agg_order_by_placeholder__ child.
    let root = expr.root();
    if root.num_children() == 0 {
        return None;
    }
    let child = root.child(0);
    if let ExprIR::Variable(var) = child.data() {
        Some(var.id)
    } else {
        None
    }
}
