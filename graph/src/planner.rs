use std::{collections::HashSet, fmt::Display, sync::Arc};

use orx_tree::{DynTree, NodeRef, Side, Traversal, Traverser};

use crate::{
    ast::{
        ExprIR, QueryExpr, QueryGraph, QueryIR, QueryNode, QueryPath, QueryRelationship, SetItem,
        SupportAggregation, Variable,
    },
    indexer::{EntityType, IndexQuery, IndexType},
    runtime::functions::GraphFn,
    tree,
};

#[derive(Clone, Debug)]
pub enum IR {
    Empty,
    Argument,
    Optional(Vec<Variable>),
    Call(Arc<GraphFn>, Vec<QueryExpr<Variable>>, Vec<(Arc<String>, Variable)>),
    Unwind(QueryExpr<Variable>, Variable),
    Create(QueryGraph<Arc<String>, Arc<String>, Variable>),
    Merge(
        QueryGraph<Arc<String>, Arc<String>, Variable>,
        Vec<SetItem<Arc<String>, Variable>>,
        Vec<SetItem<Arc<String>, Variable>>,
    ),
    Delete(Vec<QueryExpr<Variable>>, bool),
    Set(Vec<SetItem<Arc<String>, Variable>>),
    Remove(Vec<QueryExpr<Variable>>),
    NodeByLabelScan(Arc<QueryNode<Arc<String>, Variable>>),
    NodeByIndexScan {
        node: Arc<QueryNode<Arc<String>, Variable>>,
        index: Arc<String>,
        query: Arc<IndexQuery<QueryExpr<Variable>>>,
    },
    NodeByIdScan {
        node: Arc<QueryNode<Arc<String>, Variable>>,
        id: QueryExpr<Variable>,
        op: ExprIR<Variable>,
    },
    CondTraverse(Arc<QueryRelationship<Arc<String>, Arc<String>, Variable>>),
    ExpandInto(Arc<QueryRelationship<Arc<String>, Arc<String>, Variable>>),
    PathBuilder(Vec<Arc<QueryPath<Variable>>>),
    Filter(QueryExpr<Variable>),
    CartesianProduct,
    LoadCsv {
        file_path: QueryExpr<Variable>,
        headers: bool,
        delimiter: QueryExpr<Variable>,
        var: Variable,
    },
    Sort(Vec<(QueryExpr<Variable>, bool)>),
    Skip(QueryExpr<Variable>),
    Limit(QueryExpr<Variable>),
    Aggregate(
        Vec<Variable>,
        Vec<(Variable, QueryExpr<Variable>)>,
        Vec<(Variable, QueryExpr<Variable>)>,
    ),
    Project(
        Vec<(Variable, QueryExpr<Variable>)>,
        Vec<(Variable, Variable)>,
    ),
    Distinct,
    Commit,
    CreateIndex {
        label: Arc<String>,
        attrs: Vec<Arc<String>>,
        index_type: IndexType,
        entity_type: EntityType,
        options: Option<QueryExpr<Variable>>,
    },
    DropIndex {
        label: Arc<String>,
        attrs: Vec<Arc<String>>,
        index_type: IndexType,
        entity_type: EntityType,
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
            Self::Argument => write!(f, "Argument"),
            Self::Optional(_) => write!(f, "Optional"),
            Self::Call(_, _, _) => write!(f, "Call"),
            Self::Unwind(_, _) => {
                write!(f, "Unwind")
            }
            Self::Create(pattern) => write!(f, "Create | {pattern}"),
            Self::Merge(pattern, _, _) => write!(f, "Merge | {pattern}"),
            Self::Delete(_, _) => write!(f, "Delete"),
            Self::Set(_) => write!(f, "Set"),
            Self::Remove(_) => write!(f, "Remove"),
            Self::NodeByLabelScan(node) => write!(f, "Node By Label Scan | {node}"),
            Self::NodeByIndexScan { node, .. } => {
                write!(f, "Node By Index Scan | {node}")
            }
            Self::NodeByIdScan { node, .. } => {
                write!(f, "Node By Label and ID Scan | {node}")
            }
            Self::CondTraverse(rel) => write!(f, "Conditional Traverse | {rel}"),
            Self::ExpandInto(rel) => write!(f, "Expand Into | {rel}"),
            Self::PathBuilder(_) => write!(f, "PathBuilder"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::CartesianProduct => write!(f, "Cartesian Product"),
            Self::LoadCsv { .. } => write!(f, "Load CSV"),
            Self::Sort(_) => write!(f, "Sort"),
            Self::Skip(_) => write!(f, "Skip"),
            Self::Limit(_) => write!(f, "Limit"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::Project(_, _) => write!(f, "Project"),
            Self::Commit => write!(f, "Commit"),
            Self::Distinct => write!(f, "Distinct"),
            Self::CreateIndex { label, attrs, .. } => {
                write!(f, "Create Index | :{label}({attrs:?})")
            }
            Self::DropIndex { label, attrs, .. } => {
                write!(f, "Drop Index | :{label}({attrs:?})")
            }
        }
    }
}

#[derive(Default)]
pub struct Planner {
    visited: HashSet<u32>,
}

impl Planner {
    fn add_argument_to_leaves(tree: &mut DynTree<IR>) {
        let mut tr = Traversal.bfs().over_nodes();

        let leaves: Vec<_> = tree
            .root()
            .walk_with(&mut tr)
            .filter(orx_tree::NodeRef::is_leaf)
            .map(|x| x.idx())
            .collect();

        // Add Argument node as a child to each leaf
        for leaf_idx in leaves {
            tree.node_mut(leaf_idx).push_child(IR::Argument);
        }
    }

    fn plan_match(
        &mut self,
        pattern: &QueryGraph<Arc<String>, Arc<String>, Variable>,
        filter: Option<QueryExpr<Variable>>,
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
                tree!(IR::CondTraverse(relationship.clone()))
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
                    tree!(IR::CondTraverse(relationship.clone()), res)
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
        exprs: Vec<(Variable, QueryExpr<Variable>)>,
        copy_from_parent: Vec<(Variable, Variable)>,
        orderby: Vec<(QueryExpr<Variable>, bool)>,
        skip: Option<QueryExpr<Variable>>,
        limit: Option<QueryExpr<Variable>>,
        filter: Option<QueryExpr<Variable>>,
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
            tree!(IR::Project(exprs, copy_from_parent))
        };
        if write {
            res.root_mut().push_child(IR::Commit);
        }
        if distinct {
            res = tree!(IR::Distinct, res);
        }
        if !orderby.is_empty() {
            res = tree!(IR::Sort(orderby), res);
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
        q: Vec<QueryIR<Variable>>,
        write: bool,
    ) -> DynTree<IR> {
        let mut plans = Vec::with_capacity(q.len());
        for ir in q {
            plans.push(self.plan(ir));
        }
        let mut iter = plans.into_iter().rev();
        let mut res = iter.next().unwrap();
        let mut idx = res.root().idx();
        while matches!(res.node(idx).data(), |IR::Sort(_)| IR::Skip(_)
            | IR::Limit(_)
            | IR::Distinct
            | IR::Filter(_))
        {
            idx = res.node(idx).child(0).idx();
        }
        if matches!(res.node(idx).data(), |IR::Project(_, _)| IR::Aggregate(
            _,
            _,
            _
        )) && res.node(idx).num_children() > 0
            && matches!(res.node(idx).child(0).data(), IR::Commit)
        {
            idx = res.node(idx).child(0).idx();
        }
        for n in iter {
            if res.node(idx).num_children() > 0 {
                idx = res
                    .node_mut(idx)
                    .child_mut(0)
                    .push_sibling_tree(Side::Left, n);
            } else {
                idx = res.node_mut(idx).push_child_tree(n);
            }
            while matches!(res.node(idx).data(), |IR::Sort(_)| IR::Skip(_)
                | IR::Limit(_)
                | IR::Distinct
                | IR::Filter(_))
            {
                idx = res.node(idx).child(0).idx();
            }
            if matches!(res.node(idx).data(), |IR::Project(_, _)| IR::Aggregate(
                _,
                _,
                _
            )) && res.node(idx).num_children() > 0
                && matches!(res.node(idx).child(0).data(), IR::Commit)
            {
                idx = res.node(idx).child(0).idx();
            }
        }
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn plan(
        &mut self,
        ir: QueryIR<Variable>,
    ) -> DynTree<IR> {
        match ir {
            QueryIR::Call(proc, exprs, field_alias_pairs, filter) => {
                if let Some(filter) = filter {
                    return tree!(
                        IR::Filter(filter),
                        tree!(IR::Call(proc, exprs, field_alias_pairs))
                    );
                }
                if proc.name == "db.idx.fulltext.drop" {
                    let ExprIR::String(label) = exprs[0].root().data() else {
                        unreachable!()
                    };
                    return tree!(IR::DropIndex {
                        label: label.clone(),
                        attrs: vec![],
                        index_type: IndexType::Fulltext,
                        entity_type: EntityType::Node,
                    });
                }
                if proc.name == "db.idx.fulltext.createNodeIndex" {
                    let label = match exprs[0].root().data() {
                        ExprIR::String(label) => label.clone(),
                        ExprIR::Map => {
                            let mut ret = None;
                            for child in exprs[0].root().children() {
                                if let ExprIR::String(label) = child.data()
                                    && label.as_str() == "label"
                                {
                                    ret = Some(label.clone());
                                    break;
                                }
                            }
                            ret.unwrap_or_else(|| {
                                unreachable!();
                            })
                        }
                        _ => unreachable!(),
                    };
                    return tree!(IR::CreateIndex {
                        label,
                        attrs: vec![],
                        index_type: IndexType::Fulltext,
                        entity_type: EntityType::Node,
                        options: None,
                    });
                }
                tree!(IR::Call(proc, exprs, field_alias_pairs))
            }
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
            QueryIR::Merge(pattern, on_create_set_items, on_match_set_items) => {
                let create_pattern = pattern.filter_visited(&self.visited);
                let mut match_branch = self.plan_match(&pattern, None);
                Self::add_argument_to_leaves(&mut match_branch);

                tree!(
                    IR::Merge(create_pattern, on_create_set_items, on_match_set_items),
                    match_branch
                )
            }
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
                copy_from_parent,
                orderby,
                skip,
                limit,
                filter,
                write,
                ..
            } => self.plan_project(
                exprs,
                copy_from_parent,
                orderby,
                skip,
                limit,
                filter,
                distinct,
                write,
            ),
            QueryIR::Return {
                distinct,
                exprs,
                copy_from_parent,
                orderby,
                skip,
                limit,
                write,
                ..
            } => self.plan_project(
                exprs,
                copy_from_parent,
                orderby,
                skip,
                limit,
                None,
                distinct,
                write,
            ),
            QueryIR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options,
            } => tree!(IR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options
            }),
            QueryIR::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            } => {
                tree!(IR::DropIndex {
                    label,
                    attrs,
                    index_type,
                    entity_type
                })
            }
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }
}
