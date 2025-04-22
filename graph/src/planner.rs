use std::{collections::BTreeMap, vec::IntoIter};

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
    List(Vec<IR>),
    Length(Box<IR>),
    GetElement(Box<(IR, IR)>),
    GetElements(Box<(IR, Option<IR>, Option<IR>)>),
    Range(Box<(IR, IR, IR)>),
    IsNull(Box<IR>),
    IsNode(Box<IR>),
    Or(Vec<IR>),
    Xor(Vec<IR>),
    And(Vec<IR>),
    Not(Box<IR>),
    Eq(Vec<IR>),
    Neq(Vec<IR>),
    Lt(Box<(IR, IR)>),
    Gt(Box<(IR, IR)>),
    Le(Box<(IR, IR)>),
    Ge(Box<(IR, IR)>),
    In(Box<(IR, IR)>),
    Add(Vec<IR>),
    Sub(Vec<IR>),
    Mul(Vec<IR>),
    Div(Vec<IR>),
    Pow(Vec<IR>),
    FuncInvocation(String, Vec<IR>),
    Map(BTreeMap<String, IR>),
    Set(String, Box<IR>),
    If(Box<(IR, IR)>),
    For(Box<(IR, IR, IR, IR)>),
    Return(Box<IR>),
    Block(Vec<IR>),
}

pub struct Planner {
    var_id: u64,
}

impl Planner {
    #[must_use]
    pub fn new() -> Self {
        Self { var_id: 0 }
    }

    fn next_var(&mut self) -> String {
        let id = self.var_id;
        self.var_id += 1;
        format!("var_{id}")
    }

    fn plan_expr(
        &mut self,
        expr: QueryExprIR,
    ) -> IR {
        match expr {
            QueryExprIR::Null => IR::Null,
            QueryExprIR::Bool(x) => IR::Bool(x),
            QueryExprIR::Integer(x) => IR::Integer(x),
            QueryExprIR::Float(x) => IR::Float(x),
            QueryExprIR::String(x) => IR::String(x),
            QueryExprIR::Ident(x) => IR::Var(x),
            QueryExprIR::Parameter(x) => IR::Parameter(x),
            QueryExprIR::Named(name, expr) => IR::Set(name, Box::new(self.plan_expr(*expr))),
            QueryExprIR::List(exprs) => {
                IR::List(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Or(exprs) => {
                IR::Or(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Xor(exprs) => {
                IR::Xor(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::And(exprs) => {
                IR::And(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Not(expr) => IR::Not(Box::new(self.plan_expr(*expr))),
            QueryExprIR::Eq(exprs) => {
                IR::Eq(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Neq(_) => todo!(),
            QueryExprIR::Lt(_) => todo!(),
            QueryExprIR::Gt(_) => todo!(),
            QueryExprIR::Le(_) => todo!(),
            QueryExprIR::Ge(_) => todo!(),
            QueryExprIR::In(op) => IR::In(Box::new((self.plan_expr(op.0), self.plan_expr(op.1)))),
            QueryExprIR::Add(exprs) => {
                IR::Add(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Sub(exprs) => {
                IR::Sub(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Mul(exprs) => {
                IR::Mul(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Div(exprs) => {
                IR::Div(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect())
            }
            QueryExprIR::Pow(_) => todo!(),
            QueryExprIR::IsNull(expr) => IR::IsNull(Box::new(self.plan_expr(*expr))),
            QueryExprIR::GetElement(op) => {
                IR::GetElement(Box::new((self.plan_expr(op.0), self.plan_expr(op.1))))
            }
            QueryExprIR::GetElements(op) => IR::GetElements(Box::new((
                self.plan_expr(op.0),
                op.1.map(|ir| self.plan_expr(ir)),
                op.2.map(|ir| self.plan_expr(ir)),
            ))),
            QueryExprIR::StartsWith(op) => IR::FuncInvocation(
                "@starts_with".to_string(),
                vec![self.plan_expr(op.0), self.plan_expr(op.1)],
            ),
            QueryExprIR::EndsWith(op) => IR::FuncInvocation(
                "@ends_with".to_string(),
                vec![self.plan_expr(op.0), self.plan_expr(op.1)],
            ),
            QueryExprIR::Contains(op) => IR::FuncInvocation(
                "@contains".to_string(),
                vec![self.plan_expr(op.0), self.plan_expr(op.1)],
            ),
            QueryExprIR::RegexMatches(op) => IR::FuncInvocation(
                "@regex_matches".to_string(),
                vec![self.plan_expr(op.0), self.plan_expr(op.1)],
            ),
            QueryExprIR::Property(expr, name) => IR::FuncInvocation(
                "property".to_string(),
                vec![self.plan_expr(*expr), IR::String(name)],
            ),
            QueryExprIR::FuncInvocation(name, params) => match name.as_str() {
                "range" => {
                    let mut iter = params.into_iter();
                    match (iter.next(), iter.next(), iter.next(), iter.next()) {
                        (Some(length), None, None, None) => IR::Range(Box::new((
                            IR::Integer(0),
                            IR::Sub(vec![self.plan_expr(length), IR::Integer(1)]),
                            IR::Integer(1),
                        ))),
                        (Some(from), Some(to), None, None) => IR::Range(Box::new((
                            self.plan_expr(from),
                            self.plan_expr(to),
                            IR::Integer(1),
                        ))),
                        (Some(from), Some(to), Some(step), None) => IR::Range(Box::new((
                            self.plan_expr(from),
                            self.plan_expr(to),
                            self.plan_expr(step),
                        ))),
                        _ => todo!(),
                    }
                }
                name => IR::FuncInvocation(
                    name.to_string(),
                    params.into_iter().map(|ir| self.plan_expr(ir)).collect(),
                ),
            },
            QueryExprIR::Map(attrs) => IR::Map(
                attrs
                    .into_iter()
                    .map(|(k, ir)| (k, self.plan_expr(ir)))
                    .collect(),
            ),
        }
    }

    fn plan_create(
        &mut self,
        pattern: Pattern,
        iter: &mut IntoIter<QueryIR>,
    ) -> IR {
        let create_nodes = pattern
            .nodes
            .into_iter()
            .map(|n| {
                let labels = IR::List(
                    n.labels
                        .iter()
                        .map(|l| IR::String((*l).to_string()))
                        .collect(),
                );
                let attrs = IR::Map(
                    n.attrs
                        .into_iter()
                        .map(|(k, ir)| (k, self.plan_expr(ir)))
                        .collect(),
                );
                IR::Set(
                    n.alias.to_string(),
                    Box::new(IR::FuncInvocation(
                        String::from("create_node"),
                        vec![labels, attrs],
                    )),
                )
            })
            .collect();
        let create_relationships = pattern
            .relationships
            .into_iter()
            .map(|l| {
                let relationship_type = IR::String(l.relationship_type.to_string());
                let attrs = IR::Map(
                    l.attrs
                        .into_iter()
                        .map(|(k, ir)| (k, self.plan_expr(ir)))
                        .collect(),
                );
                let from = IR::Var(l.from.to_string());
                let to = IR::Var(l.to.to_string());
                IR::Set(
                    l.alias.to_string(),
                    Box::new(IR::FuncInvocation(
                        String::from("create_relationship"),
                        vec![relationship_type, from, to, attrs],
                    )),
                )
            })
            .collect();
        match iter.next() {
            Some(body_ir) => IR::Block(vec![
                IR::Block(create_nodes),
                IR::Block(create_relationships),
                self.plan_query(body_ir, iter),
            ]),
            None => IR::Block(vec![
                IR::Block(create_nodes),
                IR::Block(create_relationships),
            ]),
        }
    }

    fn plan_delete(
        &mut self,
        exprs: Vec<QueryExprIR>,
        iter: &mut IntoIter<QueryIR>,
    ) -> IR {
        let deleted_entities = exprs.into_iter().map(|ir| self.plan_expr(ir)).collect();
        match iter.next() {
            Some(body_ir) => IR::Block(vec![
                IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
                self.plan_query(body_ir, iter),
            ]),
            None => IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
        }
    }

    fn plan_unwind(
        &mut self,
        expr: QueryExprIR,
        iter: &mut IntoIter<QueryIR>,
        alias: String,
    ) -> IR {
        let list = self.plan_expr(expr);
        match list {
            IR::Range(op) => {
                let init = IR::Set(alias.to_string(), Box::new(op.0));
                let condition = IR::Le(Box::new((IR::Var(alias.to_string()), op.1)));
                let next = IR::Set(
                    alias.to_string(),
                    Box::new(IR::Add(vec![IR::Var(alias), op.2])),
                );
                let body_ir = iter.next().unwrap();
                let body = self.plan_query(body_ir, iter);
                IR::For(Box::new((init, condition, next, body)))
            }
            x => {
                let list_var = self.next_var();
                let index_var = self.next_var();
                let list = IR::Set(list_var.to_string(), Box::new(x));
                let init = IR::Set(index_var.to_string(), Box::new(IR::Integer(0)));
                let condition = IR::Lt(Box::new((
                    IR::Var(index_var.to_string()),
                    IR::Length(Box::new(IR::Var(list_var.to_string()))),
                )));
                let next = IR::Set(
                    index_var.to_string(),
                    Box::new(IR::Add(vec![
                        IR::Var(index_var.to_string()),
                        IR::Integer(1),
                    ])),
                );
                let body_ir = iter.next().unwrap();
                let body = self.plan_query(body_ir, iter);
                IR::Block(vec![
                    list,
                    IR::For(Box::new((
                        init,
                        condition,
                        next,
                        IR::Block(vec![
                            IR::Set(
                                alias,
                                Box::new(IR::GetElement(Box::new((
                                    IR::Var(list_var),
                                    IR::Var(index_var),
                                )))),
                            ),
                            body,
                        ]),
                    ))),
                ])
            }
        }
    }

    fn plan_node_scan(
        &mut self,
        node: NodePattern,
        body: IR,
    ) -> IR {
        let init = IR::Block(vec![
            IR::Set(
                format!("iter_{}", node.alias),
                Box::new(IR::FuncInvocation(
                    String::from("create_node_iter"),
                    vec![IR::List(node.labels.into_iter().map(IR::String).collect())],
                )),
            ),
            IR::Set(
                node.alias.to_string(),
                Box::new(IR::FuncInvocation(
                    String::from("next_node"),
                    vec![IR::Var(format!("iter_{}", node.alias))],
                )),
            ),
        ]);
        let condition = IR::IsNode(Box::new(IR::Var(node.alias.to_string())));
        let next = IR::Set(
            node.alias.to_string(),
            Box::new(IR::FuncInvocation(
                String::from("next_node"),
                vec![IR::Var(format!("iter_{}", node.alias))],
            )),
        );
        IR::For(Box::new((init, condition, next, body)))
    }

    fn plan_match(
        &mut self,
        pattern: Pattern,
        iter: &mut IntoIter<QueryIR>,
    ) -> IR {
        if pattern.relationships.is_empty() {
            let mut body = self.plan_query(iter.next().unwrap(), iter);
            for node in pattern.nodes.into_iter().rev() {
                body = self.plan_node_scan(node, body);
            }
            return body;
        }
        IR::Null
    }

    fn plan_query(
        &mut self,
        ir: QueryIR,
        iter: &mut IntoIter<QueryIR>,
    ) -> IR {
        match ir {
            QueryIR::Call(name, exprs) => IR::For(Box::new((
                IR::Block(vec![
                    IR::Set(
                        "res".to_string(),
                        Box::new(IR::FuncInvocation(
                            name.to_lowercase(),
                            exprs.into_iter().map(|ir| self.plan_expr(ir)).collect(),
                        )),
                    ),
                    IR::Set("i".to_string(), Box::new(IR::Integer(0))),
                ]),
                IR::Lt(Box::new((
                    IR::Var("i".to_string()),
                    IR::Length(Box::new(IR::Var("res".to_string()))),
                ))),
                IR::Set(
                    "i".to_string(),
                    Box::new(IR::Add(vec![IR::Var("i".to_string()), IR::Integer(1)])),
                ),
                IR::Return(Box::new(IR::List(vec![IR::GetElement(Box::new((
                    IR::Var("res".to_string()),
                    IR::Var("i".to_string()),
                )))]))),
            ))),
            QueryIR::Match(pattern) => self.plan_match(pattern, iter),
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, iter, alias),
            QueryIR::Where(expr) => IR::If(Box::new((
                self.plan_expr(expr),
                self.plan_query(iter.next().unwrap(), iter),
            ))),
            QueryIR::Create(pattern) => self.plan_create(pattern, iter),
            QueryIR::Delete(exprs) => self.plan_delete(exprs, iter),
            QueryIR::With(exprs) => IR::Block(vec![
                IR::Block(exprs.into_iter().map(|ir| self.plan_expr(ir)).collect()),
                self.plan_query(iter.next().unwrap(), iter),
            ]),
            QueryIR::Return(exprs) => IR::Return(Box::new(IR::List(
                exprs.into_iter().map(|ir| self.plan_expr(ir)).collect(),
            ))),
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
    ) -> IR {
        self.plan_query(ir, &mut Vec::new().into_iter())
    }
}
