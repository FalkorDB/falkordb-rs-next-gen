use std::collections::HashMap;

use crate::{graph::Graph, matrix::Iter, value::Value};

use super::ast::{QueryExprIR, QueryIR};

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
    Gt(Vec<IR>),
    Le(Vec<IR>),
    Ge(Vec<IR>),
    Add(Vec<IR>),
    Sub(Vec<IR>),
    Mul(Vec<IR>),
    Div(Vec<IR>),
    Pow(Vec<IR>),
    FuncInvocation(String, Vec<IR>),
    Map(Vec<(String, IR)>),
    Set(String, Box<IR>),
    If(Box<(IR, IR)>),
    For(Box<(IR, IR, IR, IR)>),
    Return(Box<IR>),
    Block(Vec<IR>),
}

pub struct Runtime {
    functions: HashMap<String, fn(&mut Graph, Value) -> Value>,
    iters: Vec<Iter<bool>>,
}

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        let mut functions: HashMap<String, fn(&mut Graph, Value) -> Value> = HashMap::new();
        functions.insert("create_node".to_string(), Self::create_node);
        functions.insert("create_link".to_string(), Self::create_link);
        functions.insert("delete_entity".to_string(), Self::delete_entity);
        functions.insert("create_node_iter".to_string(), Self::create_node_iter);
        functions.insert("next_node".to_string(), Self::next_node);
        functions.insert("property".to_string(), Self::property);
        Self {
            functions,
            iters: Vec::new(),
        }
    }

    fn plan_expr(&self, expr: &QueryExprIR) -> IR {
        match expr {
            QueryExprIR::Null => IR::Null,
            QueryExprIR::Bool(x) => IR::Bool(*x),
            QueryExprIR::Integer(x) => IR::Integer(*x),
            QueryExprIR::Float(x) => IR::Float(*x),
            QueryExprIR::String(x) => IR::String(x.to_string()),
            QueryExprIR::Ident(x) => IR::Var(x.to_string()),
            QueryExprIR::Param(x) => IR::Param(x.to_string()),
            QueryExprIR::Named(name, expr) => {
                IR::Set(name.to_string(), Box::new(self.plan_expr(expr)))
            }
            QueryExprIR::List(exprs) => IR::List(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Or(exprs) => IR::Or(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Xor(exprs) => IR::Xor(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::And(exprs) => IR::And(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Not(_) => todo!(),
            QueryExprIR::Eq(exprs) => IR::Eq(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Neq(_) => todo!(),
            QueryExprIR::Lt(_) => todo!(),
            QueryExprIR::Gt(_) => todo!(),
            QueryExprIR::Le(_) => todo!(),
            QueryExprIR::Ge(_) => todo!(),
            QueryExprIR::Add(exprs) => IR::Add(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Sub(exprs) => IR::Sub(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Mul(exprs) => IR::Mul(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Div(exprs) => IR::Div(exprs.iter().map(|e| self.plan_expr(e)).collect()),
            QueryExprIR::Pow(_) => todo!(),
            QueryExprIR::IsNull(expr) => IR::IsNull(Box::new(self.plan_expr(expr))),
            QueryExprIR::GetElement(op) => {
                IR::GetElement(Box::new((self.plan_expr(&op.0), self.plan_expr(&op.1))))
            }
            QueryExprIR::Property(expr, name) => IR::FuncInvocation(
                "property".to_string(),
                vec![self.plan_expr(expr), IR::String(name.to_string())],
            ),
            QueryExprIR::FuncInvocation(name, params) => match name.as_str() {
                "range" => match params.as_slice() {
                    [length] => IR::Range(Box::new((
                        IR::Integer(0),
                        self.plan_expr(length),
                        IR::Integer(1),
                    ))),
                    [from, to] => IR::Range(Box::new((
                        self.plan_expr(from),
                        self.plan_expr(to),
                        IR::Integer(1),
                    ))),
                    [from, to, step] => IR::Range(Box::new((
                        self.plan_expr(from),
                        self.plan_expr(to),
                        self.plan_expr(step),
                    ))),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            QueryExprIR::Map(attrs) => IR::Map(
                attrs
                    .iter()
                    .map(|(k, v)| (k.clone(), self.plan_expr(v)))
                    .collect(),
            ),
        }
    }

    fn plan_create(
        &self,
        pattern: &crate::ast::Pattern,
        iter: &mut std::slice::Iter<'_, QueryIR>,
    ) -> IR {
        let create_nodes = pattern
            .nodes
            .iter()
            .map(|n| {
                let labels = IR::List(
                    n.labels
                        .iter()
                        .map(|l| IR::String((*l).to_string()))
                        .collect(),
                );
                let attrs_keys = IR::List(
                    n.attrs
                        .iter()
                        .map(|(k, _)| IR::String((*k).to_string()))
                        .collect(),
                );
                let attrs_values =
                    IR::List(n.attrs.iter().map(|(_, v)| self.plan_expr(v)).collect());
                IR::Set(
                    n.alias.to_string(),
                    Box::new(IR::FuncInvocation(
                        String::from("create_node"),
                        vec![labels, attrs_keys, attrs_values],
                    )),
                )
            })
            .collect();
        let create_links = pattern
            .links
            .iter()
            .map(|l| {
                let link_type = IR::String(l.link_type.to_string());
                let attrs_keys = IR::List(
                    l.attrs
                        .iter()
                        .map(|(k, _)| IR::String((*k).to_string()))
                        .collect(),
                );
                let from = IR::Var(l.from.to_string());
                let to = IR::Var(l.to.to_string());
                let attrs_values =
                    IR::List(l.attrs.iter().map(|(_, v)| self.plan_expr(v)).collect());
                IR::Set(
                    l.alias.to_string(),
                    Box::new(IR::FuncInvocation(
                        String::from("create_link"),
                        vec![link_type, from, to, attrs_keys, attrs_values],
                    )),
                )
            })
            .collect();
        match iter.next() {
            Some(body_ir) => IR::Block(vec![
                IR::Block(create_nodes),
                IR::Block(create_links),
                self.plan_query(body_ir, iter),
            ]),
            None => IR::Block(vec![IR::Block(create_nodes), IR::Block(create_links)]),
        }
    }

    fn plan_delete(&self, exprs: &[QueryExprIR], iter: &mut std::slice::Iter<'_, QueryIR>) -> IR {
        let deleted_entities = exprs.iter().map(|e| self.plan_expr(e)).collect();
        match iter.next() {
            Some(body_ir) => IR::Block(vec![
                IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
                self.plan_query(body_ir, iter),
            ]),
            None => IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
        }
    }

    fn plan_unwind(
        &self,
        expr: &QueryExprIR,
        iter: &mut std::slice::Iter<'_, QueryIR>,
        alias: &String,
    ) -> IR {
        let list = self.plan_expr(expr);
        match list {
            IR::List(_) => {
                let list = IR::Set("list".to_string(), Box::new(list));
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
                let body = self.plan_query(body_ir, iter);
                IR::Block(vec![
                    list,
                    IR::For(Box::new((
                        init,
                        condition,
                        next,
                        IR::Block(vec![
                            IR::Set(
                                (*alias).to_string(),
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
            IR::Range(op) => {
                let init = IR::Set((*alias).to_string(), Box::new(op.0));
                let condition = IR::Lt(Box::new((IR::Var((*alias).to_string()), op.1)));
                let next = IR::Set(
                    (*alias).to_string(),
                    Box::new(IR::Add(vec![IR::Var((*alias).to_string()), op.2])),
                );
                let body_ir = iter.next().unwrap();
                let body = self.plan_query(body_ir, iter);
                IR::For(Box::new((init, condition, next, body)))
            }
            _ => unimplemented!(""),
        }
    }

    fn plan_match(
        &self,
        pattern: &crate::ast::Pattern,
        iter: &mut std::slice::Iter<'_, QueryIR>,
    ) -> IR {
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
        let body = self.plan_query(body_ir, iter);
        IR::For(Box::new((init, condition, next, body)))
    }

    fn plan_query(&self, ir: &QueryIR, iter: &mut std::slice::Iter<QueryIR>) -> IR {
        match ir {
            QueryIR::Match(pattern) => self.plan_match(pattern, iter),
            QueryIR::Unwind(expr, alias) => self.plan_unwind(expr, iter, alias),
            QueryIR::Where(expr) => IR::If(Box::new((
                self.plan_expr(expr),
                self.plan_query(iter.next().unwrap(), iter),
            ))),
            QueryIR::Create(pattern) => self.plan_create(pattern, iter),
            QueryIR::Delete(exprs) => self.plan_delete(exprs, iter),
            QueryIR::With(exprs) => IR::Block(vec![
                IR::Block(exprs.iter().map(|e| self.plan_expr(e)).collect()),
                self.plan_query(iter.next().unwrap(), iter),
            ]),
            QueryIR::Return(exprs) => IR::Return(Box::new(IR::List(
                exprs.iter().map(|e| self.plan_expr(e)).collect(),
            ))),
            QueryIR::Query(q) => {
                let iter = &mut q.iter();
                self.plan_query(iter.next().unwrap(), iter)
            }
        }
    }

    #[must_use]
    pub fn plan(&self, ir: &QueryIR, debug: bool) -> IR {
        self.plan_query(ir, &mut [].iter())
    }

    fn create_node(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Array(raw_labels), raw_keys @ Value::Array(_), raw_values @ Value::Array(_)] =>
                {
                    let mut labels = Vec::new();
                    for label in raw_labels {
                        if let Value::String(lable) = label {
                            labels.push(lable.to_string());
                        }
                    }
                    g.create_node(&labels, raw_keys.to_owned(), raw_values.to_owned())
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn delete_entity(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(nodes) => {
                for n in nodes {
                    if let Value::Node(id) = n {
                        g.delete_node(id);
                    }
                }
            }
            _ => todo!(),
        }

        Value::Null
    }

    fn create_link(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::String(link_type), Value::Node(from), Value::Node(to), keys @ Value::Array(_), values @ Value::Array(_)] => {
                    g.create_link(
                        link_type.to_string(),
                        *from,
                        *to,
                        keys.to_owned(),
                        values.to_owned(),
                    )
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn create_node_iter(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Array(raw_labels)] => {
                    let mut labels = Vec::new();
                    for l in raw_labels {
                        if let Value::String(l) = l {
                            labels.push(l.to_string());
                        }
                    }
                    g.runtime.iters.push(g.get_nodes(&labels).unwrap());
                    Value::Int(g.runtime.iters.len() as i64 - 1)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn next_node(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Int(iter)] => g.runtime.iters[*iter as usize]
                    .next()
                    .map_or_else(|| Value::Bool(false), |(n, _)| Value::Node(n)),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn property(g: &mut Graph, args: Value) -> Value {
        match args {
            Value::Array(arr) => match arr.as_slice() {
                [Value::Node(node_id), Value::String(property)] => {
                    let property_id = g.get_node_property_id(property);
                    g.get_node_property(*node_id, property_id)
                        .map_or_else(|| Value::Bool(false), |n| n)
                }
                [Value::Map(map), Value::String(property)] => {
                    map.get(property).unwrap_or(&Value::Null).clone()
                }
                _ => Value::Null,
            },
            _ => unimplemented!(),
        }
    }
}

pub fn run(
    vars: &mut HashMap<String, Value>,
    g: &mut Graph,
    result_fn: &mut dyn FnMut(&mut Graph, Value),
    ir: &IR,
) -> Value {
    match ir {
        IR::Null => Value::Null,
        IR::Bool(x) => Value::Bool(*x),
        IR::Integer(x) => Value::Int(*x),
        IR::Float(x) => Value::Float(*x),
        IR::String(x) => Value::String(x.to_string()),
        IR::Var(x) => vars.get(x).unwrap().to_owned(),
        IR::Param(_) => todo!(),
        IR::List(irs) => Value::Array(irs.iter().map(|ir| run(vars, g, result_fn, ir)).collect()),
        IR::Length(ir) => match run(vars, g, result_fn, ir) {
            Value::Array(arr) => Value::Int(arr.len() as _),
            _ => todo!(),
        },
        IR::GetElement(op) => {
            let arr = run(vars, g, result_fn, &op.0);
            let i = run(vars, g, result_fn, &op.1);
            match (arr, i) {
                (Value::Array(values), Value::Int(i)) => {
                    if i < values.len() as _ {
                        values[i as usize].to_owned()
                    } else {
                        Value::Null
                    }
                }
                _ => todo!(),
            }
        }
        IR::Range(_) => todo!(),
        IR::IsNull(ir) => match run(vars, g, result_fn, ir) {
            Value::Null => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::IsNode(ir) => match run(vars, g, result_fn, ir) {
            Value::Node(_) => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::Or(irs) => {
            for ir in irs {
                if matches!(run(vars, g, result_fn, ir), Value::Bool(true)) {
                    return Value::Bool(true);
                }
            }

            Value::Bool(false)
        }
        IR::Xor(irs) => todo!(),
        IR::And(irs) => {
            for ir in irs {
                if matches!(run(vars, g, result_fn, ir), Value::Bool(false)) {
                    return Value::Bool(false);
                }
            }

            Value::Bool(true)
        }
        IR::Not(ir) => todo!(),
        IR::Eq(irs) => irs
            .iter()
            .map(|ir| run(vars, g, result_fn, ir))
            .reduce(|a, b| Value::Bool(a == b))
            .unwrap(),
        IR::Neq(irs) => todo!(),
        IR::Lt(op) => Value::Bool(
            match (
                run(vars, g, result_fn, &op.0),
                run(vars, g, result_fn, &op.1),
            ) {
                (Value::Int(a), Value::Int(b)) => a < b,
                _ => todo!(),
            },
        ),
        IR::Gt(irs) => todo!(),
        IR::Le(irs) => todo!(),
        IR::Ge(irs) => todo!(),
        IR::Add(irs) => irs
            .iter()
            .map(|ir| run(vars, g, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Sub(irs) => todo!(),
        IR::Mul(irs) => irs
            .iter()
            .map(|ir| run(vars, g, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Div(irs) => todo!(),
        IR::Pow(irs) => todo!(),
        IR::FuncInvocation(name, irs) => {
            let args = irs.iter().map(|ir| run(vars, g, result_fn, ir)).collect();
            g.runtime.functions[name](g, Value::Array(args))
        }
        IR::Map(items) => Value::Map(
            items
                .iter()
                .map(|(key, ir)| (key.to_string(), run(vars, g, result_fn, ir)))
                .collect(),
        ),
        IR::Set(x, ir) => {
            let v = run(vars, g, result_fn, ir);
            vars.insert(x.to_string(), v);
            Value::Null
        }
        IR::If(op) => match run(vars, g, result_fn, &op.0) {
            Value::Bool(true) => run(vars, g, result_fn, &op.1),
            _ => Value::Null,
        },
        IR::For(op) => {
            run(vars, g, result_fn, &op.0);
            while run(vars, g, result_fn, &op.1) == Value::Bool(true) {
                run(vars, g, result_fn, &op.3);
                run(vars, g, result_fn, &op.2);
            }
            Value::Null
        }
        IR::Return(ir) => {
            let v = run(vars, g, result_fn, ir);
            result_fn(g, v);
            Value::Null
        }
        IR::Block(irs) => {
            for ir in irs {
                run(vars, g, result_fn, ir);
            }
            Value::Null
        }
    }
}
