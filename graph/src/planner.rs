use std::{fmt::Display, rc::Rc};

use orx_tree::{Dyn, DynTree, NodeRef};

use crate::{
    ast::{
        ExprIR, NodePattern, PathPattern, Pattern, QueryIR, RelationshipPattern,
        SupportAggregation, VarId,
    },
    tree,
};

#[derive(Debug)]
pub enum IR {
    Empty,
    Optional(Vec<VarId>),
    Call(Rc<String>, Vec<DynTree<ExprIR>>),
    Unwind(DynTree<ExprIR>, VarId),
    Create(Pattern),
    Merge(Pattern),
    Delete(Vec<DynTree<ExprIR>>, bool),
    NodeScan(NodePattern),
    RelationshipScan(RelationshipPattern),
    PathBuilder(Vec<PathPattern>),
    Filter(DynTree<ExprIR>),
    Aggregate(
        Vec<VarId>,
        Vec<(VarId, DynTree<ExprIR>)>,
        Vec<(VarId, DynTree<ExprIR>)>,
    ),
    Project(Vec<(VarId, DynTree<ExprIR>)>),
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
            Self::Unwind(_, alias) => {
                write!(f, "Unwind({})", alias.as_str())
            }
            Self::Create(pattern) => write!(f, "Create {pattern}"),
            Self::Merge(pattern) => write!(f, "Merge {pattern}"),
            Self::Delete(_, _) => write!(f, "Delete"),
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

    fn plan_project(
        &self,
        exprs: Vec<(VarId, DynTree<ExprIR>)>,
        write: bool,
    ) -> DynTree<IR> {
        let mut res = if exprs.iter().any(|e| e.1.is_aggregation()) {
            let mut group_by_keys = Vec::new();
            let mut aggregations = Vec::new();
            let mut names = Vec::new();
            for (name, expr) in exprs {
                names.push(name.clone());
                if expr.is_aggregation() {
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
                        IR::Optional(
                            pattern
                                .nodes
                                .iter()
                                .map(|n| n.alias.clone())
                                .chain(pattern.relationships.iter().map(|r| r.alias.clone()))
                                .collect()
                        ),
                        self.plan_match(pattern)
                    )
                } else {
                    self.plan_match(pattern)
                }
            }
            QueryIR::Unwind(expr, alias) => tree!(IR::Unwind(expr, alias)),
            QueryIR::Merge(pattern) => tree!(IR::Merge(pattern.clone()), self.plan_match(pattern)),
            QueryIR::Where(expr) => tree!(IR::Filter(expr)),
            QueryIR::Create(pattern) => tree!(IR::Create(pattern)),
            QueryIR::Delete(exprs, is_detach) => tree!(IR::Delete(exprs, is_detach)),
            QueryIR::With(exprs, write) | QueryIR::Return(exprs, write) => {
                self.plan_project(exprs, write)
            }
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }
}
