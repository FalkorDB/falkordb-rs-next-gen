use crate::ast::{NodePattern, Pattern, RelationshipPattern};
use crate::functions::{FnType, Functions, GraphFn, get_functions};
use crate::iter::{Aggregate, LazyReplace, LazyReplaceIter, TryFlatMap, TryMap};
use crate::value::Env;
use crate::{ast::ExprIR, graph::Graph, planner::IR, value::Contains, value::Value};
use ordermap::OrderMap;
use orx_tree::{Dyn, DynNode, DynTree, NodeIdx, NodeRef};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::iter::{empty, once, repeat_n};
use std::rc::Rc;
use std::time::{Duration, Instant};

pub trait ReturnCallback {
    fn return_value(
        &self,
        graph: &RefCell<Graph>,
        env: Env,
        return_names: &Vec<String>,
    );
}

pub struct ResultSummary {
    pub run_duration: Duration,
    pub labels_added: usize,
    pub labels_removed: usize,
    pub nodes_created: usize,
    pub relationships_created: usize,
    pub nodes_deleted: usize,
    pub relationships_deleted: usize,
    pub properties_set: usize,
    pub properties_removed: usize,
    pub return_names: Vec<String>,
}

#[derive(Default)]
pub struct Stats {
    pub nodes_created: usize,
    pub relationships_created: usize,
    pub nodes_deleted: usize,
    pub relationships_deleted: usize,
    pub properties_set: usize,
    pub properties_removed: usize,
}

#[derive(Default)]
pub struct Pending {
    pub created_nodes: BTreeMap<u64, (Vec<Rc<String>>, OrderMap<Rc<String>, Value>)>,
    pub created_relationships: BTreeMap<u64, (Rc<String>, u64, u64, OrderMap<Rc<String>, Value>)>,
}

pub struct Runtime<'a> {
    functions: &'static Functions,
    parameters: BTreeMap<String, Value>,
    pub g: &'a RefCell<Graph>,
    write: bool,
    pub pending: RefCell<Pending>,
    pub stats: RefCell<Stats>,
    pub plan: DynTree<IR>,
}

trait ReturnNames {
    fn get_return_names(&self) -> Vec<String>;
}

impl ReturnNames for DynNode<'_, IR> {
    fn get_return_names(&self) -> Vec<String> {
        match self.data() {
            IR::Project(trees) => trees.iter().map(|v| v.0.clone()).collect(),
            IR::Commit => self
                .get_child(0)
                .map_or(vec![], |child| child.get_return_names()),
            IR::Call(name, _) => vec![name.clone()],
            IR::Aggregate(name, _, _) => name.clone(),
            _ => vec![],
        }
    }
}

impl<'a> Runtime<'a> {
    #[must_use]
    pub fn new(
        g: &'a RefCell<Graph>,
        parameters: BTreeMap<String, DynTree<ExprIR>>,
        write: bool,
        plan: DynTree<IR>,
    ) -> Self {
        Self {
            functions: get_functions(),
            parameters: parameters
                .into_iter()
                .map(|(k, v)| (k, evaluate_param(v.root())))
                .collect(),
            g,
            write,
            pending: RefCell::new(Pending::default()),
            stats: RefCell::new(Stats::default()),
            plan,
        }
    }

    pub fn query<CB: ReturnCallback>(
        &mut self,
        callback: &CB,
    ) -> Result<ResultSummary, String> {
        let labels_count = self.g.borrow().get_labels_count();
        let start = Instant::now();
        let idx = self.plan.root().idx();
        let return_names = self.plan.root().get_return_names();
        for v in self.run(&idx)? {
            let v = v?;
            callback.return_value(self.g, v, &return_names);
        }
        let run_duration = start.elapsed();

        let stats = self.stats.borrow();
        Ok(ResultSummary {
            run_duration,
            labels_added: self.g.borrow().get_labels_count() - labels_count,
            labels_removed: 0,
            nodes_created: stats.nodes_created,
            relationships_created: stats.relationships_created,
            nodes_deleted: stats.nodes_deleted,
            relationships_deleted: stats.relationships_deleted,
            properties_set: stats.properties_set,
            properties_removed: stats.properties_removed,
            return_names,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn run_expr(
        &self,
        ir: DynNode<ExprIR>,
        vars: &Env,
    ) -> Result<Value, String> {
        match ir.data() {
            ExprIR::Null => Ok(Value::Null),
            ExprIR::Bool(x) => Ok(Value::Bool(*x)),
            ExprIR::Integer(x) => Ok(Value::Int(*x)),
            ExprIR::Float(x) => Ok(Value::Float(*x)),
            ExprIR::String(x) => Ok(Value::String(x.clone())),
            ExprIR::Var(x) => vars.get(x).map_or_else(
                || Err(format!("Variable {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::Parameter(x) => self.parameters.get(x).map_or_else(
                || Err(format!("Parameter {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::List => Ok(Value::List(
                ir.children()
                    .map(|ir| self.run_expr(ir, vars))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            ExprIR::Length => match self.run_expr(ir.child(0), vars)? {
                Value::List(arr) => Ok(Value::Int(arr.len() as _)),
                _ => Err(String::from("Length operator requires a list")),
            },
            ExprIR::GetElement => {
                let arr = self.run_expr(ir.child(0), vars)?;
                let i = self.run_expr(ir.child(1), vars)?;
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
                    (Value::Map(map), Value::String(key)) => map
                        .get(&key)
                        .map_or_else(|| Ok(Value::Null), |v| Ok(v.clone())),
                    (Value::Map(_), Value::Null) | (Value::Null, _) => Ok(Value::Null),
                    v => Err(format!("Type mismatch: expected List but was {v:?}")),
                }
            }
            ExprIR::GetElements => {
                let arr = self.run_expr(ir.child(0), vars)?;
                let a = self.run_expr(ir.child(1), vars)?;
                let b = self.run_expr(ir.child(2), vars)?;
                get_elements(arr, a, b)
            }
            ExprIR::IsNull => match self.run_expr(ir.child(0), vars)? {
                Value::Null => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsNode => match self.run_expr(ir.child(0), vars)? {
                Value::Node(_) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsRelationship => match self.run_expr(ir.child(0), vars)? {
                Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::Or => {
                let mut is_null = false;
                for ir in ir.children() {
                    match self.run_expr(ir, vars)? {
                        Value::Bool(true) => return Ok(Value::Bool(true)),
                        Value::Bool(false) => {}
                        Value::Null => is_null = true,
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
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
                    match self.run_expr(ir, vars)? {
                        Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                        Value::Null => return Ok(Value::Null),
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                Ok(Value::Bool(last.unwrap_or(false)))
            }
            ExprIR::And => {
                let mut is_null = false;
                for ir in ir.children() {
                    match self.run_expr(ir, vars)? {
                        Value::Bool(false) => return Ok(Value::Bool(false)),
                        Value::Bool(true) => {}
                        Value::Null => is_null = true,
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                if is_null {
                    return Ok(Value::Null);
                }

                Ok(Value::Bool(true))
            }
            ExprIR::Not => match self.run_expr(ir.child(0), vars)? {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                Value::Null => Ok(Value::Null),
                v => Err(format!(
                    "Type mismatch: expected Boolean or Null but was {}",
                    v.name()
                )),
            },
            ExprIR::Negate => match self.run_expr(ir.child(0), vars)? {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                Value::Null => Ok(Value::Null),
                v => Err(format!(
                    "Type mismatch: expected Integer, Float, or Null but was {}",
                    v.name()
                )),
            },
            ExprIR::Eq => all_equals(ir.children().map(|ir| self.run_expr(ir, vars))),
            ExprIR::Neq => all_not_equals(ir.children().map(|ir| self.run_expr(ir, vars))),
            ExprIR::Lt => match self
                .run_expr(ir.child(0), vars)?
                .partial_cmp(&self.run_expr(ir.child(1), vars)?)
            {
                Some(Ordering::Less) => Ok(Value::Bool(true)),
                Some(Ordering::Greater | Ordering::Equal) => Ok(Value::Bool(false)),
                None => Ok(Value::Null),
            },
            ExprIR::Gt => match self
                .run_expr(ir.child(0), vars)?
                .partial_cmp(&self.run_expr(ir.child(1), vars)?)
            {
                Some(Ordering::Greater) => Ok(Value::Bool(true)),
                Some(Ordering::Less | Ordering::Equal) => Ok(Value::Bool(false)),
                None => Ok(Value::Null),
            },
            ExprIR::Le => match self
                .run_expr(ir.child(0), vars)?
                .partial_cmp(&self.run_expr(ir.child(1), vars)?)
            {
                Some(Ordering::Less | Ordering::Equal) => Ok(Value::Bool(true)),
                Some(Ordering::Greater) => Ok(Value::Bool(false)),
                None => Ok(Value::Null),
            },
            ExprIR::Ge => match self
                .run_expr(ir.child(0), vars)?
                .partial_cmp(&self.run_expr(ir.child(1), vars)?)
            {
                Some(Ordering::Greater | Ordering::Equal) => Ok(Value::Bool(true)),
                Some(Ordering::Less) => Ok(Value::Bool(false)),
                None => Ok(Value::Null),
            },
            ExprIR::In => {
                let value = self.run_expr(ir.child(0), vars)?;
                let list = self.run_expr(ir.child(1), vars)?;
                list_contains(&list, &value)
            }
            ExprIR::Add => ir
                .children()
                .map(|ir| self.run_expr(ir, vars))
                .reduce(|acc, value| acc? + value?)
                .ok_or_else(|| String::from("Add operator requires at least one operand"))?,
            ExprIR::Sub => ir
                .children()
                .map(|ir| self.run_expr(ir, vars))
                .reduce(|acc, value| acc? - value?)
                .ok_or_else(|| String::from("Sub operator requires at least one argument"))?,
            ExprIR::Mul => ir
                .children()
                .map(|ir| self.run_expr(ir, vars))
                .reduce(|acc, value| acc? * value?)
                .ok_or_else(|| String::from("Mul operator requires at least one argument"))?,
            ExprIR::Div => ir
                .children()
                .map(|ir| self.run_expr(ir, vars))
                .reduce(|acc, value| acc? / value?)
                .ok_or_else(|| String::from("Div operator requires at least one argument"))?,
            ExprIR::Modulo => ir
                .children()
                .map(|ir| self.run_expr(ir, vars))
                .reduce(|acc, value| acc? % value?)
                .ok_or_else(|| String::from("Modulo operator requires at least one argument"))?,
            ExprIR::Pow => ir
                .children()
                .flat_map(|ir| self.run_expr(ir, vars))
                .reduce(|a, b| match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                    _ => Value::Null,
                })
                .ok_or_else(|| String::from("Pow operator requires at least one argument")),
            ExprIR::FuncInvocation(name, fn_type) => {
                let args = ir
                    .children()
                    .map(|ir| self.run_expr(ir, vars))
                    .collect::<Result<Vec<_>, _>>()?;
                match self.functions.get(name, fn_type) {
                    Some(GraphFn { func, write, .. }) => {
                        if !self.write && *write {
                            return Err(String::from(
                                "graph.RO_QUERY is to be executed only on read-only queries",
                            ));
                        }
                        func(self, args)
                    }
                    None => Err(format!("Function '{name}' not found")),
                }
            }
            ExprIR::Map => Ok(Value::Map(
                ir.children()
                    .map(|child| {
                        Ok((
                            if let ExprIR::Var(key) = child.data() {
                                key.clone()
                            } else {
                                todo!();
                            },
                            self.run_expr(child.child(0), vars)?,
                        ))
                    })
                    .collect::<Result<_, String>>()?,
            )),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run(
        &self,
        idx: &NodeIdx<Dyn<IR>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + '_>, String> {
        let child0_idx = self.plan.node(idx).get_child(0).map(|n| n.idx());
        let child1_idx = self.plan.node(idx).get_child(1).map(|n| n.idx());
        match self.plan.node(idx).data() {
            IR::Empty => Ok(Box::new(empty())),
            IR::Optional(vars) => {
                if let Some(child_idx) = child0_idx {
                    let iter = self.run(&child_idx)?.lazy_replace(move || {
                        let mut env = Env::new();
                        for v in vars {
                            env.insert(v.clone(), Value::Null);
                        }
                        Box::new(once(Ok(env)))
                    });
                    return Ok(Box::new(iter));
                }
                Ok(Box::new(empty()))
            }
            IR::Call(name, trees) => match self.functions.get(name, &FnType::Procedure) {
                Some(func) => {
                    let args = trees
                        .iter()
                        .map(|ir| self.run_expr(ir.root(), &Env::new()))
                        .collect::<Result<Vec<_>, _>>()?;
                    if !self.write && func.write {
                        return Err(String::from(
                            "graph.RO_QUERY is to be executed only on read-only queries",
                        ));
                    }
                    let res = (func.func)(self, args)?;
                    match res {
                        Value::List(arr) => Ok(Box::new(arr.into_iter().map(|v| {
                            let mut env = Env::new();
                            env.insert(name.clone(), v);
                            Ok(env)
                        }))),
                        _ => Err(format!("Function '{name}' must return a list")),
                    }
                }
                None => Err(format!("Function '{name}' not found")),
            },
            IR::Unwind(tree, name) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |vars| {
                            let value = self.run_expr(tree.root(), &vars);
                            match value {
                                Ok(Value::List(arr)) => arr
                                    .into_iter()
                                    .map(|v| {
                                        let mut vars = vars.clone();
                                        vars.insert(name.clone(), v);
                                        Ok(vars)
                                    })
                                    .collect::<Vec<_>>()
                                    .into_iter(),
                                Ok(_) => vec![Err(String::from("Unwind operator requires a list"))]
                                    .into_iter(),
                                Err(e) => vec![Err(e)].into_iter(),
                            }
                        },
                        |e| vec![Err(e)].into_iter(),
                    )));
                }
                let vars = Env::new();
                let value = self.run_expr(tree.root(), &vars)?;
                if let Value::List(arr) = value {
                    return Ok(Box::new(arr.into_iter().map(move |v| {
                        let mut vars = Env::new();
                        vars.insert(name.clone(), v);
                        Ok(vars)
                    })));
                }
                Ok(Box::new(once(Err(String::from(
                    "Unwind operator requires a list",
                )))))
            }
            IR::UnwindRange(start, stop, step, name) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |vars| {
                            let start = self.run_expr(start.root(), &vars);
                            let stop = self.run_expr(stop.root(), &vars);
                            let step = self.run_expr(step.root(), &vars);
                            match (start, stop, step) {
                                (
                                    Ok(Value::Int(start)),
                                    Ok(Value::Int(stop)),
                                    Ok(Value::Int(step)),
                                ) => Box::new(
                                    repeat_n(start, ((stop - start) / step + 1) as _)
                                        .enumerate()
                                        .map(move |(i, v)| {
                                            let mut vars = vars.clone();
                                            vars.insert(
                                                name.clone(),
                                                Value::Int(v + i as i64 * step),
                                            );
                                            Ok(vars)
                                        }),
                                )
                                    as Box<dyn Iterator<Item = Result<Env, String>>>,
                                _ => {
                                    todo!();
                                }
                            }
                        },
                        |e| {
                            Box::new(repeat_n(0, 1).enumerate().map(move |_| Err(e.clone())))
                                as Box<dyn Iterator<Item = Result<Env, String>>>
                        },
                    )));
                }
                let vars = Env::new();
                let start = self.run_expr(start.root(), &vars)?;
                let stop = self.run_expr(stop.root(), &vars)?;
                let step = self.run_expr(step.root(), &vars)?;
                match (start, stop, step) {
                    (Value::Int(start), Value::Int(stop), Value::Int(step)) => {
                        if step == 0 {
                            return Err(String::from("Step cannot be zero"));
                        }
                        let iter = std::iter::repeat_n(start, ((stop - start) / step + 1) as _)
                            .enumerate()
                            .map(move |(i, v)| v + i as i64 * step);
                        Ok(Box::new(iter.map(move |v| {
                            let mut vars = Env::new();
                            vars.insert(name.clone(), Value::Int(v));
                            Ok(vars)
                        })))
                    }
                    _ => Err(String::from("UnwindRange operator requires three integers")),
                }
            }
            IR::Create(pattern) => {
                let mut parent_commit = false;
                if let Some(parent) = self.plan.node(idx).parent() {
                    if matches!(parent.data(), IR::Commit) && parent.parent().is_none() {
                        parent_commit = true;
                    }
                }
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |mut vars| {
                            if let Err(e) = self.create(pattern, &mut vars) {
                                return vec![Err(e)].into_iter();
                            }

                            if parent_commit {
                                return vec![].into_iter();
                            }

                            vec![Ok(vars)].into_iter()
                        },
                        |e| vec![Err(e)].into_iter(),
                    )));
                }
                let mut vars = Env::new();
                self.create(pattern, &mut vars)?;
                if parent_commit {
                    return Ok(Box::new(empty()));
                }
                Ok(Box::new(once(Ok(vars))))
            }
            IR::Merge(pattern) => {
                if let Some(child_idx) = child1_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |vars| {
                            let cvars = vars.clone();
                            let iter =
                                (Box::new(self.run(child0_idx.as_ref().unwrap()).unwrap().try_map(
                                    move |v| {
                                        let mut vars = vars.clone();
                                        for (k, v) in v.iter() {
                                            vars.insert(k.clone(), v.clone());
                                        }
                                        Ok(vars)
                                    },
                                ))
                                    as Box<dyn Iterator<Item = Result<Env, String>>>)
                                    .lazy_replace(move || {
                                        let mut vars = cvars.clone();
                                        self.create(pattern, &mut vars);
                                        Box::new(vec![Ok(vars)].into_iter())
                                    });
                            Box::new(iter) as Box<dyn Iterator<Item = Result<Env, String>>>
                        },
                        |e| Box::new(once(Err(e))) as Box<dyn Iterator<Item = Result<Env, String>>>,
                    )));
                }
                let iter = self.run(child0_idx.as_ref().unwrap())?.lazy_replace(|| {
                    let mut vars = Env::new();
                    match self.create(pattern, &mut vars) {
                        Ok(()) => Box::new(vec![Ok(vars)].into_iter()),
                        Err(e) => Box::new(once(Err(e))),
                    }
                });
                Ok(Box::new(iter))
            }
            IR::Delete(trees) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_map(move |vars| {
                        self.delete(trees, &vars)?;
                        Ok(vars)
                    })));
                }
                Ok(Box::new(empty()))
            }
            IR::NodeScan(node_pattern) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |vars| self.node_scan(node_pattern, vars),
                        |e| Box::new(once(Err(e))),
                    )));
                }
                Ok(self.node_scan(node_pattern, Env::new()))
            }
            IR::RelationshipScan(relationship_pattern) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(
                        move |vars| self.relationship_scan(relationship_pattern, vars),
                        |e| Box::new(once(Err(e))),
                    )));
                }
                Ok(self.relationship_scan(relationship_pattern, Env::new()))
            }
            IR::Filter(tree) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.filter(move |vars| {
                        let vars = vars.clone().unwrap();
                        self.run_expr(tree.root(), &vars) == Ok(Value::Bool(true))
                    })));
                }
                Err(String::from(
                    "Filter operator requires a boolean expression",
                ))
            }
            IR::Aggregate(_, trees, trees1) => {
                if let Some(child_idx) = child0_idx {
                    let mut default_value = Env::new();
                    for (name, _) in trees1 {
                        default_value.insert(name.clone(), Value::Null);
                    }
                    let aggregator = self
                        .run(&child_idx)?
                        .aggregate(
                            move |vars| {
                                let vars = vars.clone()?;
                                let mut return_vars = Env::new();
                                for (name, tree) in trees {
                                    let value = self.run_expr(tree.root(), &vars)?;
                                    return_vars.insert(name.clone(), value);
                                }
                                Ok::<Env, String>(return_vars)
                            },
                            Ok(default_value),
                            move |x, acc| {
                                let mut x = x?;
                                let mut acc = acc?;
                                let mut tmp = trees1.iter();
                                for (name, _) in trees1 {
                                    let value = acc.get(name).unwrap().clone();
                                    x.insert(name.clone(), value);
                                }
                                if let Some(tree) = tmp.next() {
                                    let v = self.run_expr(tree.1.root(), &x)?;
                                    acc.insert(tree.0.clone(), v);
                                }
                                Ok(acc)
                            },
                        )
                        .map(move |(key, v)| {
                            let mut vars = v?;
                            for (k, v) in key?.iter() {
                                vars.insert(k.clone(), v.clone());
                            }
                            for (name, _) in trees1 {
                                vars.insert(name.clone(), vars.get(name).unwrap().clone());
                            }
                            Ok(vars)
                        });
                    return Ok(Box::new(aggregator));
                }
                Ok(Box::new(empty()))
            }
            IR::Project(trees) => {
                if let Some(child_idx) = child0_idx {
                    Ok(Box::new(self.run(&child_idx)?.try_map(move |vars| {
                        if trees.is_empty() {
                            return Ok(vars);
                        }
                        let mut return_vars = Env::new();
                        for (name, tree) in trees {
                            let value = self.run_expr(tree.root(), &vars)?;
                            return_vars.insert(name.clone(), value);
                        }
                        Ok(return_vars)
                    })))
                } else {
                    Ok(Box::new(once(()).map(move |()| {
                        let vars = Env::new();
                        let mut return_vars = Env::new();
                        for (name, tree) in trees {
                            let value = self.run_expr(tree.root(), &vars)?;
                            return_vars.insert(name.clone(), value);
                        }
                        Ok(return_vars)
                    })))
                }
            }
            IR::Commit => {
                if !self.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                let iter = self
                    .run(&child0_idx.ok_or("nothing to commit")?)?
                    .collect::<Result<Vec<_>, String>>()?
                    .into_iter()
                    .map(Ok);
                self.commit();
                Ok(Box::new(iter))
            }
        }
    }

    fn relationship_scan(
        &self,
        relationship_pattern: &'a RelationshipPattern,
        vars: Env,
    ) -> Box<dyn Iterator<Item = Result<Env, String>> + '_> {
        let iter = self
            .g
            .borrow()
            .get_relationships(&relationship_pattern.types);
        Box::new(iter.map(move |(src, dst, id)| {
            let mut vars = vars.clone();
            vars.insert(
                relationship_pattern.alias.to_string(),
                Value::Relationship(id, src, dst),
            );
            vars.insert(relationship_pattern.from.to_string(), Value::Node(src));
            vars.insert(relationship_pattern.to.to_string(), Value::Node(dst));
            Ok(vars)
        }))
    }

    fn node_scan(
        &self,
        node_pattern: &'a NodePattern,
        vars: Env,
    ) -> Box<dyn Iterator<Item = Result<Env, String>> + '_> {
        let iter = self.g.borrow().get_nodes(&node_pattern.labels);
        Box::new(iter.map(move |(v, _)| {
            let mut vars = vars.clone();
            vars.insert(node_pattern.alias.to_string(), Value::Node(v));
            Ok(vars)
        }))
    }

    fn delete(
        &self,
        trees: &Vec<orx_tree::Tree<Dyn<ExprIR>>>,
        vars: &Env,
    ) -> Result<(), String> {
        for tree in trees {
            let value = self.run_expr(tree.root(), &vars)?;
            match value {
                Value::Node(id) => {
                    self.stats.borrow_mut().nodes_deleted += 1;
                    self.g.borrow_mut().delete_node(id);
                    // self.pending.borrow_mut().nodes_deleted.push(id);
                }
                _ => {
                    return Err(String::from("Delete operator requires a node"));
                }
            }
        }
        Ok(())
    }

    fn create(
        &self,
        pattern: &Pattern,
        vars: &mut Env,
    ) -> Result<(), String> {
        for node in &pattern.nodes {
            let properties = self.run_expr(node.attrs.root(), vars)?;
            match properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_node();
                    self.pending
                        .borrow_mut()
                        .created_nodes
                        .insert(id, (node.labels.clone(), properties));
                    vars.insert(node.alias.to_string(), Value::Node(id));
                }
                _ => return Err(String::from("Invalid node properties")),
            }
        }
        for rel in &pattern.relationships {
            let (from_id, to_id) = {
                let from_id = vars
                    .get(&rel.from.to_string())
                    .ok_or_else(|| format!("Variable {} not found", rel.from))?;
                let from_id = match from_id {
                    Value::Node(id) => *id,
                    _ => return Err(String::from("Invalid node id")),
                };
                let to_id = vars
                    .get(&rel.to.to_string())
                    .ok_or_else(|| format!("Variable {} not found", rel.to))?;
                let to_id = match to_id {
                    Value::Node(id) => *id,
                    _ => return Err(String::from("Invalid node id")),
                };
                (from_id, to_id)
            };
            let properties = self.run_expr(rel.attrs.root(), vars)?;
            match properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_relationship();
                    self.pending.borrow_mut().created_relationships.insert(
                        id,
                        (
                            rel.types.first().unwrap().clone(),
                            from_id,
                            to_id,
                            properties,
                        ),
                    );
                    vars.insert(
                        rel.alias.to_string(),
                        Value::Relationship(id, from_id, to_id),
                    );
                }
                _ => {
                    return Err(String::from("Invalid relationship properties"));
                }
            }
        }
        Ok(())
    }

    fn commit(&self) {
        if !self.pending.borrow().created_nodes.is_empty() {
            self.stats.borrow_mut().nodes_created += self.pending.borrow().created_nodes.len();
            self.stats.borrow_mut().properties_set += self
                .pending
                .borrow()
                .created_nodes
                .iter()
                .flat_map(|v| v.1.1.values())
                .map(|v| match v {
                    Value::Null => 0,
                    _ => 1,
                })
                .sum::<usize>();
            self.g
                .borrow_mut()
                .create_nodes(&self.pending.borrow().created_nodes);
            self.pending.borrow_mut().created_nodes.clear();
        }
        if !self.pending.borrow().created_relationships.is_empty() {
            self.stats.borrow_mut().relationships_created +=
                self.pending.borrow().created_relationships.len();
            self.stats.borrow_mut().properties_set += self
                .pending
                .borrow()
                .created_relationships
                .iter()
                .flat_map(|v| v.1.3.values())
                .map(|v| match v {
                    Value::Null => 0,
                    _ => 1,
                })
                .sum::<usize>();
            self.g
                .borrow_mut()
                .create_relationships(&self.pending.borrow().created_relationships);
            self.pending.borrow_mut().created_relationships.clear();
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
        ExprIR::String(x) => Value::String(x.clone()),
        ExprIR::List => Value::List(expr.children().map(evaluate_param).collect()),
        ExprIR::Map => Value::Map(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::Var(key) => (key.clone(), evaluate_param(ir.child(0))),
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
        _ => Err(String::from("Invalid array range parameters.")),
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
        Err(String::from("Eq operator requires at least two arguments"))
    }
}

fn all_not_equals<I>(mut iter: I) -> Result<Value, String>
where
    I: Iterator<Item = Result<Value, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(Value::Null),
                Some(Ordering::Less | Ordering::Greater) => {}
                Some(Ordering::Equal) => return Ok(Value::Bool(false)),
            }
        }
        Ok(Value::Bool(true))
    } else {
        Err(String::from("Eq operator requires at least two arguments"))
    }
}

#[inline]
const fn logical_xor(
    a: bool,
    b: bool,
) -> bool {
    (a && !b) || (!a && b)
}
