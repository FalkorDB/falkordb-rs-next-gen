use std::collections::BTreeMap;

use crate::{ast::QueryExprIR, graph::Graph, matrix::Iter, planner::IR};

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Node(u64),
    Relationship(u64, u64, u64),
}

pub struct Runtime {
    write_functions: BTreeMap<String, fn(&mut Graph, &mut Runtime, Value) -> Value>,
    read_functions: BTreeMap<String, fn(&Graph, &mut Runtime, Value) -> Value>,
    iters: Vec<Iter<bool>>,
    parameters: BTreeMap<String, Value>,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}

impl Runtime {
    #[must_use]
    pub fn new(parameters: BTreeMap<String, Value>) -> Self {
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
            parameters,
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
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (iter.next(), iter.next(), iter.next()) {
                    (Some(Value::List(raw_labels)), Some(Value::Map(attrs)), None) => {
                        let labels = raw_labels
                            .into_iter()
                            .filter_map(|label| {
                                if let Value::String(label) = label {
                                    Some(label)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();
                        runtime.nodes_created += 1;
                        runtime.properties_set += attrs
                            .values()
                            .map(|v| match v {
                                Value::Null => 0,
                                _ => 1,
                            })
                            .sum::<i32>();
                        g.create_node(&labels, attrs)
                    }
                    _ => Value::Null,
                }
            }
            _ => Value::Null,
        }
    }

    fn delete_entity(g: &mut Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::List(nodes) => {
                for n in nodes {
                    if let Value::Node(id) = n {
                        runtime.nodes_deleted += 1;
                        g.delete_node(id);
                    }
                }
            }
            _ => todo!(),
        }

        Value::Null
    }

    fn create_relationship(g: &mut Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                ) {
                    (
                        Some(Value::String(relationship_type)),
                        Some(Value::Node(from)),
                        Some(Value::Node(to)),
                        Some(Value::Map(attrs)),
                        None,
                    ) => {
                        runtime.relationships_created += 1;
                        runtime.properties_set += attrs
                            .values()
                            .map(|v| match v {
                                Value::Null => 0,
                                _ => 1,
                            })
                            .sum::<i32>();
                        g.create_relationship(&relationship_type, from, to, attrs)
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn create_node_iter(g: &Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (iter.next(), iter.next()) {
                    (Some(Value::List(raw_labels)), None) => {
                        runtime.iters.push(
                            g.get_nodes(
                                raw_labels
                                    .into_iter()
                                    .filter_map(|label| {
                                        if let Value::String(label) = label {
                                            Some(label)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                            )
                            .unwrap(),
                        );
                        Value::Int(runtime.iters.len() as i64 - 1)
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn next_node(_g: &Graph, runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::List(args) => match args.as_slice() {
                [Value::Int(iter)] => runtime.iters[*iter as usize]
                    .next()
                    .map_or_else(|| Value::Null, |(n, _)| Value::Node(n)),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn property(g: &Graph, _runtime: &mut Self, args: Value) -> Value {
        match args {
            Value::List(arr) => match arr.as_slice() {
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
        Value::List(
            g.get_labels()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }

    fn types(g: &Graph, _runtime: &mut Self, _args: Value) -> Value {
        Value::List(
            g.get_types()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }

    fn properties(g: &Graph, _runtime: &mut Self, _args: Value) -> Value {
        Value::List(
            g.get_properties()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        )
    }
}

#[allow(clippy::too_many_lines)]
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
        IR::Var(x) => vars.get(x).unwrap_or(&Value::Null).to_owned(),
        IR::Parameter(x) => runtime.parameters.get(x).unwrap_or(&Value::Null).to_owned(),
        IR::List(irs) => Value::List(
            irs.iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect(),
        ),
        IR::Length(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::List(arr) => Value::Int(arr.len() as _),
            _ => Value::Null,
        },
        IR::GetElement(op) => {
            let arr = ro_run(vars, g, runtime, result_fn, &op.0);
            let i = ro_run(vars, g, runtime, result_fn, &op.1);
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        values[i as usize].clone()
                    } else {
                        Value::Null
                    }
                }
                _ => Value::Null,
            }
        }
        IR::Range(_) => Value::Null,
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
        IR::Xor(irs) => Value::Null,
        IR::And(irs) => {
            let mut is_null = false;
            for ir in irs {
                match ro_run(vars, g, runtime, result_fn, ir) {
                    Value::Bool(false) => return Value::Bool(false),
                    Value::Null => is_null = true,
                    _ => {}
                }
            }
            if is_null {
                return Value::Null;
            }

            Value::Bool(true)
        }
        IR::Not(ir) => match ro_run(vars, g, runtime, result_fn, ir) {
            Value::Bool(b) => Value::Bool(!b),
            _ => Value::Null,
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
            _ => Value::Null,
        },
        IR::Gt(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
            _ => Value::Null,
        },
        IR::Le(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
            _ => Value::Null,
        },
        IR::Ge(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0),
            ro_run(vars, g, runtime, result_fn, &op.1),
        ) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
            _ => Value::Null,
        },
        IR::Add(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                (Value::List(a), Value::List(b)) => Value::List(a.into_iter().chain(b).collect()),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Sub(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Mul(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Div(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::Pow(irs) => irs
            .iter()
            .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
                _ => Value::Null,
            })
            .unwrap(),
        IR::FuncInvocation(name, irs) => {
            let args = irs
                .iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect();
            runtime.read_functions[name](g, runtime, Value::List(args))
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

#[allow(clippy::too_many_lines)]
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
        IR::Parameter(x) => runtime.parameters.get(x).unwrap_or(&Value::Null).to_owned(),
        IR::List(irs) => Value::List(
            irs.iter()
                .map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect(),
        ),
        IR::Length(ir) => match run(vars, g, runtime, result_fn, ir) {
            Value::List(arr) => Value::Int(arr.len() as _),
            _ => Value::Null,
        },
        IR::GetElement(op) => {
            let arr = run(vars, g, runtime, result_fn, &op.0);
            let i = run(vars, g, runtime, result_fn, &op.1);
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
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
            let mut is_null = false;
            for ir in irs {
                match run(vars, g, runtime, result_fn, ir) {
                    Value::Bool(false) => return Value::Bool(false),
                    Value::Null => is_null = true,
                    _ => {}
                }
            }
            if is_null {
                return Value::Null;
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
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                (Value::List(a), Value::List(b)) => Value::List(a.into_iter().chain(b).collect()),
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
            let mut args = Vec::with_capacity(irs.len());
            args.extend(irs.iter().map(|ir| run(vars, g, runtime, result_fn, ir)));
            if runtime.write_functions.contains_key(name) {
                runtime.write_functions[name](g, runtime, Value::List(args))
            } else if runtime.read_functions.contains_key(name) {
                runtime.read_functions[name](g, runtime, Value::List(args))
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

pub fn evaluate_param(expr: QueryExprIR) -> Value {
    match expr {
        QueryExprIR::Null => Value::Null,
        QueryExprIR::Bool(x) => Value::Bool(x),
        QueryExprIR::Integer(x) => Value::Int(x),
        QueryExprIR::Float(x) => Value::Float(x),
        QueryExprIR::String(x) => Value::String(x.to_string()),
        QueryExprIR::List(irs) => {
            Value::List(irs.into_iter().map(|ir| evaluate_param(ir)).collect())
        }
        QueryExprIR::Map(irs) => Value::Map(
            irs.into_iter()
                .map(|(key, ir)| (key.to_string(), evaluate_param(ir)))
                .collect(),
        ),
        _ => todo!(),
    }
}
