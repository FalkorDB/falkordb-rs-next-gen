use crate::functions::FnType::Write;
use crate::functions::{FnType, Functions, GraphFn, get_functions};
use crate::{ast::QueryExprIR, graph::Graph, planner::IR, value::Contains, value::Value};
use crate::{matrix, tensor};
use FnType::Read;
use orx_tree::{DynNode, NodeRef};
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub struct Runtime {
    functions: &'static Functions,
    pub agg_ctxs: BTreeMap<u64, (Value, Value)>,
    pub node_iters: Vec<matrix::Iter<bool>>,
    pub relationship_iters: Vec<tensor::Iter>,
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
        Self {
            functions: get_functions(),
            agg_ctxs: BTreeMap::new(),
            node_iters: Vec::new(),
            relationship_iters: Vec::new(),
            parameters,
            nodes_created: 0,
            relationships_created: 0,
            nodes_deleted: 0,
            relationships_deleted: 0,
            properties_set: 0,
            properties_removed: 0,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn ro_run(
    vars: &mut BTreeMap<String, Value>,
    g: &Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
    ir: &DynNode<IR>,
) -> Result<Value, String> {
    match ir.data() {
        IR::Null => Ok(Value::Null),
        IR::Bool(x) => Ok(Value::Bool(*x)),
        IR::Integer(x) => Ok(Value::Int(*x)),
        IR::Float(x) => Ok(Value::Float(*x)),
        IR::String(x) => Ok(Value::String(x.to_string())),
        IR::Var(x) => vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::List => Ok(Value::List(
            ir.children()
                .map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        IR::Length => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        IR::GetElement => {
            let arr = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            let i = ro_run(vars, g, runtime, result_fn, &ir.child(1))?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected List but was {v:?}")),
            }
        }
        IR::GetElements => {
            let arr = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            let a = ro_run(vars, g, runtime, result_fn, &ir.child(1))?;
            let b = ro_run(vars, g, runtime, result_fn, &ir.child(2))?;
            get_elements(arr, a, b)
        }
        IR::Range => {
            let start = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            let end = ro_run(vars, g, runtime, result_fn, &ir.child(1))?;
            let step = ro_run(vars, g, runtime, result_fn, &ir.child(2))?;
            match (start, end, step) {
                (Value::Int(start), Value::Int(end), Value::Int(step)) => {
                    Ok(Value::List(if step < 0 {
                        (end..=start)
                            .step_by((-step) as usize)
                            .map(Value::Int)
                            .collect()
                    } else {
                        (start..=end)
                            .step_by(step as usize)
                            .map(Value::Int)
                            .collect()
                    }))
                }
                _ => Err("Range operator requires two integers".to_string()),
            }
        }
        IR::IsNull => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsNode => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsRelationship => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::Or => {
            let mut is_null = false;
            for ir in ir.children() {
                match ro_run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        IR::Xor => {
            let mut last = None;
            for ir in ir.children() {
                match ro_run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }

        IR::And => {
            let mut is_null = false;
            for ir in ir.children() {
                match ro_run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        IR::Not => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        IR::Negate => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Null => Ok(Value::Null),
            _ => {
                Err("InvalidArgumentType: Negate operator requires an Integer or Float".to_string())
            }
        },
        IR::Eq => all_equals(
            ir.children()
                .map(|ir| ro_run(vars, g, runtime, result_fn, &ir)),
        ),
        IR::Neq => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        IR::Lt => match (
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?,
            ro_run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        IR::Gt => match (
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?,
            ro_run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        IR::Le => match (
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?,
            ro_run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        IR::Ge => match (
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?,
            ro_run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        IR::In => {
            let value = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            let list = ro_run(vars, g, runtime, result_fn, &ir.child(1))?;
            list_contains(&list, &value)
        }
        IR::Add => ir
            .children()
            .map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|acc, value| acc? + value?)
            .ok_or_else(|| "Add operator requires at least one operand".to_string())?,
        IR::Sub => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        IR::Mul => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        IR::Div => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        IR::Pow => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        IR::Modulo => ir
            .children()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        IR::FuncInvocation(name) => {
            let args = ir
                .children()
                .map(|ir| ro_run(vars, g, runtime, result_fn, &ir))
                .collect::<Result<Vec<_>, _>>()?;
            match runtime.functions.get(name) {
                Some(GraphFn {
                    fn_type: Read(func),
                    ..
                }) => func(g, runtime, args),
                Some(GraphFn {
                    fn_type: Write(_), ..
                }) => Err(format!(
                    "Function '{name}' is a write function and cannot be invoked in a read only context"
                )),
                None => Err(format!("Function '{name}' not found")),
            }
        }

        IR::Map => Ok(Value::Map(
            ir.children()
                .map(|child| {
                    (
                        child.data().to_string(),
                        ro_run(vars, g, runtime, result_fn, &child.child(0)).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        IR::Set(x) => {
            let v = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            vars.insert(x.to_string(), v.clone());
            Ok(v)
        }
        IR::If => match ro_run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Bool(true) => ro_run(vars, g, runtime, result_fn, &ir.child(1)),
            _ => Ok(Value::Null),
        },
        IR::For => {
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            while ro_run(vars, g, runtime, result_fn, &ir.child(1))? == Value::Bool(true) {
                ro_run(vars, g, runtime, result_fn, &ir.child(3))?;
                ro_run(vars, g, runtime, result_fn, &ir.child(2))?;
            }
            Ok(Value::Null)
        }
        IR::Return => {
            let v = ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            result_fn(g, v);
            Ok(Value::Null)
        }
        IR::ReturnAggregation => {
            ro_run(vars, g, runtime, result_fn, &ir.child(0))?;
            for (keys, r) in runtime.agg_ctxs.values_mut() {
                if let Value::List(keys) = keys {
                    keys.push(r.clone());
                    result_fn(g, Value::List(keys.clone()));
                } else {
                    result_fn(g, Value::List(vec![r.clone()]));
                }
            }
            runtime.agg_ctxs.clear();
            Ok(Value::Null)
        }
        IR::Block => {
            let mut v = Value::Null;
            for ir in ir.children() {
                v = ro_run(vars, g, runtime, result_fn, &ir)?;
            }
            Ok(v)
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn run(
    vars: &mut BTreeMap<String, Value>,
    g: &mut Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
    ir: &DynNode<IR>,
) -> Result<Value, String> {
    match ir.data() {
        IR::Null => Ok(Value::Null),
        IR::Bool(x) => Ok(Value::Bool(*x)),
        IR::Integer(x) => Ok(Value::Int(*x)),
        IR::Float(x) => Ok(Value::Float(*x)),
        IR::String(x) => Ok(Value::String(x.to_string())),
        IR::Var(x) => vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::List => Ok(Value::List(
            ir.children()
                .map(|ir| run(vars, g, runtime, result_fn, &ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        IR::Length => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        IR::GetElement => {
            let arr = run(vars, g, runtime, result_fn, &ir.child(0))?;
            let i = run(vars, g, runtime, result_fn, &ir.child(1))?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected List but was {v:?}")),
            }
        }
        IR::GetElements => {
            let arr = run(vars, g, runtime, result_fn, &ir.child(0))?;
            let a = run(vars, g, runtime, result_fn, &ir.child(1))?;
            let b = run(vars, g, runtime, result_fn, &ir.child(2))?;
            get_elements(arr, a, b)
        }
        IR::Range => {
            let start = run(vars, g, runtime, result_fn, &ir.child(0))?;
            let end = run(vars, g, runtime, result_fn, &ir.child(1))?;
            let step = run(vars, g, runtime, result_fn, &ir.child(2))?;
            match (start, end, step) {
                (Value::Int(start), Value::Int(end), Value::Int(step)) => {
                    if start >= end && step < 0 {
                        Ok(Value::List(
                            (end..=start)
                                .rev()
                                .step_by(step.unsigned_abs() as usize)
                                .map(Value::Int)
                                .collect(),
                        ))
                    } else if step < 0 {
                        Ok(Value::List(vec![]))
                    } else {
                        Ok(Value::List(
                            (start..=end)
                                .step_by(step as usize)
                                .map(Value::Int)
                                .collect(),
                        ))
                    }
                }
                _ => Err("Range operator requires two integers".to_string()),
            }
        }
        IR::IsNull => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsNode => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsRelationship => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::Or => {
            let mut is_null = false;
            for ir in ir.children() {
                match run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {:?}", &ir)),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        IR::Xor => {
            let mut last = None;
            for ir in ir.children() {
                match run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }
        IR::And => {
            let mut is_null = false;
            for ir in ir.children() {
                match run(vars, g, runtime, result_fn, &ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        IR::Not => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        IR::Negate => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Null => Ok(Value::Null),
            _ => {
                Err("InvalidArgumentType: Negate operator requires an Integer or Float".to_string())
            }
        },
        IR::Eq => all_equals(
            ir.children()
                .map(|ir| run(vars, g, runtime, result_fn, &ir)),
        ),
        IR::Neq => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        IR::Lt => match (
            run(vars, g, runtime, result_fn, &ir.child(0))?,
            run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        IR::Gt => match (
            run(vars, g, runtime, result_fn, &ir.child(0))?,
            run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        IR::Le => match (
            run(vars, g, runtime, result_fn, &ir.child(0))?,
            run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        IR::Ge => match (
            run(vars, g, runtime, result_fn, &ir.child(0))?,
            run(vars, g, runtime, result_fn, &ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        IR::In => {
            let value = run(vars, g, runtime, result_fn, &ir.child(0))?;
            let list = run(vars, g, runtime, result_fn, &ir.child(1))?;
            list_contains(&list, &value)
        }
        IR::Add => ir
            .children()
            .map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|acc, value| acc? + value?)
            .ok_or_else(|| "Add operator requires at least one operand".to_string())?,
        IR::Sub => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        IR::Mul => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        IR::Div => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        IR::Pow => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        IR::Modulo => ir
            .children()
            .flat_map(|ir| run(vars, g, runtime, result_fn, &ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        IR::FuncInvocation(name) => {
            let args = ir
                .children()
                .map(|ir| run(vars, g, runtime, result_fn, &ir))
                .collect::<Result<Vec<_>, _>>()?;

            match runtime.functions.get(name) {
                Some(GraphFn {
                    fn_type: Read(func),
                    ..
                }) => func(g, runtime, args),
                Some(GraphFn {
                    fn_type: Write(func),
                    ..
                }) => func(g, runtime, args),
                None => Err(format!("Function '{name}' not found")),
            }
        }
        IR::Map => Ok(Value::Map(
            ir.children()
                .map(|child| {
                    (
                        child.data().to_string(),
                        run(vars, g, runtime, result_fn, &child.child(0)).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        IR::Set(x) => {
            let v = run(vars, g, runtime, result_fn, &ir.child(0))?;
            vars.insert(x.to_string(), v.clone());
            Ok(v)
        }
        IR::If => match run(vars, g, runtime, result_fn, &ir.child(0))? {
            Value::Bool(true) => run(vars, g, runtime, result_fn, &ir.child(1)),
            _ => Ok(Value::Null),
        },
        IR::For => {
            run(vars, g, runtime, result_fn, &ir.child(0))?;
            while run(vars, g, runtime, result_fn, &ir.child(1))? == Value::Bool(true) {
                run(vars, g, runtime, result_fn, &ir.child(3))?;
                run(vars, g, runtime, result_fn, &ir.child(2))?;
            }
            Ok(Value::Null)
        }
        IR::Return => {
            let v = run(vars, g, runtime, result_fn, &ir.child(0))?;
            result_fn(g, v);
            Ok(Value::Null)
        }
        IR::ReturnAggregation => {
            run(vars, g, runtime, result_fn, &ir.child(0))?;
            for (keys, r) in runtime.agg_ctxs.values_mut() {
                if let Value::List(keys) = keys {
                    keys.push(r.clone());
                    result_fn(g, Value::List(keys.clone()));
                } else {
                    result_fn(g, Value::List(vec![r.clone()]));
                }
            }
            runtime.agg_ctxs.clear();
            Ok(Value::Null)
        }
        IR::Block => {
            let mut v = Value::Null;
            for ir in ir.children() {
                v = run(vars, g, runtime, result_fn, &ir)?;
            }
            Ok(v)
        }
    }
}

#[must_use]
pub fn evaluate_param(expr: QueryExprIR) -> Value {
    match expr {
        QueryExprIR::Null => Value::Null,
        QueryExprIR::Bool(x) => Value::Bool(x),
        QueryExprIR::Integer(x) => Value::Int(x),
        QueryExprIR::Float(x) => Value::Float(x),
        QueryExprIR::String(x) => Value::String(x),
        QueryExprIR::List(irs) => Value::List(irs.into_iter().map(evaluate_param).collect()),
        QueryExprIR::Map(irs) => Value::Map(
            irs.into_iter()
                .map(|(key, ir)| (key, evaluate_param(ir)))
                .collect(),
        ),
        QueryExprIR::Negate(exp) => {
            let v = evaluate_param(*exp);
            match v {
                Value::Int(i) => Value::Int(-i),
                Value::Float(f) => Value::Float(-f),
                _ => Value::Null,
            }
        }
        _ => todo!(),
    }
}

fn get_elements(
    arr: Value,
    start: Value,
    end: Value,
) -> Result<Value, String> {
    match (arr, start, end) {
        (Value::List(values), Value::Int(mut start), Value::Int(mut end)) => {
            if start < 0 {
                start = (values.len() as i64 + start).max(0);
            }
            if end < 0 {
                end = (values.len() as i64 + end).max(0);
            } else {
                end = end.min(values.len() as i64);
            }
            if start > end {
                return Ok(Value::List(vec![]));
            }
            Ok(Value::List(values[start as usize..end as usize].to_vec()))
        }
        (_, Value::Null, _) | (_, _, Value::Null) => Ok(Value::Null),
        _ => Err("Invalid array range parameters.".to_string()),
    }
}

fn list_contains(
    list: &Value,
    value: &Value,
) -> Result<Value, String> {
    match list {
        Value::List(l) => Ok(Contains::contains(l, value)),
        Value::Null => Ok(Value::Null),
        _ => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            list.name()
        )),
    }
}

// the semantic of Eq [1, 2, 3] is: 1 EQ 2 AND 2 EQ 3
fn all_equals<I>(mut iter: I) -> Result<Value, String>
where
    I: Iterator<Item = Result<Value, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(Value::Null),
                Some(Ordering::Less | Ordering::Greater) => return Ok(Value::Bool(false)),
                Some(Ordering::Equal) => {}
            }
        }
        Ok(Value::Bool(true))
    } else {
        Err("Eq operator requires at least two arguments".to_string())
    }
}

#[inline]
const fn logical_xor(
    a: bool,
    b: bool,
) -> bool {
    (a && !b) || (!a && b)
}
