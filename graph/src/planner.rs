use std::{fmt::Display, rc::Rc};

use orx_tree::{Dyn, DynTree, NodeMut, NodeRef};

use crate::{
    ast::{
        ExprIR, NodePattern, PathPattern, Pattern, QueryIR, RelationshipPattern, SupportAggregation,
    },
    functions::FnType,
    tree,
};

#[derive(Clone, Debug)]
pub enum IR {
    Empty,
    Optional(Vec<Rc<String>>),
    Call(Rc<String>, Vec<DynTree<ExprIR>>),
    Unwind(DynTree<ExprIR>, Rc<String>),
    UnwindRange(
        DynTree<ExprIR>,
        DynTree<ExprIR>,
        DynTree<ExprIR>,
        Rc<String>,
    ),
    Create(Pattern),
    Merge(Pattern),
    Delete(Vec<DynTree<ExprIR>>),
    NodeScan(NodePattern),
    RelationshipScan(RelationshipPattern),
    PathBuilder(Vec<PathPattern>),
    Filter(DynTree<ExprIR>),
    Aggregate(
        Vec<Rc<String>>,
        Vec<(Rc<String>, DynTree<ExprIR>)>,
        Vec<(Rc<String>, DynTree<ExprIR>)>,
    ),
    Project(Vec<(Rc<String>, DynTree<ExprIR>)>),
    Commit,
}

impl Display for IR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Optional(_) => write!(f, "Optional"),
            Self::Call(name, _) => write!(f, "Call({name})"),
            Self::Unwind(_, alias) => write!(f, "Unwind({alias})"),
            Self::UnwindRange(_, _, _, alias) => write!(f, "UnwindRange({alias})"),
            Self::Create(pattern) => write!(f, "Create {pattern}"),
            Self::Merge(pattern) => write!(f, "Merge {pattern}"),
            Self::Delete(_) => write!(f, "Delete"),
            Self::NodeScan(node) => write!(f, "NodeScan {node}"),
            Self::RelationshipScan(rel) => write!(f, "RelationshipScan {rel}"),
            Self::PathBuilder(_) => write!(f, "PathBuilder"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::Project(_) => write!(f, "Project"),
            Self::Commit => write!(f, "Commit"),
        }
    }
}

pub struct Planner {}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    fn plan_aggregation(
        acc_name: Rc<String>,
        expr: &mut NodeMut<Dyn<ExprIR>>,
    ) {
        match expr.data() {
            ExprIR::FuncInvocation(_, FnType::Aggregation) => {
                expr.push_child_tree(tree!(ExprIR::Var(acc_name)));
            }
            _ => unreachable!(),
        }
    }

    fn plan_match(
        &self,
        mut pattern: Pattern,
    ) -> DynTree<IR> {
        if pattern.relationships.is_empty() && !pattern.nodes.is_empty() {
            let mut iter = pattern.nodes.into_iter().rev();
            let mut res = tree!(IR::NodeScan(iter.next().unwrap()));
            for node in iter {
                res = tree!(IR::NodeScan(node), res);
            }
            if !pattern.paths.is_empty() {
                res = tree!(IR::PathBuilder(pattern.paths), res);
            }
            return res;
        }
        if pattern.relationships.len() == 1 {
            let mut res = tree!(IR::RelationshipScan(pattern.relationships.pop().unwrap()));
            if !pattern.paths.is_empty() {
                res = tree!(IR::PathBuilder(pattern.paths), res);
            }
            return res;
        }
        tree!(IR::Empty)
    }

    fn plan_unwind(
        &self,
        expr: orx_tree::Tree<Dyn<ExprIR>>,
        alias: Rc<String>,
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
        &self,
        exprs: Vec<(Rc<String>, DynTree<ExprIR>)>,
        write: bool,
    ) -> DynTree<IR> {
        let mut res = if exprs.iter().any(|e| e.1.root().is_aggregation()) {
            let mut group_by_keys = Vec::new();
            let mut aggregations = Vec::new();
            let mut names = Vec::new();
            for (name, mut expr) in exprs {
                names.push(name.clone());
                if expr.root().is_aggregation() {
                    Self::plan_aggregation(name.clone(), &mut expr.root_mut());
                    aggregations.push((name, expr));
                } else {
                    group_by_keys.push((name, expr));
                }
            }
            tree!(IR::Aggregate(names, group_by_keys, aggregations))
        } else {
            tree!(IR::Project(exprs))
        };
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    fn plan_query(
        &self,
        q: Vec<QueryIR>,
        write: bool,
    ) -> DynTree<IR> {
        let iter = &mut q.into_iter().rev();
        let mut res = self.plan(iter.next().unwrap());
        let mut idx = res.root().idx();
        if matches!(res.node(&idx).data(), IR::Commit) {
            idx = res.node(&idx).child(0).idx();
        }
        for e in iter {
            let n = self.plan(e);
            idx = res.node_mut(&idx).push_child_tree(n);
            if matches!(res.node(&idx).data(), IR::Commit) {
                idx = res.node(&idx).child(0).idx();
            }
        }
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    #[must_use]
    pub fn plan(
        &self,
        ir: QueryIR,
    ) -> DynTree<IR> {
        match ir {
            QueryIR::Call(name, exprs) => tree!(IR::Call(name, exprs)),
            QueryIR::Match(pattern, optional) => {
                if optional {
                    tree!(
                        IR::Optional(pattern.nodes.iter().map(|n| n.alias.to_string()).collect()),
                        self.plan_match(pattern)
                    )
                } else {
                    self.plan_match(pattern)
                }
            }
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, alias),
            QueryIR::Merge(pattern) => tree!(IR::Merge(pattern.clone()), self.plan_match(pattern)),
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
