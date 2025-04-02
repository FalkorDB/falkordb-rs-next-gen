use std::{collections::BTreeMap, vec::IntoIter};

use crate::ast::{Pattern, QueryExprIR, QueryIR};

#[derive(Clone, Debug)]
pub enum IR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Var(String),
    Param(String),
    List(Vec<IR>),
    Length(Box<IR>),
    GetElement(Box<(IR, IR)>),
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

fn plan_expr(expr: QueryExprIR) -> IR {
    match expr {
        QueryExprIR::Null => IR::Null,
        QueryExprIR::Bool(x) => IR::Bool(x),
        QueryExprIR::Integer(x) => IR::Integer(x),
        QueryExprIR::Float(x) => IR::Float(x),
        QueryExprIR::String(x) => IR::String(x),
        QueryExprIR::Ident(x) => IR::Var(x),
        QueryExprIR::Param(x) => IR::Param(x),
        QueryExprIR::Named(name, expr) => IR::Set(name, Box::new(plan_expr(*expr))),
        QueryExprIR::List(exprs) => IR::List(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Or(exprs) => IR::Or(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Xor(exprs) => IR::Xor(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::And(exprs) => IR::And(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Not(expr) => IR::Not(Box::new(plan_expr(*expr))),
        QueryExprIR::Eq(exprs) => IR::Eq(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Neq(_) => todo!(),
        QueryExprIR::Lt(_) => todo!(),
        QueryExprIR::Gt(_) => todo!(),
        QueryExprIR::Le(_) => todo!(),
        QueryExprIR::Ge(_) => todo!(),
        QueryExprIR::Add(exprs) => IR::Add(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Sub(exprs) => IR::Sub(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Mul(exprs) => IR::Mul(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Div(exprs) => IR::Div(exprs.into_iter().map(plan_expr).collect()),
        QueryExprIR::Pow(_) => todo!(),
        QueryExprIR::IsNull(expr) => IR::IsNull(Box::new(plan_expr(*expr))),
        QueryExprIR::GetElement(op) => IR::GetElement(Box::new((plan_expr(op.0), plan_expr(op.1)))),
        QueryExprIR::Property(expr, name) => IR::FuncInvocation(
            "property".to_string(),
            vec![plan_expr(*expr), IR::String(name)],
        ),
        QueryExprIR::FuncInvocation(name, params) => match name.as_str() {
            "range" => {
                let mut iter = params.into_iter();
                match (iter.next(), iter.next(), iter.next(), iter.next()) {
                    (Some(length), None, None, None) => IR::Range(Box::new((
                        IR::Integer(0),
                        IR::Sub(vec![plan_expr(length), IR::Integer(1)]),
                        IR::Integer(1),
                    ))),
                    (Some(from), Some(to), None, None) => {
                        IR::Range(Box::new((plan_expr(from), plan_expr(to), IR::Integer(1))))
                    }
                    (Some(from), Some(to), Some(step), None) => {
                        IR::Range(Box::new((plan_expr(from), plan_expr(to), plan_expr(step))))
                    }
                    _ => todo!(),
                }
            }
            name => IR::FuncInvocation(
                name.to_string(),
                params.into_iter().map(plan_expr).collect(),
            ),
        },
        QueryExprIR::Map(attrs) => {
            IR::Map(attrs.into_iter().map(|(k, v)| (k, plan_expr(v))).collect())
        }
    }
}

fn plan_create(pattern: Pattern, iter: &mut IntoIter<QueryIR>) -> IR {
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
                    .map(|(k, v)| (k, plan_expr(v)))
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
                    .map(|(k, v)| (k, plan_expr(v)))
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
            plan_query(body_ir, iter),
        ]),
        None => IR::Block(vec![
            IR::Block(create_nodes),
            IR::Block(create_relationships),
        ]),
    }
}

fn plan_delete(exprs: Vec<QueryExprIR>, iter: &mut IntoIter<QueryIR>) -> IR {
    let deleted_entities = exprs.into_iter().map(plan_expr).collect();
    match iter.next() {
        Some(body_ir) => IR::Block(vec![
            IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
            plan_query(body_ir, iter),
        ]),
        None => IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
    }
}

fn plan_unwind(expr: QueryExprIR, iter: &mut IntoIter<QueryIR>, alias: String) -> IR {
    let list = plan_expr(expr);
    match list {
        IR::Range(op) => {
            let init = IR::Set(alias.to_string(), Box::new(op.0));
            let condition = IR::Le(Box::new((IR::Var(alias.to_string()), op.1)));
            let next = IR::Set(
                alias.to_string(),
                Box::new(IR::Add(vec![IR::Var(alias), op.2])),
            );
            let body_ir = iter.next().unwrap();
            let body = plan_query(body_ir, iter);
            IR::For(Box::new((init, condition, next, body)))
        }
        x => {
            let list = IR::Set("list".to_string(), Box::new(x));
            let init = IR::Set("i".to_string(), Box::new(IR::Integer(0)));
            let condition = IR::Lt(Box::new((
                IR::Var("i".to_string()),
                IR::Length(Box::new(IR::Var("list".to_string()))),
            )));
            let next = IR::Set(
                "i".to_string(),
                Box::new(IR::Add(vec![IR::Var("i".to_string()), IR::Integer(1)])),
            );
            let body_ir = iter.next().unwrap();
            let body = plan_query(body_ir, iter);
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
                                IR::Var("list".to_string()),
                                IR::Var("i".to_string()),
                            )))),
                        ),
                        body,
                    ]),
                ))),
            ])
        }
    }
}

fn plan_match(pattern: Pattern, iter: &mut IntoIter<QueryIR>) -> IR {
    let n = pattern.nodes.first().unwrap();
    let labels = IR::List(
        n.labels
            .iter()
            .map(|l| IR::String((*l).to_string()))
            .collect(),
    );
    let vec = vec![
        IR::Set(
            "iter".to_string(),
            Box::new(IR::FuncInvocation(
                String::from("create_node_iter"),
                vec![labels],
            )),
        ),
        IR::Set(
            n.alias.to_string(),
            Box::new(IR::FuncInvocation(
                String::from("next_node"),
                vec![IR::Var("iter".to_string())],
            )),
        ),
    ];
    let init = IR::Block(vec);
    let condition = IR::IsNode(Box::new(IR::Var(n.alias.to_string())));
    let next = IR::Set(
        n.alias.to_string(),
        Box::new(IR::FuncInvocation(
            String::from("next_node"),
            vec![IR::Var("iter".to_string())],
        )),
    );
    let body_ir = iter.next().unwrap();
    let body = plan_query(body_ir, iter);
    IR::For(Box::new((init, condition, next, body)))
}

fn plan_query(ir: QueryIR, iter: &mut IntoIter<QueryIR>) -> IR {
    match ir {
        QueryIR::Call(name, exprs) => IR::For(Box::new((
            IR::Block(vec![
                IR::Set(
                    "res".to_string(),
                    Box::new(IR::FuncInvocation(
                        name.to_lowercase(),
                        exprs.into_iter().map(plan_expr).collect(),
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
        QueryIR::Match(pattern) => plan_match(pattern, iter),
        QueryIR::Unwind(expr, alias) => plan_unwind(expr, iter, alias),
        QueryIR::Where(expr) => IR::If(Box::new((
            plan_expr(expr),
            plan_query(iter.next().unwrap(), iter),
        ))),
        QueryIR::Create(pattern) => plan_create(pattern, iter),
        QueryIR::Delete(exprs) => plan_delete(exprs, iter),
        QueryIR::With(exprs) => IR::Block(vec![
            IR::Block(exprs.into_iter().map(plan_expr).collect()),
            plan_query(iter.next().unwrap(), iter),
        ]),
        QueryIR::Return(exprs) => IR::Return(Box::new(IR::List(
            exprs.into_iter().map(plan_expr).collect(),
        ))),
        QueryIR::Query(q) => {
            let iter = &mut q.into_iter();
            plan_query(iter.next().unwrap(), iter)
        }
    }
}

#[must_use]
pub fn plan(ir: QueryIR, debug: bool) -> IR {
    plan_query(ir, &mut Vec::new().into_iter())
}
