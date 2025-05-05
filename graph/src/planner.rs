use std::{
    fmt::{Debug, Display},
    vec::IntoIter,
};

use orx_tree::{Dyn, DynTree, NodeMut, NodeRef};

use crate::{
    ast::{Alias, ExprIR, NodePattern, Pattern, QueryIR, RelationshipPattern, SupportAggregation},
    tree,
};

#[derive(Clone, Debug)]
pub enum IR {
    Expr(DynTree<ExprIR>),
    If,
    For,
    Return,
    ReturnAggregation,
    Block,
}

impl Display for IR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Expr(expr) => expr.root().data().fmt(f),
            Self::If => write!(f, "if"),
            Self::For => write!(f, "for"),
            Self::Return => write!(f, "return"),
            Self::ReturnAggregation => write!(f, "return_aggregation"),
            Self::Block => write!(f, "block"),
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
        &mut self,
        agg_ctx_var: String,
        expr: &mut NodeMut<Dyn<ExprIR>>,
    ) {
        match expr.data() {
            ExprIR::FuncInvocation(_) => {
                expr.push_child_tree(tree!(ExprIR::Var(agg_ctx_var)));
            }
            ExprIR::Set(_) => {
                self.plan_aggregation(agg_ctx_var, &mut expr.child_mut(0));
            }
            _ => unreachable!(),
        }
    }

    fn plan_create(
        &mut self,
        pattern: Pattern,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        if let Some(body_ir) = iter.next() {
            tree!(
                IR::Block ;
                pattern.nodes.into_iter().map(|n| {
                    tree!(IR::Expr(tree!(
                        ExprIR::Set(n.alias.to_string()),
                        tree!(
                            ExprIR::FuncInvocation(String::from("create_node")),
                            tree!(ExprIR::List ; n
                                .labels
                                .iter()
                                .map(|l| tree!(ExprIR::String((*l).to_string())))),
                            n.attrs
                        )
                    )))
                }),
                pattern.relationships.into_iter().map(|l| {
                    tree!(IR::Expr(tree!(
                        ExprIR::Set(l.alias.to_string()),
                        tree!(
                            ExprIR::FuncInvocation(String::from("create_relationship")),
                            tree!(ExprIR::String(l.relationship_type.to_string())),
                            tree!(ExprIR::Var(l.from.to_string())),
                            tree!(ExprIR::Var(l.to.to_string())),
                            l.attrs
                        )
                    )))
                }),
                [self.plan_query(body_ir, iter)]
            )
        } else {
            tree!(
                IR::Block ;
                pattern.nodes.into_iter().map(|n| {
                    tree!(IR::Expr(tree!(
                        ExprIR::Set(n.alias.to_string()),
                        tree!(
                            ExprIR::FuncInvocation(String::from("create_node")),
                            tree!(ExprIR::List ; n
                                .labels
                                .iter()
                                .map(|l| tree!(ExprIR::String((*l).to_string())))),
                            n.attrs
                        )
                    )))
                }),
                pattern.relationships.into_iter().map(|l| {
                    tree!(IR::Expr(tree!(
                        ExprIR::Set(l.alias.to_string()),
                        tree!(
                            ExprIR::FuncInvocation(String::from("create_relationship")),
                            tree!(ExprIR::String(l.relationship_type.to_string())),
                            tree!(ExprIR::Var(l.from.to_string())),
                            tree!(ExprIR::Var(l.to.to_string())),
                            l.attrs
                        )
                    )))
                })
            )
        }
    }

    fn plan_delete(
        &mut self,
        exprs: Vec<DynTree<ExprIR>>,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        if let Some(body_ir) = iter.next() {
            tree!(
                IR::Block,
                tree!(IR::Expr(
                    tree!(ExprIR::FuncInvocation(String::from("delete_entity")) ; exprs)
                )),
                self.plan_query(body_ir, iter)
            )
        } else {
            tree!(IR::Expr(
                tree!(ExprIR::FuncInvocation(String::from("delete_entity")) ; exprs)
            ))
        }
    }

    fn plan_unwind(
        &mut self,
        list: DynTree<ExprIR>,
        iter: &mut IntoIter<QueryIR>,
        alias: String,
    ) -> DynTree<IR> {
        if matches!(list.root().data(), ExprIR::FuncInvocation(f) if f == &"range".to_string()) {
            let init = tree!(IR::Expr(tree!(
                ExprIR::Set(alias.to_string()),
                list.root().child(0).as_cloned_subtree()
            )));
            let condition = tree!(IR::Expr(tree!(
                ExprIR::Le,
                tree!(ExprIR::Var(alias.to_string())),
                list.root().child(1).as_cloned_subtree()
            )));
            let next = tree!(IR::Expr(tree!(
                ExprIR::Set(alias.to_string()),
                tree!(
                    ExprIR::Add,
                    tree!(ExprIR::Var(alias)),
                    list.root()
                        .get_child(2)
                        .unwrap_or(tree!(ExprIR::Integer(1)).root())
                        .as_cloned_subtree()
                )
            )));
            let body_ir = iter.next().unwrap();
            let body = self.plan_query(body_ir, iter);
            tree!(IR::For, init, condition, next, body)
        } else {
            let list_var = self.next_var();
            let index_var = self.next_var();
            let list = tree!(IR::Expr(tree!(ExprIR::Set(list_var.to_string()), list)));
            let init = tree!(IR::Expr(tree!(
                ExprIR::Set(index_var.to_string()),
                tree!(ExprIR::Integer(0))
            )));
            let condition = tree!(IR::Expr(tree!(
                ExprIR::Lt,
                tree!(ExprIR::Var(index_var.to_string())),
                tree!(ExprIR::Length, tree!(ExprIR::Var(list_var.to_string())))
            )));
            let next = tree!(IR::Expr(tree!(
                ExprIR::Set(index_var.to_string()),
                tree!(
                    ExprIR::Add,
                    tree!(ExprIR::Var(index_var.to_string())),
                    tree!(ExprIR::Integer(1))
                )
            )));
            let body_ir = iter.next().unwrap();
            let body = self.plan_query(body_ir, iter);
            tree!(
                IR::Block,
                list,
                tree!(
                    IR::For,
                    init,
                    condition,
                    next,
                    tree!(
                        IR::Block,
                        tree!(IR::Expr(tree!(
                            ExprIR::Set(alias),
                            tree!(
                                ExprIR::GetElement,
                                tree!(ExprIR::Var(list_var)),
                                tree!(ExprIR::Var(index_var))
                            )
                        ))),
                        body
                    )
                )
            )
        }
    }

    fn plan_node_scan(
        &mut self,
        node: NodePattern,
        body: DynTree<IR>,
    ) -> DynTree<IR> {
        let init = tree!(
            IR::Block,
            tree!(IR::Expr(tree!(
                ExprIR::Set(format!("iter_{}", node.alias)),
                tree!(
                    ExprIR::FuncInvocation(String::from("create_node_iter")),
                    tree!(ExprIR::List ; node
                            .labels
                            .into_iter()
                            .map(|label| tree!(ExprIR::String(label))))
                )
            ))),
            tree!(IR::Expr(tree!(
                ExprIR::Set(node.alias.to_string()),
                tree!(
                    ExprIR::FuncInvocation(String::from("next_node")),
                    tree!(ExprIR::Var(format!("iter_{}", node.alias)))
                )
            )))
        );
        let condition = tree!(IR::Expr(tree!(
            ExprIR::IsNode,
            tree!(ExprIR::Var(node.alias.to_string()))
        )));
        let next = tree!(IR::Expr(tree!(
            ExprIR::Set(node.alias.to_string()),
            tree!(
                ExprIR::FuncInvocation(String::from("next_node")),
                tree!(ExprIR::Var(format!("iter_{}", node.alias)))
            )
        )));
        tree!(IR::For, init, condition, next, body)
    }

    fn plan_relationship_scan(
        &mut self,
        rel: &RelationshipPattern,
        body: DynTree<IR>,
    ) -> DynTree<IR> {
        let mut init = tree!(
            IR::Block,
            tree!(IR::Expr(tree!(
                ExprIR::Set(format!("iter_{}", rel.alias)),
                tree!(
                    ExprIR::FuncInvocation(String::from("create_relationship_iter")),
                    tree!(ExprIR::String(rel.relationship_type.to_string()))
                )
            ))),
            tree!(IR::Expr(tree!(
                ExprIR::Set(rel.alias.to_string()),
                tree!(
                    ExprIR::FuncInvocation(String::from("next_relationship")),
                    tree!(ExprIR::Var(format!("iter_{}", rel.alias)))
                )
            )))
        );
        let condition = tree!(IR::Expr(tree!(
            ExprIR::IsRelationship,
            tree!(ExprIR::Var(rel.alias.to_string()))
        )));
        let mut next = tree!(
            IR::Block,
            tree!(IR::Expr(tree!(
                ExprIR::Set(rel.alias.to_string()),
                tree!(
                    ExprIR::FuncInvocation("next_relationship".to_string()),
                    tree!(ExprIR::Var(format!("iter_{}", rel.alias)))
                )
            )))
        );
        if let Alias::String(from) = &rel.from {
            init.root_mut().push_child_tree(tree!(IR::Expr(tree!(
                ExprIR::Set(from.to_string()),
                tree!(
                    ExprIR::FuncInvocation("startnode".to_string()),
                    tree!(ExprIR::Var(rel.alias.to_string()))
                )
            ))));
            next.root_mut().push_child_tree(tree!(IR::Expr(tree!(
                ExprIR::Set(from.to_string()),
                tree!(
                    ExprIR::FuncInvocation("startnode".to_string()),
                    tree!(ExprIR::Var(rel.alias.to_string()))
                )
            ))));
        }
        if let Alias::String(to) = &rel.to {
            init.root_mut().push_child_tree(tree!(IR::Expr(tree!(
                ExprIR::Set(to.to_string()),
                tree!(
                    ExprIR::FuncInvocation("endnode".to_string()),
                    tree!(ExprIR::Var(rel.alias.to_string()))
                )
            ))));
            next.root_mut().push_child_tree(tree!(IR::Expr(tree!(
                ExprIR::Set(to.to_string()),
                tree!(
                    ExprIR::FuncInvocation("endnode".to_string()),
                    tree!(ExprIR::Var(rel.alias.to_string()))
                )
            ))));
        }
        tree!(IR::For, init, condition, next, body)
    }

    fn plan_match(
        &mut self,
        pattern: Pattern,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        if pattern.relationships.is_empty() {
            let mut body = self.plan_query(iter.next().unwrap(), iter);
            for node in pattern.nodes.into_iter().rev() {
                body = self.plan_node_scan(node, body);
            }
            return body;
        }
        if pattern.relationships.len() == 1 {
            let rel = &pattern.relationships[0];
            let body = self.plan_query(iter.next().unwrap(), iter);
            return self.plan_relationship_scan(rel, body);
        }
        tree!(IR::Expr(tree!(ExprIR::Null)))
    }

    fn plan_query(
        &mut self,
        ir: QueryIR,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        match ir {
            QueryIR::Call(name, exprs) => tree!(
                IR::For,
                tree!(
                    IR::Block,
                    tree!(IR::Expr(tree!(
                        ExprIR::Set("res".to_string()),
                        tree!(ExprIR::FuncInvocation(name.to_lowercase()) ; exprs)
                    ))),
                    tree!(IR::Expr(tree!(
                        ExprIR::Set("i".to_string()),
                        tree!(ExprIR::Integer(0))
                    )))
                ),
                tree!(IR::Expr(tree!(
                    ExprIR::Lt,
                    tree!(ExprIR::Var("i".to_string())),
                    tree!(ExprIR::Length, tree!(ExprIR::Var("res".to_string())))
                ))),
                tree!(IR::Expr(tree!(
                    ExprIR::Set("i".to_string()),
                    tree!(
                        ExprIR::Add,
                        tree!(ExprIR::Var("i".to_string())),
                        tree!(ExprIR::Integer(1))
                    )
                ))),
                tree!(
                    IR::Return,
                    tree!(IR::Expr(tree!(
                        ExprIR::List,
                        tree!(
                            ExprIR::GetElement,
                            tree!(ExprIR::Var("res".to_string())),
                            tree!(ExprIR::Var("i".to_string()))
                        )
                    )))
                )
            ),
            QueryIR::Match(pattern) => self.plan_match(pattern, iter),
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, iter, alias),
            QueryIR::Where(expr) => {
                tree!(
                    IR::If,
                    tree!(IR::Expr(expr)),
                    self.plan_query(iter.next().unwrap(), iter)
                )
            }
            QueryIR::Create(pattern) => self.plan_create(pattern, iter),
            QueryIR::Delete(exprs) => self.plan_delete(exprs, iter),
            QueryIR::With(exprs, _) => tree!(
                IR::Block ;
                exprs.into_iter().map(|e| tree!(IR::Expr(e))),
                [self.plan_query(iter.next().unwrap(), iter)]
            ),
            QueryIR::Return(exprs, _) => {
                if exprs.iter().any(|v| v.root().is_aggregation()) {
                    let mut group_by_keys = Vec::new();
                    let mut aggregations = Vec::new();
                    let agg_ctx_var = self.next_var();
                    for mut expr in exprs {
                        if expr.root().is_aggregation() {
                            self.plan_aggregation(agg_ctx_var.to_string(), &mut expr.root_mut());
                            aggregations.push(expr);
                        } else {
                            group_by_keys.push(expr);
                        }
                    }
                    tree!(
                        IR::Block ;
                        [tree!(IR::Expr(tree!(
                            ExprIR::Set(agg_ctx_var),
                            tree!(ExprIR::FuncInvocation(
                                "create_aggregate_ctx".to_string()) ;
                                group_by_keys
                            )
                        )))],
                        aggregations.into_iter().map(|e| tree!(IR::Expr(e)))
                    )
                } else {
                    tree!(IR::Return, tree!(IR::Expr(tree!(ExprIR::List ; exprs))))
                }
            }
            QueryIR::Query(q, _) => {
                let mut is_agg = false;
                if let Some(QueryIR::Return(exprs, _)) = q.last() {
                    is_agg = exprs.iter().any(|v| v.root().is_aggregation());
                }
                let iter = &mut q.into_iter();
                let res = self.plan_query(iter.next().unwrap(), iter);
                if is_agg {
                    return tree!(IR::ReturnAggregation, res);
                }
                res
            }
        }
    }

    #[must_use]
    pub fn plan(
        &mut self,
        ir: QueryIR,
        _debug: bool,
    ) -> DynTree<IR> {
        self.plan_query(ir, &mut Vec::new().into_iter())
    }
}
