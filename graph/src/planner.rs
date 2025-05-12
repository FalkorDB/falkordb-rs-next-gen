use std::fmt::Display;

use orx_tree::{Dyn, DynTree, NodeMut, NodeRef};

use crate::{
    ast::{ExprIR, NodePattern, Pattern, QueryIR, RelationshipPattern, SupportAggregation},
    functions::FnType,
    tree,
};

#[derive(Clone, Debug)]
pub enum IR {
    Empty,
    Call(String, Vec<DynTree<ExprIR>>),
    Unwind(DynTree<ExprIR>, String),
    UnwindRange(DynTree<ExprIR>, DynTree<ExprIR>, DynTree<ExprIR>, String),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>),
    NodeScan(NodePattern),
    RelationshipScan(RelationshipPattern),
    Filter(DynTree<ExprIR>),
    Aggregate(String, Vec<DynTree<ExprIR>>, Vec<DynTree<ExprIR>>),
    Project(Vec<DynTree<ExprIR>>),
    Commit,
}

impl Display for IR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Call(name, _) => write!(f, "Call({name})"),
            Self::Unwind(_, alias) => write!(f, "Unwind({alias})"),
            Self::UnwindRange(_, _, _, alias) => write!(f, "UnwindRange({alias})"),
            Self::Create(pattern) => write!(f, "Create {pattern}"),
            Self::Delete(_) => write!(f, "Delete"),
            Self::NodeScan(node) => write!(f, "NodeScan {node}"),
            Self::RelationshipScan(rel) => write!(f, "RelationshipScan {rel}"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::Project(_) => write!(f, "Project"),
            Self::Commit => write!(f, "Commit"),
        }
    }
}

pub struct Planner {
    var_id: u64,
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner {
    #[must_use]
    pub const fn new() -> Self {
        Self { var_id: 0 }
    }

    fn next_var(&mut self) -> String {
        let id = self.var_id;
        self.var_id += 1;
        format!("var_{id}")
    }

    fn plan_aggregation(
        agg_ctx_var: String,
        expr: &mut NodeMut<Dyn<ExprIR>>,
    ) {
        match expr.data() {
            ExprIR::FuncInvocation(_, FnType::Aggregation) => {
                expr.push_child_tree(tree!(ExprIR::Var(agg_ctx_var)));
            }
            ExprIR::Set(_) => {
                Self::plan_aggregation(agg_ctx_var, &mut expr.child_mut(0));
            }
            _ => unreachable!(),
        }
    }

    fn plan_match(
        &mut self,
        mut pattern: Pattern,
    ) -> DynTree<IR> {
        if pattern.relationships.is_empty() && !pattern.nodes.is_empty() {
            let mut iter = pattern.nodes.into_iter().rev();
            let mut body = tree!(IR::NodeScan(iter.next().unwrap()));
            for node in iter {
                body = tree!(IR::NodeScan(node), body);
            }
            return body;
        }
        if pattern.relationships.len() == 1 {
            return tree!(IR::RelationshipScan(pattern.relationships.pop().unwrap()));
        }
        tree!(IR::Empty)
    }

    fn plan_unwind(
        &mut self,
        expr: orx_tree::Tree<Dyn<ExprIR>>,
        alias: String,
    ) -> orx_tree::Tree<Dyn<IR>> {
        let root = expr.root();
        if matches!(root.data(), ExprIR::FuncInvocation(name, _) if name == "range") {
            let start = root.child(0).clone_as_tree();
            let end = root.child(1).clone_as_tree();
            let step = root
                .get_child(2)
                .map_or_else(|| tree!(ExprIR::Integer(1)), |v| v.clone_as_tree());
            return tree!(IR::UnwindRange(start, end, step, alias));
        }
        tree!(IR::Unwind(expr, alias))
    }

    fn plan_project(
        &mut self,
        exprs: Vec<DynTree<ExprIR>>,
        write: bool,
    ) -> DynTree<IR> {
        let mut res = if exprs.iter().any(|e| e.root().is_aggregation()) {
            let mut group_by_keys = Vec::new();
            let mut aggregations = Vec::new();
            let agg_ctx_var = self.next_var();
            for mut expr in exprs {
                if expr.root().is_aggregation() {
                    Self::plan_aggregation(agg_ctx_var.to_string(), &mut expr.root_mut());
                    aggregations.push(expr);
                } else {
                    group_by_keys.push(expr);
                }
            }
            tree!(IR::Aggregate(agg_ctx_var, group_by_keys, aggregations))
        } else {
            tree!(IR::Project(exprs))
        };
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    fn plan_query(
        &mut self,
        q: Vec<QueryIR>,
        write: bool,
    ) -> DynTree<IR> {
        let iter = &mut q.into_iter();
        let mut res = self.plan(iter.next().unwrap());
        for e in iter {
            let mut n = self.plan(e);
            let mut root = n.root_mut();
            if root.num_children() == 1 {
                let mut child = root.child_mut(0);
                child.push_child_tree(res);
            } else {
                root.push_child_tree(res);
            }
            res = n;
        }
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    #[must_use]
    pub fn plan(
        &mut self,
        ir: QueryIR,
    ) -> DynTree<IR> {
        match ir {
            QueryIR::Call(name, exprs) => tree!(IR::Call(name, exprs)),
            QueryIR::Match(pattern) => self.plan_match(pattern),
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, alias),
            QueryIR::Where(expr) => tree!(IR::Filter(expr)),
            QueryIR::Create(pattern) => tree!(IR::Create(pattern)),
            QueryIR::Delete(exprs) => tree!(IR::Delete(exprs)),
            QueryIR::With(exprs, write) | QueryIR::Return(exprs, write) => {
                self.plan_project(exprs, write)
            }
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }
}
