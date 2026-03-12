//! Query plan optimization passes.
//!
//! The optimizer transforms the logical execution plan to improve performance.
//! Current optimizations include:
//!
//! ## Index Utilization
//!
//! Replaces `NodeByLabelScan` + `Filter` with `NodeByIndexScan` when:
//! - A range index exists on the filtered property
//! - The filter uses equality (=), less than (<), or greater than (>)
//!
//! Example transformation:
//! ```text
//! Before: NodeByLabelScan(:Person) → Filter(n.age = 30)
//! After:  NodeByIndexScan(:Person, age, Equal(30))
//! ```
//!
//! ## Node By Label And ID Optimization
//!
//! Replaces label scan + ID filter with direct ID lookup:
//! ```text
//! Before: NodeByLabelScan(:Person) → Filter(id(n) = 42)
//! After:  NodeByLabelAndIdScan(:Person, 42)
//! ```
//!
//! ## Good Practice
//!
//! The optimizer uses a collect-then-iterate pattern when modifying the tree
//! to avoid issues with mutable iteration. This is a common pattern when
//! working with tree structures that need in-place modification.

use std::collections::HashSet;
use std::sync::Arc;

use orx_tree::{Bfs, Dyn, DynNode, DynTree, NodeIdx, NodeRef};

use crate::{
    graph::graph::Graph,
    index::indexer::{IndexQuery, IndexType},
    parser::ast::{ExprIR, QueryExpr, QueryNode, Variable},
    runtime::runtime::GetVariables,
    tree,
};

use super::IR;

type IndexScanResult = Option<(
    Arc<QueryNode<Arc<String>, Variable>>,
    Arc<String>,
    Arc<IndexQuery<QueryExpr<Variable>>>,
)>;

fn extract_attribute_from_subtree(
    tree: &DynTree<ExprIR<Variable>>,
    root_idx: NodeIdx<Dyn<ExprIR<Variable>>>,
) -> Option<Arc<String>> {
    let indices = tree.node(root_idx).indices::<Bfs>().collect::<Vec<_>>();
    for idx in indices {
        let node = tree.node(idx);
        if let ExprIR::Property(attr) = node.data() {
            return Some(attr.clone());
        }
    }
    None
}

/// Extract the attribute and the expression from the filter
///
/// Look for single attribute (either left or right child)
/// And a an expression that does not depends on graph entity - so we can pass it to the filter for a single query
///
/// Returns the attribute, the expression, and the child index of the attribute.
fn extract_attribute_and_expression_from_filter(
    filter: &DynTree<ExprIR<Variable>>
) -> Option<(
    Arc<String>,
    NodeIdx<Dyn<ExprIR<Variable>>>,
    NodeIdx<Dyn<ExprIR<Variable>>>,
)> {
    let lhs_idx = filter.root().child(0).idx();
    let rhs_idx = filter.root().child(1).idx();

    match (
        extract_attribute_from_subtree(filter, lhs_idx),
        extract_attribute_from_subtree(filter, rhs_idx),
    ) {
        (Some(_), Some(_)) => None,
        (Some(attr), _) => Some((attr, lhs_idx, rhs_idx)),
        (_, Some(attr)) => Some((attr, rhs_idx, lhs_idx)),
        _ => None,
    }
}

/// Builds an index scan for a simple property filter (e.g. `n.age = 30`).
fn try_property_index_scan(
    node: &Arc<QueryNode<Arc<String>, Variable>>,
    attr: &Arc<String>,
    op: &ExprIR<Variable>,
    constant_node: DynTree<ExprIR<Variable>>,
) -> IndexScanResult {
    match op {
        ExprIR::Eq => Some((
            node.clone(),
            node.labels[0].clone(),
            Arc::new(IndexQuery::Equal(attr.clone(), Arc::new(constant_node))),
        )),
        ExprIR::Gt => Some((
            node.clone(),
            node.labels[0].clone(),
            Arc::new(IndexQuery::Range(
                attr.clone(),
                Some(Arc::new(constant_node)),
                None,
            )),
        )),
        ExprIR::Lt => Some((
            node.clone(),
            node.labels[0].clone(),
            Arc::new(IndexQuery::Range(
                attr.clone(),
                None,
                Some(Arc::new(constant_node)),
            )),
        )),
        _ => None,
    }
}

/// Builds an index scan for a distance filter (e.g. `distance(n.loc, point(...)) < 100`).
fn try_distance_index_scan(
    node: &Arc<QueryNode<Arc<String>, Variable>>,
    attr: &Arc<String>,
    filter: &DynTree<ExprIR<Variable>>,
    attribute_side: NodeIdx<Dyn<ExprIR<Variable>>>,
    constant_node: DynTree<ExprIR<Variable>>,
) -> IndexScanResult {
    let operand = filter.root().data();
    // If attribute side is 0 than operand must be <
    // else if attribute_side == 1 than operand must be >
    // Fail in any other case
    match operand {
        ExprIR::Lt => {
            if filter.root().child(0).idx() != attribute_side {
                return None;
            }
        }
        ExprIR::Gt => {
            if filter.root().child(1).idx() != attribute_side {
                return None;
            }
        }
        _ => return None,
    }
    let child_0_idx = filter.node(attribute_side).child(0).idx();
    let child_1_idx = filter.node(attribute_side).child(1).idx();
    match (
        extract_attribute_from_subtree(filter, child_0_idx),
        extract_attribute_from_subtree(filter, child_1_idx),
    ) {
        (Some(_), None) => Some((
            node.clone(),
            node.labels[0].clone(),
            Arc::new(IndexQuery::Point {
                key: attr.clone(),
                point: Arc::new(filter.node(child_1_idx).clone_as_tree()),
                radius: Arc::new(constant_node),
            }),
        )),
        (None, Some(_)) => Some((
            node.clone(),
            node.labels[0].clone(),
            Arc::new(IndexQuery::Point {
                key: attr.clone(),
                point: Arc::new(filter.node(child_0_idx).clone_as_tree()),
                radius: Arc::new(constant_node),
            }),
        )),
        _ => None,
    }
}

/// Attempts to replace label scans with index scans where applicable.
///
/// Scans the plan for patterns like:
/// `NodeByLabelScan` → `Filter(property = value)`
///
/// If an index exists on the filtered property, replaces with `NodeByIndexScan`.
fn utilize_index(
    optimized_plan: &mut DynTree<IR>,
    graph: &Graph,
) {
    let indices = optimized_plan.root().indices::<Bfs>().collect::<Vec<_>>();

    for idx in indices {
        // Try to replace label scans with index scans where applicable.
        let node = if let IR::NodeByLabelScan(node) = optimized_plan.node(idx).data()
            && !node.labels.is_empty()
            && let IR::Filter(filter) = optimized_plan.node(idx).parent().unwrap().data()
            // If the filter is an equality, greater than, or less than expression
            && matches!(filter.root().data(), ExprIR::Eq | ExprIR::Gt | ExprIR::Lt)
            // If we managed to extract the attribute and expression from the filter
            && let Some((attr, attr_side, constant_side)) = extract_attribute_and_expression_from_filter(filter)
            // If the attribute is indexed
            && graph.is_indexed(&node.labels[0], &attr, &IndexType::Range)
        {
            // Check if the attribute side is a propetry function or more complexed function
            // If it is a property function, we can handle it right away since we know everything: Attribute, expression and operation
            // If it is a more complexed function, we need to understand which function it is and if we can apply an index scan on.
            match filter.node(attr_side).data() {
                ExprIR::FuncInvocation(func) => {
                    let constant_node = filter.node(constant_side).clone_as_tree();
                    match func.name.as_str() {
                        "distance" => {
                            try_distance_index_scan(node, &attr, filter, attr_side, constant_node)
                        }
                        _ => None,
                    }
                }
                ExprIR::Property(attr) => {
                    let constant_node = filter.node(constant_side).clone_as_tree();
                    try_property_index_scan(node, attr, filter.root().data(), constant_node)
                }
                _ => None,
            }
        } else {
            None
        };
        if let Some((node, index, query)) = node {
            let mut op = optimized_plan.node_mut(idx);
            *op.data_mut() = IR::NodeByIndexScan { node, index, query };
            op.parent_mut().unwrap().take_out();
            break;
        }

        let node = if let IR::NodeByLabelScan(node) = optimized_plan.node(idx).data() {
            get_index(graph, node)
        } else {
            None
        };
        if let Some((node, attr, filter)) = node
            && !node.labels.is_empty()
        {
            let mut op = optimized_plan.node_mut(idx);
            *op.data_mut() = IR::NodeByIndexScan {
                node: node.clone(),
                index: node.labels[0].clone(),
                query: Arc::new(IndexQuery::Equal(
                    attr.clone(),
                    Arc::new(filter.root().child(1).clone_as_tree()),
                )),
            };
        }
    }
}

fn get_id_filter(
    filter: &DynNode<ExprIR<Variable>>,
    node_alias: &Variable,
) -> Option<(QueryExpr<Variable>, ExprIR<Variable>)> {
    if matches!(
        filter.data(),
        ExprIR::Eq | ExprIR::Gt | ExprIR::Ge | ExprIR::Lt | ExprIR::Le
    ) && let ExprIR::FuncInvocation(inner_func) = filter.child(0).data()
        && inner_func.name == "id"
        && let ExprIR::Variable(var) = filter.child(0).child(0).data()
        && var == node_alias
    {
        Some((
            Arc::new(filter.child(1).clone_as_tree()),
            filter.data().clone(),
        ))
    } else if matches!(
        filter.data(),
        ExprIR::Eq | ExprIR::Gt | ExprIR::Ge | ExprIR::Lt | ExprIR::Le
    ) && let ExprIR::FuncInvocation(inner_func) = filter.child(1).data()
        && inner_func.name == "id"
        && let ExprIR::Variable(var) = filter.child(1).child(0).data()
        && var == node_alias
    {
        let op = match filter.data() {
            ExprIR::Eq => ExprIR::Eq,
            ExprIR::Gt => ExprIR::Lt,
            ExprIR::Ge => ExprIR::Le,
            ExprIR::Lt => ExprIR::Gt,
            ExprIR::Le => ExprIR::Ge,
            _ => unreachable!(),
        };
        Some((Arc::new(filter.child(0).clone_as_tree()), op))
    } else {
        None
    }
}

/// Collects all variable IDs referenced in an expression tree.
fn collect_expr_variables(expr: &DynTree<ExprIR<Variable>>) -> HashSet<u32> {
    let mut vars = HashSet::new();
    for idx in expr.root().indices::<Bfs>() {
        if let ExprIR::Variable(var) = expr.node(idx).data() {
            vars.insert(var.id);
        }
    }
    vars
}

/// Collects all variable IDs provided by a plan subtree.
fn collect_subtree_variables(node: &DynNode<IR>) -> HashSet<u32> {
    let mut vars = HashSet::new();
    for var in node.get_variables() {
        vars.insert(var.id);
    }
    vars
}

/// Pushes filter conjuncts down through nodes.
///
/// Transforms:
/// ```text
/// Filter(AND(cond_a, cond_b))
///   └─ CartesianProduct
///        ├─ ChildA
///        └─ ChildB
/// ```
/// Into:
/// ```text
/// CartesianProduct
///   ├─ Filter(cond_a)
///   │    └─ ChildA
///   └─ Filter(cond_b)
///        └─ ChildB
/// ```
///
/// Each conjunct is routed to the child whose variables fully cover the
/// conjunct's referenced variables. Conjuncts that span multiple children
/// remain at the current level.
fn push_filters_down(optimized_plan: &mut DynTree<IR>) {
    loop {
        let mut changed = false;
        let indices = optimized_plan.root().indices::<Bfs>().collect::<Vec<_>>();
        for idx in indices {
            let IR::Filter(filter) = optimized_plan.node(idx).data() else {
                continue;
            };

            // Merge stacked filters: if this filter's child is also a filter,
            // combine their conjuncts into a single AND filter.
            if let Some(child) = optimized_plan.node(idx).get_child(0)
                && let IR::Filter(child_filter) = child.data()
            {
                let child_filter = child_filter.clone();
                let filter = filter.clone();
                let child_idx = child.idx();

                // Flatten conjuncts from both filters
                let mut conjuncts: Vec<DynTree<ExprIR<Variable>>> = vec![];
                for f in [&filter, &child_filter] {
                    if matches!(f.root().data(), ExprIR::And) {
                        conjuncts.extend(f.root().children().map(|c| c.clone_as_tree()));
                    } else {
                        conjuncts.push((**f).clone());
                    }
                }

                let merged = if conjuncts.len() == 1 {
                    Arc::new(conjuncts.into_iter().next().unwrap())
                } else {
                    Arc::new(tree!(ExprIR::And; conjuncts))
                };
                *optimized_plan.node_mut(idx).data_mut() = IR::Filter(merged);
                optimized_plan.node_mut(child_idx).take_out();

                changed = true;
                break;
            }

            if !optimized_plan
                .node(idx)
                .children()
                .any(|c| c.num_children() > 0)
            {
                continue; // Skip if filter already is downstream
            }
            let filter = filter.clone();

            // Split filter into individual conjuncts
            let conjuncts: Vec<DynTree<ExprIR<Variable>>> =
                if matches!(filter.root().data(), ExprIR::And) {
                    filter
                        .root()
                        .children()
                        .map(|c| c.clone_as_tree())
                        .collect()
                } else {
                    vec![(*filter).clone()]
                };

            // Collect children and the variables they provide
            let children: Vec<_> = optimized_plan
                .node(idx)
                .children()
                .filter(|c| {
                    c.num_children() > 0
                        && !matches!(
                            c.data(),
                            IR::Project(..)
                                | IR::Aggregate(..)
                                | IR::Argument
                                | IR::SemiApply
                                | IR::AntiSemiApply
                                | IR::OrApplyMultiplexer(_)
                        )
                })
                .flat_map(|c| c.children().collect::<Vec<_>>())
                .map(|c| (c.idx(), collect_subtree_variables(&c)))
                .collect();

            // Route each conjunct to the child that provides all its variables
            let mut child_conjuncts: Vec<Vec<DynTree<ExprIR<Variable>>>> =
                vec![vec![]; children.len()];
            let mut remaining: Vec<DynTree<ExprIR<Variable>>> = vec![];

            for conjunct in conjuncts {
                let conj_vars = collect_expr_variables(&conjunct);
                let matched_child = children
                    .iter()
                    .enumerate()
                    .find(|(_, (_, child_vars))| conj_vars.iter().all(|v| child_vars.contains(v)))
                    .map(|(i, _)| i);
                if let Some(i) = matched_child {
                    child_conjuncts[i].push(conjunct);
                } else {
                    remaining.push(conjunct);
                }
            }

            // Skip if nothing can be pushed down
            if child_conjuncts.iter().all(Vec::is_empty) {
                continue;
            }

            // For each child with matching conjuncts: add a Filter-wrapped
            // clone as a sibling, then prune the original.
            for (i, conjuncts) in child_conjuncts.into_iter().enumerate() {
                if conjuncts.is_empty() {
                    continue;
                }

                let child_idx = children[i].0;

                // Build the filter expression for this child
                let filter_expr = if conjuncts.len() == 1 {
                    Arc::new(conjuncts.into_iter().next().unwrap())
                } else {
                    Arc::new(tree!(ExprIR::And; conjuncts))
                };

                // Insert the new filter node above the child
                optimized_plan
                    .node_mut(child_idx)
                    .push_parent(IR::Filter(filter_expr));
            }

            // Update or remove the original Filter
            if remaining.is_empty() {
                optimized_plan.node_mut(idx).take_out();
            } else if remaining.len() == 1 {
                *optimized_plan.node_mut(idx).data_mut() =
                    IR::Filter(Arc::new(remaining.into_iter().next().unwrap()));
            } else {
                *optimized_plan.node_mut(idx).data_mut() =
                    IR::Filter(Arc::new(tree!(ExprIR::And; remaining)));
            }

            changed = true;
            break; // Restart traversal after structural modification
        }

        if !changed {
            break;
        }
    }
}

/// Replaces label scan + ID filter with direct node ID lookup.
fn utilize_node_by_id(optimized_plan: &mut DynTree<IR>) {
    let indices = optimized_plan.root().indices::<Bfs>().collect::<Vec<_>>();

    for idx in indices {
        let mut filters = vec![];
        let node = match optimized_plan.node(idx).data() {
            IR::NodeByLabelScan(node) | IR::AllNodeScan(node) => node.clone(),
            _ => continue,
        };
        if let IR::Filter(filter) = optimized_plan.node(idx).parent().unwrap().data() {
            if let Some((id, op)) = get_id_filter(&filter.root(), &node.alias) {
                filters.push((id, op));
            } else if matches!(filter.root().data(), ExprIR::And) {
                for child in filter.root().children() {
                    if let Some((id, op)) = get_id_filter(&child, &node.alias) {
                        filters.push((id, op));
                    } else {
                        filters.clear();
                        break;
                    }
                }
            }
        }
        if !filters.is_empty() {
            let mut new_op = optimized_plan.node_mut(idx);
            if node.labels.is_empty() {
                *new_op.data_mut() = IR::NodeByIdSeek {
                    node: node.clone(),
                    filter: filters,
                };
            } else {
                *new_op.data_mut() = IR::NodeByLabelAndIdScan {
                    node: node.clone(),
                    filter: filters,
                };
            }
            new_op.parent_mut().unwrap().take_out();
        }
    }
}

/// Optimizes a query execution plan.
///
/// Applies all optimization passes to the plan and returns the optimized version.
/// The original plan is not modified.
///
/// # Arguments
/// * `plan` - The unoptimized execution plan
/// * `graph` - The graph (needed to check for index availability)
///
/// # Returns
/// An optimized copy of the plan
#[must_use]
pub fn optimize(
    plan: &DynTree<IR>,
    graph: &Graph,
) -> DynTree<IR> {
    let mut optimized_plan = plan.clone();

    push_filters_down(&mut optimized_plan);
    utilize_index(&mut optimized_plan, graph);
    utilize_node_by_id(&mut optimized_plan);

    optimized_plan
}

/// Checks if a node pattern has an indexed property filter.
fn get_index(
    graph: &Graph,
    node: &Arc<QueryNode<Arc<String>, Variable>>,
) -> Option<(
    Arc<QueryNode<Arc<String>, Variable>>,
    Arc<String>,
    DynTree<ExprIR<Variable>>,
)> {
    for label in node.labels.iter() {
        for attr in node.attrs.root().children() {
            if let ExprIR::String(attr_str) = attr.data()
                && graph.is_indexed(label, attr_str, &IndexType::Range)
            {
                return Some((
                    node.clone(),
                    attr_str.clone(),
                    tree!(
                        ExprIR::Eq,
                        tree!(
                            ExprIR::Property(attr_str.clone()),
                            tree!(ExprIR::Variable(node.alias.clone()))
                        ),
                        attr.child(0).as_cloned_subtree()
                    ),
                ));
            }
        }
    }
    None
}
