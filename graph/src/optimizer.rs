use std::rc::Rc;

use orx_tree::{Bfs, DynTree, NodeRef};

use crate::{
    ast::{ExprIR, QueryNode},
    graph::graph::Graph,
    indexer::IndexQuery,
    planner::IR,
    runtime::functions::{FnType, get_functions},
    tree,
};

pub fn optimize(
    plan: &DynTree<IR>,
    graph: &Graph,
) -> DynTree<IR> {
    let mut optimized_plan = plan.clone();

    let indices = optimized_plan.root().indices::<Bfs>().collect::<Vec<_>>();

    for index in indices {
        // let node = if let IR::NodeByLabelScan(node) = optimized_plan.node(&index).data()
        //     && let IR::Filter(filter) = optimized_plan.node(&index).parent().unwrap().data()
        //     && let ExprIR::FuncInvocation(func) = filter.root().data()
        //     && func.name == "contains"
        //     && let ExprIR::FuncInvocation(inner_func) = filter.root().child(0).data()
        //     && inner_func.name == "property"
        //     && let ExprIR::String(attr) = filter.root().child(0).child(1).data()
        //     && graph.is_indexed(&node.labels[0], attr)
        // {
        //     Some((node.clone(), attr.clone(), filter.clone()))
        // } else {
        //     None
        // };
        // if let Some((node, attr, filter)) = node {
        //     let mut op = optimized_plan.node_mut(&index);
        //     *op.data_mut() = IR::NodeByIndexScan {
        //         node: node.clone(),
        //         index: node.labels[0].clone(),
        //         query: Rc::new(IndexQuery::Contains(
        //             attr.clone(),
        //             Rc::new(filter.root().child(1).clone_as_tree()),
        //         )),
        //     };
        //     op.parent_mut().unwrap().take_out();
        // }

        let node = if let IR::NodeByLabelScan(node) = optimized_plan.node(&index).data() {
            get_index(graph, node)
        } else {
            None
        };
        if let Some((node, attr, filter)) = node {
            let mut op = optimized_plan.node_mut(&index);
            *op.data_mut() = IR::NodeByIndexScan {
                node: node.clone(),
                index: node.labels[0].clone(),
                query: Rc::new(IndexQuery::Equal(
                    attr.clone(),
                    Rc::new(filter.root().child(1).clone_as_tree()),
                )),
            };
        }
    }

    optimized_plan
}

fn get_index(
    graph: &Graph,
    node: &Rc<crate::ast::QueryNode>,
) -> Option<(Rc<QueryNode>, Rc<String>, DynTree<ExprIR>)> {
    for label in &node.labels {
        for attr in node.attrs.root().children() {
            if let ExprIR::String(attr_str) = attr.data()
                && graph.is_indexed(label, attr_str)
            {
                return Some((
                    node.clone(),
                    attr_str.clone(),
                    tree!(
                        ExprIR::Eq,
                        tree!(
                            ExprIR::FuncInvocation(
                                get_functions().get("property", &FnType::Internal).unwrap()
                            ),
                            tree!(ExprIR::Variable(node.alias.clone())),
                            tree!(ExprIR::String(attr_str.clone()))
                        ),
                        attr.child(0).as_cloned_subtree()
                    ),
                ));
            }
        }
    }
    None
}
