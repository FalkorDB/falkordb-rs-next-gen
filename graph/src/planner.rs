use std::{collections::HashSet, fmt::Display, rc::Rc};

use orx_tree::{DynTree, NodeRef, Side};

use crate::{
    ast::{
        QueryExpr, QueryGraph, QueryIR, QueryNode, QueryPath, QueryRelationship,
        SupportAggregation, Variable,
    },
    indexer::IndexQuery,
    tree,
};

#[derive(Clone, Debug)]
pub enum IR {
    Empty,
    Optional(Vec<Variable>),
    Call(Rc<String>, Vec<QueryExpr>),
    Unwind(QueryExpr, Variable),
    Create(QueryGraph),
    Merge(
        QueryGraph,
        Vec<(QueryExpr, QueryExpr, bool)>,
        Vec<(QueryExpr, QueryExpr, bool)>,
    ),
    Delete(Vec<QueryExpr>, bool),
    Set(Vec<(QueryExpr, QueryExpr, bool)>),
    Remove(Vec<QueryExpr>),
    NodeByLabelScan(Rc<QueryNode>),
    NodeByIndexScan {
        node: Rc<QueryNode>,
        index: Rc<String>,
        query: Rc<IndexQuery<QueryExpr>>,
    },
    RelationshipScan(Rc<QueryRelationship>),
    ExpandInto(Rc<QueryRelationship>),
    PathBuilder(Vec<Rc<QueryPath>>),
    Filter(QueryExpr),
    CartesianProduct,
    LoadCsv {
        file_path: QueryExpr,
        headers: bool,
        delimiter: QueryExpr,
        var: Variable,
    },
    Sort(Vec<(QueryExpr, bool)>),
    Skip(QueryExpr),
    Limit(QueryExpr),
    Aggregate(
        Vec<Variable>,
        Vec<(Variable, QueryExpr)>,
        Vec<(Variable, QueryExpr)>,
    ),
    Project(Vec<(Variable, QueryExpr)>),
    Distinct,
    Commit,
    CreateIndex {
        label: Rc<String>,
        attrs: Vec<Rc<String>>,
    },
    DropIndex {
        label: Rc<String>,
        attrs: Vec<Rc<String>>,
    },
}

#[cfg_attr(tarpaulin, skip)]
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
            Self::Merge(pattern, _, _) => write!(f, "Merge {pattern}"),
            Self::Delete(_, _) => write!(f, "Delete"),
            Self::Set(_) => write!(f, "Set"),
            Self::Remove(_) => write!(f, "Remove"),
            Self::NodeByLabelScan(node) => write!(f, "Node By Label Scan {node}"),
            Self::NodeByIndexScan { node, .. } => {
                write!(f, "Node By Index Scan {node}")
            }
            Self::RelationshipScan(rel) => write!(f, "RelationshipScan {rel}"),
            Self::ExpandInto(rel) => write!(f, "ExpandInto {rel}"),
            Self::PathBuilder(_) => write!(f, "PathBuilder"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::CartesianProduct => write!(f, "Cartesian Product"),
            Self::LoadCsv { .. } => write!(f, "LoadCsv"),
            Self::Sort(_) => write!(f, "Sort"),
            Self::Skip(_) => write!(f, "Skip"),
            Self::Limit(_) => write!(f, "Limit"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::Project(_) => write!(f, "Project"),
            Self::Commit => write!(f, "Commit"),
            Self::Distinct => write!(f, "Distinct"),
            Self::CreateIndex { label, attrs } => {
                write!(f, "CreateIndex on :{label}({attrs:?})")
            }
            Self::DropIndex { label, attrs } => {
                write!(f, "DropIndex on :{label}({attrs:?})")
            }
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
        filter: Option<QueryExpr>,
    ) -> DynTree<IR> {
        let mut vec = vec![];
        for component in pattern.connected_components() {
            let relationships = component.relationships();
            if relationships.is_empty() {
                let nodes = component.nodes();
                debug_assert_eq!(nodes.len(), 1);
                let node = nodes[0].clone();
                let mut res = tree!(IR::NodeByLabelScan(node.clone()));
                self.visited.insert(node.alias.id);
                let paths = component.paths();
                if !paths.is_empty() {
                    res = tree!(IR::PathBuilder(paths), res);
                }
                vec.push(res);
                continue;
            }
            let mut iter = relationships.into_iter();
            let relationship = iter.next().unwrap();
            let mut res = if relationship.from.alias.id == relationship.to.alias.id {
                tree!(
                    IR::ExpandInto(relationship.clone()),
                    tree!(IR::NodeByLabelScan(relationship.from.clone()))
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
                        tree!(IR::NodeByLabelScan(relationship.from.clone()), res)
                    )
                } else {
                    tree!(IR::RelationshipScan(relationship.clone()), res)
                };
                self.visited.insert(relationship.from.alias.id);
                self.visited.insert(relationship.to.alias.id);
                self.visited.insert(relationship.alias.id);
            }
            let paths = component.paths();
            if !paths.is_empty() {
                res = tree!(IR::PathBuilder(paths), res);
            }
            vec.push(res);
        }
        let mut res = if vec.len() == 1 {
            vec.pop().unwrap()
        } else {
            tree!(IR::CartesianProduct; vec)
        };
        if let Some(filter) = filter {
            res = tree!(IR::Filter(filter), res);
        }
        res
    }

    #[allow(clippy::too_many_arguments)]
    fn plan_project(
        &mut self,
        exprs: Vec<(Variable, QueryExpr)>,
        orderby: Vec<(QueryExpr, bool)>,
        skip: Option<QueryExpr>,
        limit: Option<QueryExpr>,
        filter: Option<QueryExpr>,
        distinct: bool,
        write: bool,
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
        if let Some(filter) = filter {
            res = tree!(IR::Filter(filter), res);
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
        while matches!(
            res.node(&idx).data(),
            IR::Commit | IR::Sort(_) | IR::Skip(_) | IR::Limit(_) | IR::Distinct | IR::Filter(_)
        ) {
            idx = res.node(&idx).child(0).idx();
        }
        for n in iter {
            if res.node(&idx).num_children() > 0 {
                idx = res
                    .node_mut(&idx)
                    .child_mut(0)
                    .push_sibling_tree(Side::Left, n);
            } else {
                idx = res.node_mut(&idx).push_child_tree(n);
            }
            while matches!(
                res.node(&idx).data(),
                IR::Commit
                    | IR::Sort(_)
                    | IR::Skip(_)
                    | IR::Limit(_)
                    | IR::Distinct
                    | IR::Filter(_)
            ) {
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
            QueryIR::Match {
                pattern,
                filter,
                optional,
            } => {
                if optional {
                    tree!(
                        IR::Optional(
                            pattern
                                .variables()
                                .iter()
                                .filter(|v| !self.visited.contains(&v.id))
                                .cloned()
                                .collect()
                        ),
                        self.plan_match(&pattern, filter)
                    )
                } else {
                    self.plan_match(&pattern, filter)
                }
            }
            QueryIR::Unwind(expr, alias) => tree!(IR::Unwind(expr, alias)),
            QueryIR::Merge(pattern, on_create_set_items, on_match_set_items) => tree!(
                IR::Merge(
                    pattern.filter_visited(&self.visited),
                    on_create_set_items,
                    on_match_set_items
                ),
                self.plan_match(&pattern, None)
            ),
            QueryIR::Create(pattern) => {
                tree!(IR::Create(pattern.filter_visited(&self.visited)))
            }
            QueryIR::Delete(exprs, is_detach) => tree!(IR::Delete(exprs, is_detach)),
            QueryIR::Set(items) => tree!(IR::Set(items)),
            QueryIR::Remove(items) => tree!(IR::Remove(items)),
            QueryIR::LoadCsv {
                file_path,
                headers,
                delimiter,
                var,
            } => {
                tree!(IR::LoadCsv {
                    file_path,
                    headers,
                    delimiter,
                    var,
                })
            }
            QueryIR::With {
                distinct,
                exprs,
                orderby,
                skip,
                limit,
                filter,
                write,
                ..
            } => self.plan_project(exprs, orderby, skip, limit, filter, distinct, write),
            QueryIR::Return {
                distinct,
                exprs,
                orderby,
                skip,
                limit,
                write,
                ..
            } => self.plan_project(exprs, orderby, skip, limit, None, distinct, write),
            QueryIR::CreateIndex { label, attrs } => tree!(IR::CreateIndex { label, attrs }),
            QueryIR::DropIndex { label, attrs } => {
                tree!(IR::DropIndex { label, attrs })
            }
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }
}
