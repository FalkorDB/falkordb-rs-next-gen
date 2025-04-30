use std::{fmt::Display, vec::IntoIter};

use orx_tree::{DynTree, NodeRef};

use crate::ast::{NodePattern, Pattern, QueryExprIR, QueryIR};

#[derive(Clone, Debug)]
pub enum IR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Var(String),
    Parameter(String),
    List,
    Length,
    GetElement,
    GetElements,
    Range,
    IsNull,
    IsNode,
    Or,
    Xor,
    And,
    Not,
    Negate,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    In,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Modulo,
    FuncInvocation(String),
    Map,
    Set(String),
    If,
    For,
    Return,
    Block,
}

impl Display for IR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Integer(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Var(x) => write!(f, "{x}"),
            Self::Parameter(x) => write!(f, "${x}"),
            Self::List => write!(f, "list"),
            Self::Length => write!(f, "length"),
            Self::GetElement => write!(f, "get_element"),
            Self::GetElements => write!(f, "get_elements"),
            Self::Range => write!(f, "range"),
            Self::IsNull => write!(f, "is_null"),
            Self::IsNode => write!(f, "is_node"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::And => write!(f, "and"),
            Self::Not => write!(f, "not"),
            Self::Negate => write!(f, "negate"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Lt => write!(f, "lt"),
            Self::Gt => write!(f, "gt"),
            Self::Le => write!(f, "le"),
            Self::Ge => write!(f, "ge"),
            Self::In => write!(f, "in"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Pow => write!(f, "^"),
            Self::Modulo => write!(f, "%"),
            Self::FuncInvocation(x) => write!(f, "{x}()"),
            Self::Map => write!(f, "{{}}"),
            Self::Set(x) => write!(f, "{x} ="),
            Self::If => write!(f, "if"),
            Self::For => write!(f, "for"),
            Self::Return => write!(f, "return"),
            Self::Block => write!(f, "block"),
        }
    }
}

pub struct Planner {
    var_id: u64,
}

macro_rules! tree {
    ($value:expr) => {
        DynTree::new($value)
    };
    ($value:expr, $($child:expr),*) => {
        {
            let mut n = DynTree::new($value);
            let mut root = n.root_mut();
            $(root.push_child_tree($child);)*
            n
        }
    };
    ($value:expr ; $iter:expr) => {
        {
            let mut n = DynTree::new($value);
            let mut root = n.root_mut();
            for child in $iter {
                root.push_child_tree(child);
            }
            n
        }
    };
    ($value:expr => $self:expr => $child:expr) => {
        {
            let mut n = DynTree::new($value);
            let mut root = n.root_mut();
            for child in $child.into_iter().map(|ir| $self.plan_expr(ir)) {
                root.push_child_tree(child);
            }
            n
        }
    };
    () => {};
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

    #[allow(clippy::only_used_in_recursion)]
    fn plan_expr(
        &mut self,
        expr: QueryExprIR,
    ) -> DynTree<IR> {
        match expr {
            QueryExprIR::Null => tree!(IR::Null),
            QueryExprIR::Bool(x) => tree!(IR::Bool(x)),
            QueryExprIR::Integer(x) => tree!(IR::Integer(x)),
            QueryExprIR::Float(x) => tree!(IR::Float(x)),
            QueryExprIR::String(x) => tree!(IR::String(x)),
            QueryExprIR::Ident(x) => tree!(IR::Var(x)),
            QueryExprIR::Parameter(x) => tree!(IR::Parameter(x)),
            QueryExprIR::Named(name, expr) => tree!(IR::Set(name), self.plan_expr(*expr)),
            QueryExprIR::List(exprs) => tree!(IR::List => self => exprs),
            QueryExprIR::Or(exprs) => tree!(IR::Or => self => exprs),
            QueryExprIR::Xor(exprs) => tree!(IR::Xor => self => exprs),
            QueryExprIR::And(exprs) => tree!(IR::And => self => exprs),
            QueryExprIR::Not(expr) => tree!(IR::Not, self.plan_expr(*expr)),
            QueryExprIR::Negate(expr) => tree!(IR::Negate, self.plan_expr(*expr)),
            QueryExprIR::Eq(exprs) => tree!(IR::Eq => self => exprs),
            QueryExprIR::Neq(_) => todo!(),
            QueryExprIR::Lt(_) => todo!(),
            QueryExprIR::Gt(_) => todo!(),
            QueryExprIR::Le(_) => todo!(),
            QueryExprIR::Ge(_) => todo!(),
            QueryExprIR::In(op) => tree!(IR::In, self.plan_expr(op.0), self.plan_expr(op.1)),
            QueryExprIR::Add(exprs) => tree!(IR::Add => self => exprs),
            QueryExprIR::Sub(exprs) => tree!(IR::Sub => self => exprs),
            QueryExprIR::Mul(exprs) => tree!(IR::Mul => self => exprs),
            QueryExprIR::Div(exprs) => tree!(IR::Div => self => exprs),
            QueryExprIR::Pow(exprs) => tree!(IR::Pow => self => exprs),
            QueryExprIR::Modulo(exprs) => tree!(IR::Modulo => self => exprs),
            QueryExprIR::IsNull(expr) => tree!(IR::IsNull, self.plan_expr(*expr)),
            QueryExprIR::GetElement(op) => {
                tree!(IR::GetElement, self.plan_expr(op.0), self.plan_expr(op.1))
            }
            QueryExprIR::GetElements(op) => {
                let list_var = self.next_var();
                let list = tree!(IR::Set(list_var.to_string()), self.plan_expr(op.0));

                tree!(
                    IR::Block,
                    list,
                    tree!(
                        IR::GetElements,
                        tree!(IR::Var(list_var.to_string())),
                        op.1.map_or_else(|| tree!(IR::Integer(0)), |ir| self.plan_expr(ir)),
                        op.2.map_or_else(
                            || tree!(IR::Length, tree!(IR::Var(list_var))),
                            |ir| self.plan_expr(ir)
                        )
                    )
                )
            }
            QueryExprIR::FuncInvocation(name, exprs) => match name.as_str() {
                "range" => {
                    let mut iter = exprs.into_iter();
                    match (iter.next(), iter.next(), iter.next(), iter.next()) {
                        (Some(length), None, None, None) => tree!(
                            IR::Range,
                            self.plan_expr(QueryExprIR::Integer(0)),
                            self.plan_expr(QueryExprIR::Sub(
                                vec![length, QueryExprIR::Integer(1),]
                            )),
                            self.plan_expr(QueryExprIR::Integer(1))
                        ),
                        (Some(from), Some(to), None, None) => tree!(
                            IR::Range,
                            self.plan_expr(from),
                            self.plan_expr(to),
                            self.plan_expr(QueryExprIR::Integer(1))
                        ),
                        (Some(from), Some(to), Some(step), None) => tree!(
                            IR::Range,
                            self.plan_expr(from),
                            self.plan_expr(to),
                            self.plan_expr(step)
                        ),
                        _ => todo!(),
                    }
                }
                name => tree!(IR::FuncInvocation(name.to_string()) => self => exprs),
            },
            QueryExprIR::Map(attrs) => tree!(IR::Map ;
                attrs
                    .into_iter()
                    .map(|(k, ir)| tree!(IR::Var(k), self.plan_expr(ir)))
            ),
        }
    }

    fn plan_create(
        &mut self,
        pattern: Pattern,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        if let Some(body_ir) = iter.next() {
            tree!(
                IR::Block,
                tree!(IR::Block ; pattern.nodes.into_iter().map(|n| {
                    tree!(
                        IR::Set(n.alias.to_string()),
                        tree!(
                            IR::FuncInvocation(String::from("create_node")),
                            tree!(IR::List ; n
                                .labels
                                .iter()
                                .map(|l| tree!(IR::String((*l).to_string())))),
                            tree!(IR::Map ;
                                n.attrs
                                    .into_iter()
                                    .map(|(k, ir)| tree!(IR::Var(k), self.plan_expr(ir)))
                            )
                        )
                    )
                })),
                tree!(IR::Block ; pattern.relationships.into_iter().map(|l| {
                    tree!(
                        IR::Set(l.alias.to_string()),
                        tree!(
                            IR::FuncInvocation(String::from("create_relationship")),
                            tree!(IR::String(l.relationship_type.to_string())),
                            tree!(IR::Var(l.from.to_string())),
                            tree!(IR::Var(l.to.to_string())),
                            tree!(IR::Map ;
                                l.attrs
                                    .into_iter()
                                    .map(|(k, ir)| tree!(IR::Var(k), self.plan_expr(ir))))
                        )
                    )
                })),
                self.plan_query(body_ir, iter)
            )
        } else {
            tree!(
                IR::Block,
                tree!(IR::Block ; pattern.nodes.into_iter().map(|n| {
                    tree!(
                        IR::Set(n.alias.to_string()),
                        tree!(
                            IR::FuncInvocation(String::from("create_node")),
                            tree!(IR::List ; n
                                .labels
                                .iter()
                                .map(|l| tree!(IR::String((*l).to_string())))),
                            tree!(IR::Map ;
                                n.attrs
                                    .into_iter()
                                    .map(|(k, ir)| tree!(IR::Var(k), self.plan_expr(ir))))
                        )
                    )
                })),
                tree!(IR::Block ; pattern.relationships.into_iter().map(|l| {
                    tree!(
                        IR::Set(l.alias.to_string()),
                        tree!(
                            IR::FuncInvocation(String::from("create_relationship")),
                            tree!(IR::String(l.relationship_type.to_string())),
                            tree!(IR::Var(l.from.to_string())),
                            tree!(IR::Var(l.to.to_string())),
                            tree!(IR::Map ;
                                l.attrs
                                    .into_iter()
                                    .map(|(k, ir)| tree!(IR::Var(k), self.plan_expr(ir))))
                        )
                    )
                }))
            )
        }
    }

    fn plan_delete(
        &mut self,
        exprs: Vec<QueryExprIR>,
        iter: &mut IntoIter<QueryIR>,
    ) -> DynTree<IR> {
        if let Some(body_ir) = iter.next() {
            tree!(
                IR::Block,
                tree!(IR::FuncInvocation(String::from("delete_entity")) => self => exprs),
                self.plan_query(body_ir, iter)
            )
        } else {
            tree!(IR::FuncInvocation(String::from("delete_entity")) => self => exprs)
        }
    }

    fn plan_unwind(
        &mut self,
        expr: QueryExprIR,
        iter: &mut IntoIter<QueryIR>,
        alias: String,
    ) -> DynTree<IR> {
        let list = self.plan_expr(expr);
        if matches!(list.root().data(), IR::Range) {
            let init = tree!(
                IR::Set(alias.to_string()),
                list.root().child(0).as_cloned_subtree()
            );
            let condition = tree!(
                IR::Le,
                tree!(IR::Var(alias.to_string())),
                list.root().child(1).as_cloned_subtree()
            );
            let next = tree!(
                IR::Set(alias.to_string()),
                tree!(
                    IR::Add,
                    tree!(IR::Var(alias)),
                    list.root().child(2).as_cloned_subtree()
                )
            );
            let body_ir = iter.next().unwrap();
            let body = self.plan_query(body_ir, iter);
            tree!(IR::For, init, condition, next, body)
        } else {
            let list_var = self.next_var();
            let index_var = self.next_var();
            let list = tree!(IR::Set(list_var.to_string()), list);
            let init = tree!(IR::Set(index_var.to_string()), tree!(IR::Integer(0)));
            let condition = tree!(
                IR::Lt,
                tree!(IR::Var(index_var.to_string())),
                tree!(IR::Length, tree!(IR::Var(list_var.to_string())))
            );
            let next = tree!(
                IR::Set(index_var.to_string()),
                tree!(
                    IR::Add,
                    tree!(IR::Var(index_var.to_string())),
                    tree!(IR::Integer(1))
                )
            );
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
                        tree!(
                            IR::Set(alias),
                            tree!(
                                IR::GetElement,
                                tree!(IR::Var(list_var)),
                                tree!(IR::Var(index_var))
                            )
                        ),
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
            tree!(
                IR::Set(format!("iter_{}", node.alias)),
                tree!(
                    IR::FuncInvocation(String::from("create_node_iter")),
                    tree!(IR::List ; node
                            .labels
                            .into_iter()
                            .map(|label| tree!(IR::String(label))))
                )
            ),
            tree!(
                IR::Set(node.alias.to_string()),
                tree!(
                    IR::FuncInvocation(String::from("next_node")),
                    tree!(IR::Var(format!("iter_{}", node.alias)))
                )
            )
        );
        let condition = tree!(IR::IsNode, tree!(IR::Var(node.alias.to_string())));
        let next = tree!(
            IR::Set(node.alias.to_string()),
            tree!(
                IR::FuncInvocation(String::from("next_node")),
                tree!(IR::Var(format!("iter_{}", node.alias)))
            )
        );
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
        tree!(IR::Null)
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
                    tree!(
                        IR::Set("res".to_string()),
                        tree!(IR::FuncInvocation(name.to_lowercase()) => self => exprs)
                    ),
                    tree!(IR::Set("i".to_string()), tree!(IR::Integer(0)))
                ),
                tree!(
                    IR::Lt,
                    tree!(IR::Var("i".to_string())),
                    tree!(IR::Length, tree!(IR::Var("res".to_string())))
                ),
                tree!(
                    IR::Set("i".to_string()),
                    tree!(
                        IR::Add,
                        tree!(IR::Var("i".to_string())),
                        tree!(IR::Integer(1))
                    )
                ),
                tree!(
                    IR::Return,
                    tree!(
                        IR::List,
                        tree!(
                            IR::GetElement,
                            tree!(IR::Var("res".to_string())),
                            tree!(IR::Var("i".to_string()))
                        )
                    )
                )
            ),
            QueryIR::Match(pattern) => self.plan_match(pattern, iter),
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, iter, alias),
            QueryIR::Where(expr) => tree!(
                IR::If,
                self.plan_expr(expr),
                self.plan_query(iter.next().unwrap(), iter)
            ),
            QueryIR::Create(pattern) => self.plan_create(pattern, iter),
            QueryIR::Delete(exprs) => self.plan_delete(exprs, iter),
            QueryIR::With(exprs) => tree!(
                IR::Block,
                tree!(IR::Block => self => exprs),
                self.plan_query(iter.next().unwrap(), iter)
            ),
            QueryIR::Return(exprs) => tree!(IR::Return, tree!(IR::List => self => exprs)),
            QueryIR::Query(q) => {
                let iter = &mut q.into_iter();
                self.plan_query(iter.next().unwrap(), iter)
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
