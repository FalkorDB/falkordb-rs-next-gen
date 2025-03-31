use std::collections::BTreeMap;

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

pub struct Runtime {
    write_functions: BTreeMap<String, fn(&mut Graph, &mut Runtime, Value) -> Value>,
    read_functions: BTreeMap<String, fn(&Graph, &mut Runtime, Value) -> Value>,
    iters: Vec<Iter<bool>>,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        let mut write_functions: BTreeMap<String, fn(&mut Graph, &mut Runtime, Value) -> Value> =
            BTreeMap::new();
        let mut read_functions: BTreeMap<String, fn(&Graph, &mut Runtime, Value) -> Value> =
            BTreeMap::new();
        write_functions.insert("create_node".to_string(), Self::create_node);
        write_functions.insert("create_relationship".to_string(), Self::create_relationship);
        write_functions.insert("delete_entity".to_string(), Self::delete_entity);
        read_functions.insert("create_node_iter".to_string(), Self::create_node_iter);
        read_functions.insert("next_node".to_string(), Self::next_node);
        read_functions.insert("property".to_string(), Self::property);
        read_functions.insert("db.labels".to_string(), Self::labels);
        read_functions.insert("db.relationshiptypes".to_string(), Self::types);
        read_functions.insert("db.propertykeys".to_string(), Self::properties);

        Self {
            write_functions,
            read_functions,
            iters: Vec::new(),
            nodes_created: 0,
            relationships_created: 0,
            nodes_deleted: 0,
            relationships_deleted: 0,
            properties_set: 0,
            properties_removed: 0,
        }
    }

    fn create_node(g: &mut Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Array(raw_labels), Value::Map(attrs)] => {
                    let labels = raw_labels
                        .iter()
                        .filter_map(|label| {
                            if let Value::String(label) = label {
                                Some(label.to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    runtime.nodes_created += 1;
                    runtime.properties_set += attrs.len() as i32;
                    g.create_node(&labels, attrs)
                }
                _ => Value::Null,
            },
            _ => Value::Null,
        }
    }

    fn delete_entity(g: &mut Graph, _runtime: &mut Self, args: Value) -> Value {
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

    fn create_relationship(g: &mut Graph, _runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::String(relationship_type), Value::Node(from), Value::Node(to), Value::Map(attrs)] => {
                    g.create_relationship(relationship_type, *from, *to, attrs)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn create_node_iter(g: &Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Array(raw_labels)] => {
                    let mut labels = Vec::new();
                    for l in raw_labels {
                        if let Value::String(l) = l {
                            labels.push(l.to_string());
                        }
                    }
                    runtime.iters.push(g.get_nodes(&labels).unwrap());
                    Value::Int(runtime.iters.len() as i64 - 1)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn next_node(_g: &Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::Array(args) => match args.as_slice() {
                [Value::Int(iter)] => runtime.iters[*iter as usize]
                    .next()
                    .map_or_else(|| Value::Bool(false), |(n, _)| Value::Node(n)),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn property(g: &Graph, _runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::Array(arr) => match arr.as_slice() {
                [Value::Node(node_id), Value::String(property)] => g
                    .get_node_property_id(property)
                    .map_or(Value::Null, |property_id| {
                        g.get_node_property(*node_id, property_id)
                            .map_or(Value::Null, |n| n)
                    }),
                [Value::Map(map), Value::String(property)] => {
                    map.get(property).unwrap_or(&Value::Null).clone()
                }
                _ => Value::Null,
            },
            _ => unimplemented!(),
        }
    }

    fn labels(g: &Graph, _runtime: &mut Self, _args: Value) -> Value {
        Value::Array(
            g.get_labels()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }

    fn types(g: &Graph, _runtime: &mut Self, _args: Value) -> Value {
        Value::Array(
            g.get_types()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }

    fn properties(g: &Graph, _runtime: &mut Self, _args: Value) -> Value {
        Value::Array(
            g.get_properties()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }
}

fn plan_expr(expr: &QueryExprIR) -> IR {
    match expr {
        QueryExprIR::Null => IR::Null,
        QueryExprIR::Bool(x) => IR::Bool(*x),
        QueryExprIR::Integer(x) => IR::Integer(*x),
        QueryExprIR::Float(x) => IR::Float(*x),
        QueryExprIR::String(x) => IR::String(x.to_string()),
        QueryExprIR::Ident(x) => IR::Var(x.to_string()),
        QueryExprIR::Param(x) => IR::Param(x.to_string()),
        QueryExprIR::Named(name, expr) => IR::Set(name.to_string(), Box::new(plan_expr(expr))),
        QueryExprIR::List(exprs) => IR::List(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Or(exprs) => IR::Or(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Xor(exprs) => IR::Xor(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::And(exprs) => IR::And(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Not(expr) => IR::Not(Box::new(plan_expr(expr))),
        QueryExprIR::Eq(exprs) => IR::Eq(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Neq(_) => todo!(),
        QueryExprIR::Lt(_) => todo!(),
        QueryExprIR::Gt(_) => todo!(),
        QueryExprIR::Le(_) => todo!(),
        QueryExprIR::Ge(_) => todo!(),
        QueryExprIR::Add(exprs) => IR::Add(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Sub(exprs) => IR::Sub(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Mul(exprs) => IR::Mul(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Div(exprs) => IR::Div(exprs.iter().map(plan_expr).collect()),
        QueryExprIR::Pow(_) => todo!(),
        QueryExprIR::IsNull(expr) => IR::IsNull(Box::new(plan_expr(expr))),
        QueryExprIR::GetElement(op) => {
            IR::GetElement(Box::new((plan_expr(&op.0), plan_expr(&op.1))))
        }
        QueryExprIR::Property(expr, name) => IR::FuncInvocation(
            "property".to_string(),
            vec![plan_expr(expr), IR::String(name.to_string())],
        ),
        QueryExprIR::FuncInvocation(name, params) => match name.as_str() {
            "range" => match params.as_slice() {
                [length] => IR::Range(Box::new((
                    IR::Integer(0),
                    plan_expr(length),
                    IR::Integer(1),
                ))),
                [from, to] => IR::Range(Box::new((plan_expr(from), plan_expr(to), IR::Integer(1)))),
                [from, to, step] => {
                    IR::Range(Box::new((plan_expr(from), plan_expr(to), plan_expr(step))))
                }
                _ => todo!(),
            },
            name => IR::FuncInvocation(name.to_string(), params.iter().map(plan_expr).collect()),
        },
        QueryExprIR::Map(attrs) => IR::Map(
            attrs
                .iter()
                .map(|(k, v)| (k.clone(), plan_expr(v)))
                .collect(),
        ),
    }
}

fn plan_create(pattern: &crate::ast::Pattern, iter: &mut std::slice::Iter<'_, QueryIR>) -> IR {
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
            let attrs = IR::Map(
                n.attrs
                    .iter()
                    .map(|(k, v)| (k.to_string(), plan_expr(v)))
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
        .iter()
        .map(|l| {
            let relationship_type = IR::String(l.relationship_type.to_string());
            let attrs = IR::Map(
                l.attrs
                    .iter()
                    .map(|(k, v)| (k.to_string(), plan_expr(v)))
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

fn plan_delete(exprs: &[QueryExprIR], iter: &mut std::slice::Iter<'_, QueryIR>) -> IR {
    let deleted_entities = exprs.iter().map(plan_expr).collect();
    match iter.next() {
        Some(body_ir) => IR::Block(vec![
            IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
            plan_query(body_ir, iter),
        ]),
        None => IR::FuncInvocation(String::from("delete_entity"), deleted_entities),
    }
}

fn plan_unwind(expr: &QueryExprIR, iter: &mut std::slice::Iter<'_, QueryIR>, alias: &String) -> IR {
    let list = plan_expr(expr);
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
            let body = plan_query(body_ir, iter);
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
            let body = plan_query(body_ir, iter);
            IR::For(Box::new((init, condition, next, body)))
        }
        _ => unimplemented!(""),
    }
}

fn plan_match(pattern: &crate::ast::Pattern, iter: &mut std::slice::Iter<'_, QueryIR>) -> IR {
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

fn plan_query(ir: &QueryIR, iter: &mut std::slice::Iter<QueryIR>) -> IR {
    match ir {
        QueryIR::Call(name, exprs) => IR::For(Box::new((
            IR::Block(vec![
                IR::Set(
                    "labels".to_string(),
                    Box::new(IR::FuncInvocation(
                        name.to_lowercase(),
                        exprs.iter().map(plan_expr).collect(),
                    )),
                ),
                IR::Set("i".to_string(), Box::new(IR::Integer(0))),
            ]),
            IR::Lt(Box::new((
                IR::Var("i".to_string()),
                IR::Length(Box::new(IR::Var("labels".to_string()))),
            ))),
            IR::Set(
                "i".to_string(),
                Box::new(IR::Add(vec![IR::Var("i".to_string()), IR::Integer(1)])),
            ),
            IR::Return(Box::new(IR::List(vec![IR::GetElement(Box::new((
                IR::Var("labels".to_string()),
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
            IR::Block(exprs.iter().map(plan_expr).collect()),
            plan_query(iter.next().unwrap(), iter),
        ]),
        QueryIR::Return(exprs) => {
            IR::Return(Box::new(IR::List(exprs.iter().map(plan_expr).collect())))
        }
        QueryIR::Query(q) => {
            let iter = &mut q.iter();
            plan_query(iter.next().unwrap(), iter)
        }
    }
}

#[must_use]
pub fn plan(ir: &QueryIR, debug: bool) -> IR {
    plan_query(ir, &mut [].iter())
}

pub fn ro_run(
    vars: &mut BTreeMap<String, Value>,
    g: &Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
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
        IR::List(irs) => Value::Array(
            irs.iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect(),
        ),
        IR::Length(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::Array(arr) => Value::Int(arr.len() as _),
            _ => todo!(),
        },
        IR::GetElement(op) => {
            let arr = ro_run(vars, g, runtime, result_fn, &op.0);
            let i = ro_run(vars, g, runtime, result_fn, &op.1);
            match (arr, i) {
                (Value::Array(values), Value::Int(i)) => {
                    if i < values.len() as _ {
                        values[i as usize].clone()
                    } else {
                        Value::Null
                    }
                }
                _ => todo!(),
            }
        }
        IR::Range(_) => todo!(),
        IR::IsNull(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::Null => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::IsNode(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::Node(_) => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::Or(irs) => {
            for ir in irs {
                if matches!(ro_run(vars, g, runtime, result_fn, ir), Value::Bool(true)) {
                    return Value::Bool(true);
                }
            }

            Value::Bool(false)
        }
        IR::Xor(irs) => todo!(),
        IR::And(irs) => {
            for ir in irs {
                if matches!(ro_run(vars, g, runtime, result_fn, ir), Value::Bool(false)) {
                    return Value::Bool(false);
                }
            }

            Value::Bool(true)
        }
        IR::Not(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::Bool(b) => Value::Bool(!b),
            _ => todo!(),
        },
        IR::Eq(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a == b))
            .unwrap(),
        IR::Neq(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .unwrap(),
        IR::Lt(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
            _ => todo!(),
        },
        IR::Gt(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
            _ => todo!(),
        },
        IR::Le(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
            _ => todo!(),
        },
        IR::Ge(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
            _ => todo!(),
        },
        IR::Add(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Sub(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Mul(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Div(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                _ => todo!(),
            })
            .unwrap(),
        IR::Pow(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
                _ => todo!(),
            })
            .unwrap(),
        IR::FuncInvocation(name, irs) => {
            let args = irs
                .iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect();
            runtime.read_functions[name](g, runtime, Value::Array(args))
        }
        IR::Map(items) => Value::Map(
            items
                .iter()
                .map(|(key, ir)| (key.to_string(), ro_run(vars, g, runtime, result_fn, ir)))
                .collect(),
        ),
        IR::Set(x, ir) => {
            let v = ro_run(vars, g, runtime, result_fn, ir);
            vars.insert(x.to_string(), v.clone());
            v
        }
        IR::If(op) => match ro_run(vars, g, runtime, result_fn, &op.0) {
            Value::Bool(true) => ro_run(vars, g, runtime, result_fn, &op.1),
            _ => Value::Null,
        },
        IR::For(op) => {
            ro_run(vars, g, runtime, result_fn, &op.0);
            while ro_run(vars, g, runtime, result_fn, &op.1) == Value::Bool(true) {
                ro_run(vars, g, runtime, result_fn, &op.3);
                ro_run(vars, g, runtime, result_fn, &op.2);
            }
            Value::Null
        }
        IR::Return(ir) => {
            let v = ro_run(vars, g, runtime, result_fn, ir);
            result_fn(g, v);
            Value::Null
        }
        IR::Block(irs) => {
            for ir in irs {
                ro_run(vars, g, runtime, result_fn, ir);
            }
            Value::Null
        }
    }
}

pub fn run(
    vars: &mut BTreeMap<String, Value>,
    g: &mut Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
    ir: &IR,
) -> Value {
    match ir {
        IR::Null => Value::Null,
        IR::Bool(x) => Value::Bool(*x),
        IR::Integer(x) => Value::Int(*x),
        IR::Float(x) => Value::Float(*x),
        IR::String(x) => Value::String(x.to_string()),
        IR::Var(x) => vars.get(x).unwrap_or(&Value::Null).to_owned(),
        IR::Param(_) => todo!(),
        IR::List(irs) => Value::Array(
            irs.iter()
                .map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect(),
        ),
        IR::Length(ir) => match run(vars, g, runtime, result_fn, ir) {
            Value::Array(arr) => Value::Int(arr.len() as _),
            _ => todo!(),
        },
        IR::GetElement(op) => {
            let arr = run(vars, g, runtime, result_fn, &op.0);
            let i = run(vars, g, runtime, result_fn, &op.1);
            match (arr, i) {
                (Value::Array(values), Value::Int(i)) => {
                    if i < values.len() as _ {
                        values[i as usize].clone()
                    } else {
                        Value::Null
                    }
                }
                _ => Value::Null,
            }
        }
        IR::Range(_) => Value::Null,
        IR::IsNull(ir) => match run(vars, g, runtime, result_fn, ir) {
            Value::Null => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::IsNode(ir) => match run(vars, g, runtime, result_fn, ir) {
            Value::Node(_) => Value::Bool(true),
            _ => Value::Bool(false),
        },
        IR::Or(irs) => {
            for ir in irs {
                if matches!(run(vars, g, runtime, result_fn, ir), Value::Bool(true)) {
                    return Value::Bool(true);
                }
            }

            Value::Bool(false)
        }
        IR::Xor(irs) => Value::Null,
        IR::And(irs) => {
            for ir in irs {
                if matches!(run(vars, g, runtime, result_fn, ir), Value::Bool(false)) {
                    return Value::Bool(false);
                }
            }

            Value::Bool(true)
        }
        IR::Not(ir) => match run(vars, g, runtime, result_fn, ir) {
            Value::Bool(b) => Value::Bool(!b),
            _ => Value::Null,
        },
        IR::Eq(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a == b))
            .unwrap(),
        IR::Neq(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .unwrap(),
        IR::Lt(op) => match (
            run(vars, g, runtime, result_fn, &op.0),
            run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
            _ => Value::Null,
        },
        IR::Gt(op) => match (
            run(vars, g, runtime, result_fn, &op.0),
            run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
            _ => Value::Null,
        },
        IR::Le(op) => match (
            run(vars, g, runtime, result_fn, &op.0),
            run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
            _ => Value::Null,
        },
        IR::Ge(op) => match (
            run(vars, g, runtime, result_fn, &op.0),
            run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
            _ => Value::Null,
        },
        IR::Add(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Sub(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Mul(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Div(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Pow(irs) => irs
            .iter()
            .map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::FuncInvocation(name, irs) => {
            let args = irs
                .iter()
                .map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect();
            if runtime.write_functions.contains_key(name) {
                runtime.write_functions[name](g, runtime, Value::Array(args))
            } else if runtime.read_functions.contains_key(name) {
                runtime.read_functions[name](g, runtime, Value::Array(args))
            } else {
                Value::Null
            }
        }
        IR::Map(items) => Value::Map(
            items
                .iter()
                .map(|(key, ir)| (key.to_string(), run(vars, g, runtime, result_fn, ir)))
                .collect(),
        ),
        IR::Set(x, ir) => {
            let v = run(vars, g, runtime, result_fn, ir);
            vars.insert(x.to_string(), v.clone());
            v
        }
        IR::If(op) => match run(vars, g, runtime, result_fn, &op.0) {
            Value::Bool(true) => run(vars, g, runtime, result_fn, &op.1),
            _ => Value::Null,
        },
        IR::For(op) => {
            run(vars, g, runtime, result_fn, &op.0);
            while run(vars, g, runtime, result_fn, &op.1) == Value::Bool(true) {
                run(vars, g, runtime, result_fn, &op.3);
                run(vars, g, runtime, result_fn, &op.2);
            }
            Value::Null
        }
        IR::Return(ir) => {
            let v = run(vars, g, runtime, result_fn, ir);
            result_fn(g, v);
            Value::Null
        }
        IR::Block(irs) => {
            for ir in irs {
                run(vars, g, runtime, result_fn, ir);
            }
            Value::Null
        }
    }
}
