use crate::functions::{FnType, Functions, GraphFn, get_functions};
use crate::{ast::ExprIR, graph::Graph, planner::IR, value::Contains, value::Value};
use crate::{matrix, tensor};
use orx_tree::{DynNode, DynTree, NodeRef};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

pub trait ReturnCallback {
    fn return_value(
        &self,
        graph: &RefCell<Graph>,
        value: Value,
    );
}

pub struct ResultSummary {
    pub run_duration: Duration,
    pub labels_added: usize,
    pub labels_removed: i32,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}

pub struct Runtime<'a> {
    functions: &'static Functions,
    pub agg_ctxs: BTreeMap<u64, (Value, Value)>,
    pub node_iters: Vec<matrix::Iter<bool>>,
    pub relationship_iters: Vec<tensor::Iter>,
    parameters: BTreeMap<String, Value>,
    vars: BTreeMap<String, Value>,
    pub g: &'a RefCell<Graph>,
    write: bool,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}

impl<'a> Runtime<'a> {
    #[must_use]
    pub fn new(
        g: &'a RefCell<Graph>,
        parameters: BTreeMap<String, DynTree<ExprIR>>,
        write: bool,
    ) -> Self {
        Self {
            functions: get_functions(),
            agg_ctxs: BTreeMap::new(),
            node_iters: Vec::new(),
            relationship_iters: Vec::new(),
            parameters: parameters
                .into_iter()
                .map(|(k, v)| (k, evaluate_param(v.root())))
                .collect(),
            vars: BTreeMap::new(),
            g,
            write,
            nodes_created: 0,
            relationships_created: 0,
            nodes_deleted: 0,
            relationships_deleted: 0,
            properties_set: 0,
            properties_removed: 0,
        }
    }

    pub fn query<CB: ReturnCallback>(
        &mut self,
        plan: DynTree<IR>,
        callback: &CB,
    ) -> Result<ResultSummary, String> {
        let labels_count = self.g.borrow().get_labels_count();
        let start = Instant::now();
        self.run(callback, &plan.root())?;
        let run_duration = start.elapsed();

        Ok(ResultSummary {
            run_duration,
            labels_added: self.g.borrow().get_labels_count() - labels_count,
            labels_removed: 0,
            nodes_created: self.nodes_created,
            relationships_created: self.relationships_created,
            nodes_deleted: self.nodes_deleted,
            relationships_deleted: self.relationships_deleted,
            properties_set: self.properties_set,
            properties_removed: self.properties_removed,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn run_expr(
        &mut self,
        ir: &DynNode<ExprIR>,
    ) -> Result<Value, String> {
        match ir.data() {
            ExprIR::Null => Ok(Value::Null),
            ExprIR::Bool(x) => Ok(Value::Bool(*x)),
            ExprIR::Integer(x) => Ok(Value::Int(*x)),
            ExprIR::Float(x) => Ok(Value::Float(*x)),
            ExprIR::String(x) => Ok(Value::String(x.to_string())),
            ExprIR::Var(x) => self.vars.get(x).map_or_else(
                || Err(format!("Variable {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::Parameter(x) => self.parameters.get(x).map_or_else(
                || Err(format!("Parameter {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::List => Ok(Value::List(
                ir.children()
                    .map(|ir| self.run_expr(&ir))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            ExprIR::Length => match self.run_expr(&ir.child(0))? {
                Value::List(arr) => Ok(Value::Int(arr.len() as _)),
                _ => Err("Length operator requires a list".to_string()),
            },
            ExprIR::GetElement => {
                let arr = self.run_expr(&ir.child(0))?;
                let i = self.run_expr(&ir.child(1))?;
                match (arr, i) {
                    (Value::List(values), Value::Int(i)) => {
                        if i >= 0 && i < values.len() as _ {
                            Ok(values[i as usize].clone())
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    (Value::List(_), v) => {
                        Err(format!("Type mismatch: expected Bool but was {v:?}"))
                    }
                    v => Err(format!("Type mismatch: expected List but was {v:?}")),
                }
            }
            ExprIR::GetElements => {
                let arr = self.run_expr(&ir.child(0))?;
                let a = self.run_expr(&ir.child(1))?;
                let b = self.run_expr(&ir.child(2))?;
                get_elements(arr, a, b)
            }
            ExprIR::IsNull => match self.run_expr(&ir.child(0))? {
                Value::Null => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsNode => match self.run_expr(&ir.child(0))? {
                Value::Node(_) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsRelationship => match self.run_expr(&ir.child(0))? {
                Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::Or => {
                let mut is_null = false;
                for ir in ir.children() {
                    match self.run_expr(&ir)? {
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
            ExprIR::Xor => {
                let mut last = None;
                for ir in ir.children() {
                    match self.run_expr(&ir)? {
                        Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                        Value::Null => return Ok(Value::Null),
                        _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                Ok(Value::Bool(last.unwrap_or(false)))
            }
            ExprIR::And => {
                let mut is_null = false;
                for ir in ir.children() {
                    match self.run_expr(&ir)? {
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
            ExprIR::Not => match self.run_expr(&ir.child(0))? {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                Value::Null => Ok(Value::Null),
                _ => {
                    Err("InvalidArgumentType: Not operator requires a boolean or null".to_string())
                }
            },
            ExprIR::Negate => match self.run_expr(&ir.child(0))? {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                Value::Null => Ok(Value::Null),
                _ => Err(
                    "InvalidArgumentType: Negate operator requires an Integer or Float".to_string(),
                ),
            },
            ExprIR::Eq => all_equals(ir.children().map(|ir| self.run_expr(&ir))),
            ExprIR::Neq => ir
                .children()
                .flat_map(|ir| self.run_expr(&ir))
                .reduce(|a, b| Value::Bool(a != b))
                .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
            ExprIR::Lt => match (self.run_expr(&ir.child(0))?, self.run_expr(&ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                _ => Err("Lt operator requires two integers".to_string()),
            },
            ExprIR::Gt => match (self.run_expr(&ir.child(0))?, self.run_expr(&ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                _ => Err("Gt operator requires two integers".to_string()),
            },
            ExprIR::Le => match (self.run_expr(&ir.child(0))?, self.run_expr(&ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                _ => Err("Le operator requires two integers".to_string()),
            },
            ExprIR::Ge => match (self.run_expr(&ir.child(0))?, self.run_expr(&ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                _ => Err("Ge operator requires two integers".to_string()),
            },
            ExprIR::In => {
                let value = self.run_expr(&ir.child(0))?;
                let list = self.run_expr(&ir.child(1))?;
                list_contains(&list, &value)
            }
            ExprIR::Add => ir
                .children()
                .map(|ir| self.run_expr(&ir))
                .reduce(|acc, value| acc? + value?)
                .ok_or_else(|| "Add operator requires at least one operand".to_string())?,
            ExprIR::Sub => ir
                .children()
                .map(|ir| self.run_expr(&ir))
                .reduce(|acc, value| acc? - value?)
                .ok_or_else(|| "Sub operator requires at least one argument".to_string())?,
            ExprIR::Mul => ir
                .children()
                .map(|ir| self.run_expr(&ir))
                .reduce(|acc, value| acc? * value?)
                .ok_or_else(|| "Mul operator requires at least one argument".to_string())?,
            ExprIR::Div => ir
                .children()
                .map(|ir| self.run_expr(&ir))
                .reduce(|acc, value| acc? / value?)
                .ok_or_else(|| "Div operator requires at least one argument".to_string())?,
            ExprIR::Modulo => ir
                .children()
                .map(|ir| self.run_expr(&ir))
                .reduce(|acc, value| acc? % value?)
                .ok_or_else(|| "Modulo operator requires at least one argument".to_string())?,
            ExprIR::Pow => ir
                .children()
                .flat_map(|ir| self.run_expr(&ir))
                .reduce(|a, b| match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                    _ => Value::Null,
                })
                .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
            ExprIR::FuncInvocation(name) => {
                let args = ir
                    .children()
                    .map(|ir| self.run_expr(&ir))
                    .collect::<Result<Vec<_>, _>>()?;
                match self.functions.get(name) {
                    Some(GraphFn {
                        fn_type: FnType::Read(func),
                        ..
                    }) => func(self, args),
                    Some(GraphFn {
                        fn_type: FnType::Write(func),
                        ..
                    }) => {
                        if !self.write {
                            return Err(
                                "graph.RO_QUERY is to be executed only on read-only queries"
                                    .to_string(),
                            );
                        }
                        func(self, args)
                    }
                    None => Err(format!("Function '{name}' not found")),
                }
            }
            ExprIR::Map => Ok(Value::Map(
                ir.children()
                    .map(|child| {
                        (
                            if let ExprIR::Var(key) = child.data() {
                                key.to_string()
                            } else {
                                todo!();
                            },
                            self.run_expr(&child.child(0)).unwrap_or(Value::Null),
                        )
                    })
                    .collect(),
            )),
            ExprIR::Set(x) => {
                let v = self.run_expr(&ir.child(0))?;
                self.vars.insert(x.to_string(), v.clone());
                Ok(v)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run<CB: ReturnCallback>(
        &mut self,
        callback: &CB,
        ir: &DynNode<IR>,
    ) -> Result<Value, String> {
        match ir.data() {
            IR::Expr(expr) => self.run_expr(&expr.root()),
            IR::If => match self.run(callback, &ir.child(0))? {
                Value::Bool(true) => self.run(callback, &ir.child(1)),
                _ => Ok(Value::Null),
            },
            IR::For => {
                self.run(callback, &ir.child(0))?;
                while self.run(callback, &ir.child(1))? == Value::Bool(true) {
                    self.run(callback, &ir.child(3))?;
                    self.run(callback, &ir.child(2))?;
                }
                Ok(Value::Null)
            }
            IR::Return => {
                let v = self.run(callback, &ir.child(0))?;
                callback.return_value(&self.g, v);
                Ok(Value::Null)
            }
            IR::ReturnAggregation => {
                self.run(callback, &ir.child(0))?;
                for (keys, r) in self.agg_ctxs.values_mut() {
                    if let Value::List(keys) = keys {
                        keys.push(r.clone());
                        callback.return_value(&self.g, Value::List(keys.clone()));
                    } else {
                        callback.return_value(&self.g, Value::List(vec![r.clone()]));
                    }
                }
                self.agg_ctxs.clear();
                Ok(Value::Null)
            }
            IR::Block => {
                let mut v = Value::Null;
                for ir in ir.children() {
                    v = self.run(callback, &ir)?;
                }
                Ok(v)
            }
        }
    }
}

#[must_use]
fn evaluate_param(expr: DynNode<ExprIR>) -> Value {
    match expr.data() {
        ExprIR::Null => Value::Null,
        ExprIR::Bool(x) => Value::Bool(*x),
        ExprIR::Integer(x) => Value::Int(*x),
        ExprIR::Float(x) => Value::Float(*x),
        ExprIR::String(x) => Value::String(x.to_string()),
        ExprIR::List => Value::List(expr.children().map(evaluate_param).collect()),
        ExprIR::Map => Value::Map(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::Var(key) => (key.to_string(), evaluate_param(ir.child(0))),
                    _ => todo!(),
                })
                .collect(),
        ),
        ExprIR::Negate => {
            let v = evaluate_param(expr.child(0));
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
