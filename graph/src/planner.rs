use std::{collections::HashSet, fmt::Display, rc::Rc};

use orx_tree::{DynTree, NodeRef};

use crate::{
    ast::{
        ExprIR, QueryGraph, QueryIR, QueryNode, QueryPath, QueryRelationship, SupportAggregation,
        Variable,
    },
    tree,
};

#[derive(Debug)]
pub enum IR {
    Empty,
    Optional(Vec<Variable>),
    Call(Rc<String>, Vec<DynTree<ExprIR>>),
    Unwind(DynTree<ExprIR>, Variable),
    Create(QueryGraph),
    Merge(QueryGraph),
    Delete(Vec<DynTree<ExprIR>>, bool),
    Set(Vec<(DynTree<ExprIR>, DynTree<ExprIR>, bool)>),
    Remove(Vec<DynTree<ExprIR>>),
    NodeScan(Rc<QueryNode>),
    RelationshipScan(Rc<QueryRelationship>),
    ExpandInto(Rc<QueryRelationship>),
    PathBuilder(Vec<Rc<QueryPath>>),
    Filter(DynTree<ExprIR>),
    CartesianProduct,
    Sort(Vec<(DynTree<ExprIR>, bool)>),
    Skip(DynTree<ExprIR>),
    Limit(DynTree<ExprIR>),
    Aggregate(
        Vec<Variable>,
        Vec<(Variable, DynTree<ExprIR>)>,
        Vec<(Variable, DynTree<ExprIR>)>,
    ),
    Project(Vec<(Variable, DynTree<ExprIR>)>),
    Distinct,
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
            Self::Set(_) => write!(f, "Set"),
            Self::Remove(_) => write!(f, "Remove"),
            Self::NodeScan(node) => write!(f, "NodeScan {node}"),
            Self::RelationshipScan(rel) => write!(f, "RelationshipScan {rel}"),
            Self::ExpandInto(rel) => write!(f, "ExpandInto {rel}"),
            Self::PathBuilder(_) => write!(f, "PathBuilder"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::CartesianProduct => write!(f, "CartesianProduct"),
            Self::Sort(_) => write!(f, "Sort"),
            Self::Skip(_) => write!(f, "Skip"),
            Self::Limit(_) => write!(f, "Limit"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::Project(_) => write!(f, "Project"),
            Self::Commit => write!(f, "Commit"),
            Self::Distinct => write!(f, "Distinct"),
        }
    }
}

#[derive(Default)]
pub struct Planner {
    visited: HashSet<u32>,
}

impl Planner {
    fn plan_match(
        &mut self,
        pattern: &QueryGraph,
    ) -> DynTree<IR> {
        let mut vec = vec![];
        for component in pattern.connected_components() {
            if component.relationships.is_empty() {
                debug_assert_eq!(component.nodes.len(), 1);
                let mut res = tree!(IR::NodeScan(component.nodes[0].clone()));
                self.visited.insert(component.nodes[0].alias.id);
                if !component.paths.is_empty() {
                    res = tree!(IR::PathBuilder(component.paths), res);
                }
                vec.push(res);
                continue;
            }
            let mut iter = component.relationships.into_iter();
            let relationship = iter.next().unwrap();
            let mut res = if relationship.from.alias.id == relationship.to.alias.id {
                tree!(
                    IR::ExpandInto(relationship.clone()),
                    tree!(IR::NodeScan(relationship.from.clone()))
                )
            } else {
                tree!(IR::RelationshipScan(relationship.clone()))
            };
            self.visited.insert(relationship.from.alias.id);
            self.visited.insert(relationship.to.alias.id);
            self.visited.insert(relationship.alias.id);
            for relationship in iter {
                res = if relationship.from.alias.id == relationship.to.alias.id {
                    tree!(
                        IR::ExpandInto(relationship.clone()),
                        tree!(IR::NodeScan(relationship.from.clone()), res)
                    )
                } else {
                    tree!(IR::RelationshipScan(relationship.clone()), res)
                };
                self.visited.insert(relationship.from.alias.id);
                self.visited.insert(relationship.to.alias.id);
                self.visited.insert(relationship.alias.id);
            }
            if !component.paths.is_empty() {
                res = tree!(IR::PathBuilder(component.paths), res);
            }
            vec.push(res);
        }
        if vec.len() == 1 {
            return vec.pop().unwrap();
        }
        tree!(IR::CartesianProduct; vec)
    }

    fn plan_project(
        &mut self,
        exprs: Vec<(Variable, DynTree<ExprIR>)>,
        orderby: Vec<(DynTree<ExprIR>, bool)>,
        skip: Option<DynTree<ExprIR>>,
        limit: Option<DynTree<ExprIR>>,
        write: bool,
        distinct: bool,
    ) -> DynTree<IR> {
        for expr in &exprs {
            self.visited.insert(expr.0.id);
        }
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
        if distinct {
            res = tree!(IR::Distinct, res);
        }
        if !orderby.is_empty() {
            res = tree!(IR::Sort(orderby), res);
        }
        if write {
            res = tree!(IR::Commit, res);
        }
        if let Some(skip_expr) = skip {
            res = tree!(IR::Skip(skip_expr), res);
        }
        if let Some(limit_expr) = limit {
            res = tree!(IR::Limit(limit_expr), res);
        }
        res
    }

    fn plan_query(
        &mut self,
        q: Vec<QueryIR>,
        write: bool,
    ) -> DynTree<IR> {
        let mut plans = Vec::with_capacity(q.len());
        for ir in q {
            plans.push(self.plan(ir));
        }
        let mut iter = plans.into_iter().rev();
        let mut res = iter.next().unwrap();
        let mut idx = res.root().idx();
        while matches!(res.node(&idx).data(), IR::Commit)
            || matches!(res.node(&idx).data(), IR::Sort(_))
            || matches!(res.node(&idx).data(), IR::Skip(_))
            || matches!(res.node(&idx).data(), IR::Limit(_))
            || matches!(res.node(&idx).data(), IR::Distinct)
        {
            idx = res.node(&idx).child(0).idx();
        }
        for n in iter {
            idx = res.node_mut(&idx).push_child_tree(n);
            while matches!(res.node(&idx).data(), IR::Commit)
                || matches!(res.node(&idx).data(), IR::Sort(_))
                || matches!(res.node(&idx).data(), IR::Skip(_))
                || matches!(res.node(&idx).data(), IR::Limit(_))
                || matches!(res.node(&idx).data(), IR::Distinct)
            {
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
        &mut self,
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
                                .chain(pattern.paths.iter().map(|p| p.var.clone()))
                                .filter(|v| !self.visited.contains(&v.id))
                                .collect()
                        ),
                        self.plan_match(&pattern)
                    )
                } else {
                    self.plan_match(&pattern)
                }
            }
            QueryIR::Unwind(expr, alias) => tree!(IR::Unwind(expr, alias)),
            QueryIR::Merge(pattern) => tree!(
                IR::Merge(pattern.filter_visited(&self.visited)),
                self.plan_match(&pattern)
            ),
            QueryIR::Create(pattern) => {
                tree!(IR::Create(pattern.filter_visited(&self.visited)))
            }
            QueryIR::Delete(exprs, is_detach) => tree!(IR::Delete(exprs, is_detach)),
            QueryIR::Set(items) => tree!(IR::Set(items)),
            QueryIR::Remove(items) => tree!(IR::Remove(items)),
            QueryIR::Where(expr) => tree!(IR::Filter(expr)),
            QueryIR::With {
                distinct,
                exprs,
                orderby,
                skip,
                limit,
                write,
                ..
            }
            | QueryIR::Return {
                distinct,
                exprs,
                orderby,
                skip,
                limit,
                write,
                ..
            } => self.plan_project(exprs, orderby, skip, limit, write, distinct),
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }
}
