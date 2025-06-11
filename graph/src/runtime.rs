use crate::ast::{NodePattern, Pattern, QuantifierType, RelationshipPattern, VarId};
use crate::functions::{FnType, Functions, Type, get_functions};
use crate::iter::{Aggregate, LazyReplace, TryFlatMap, TryMap};
use crate::value::{DisjointOrNull, Env, RcValue};
use crate::{ast::ExprIR, graph::Graph, planner::IR, value::Contains, value::Value};
use hashbrown::{HashMap, HashSet};
use once_cell::unsync::Lazy;
use ordermap::OrderMap;
use orx_tree::{Dyn, DynNode, DynTree, NodeIdx, NodeRef};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::iter::{empty, once, repeat_with};
use std::rc::Rc;
use std::time::{Duration, Instant};
use tracing::instrument;

pub trait ReturnCallback {
    fn return_value(
        &self,
        graph: &RefCell<Graph>,
        env: Env,
        return_names: &Vec<VarId>,
    );
}

pub struct ResultSummary<CB: ReturnCallback> {
    pub callback: CB,
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
    pub created_nodes: HashMap<u64, (Vec<Rc<String>>, OrderMap<Rc<String>, RcValue>)>,
    pub created_relationships: HashMap<u64, (Rc<String>, u64, u64, OrderMap<Rc<String>, RcValue>)>,
    pub deleted_nodes: HashSet<u64>,
    pub deleted_relationships: HashSet<(u64, u64, u64)>,
}

pub struct Runtime<'a> {
    functions: &'static Functions,
    parameters: HashMap<String, RcValue>,
    pub g: &'a RefCell<Graph>,
    write: bool,
    pub pending: Lazy<RefCell<Pending>>,
    pub stats: RefCell<Stats>,
    pub plan: Rc<DynTree<IR>>,
}

trait ReturnNames {
    fn get_return_names(&self) -> Vec<VarId>;
}

impl ReturnNames for DynNode<'_, IR> {
    fn get_return_names(&self) -> Vec<VarId> {
        match self.data() {
            IR::Project(trees) => trees.iter().map(|v| v.0.clone()).collect(),
            IR::Commit => self
                .get_child(0)
                .map_or(vec![], |child| child.get_return_names()),
            IR::Call(name, _) => vec![VarId {
                name: Some(name.clone()),
                id: 0,
                ty: Type::Any,
            }],
            IR::Aggregate(names, _, _) => names.clone(),
            _ => vec![],
        }
    }
}

impl Debug for Env {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> Runtime<'a> {
    #[must_use]
    pub fn new(
        g: &'a RefCell<Graph>,
        parameters: HashMap<String, DynTree<ExprIR>>,
        write: bool,
        plan: Rc<DynTree<IR>>,
    ) -> Self {
        Self {
            functions: get_functions(),
            parameters: parameters
                .into_iter()
                .map(|(k, v)| (k, evaluate_param(v.root())))
                .collect(),
            g,
            write,
            pending: Lazy::new(|| RefCell::new(Pending::default())),
            stats: RefCell::new(Stats::default()),
            plan,
        }
    }

    pub fn query<CB: ReturnCallback>(
        &mut self,
        callback: CB,
    ) -> Result<ResultSummary<CB>, String> {
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
            callback,
            run_duration,
            labels_added: self.g.borrow().get_labels_count() - labels_count,
            labels_removed: 0,
            nodes_created: stats.nodes_created,
            relationships_created: stats.relationships_created,
            nodes_deleted: stats.nodes_deleted,
            relationships_deleted: stats.relationships_deleted,
            properties_set: stats.properties_set,
            properties_removed: stats.properties_removed,
            return_names: return_names
                .into_iter()
                .map(|v| String::from(v.name.unwrap().as_str()))
                .collect(),
        })
    }

    #[instrument(name = "run_agg_expr", level = "debug", skip(self, ir), fields(expr_type = ?ir.data()))]
    fn run_agg_expr(
        &self,
        ir: DynNode<ExprIR>,
        curr: &mut Env,
        acc: &mut Env,
    ) -> Result<(), String> {
        match ir.data() {
            ExprIR::FuncInvocation(func) if func.is_aggregate() => {
                let key = match ir.child(ir.num_children() - 1).data() {
                    ExprIR::Var(key) => key.clone(),
                    _ => {
                        return Err(String::from(
                            "Aggregation function must end with a variable",
                        ));
                    }
                };
                curr.insert(&key, acc.get(&key).unwrap_or_else(RcValue::null));
                acc.insert(&key, self.run_expr(ir, curr, false)?);
            }
            _ => {
                for child in ir.children() {
                    self.run_agg_expr(child, curr, acc)?;
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[instrument(name = "run_expr", level = "debug", skip(self), fields(expr_type = ?ir.data()))]
    fn run_expr(
        &self,
        ir: DynNode<ExprIR>,
        env: &Env,
        finalize_agg: bool,
    ) -> Result<RcValue, String> {
        match ir.data() {
            ExprIR::Null => Ok(RcValue::null()),
            ExprIR::Bool(x) => Ok(RcValue::new(Value::Bool(*x))),
            ExprIR::Integer(x) => Ok(RcValue::new(Value::Int(*x))),
            ExprIR::Float(x) => Ok(RcValue::new(Value::Float(*x))),
            ExprIR::String(x) => Ok(RcValue::new(Value::String(x.clone()))),
            ExprIR::Var(x) => env
                .get(x)
                .map_or_else(|| Err(format!("Variable {} not found", x.as_str())), Ok),
            ExprIR::Parameter(x) => self.parameters.get(x).map_or_else(
                || Err(format!("Parameter {x} not found")),
                |v| Ok(v.clone()),
            ),
            ExprIR::List => Ok(RcValue::new(Value::List(
                ir.children()
                    .map(|ir| self.run_expr(ir, env, finalize_agg))
                    .collect::<Result<Vec<_>, _>>()?,
            ))),
            ExprIR::Length => match &*(self.run_expr(ir.child(0), env, finalize_agg)?) {
                Value::List(arr) => Ok(RcValue::new(Value::Int(arr.len() as _))),
                _ => Err(String::from("Length operator requires a list")),
            },
            ExprIR::GetElement => {
                let arr = self.run_expr(ir.child(0), env, finalize_agg)?;
                let i = self.run_expr(ir.child(1), env, finalize_agg)?;
                match (&*arr, &*i) {
                    (Value::List(values), Value::Int(i)) => {
                        if *i >= 0 && *i < values.len() as _ {
                            Ok(values[*i as usize].clone())
                        } else {
                            Ok(RcValue::null())
                        }
                    }
                    (Value::List(_), v) => {
                        Err(format!("Type mismatch: expected Bool but was {v:?}"))
                    }
                    (Value::Map(map), Value::String(key)) => map
                        .get(key)
                        .map_or_else(|| Ok(RcValue::null()), |v| Ok(v.clone())),
                    (Value::Map(_), Value::Null) | (Value::Null, _) => Ok(RcValue::null()),
                    v => Err(format!("Type mismatch: expected List but was {v:?}")),
                }
            }
            ExprIR::GetElements => {
                let arr = self.run_expr(ir.child(0), env, finalize_agg)?;
                let a = self.run_expr(ir.child(1), env, finalize_agg)?;
                let b = self.run_expr(ir.child(2), env, finalize_agg)?;
                get_elements(arr, a, b)
            }
            ExprIR::IsNode => match *self.run_expr(ir.child(0), env, finalize_agg)? {
                Value::Node(_) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::IsRelationship => match *self.run_expr(ir.child(0), env, finalize_agg)? {
                Value::Relationship(_, _, _) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::Or => {
                let mut is_null = false;
                for ir in ir.children() {
                    match &*self.run_expr(ir, env, finalize_agg)? {
                        Value::Bool(true) => return Ok(RcValue::bool(true)),
                        Value::Bool(false) => {}
                        Value::Null => is_null = true,
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                if is_null {
                    return Ok(RcValue::null());
                }

                Ok(RcValue::bool(false))
            }
            ExprIR::Xor => {
                let mut last = None;
                for ir in ir.children() {
                    match &*self.run_expr(ir, env, finalize_agg)? {
                        Value::Bool(b) => last = Some(last.map_or(*b, |l| logical_xor(l, *b))),
                        Value::Null => return Ok(RcValue::null()),
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                Ok(RcValue::bool(last.unwrap_or(false)))
            }
            ExprIR::And => {
                let mut is_null = false;
                for ir in ir.children() {
                    match &*self.run_expr(ir, env, finalize_agg)? {
                        Value::Bool(false) => return Ok(RcValue::bool(false)),
                        Value::Bool(true) => {}
                        Value::Null => is_null = true,
                        ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                    }
                }
                if is_null {
                    return Ok(RcValue::null());
                }

                Ok(RcValue::bool(true))
            }
            ExprIR::Not => match &*self.run_expr(ir.child(0), env, finalize_agg)? {
                Value::Bool(b) => Ok(RcValue::bool(!b)),
                Value::Null => Ok(RcValue::null()),
                v => Err(format!(
                    "Type mismatch: expected Boolean or Null but was {}",
                    v.name()
                )),
            },
            ExprIR::Negate => match &*self.run_expr(ir.child(0), env, finalize_agg)? {
                Value::Int(i) => Ok(RcValue::int(-i)),
                Value::Float(f) => Ok(RcValue::float(-f)),
                Value::Null => Ok(RcValue::null()),
                v => Err(format!(
                    "Type mismatch: expected Integer, Float, or Null but was {}",
                    v.name()
                )),
            },
            ExprIR::Eq => all_equals(ir.children().map(|ir| self.run_expr(ir, env, finalize_agg))),
            ExprIR::Neq => {
                all_not_equals(ir.children().map(|ir| self.run_expr(ir, env, finalize_agg)))
            }
            ExprIR::Lt => match self
                .run_expr(ir.child(0), env, finalize_agg)?
                .compare_value(&*(self.run_expr(ir.child(1), env, finalize_agg)?))
            {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => Ok(RcValue::null()),
                (_, DisjointOrNull::NaN) => Ok(RcValue::bool(false)),
                (Ordering::Less, _) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::Gt => match self
                .run_expr(ir.child(0), env, finalize_agg)?
                .compare_value(&*(self.run_expr(ir.child(1), env, finalize_agg)?))
            {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => Ok(RcValue::null()),
                (_, DisjointOrNull::NaN) => Ok(RcValue::bool(false)),
                (Ordering::Greater, _) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::Le => match self
                .run_expr(ir.child(0), env, finalize_agg)?
                .compare_value(&*(self.run_expr(ir.child(1), env, finalize_agg)?))
            {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => Ok(RcValue::null()),
                (_, DisjointOrNull::NaN) => Ok(RcValue::bool(false)),
                (Ordering::Less | Ordering::Equal, _) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::Ge => match self
                .run_expr(ir.child(0), env, finalize_agg)?
                .compare_value(&*(self.run_expr(ir.child(1), env, finalize_agg)?))
            {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => Ok(RcValue::null()),
                (_, DisjointOrNull::NaN) => Ok(RcValue::bool(false)),
                (Ordering::Greater | Ordering::Equal, _) => Ok(RcValue::bool(true)),
                _ => Ok(RcValue::bool(false)),
            },
            ExprIR::In => {
                let value = self.run_expr(ir.child(0), env, finalize_agg)?;
                let list = self.run_expr(ir.child(1), env, finalize_agg)?;
                list_contains(list, value)
            }
            ExprIR::Add => ir
                .children()
                .map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|acc, value| acc? + value?)
                .ok_or_else(|| String::from("Add operator requires at least one operand"))?,
            ExprIR::Sub => ir
                .children()
                .map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|acc, value| acc? - value?)
                .ok_or_else(|| String::from("Sub operator requires at least one argument"))?,
            ExprIR::Mul => ir
                .children()
                .map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|acc, value| acc? * value?)
                .ok_or_else(|| String::from("Mul operator requires at least one argument"))?,
            ExprIR::Div => ir
                .children()
                .map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|acc, value| acc? / value?)
                .ok_or_else(|| String::from("Div operator requires at least one argument"))?,
            ExprIR::Modulo => ir
                .children()
                .map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|acc, value| acc? % value?)
                .ok_or_else(|| String::from("Modulo operator requires at least one argument"))?,
            ExprIR::Pow => ir
                .children()
                .flat_map(|ir| self.run_expr(ir, env, finalize_agg))
                .reduce(|a, b| match (&*a, &*b) {
                    (Value::Int(a), Value::Int(b)) => RcValue::float((*a as f64).powf(*b as _)),
                    _ => RcValue::null(),
                })
                .ok_or_else(|| String::from("Pow operator requires at least one argument")),
            ExprIR::FuncInvocation(func) => {
                if finalize_agg {
                    if let FnType::Aggregation(aggregation_return_type) = &func.fn_type {
                        match ir.child(ir.num_children() - 1).data() {
                            ExprIR::Var(key) => {
                                return Ok(env
                                    .get(key)
                                    .unwrap_or_else(|| aggregation_return_type.clone()));
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                let args = ir
                    .children()
                    .map(|ir| self.run_expr(ir, env, finalize_agg))
                    .collect::<Result<Vec<_>, _>>()?;

                func.validate_args_type(&args)?;
                if !self.write && func.write {
                    return Err(String::from(
                        "graph.RO_QUERY is to be executed only on read-only queries",
                    ));
                }
                (func.func)(self, args)
            }
            ExprIR::Map => Ok(RcValue::map(
                ir.children()
                    .map(|child| {
                        Ok((
                            if let ExprIR::String(key) = child.data() {
                                key.clone()
                            } else {
                                todo!();
                            },
                            self.run_expr(child.child(0), env, finalize_agg)?,
                        ))
                    })
                    .collect::<Result<_, String>>()?,
            )),
            ExprIR::Quantifier(quantifier, var) => {
                let list = self.run_expr(ir.child(0), env, finalize_agg)?;
                match &*list {
                    Value::List(values) => {
                        let mut env = env.clone();
                        let mut t = 0;
                        let mut f = 0;
                        let mut n = 0;
                        for value in values {
                            env.insert(var, value.clone());

                            match &*self.run_expr(ir.child(1), &env, finalize_agg)? {
                                Value::Bool(true) => t += 1,
                                Value::Bool(false) => f += 1,
                                Value::Null => n += 1,
                                value => {
                                    return Err(format!(
                                        "Type mismatch: expected Boolean but was {}",
                                        value.name()
                                    ));
                                }
                            }
                        }

                        Ok(self.eval_quantifier(quantifier, t, f, n))
                    }
                    value => Err(format!(
                        "Type mismatch: expected List but was {}",
                        value.name()
                    )),
                }
            }
            ExprIR::ListComprehension(var) => {
                let iter = self.run_iter_expr(ir.child(0), env)?;
                let mut env = env.clone();
                let mut acc = vec![];
                for value in iter {
                    env.insert(var, value.clone());
                    match *self.run_expr(ir.child(1), &env, finalize_agg)? {
                        Value::Bool(true) => {}
                        _ => continue,
                    }
                    acc.push(self.run_expr(ir.child(2), &env, finalize_agg)?);
                }

                Ok(RcValue::list(acc))
            }
        }
    }

    #[instrument(name = "run_iter_expr", level = "debug", skip(self), fields(expr_type = ?ir.data()))]
    fn run_iter_expr(
        &self,
        ir: DynNode<ExprIR>,
        env: &Env,
    ) -> Result<Box<dyn Iterator<Item = RcValue>>, String> {
        match ir.data() {
            ExprIR::FuncInvocation(func) if func.name == "range" => {
                let start = self.run_expr(ir.child(0), env, false);
                let stop = self.run_expr(ir.child(1), env, false);
                let step = ir
                    .get_child(2)
                    .map_or_else(|| Ok(RcValue::int(1)), |c| self.run_expr(c, env, false));
                match (start.as_deref(), stop.as_deref(), step.as_deref()) {
                    (Ok(Value::Int(start)), Ok(Value::Int(stop)), Ok(Value::Int(step))) => {
                        let mut curr = *start;
                        let step = *step;
                        return Ok(Box::new(
                            repeat_with(move || {
                                let tmp = curr;
                                curr += step;
                                RcValue::int(tmp)
                            })
                            .take(((stop - start) / step + 1) as usize),
                        ));
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            _ => {
                let res = self.run_expr(ir, env, false)?;
                match &*res {
                    Value::List(arr) => Ok(Box::new(arr.clone().into_iter())),
                    _ => Err(String::from("Expr must return a list")),
                }
            }
        }
    }

    fn eval_quantifier(
        &self,
        quantifier_type: &QuantifierType,
        true_count: usize,
        false_count: usize,
        null_count: usize,
    ) -> RcValue {
        match quantifier_type {
            QuantifierType::All => {
                if false_count > 0 {
                    RcValue::bool(false)
                } else if null_count > 0 {
                    RcValue::null()
                } else {
                    RcValue::bool(true)
                }
            }
            QuantifierType::Any => {
                if true_count > 0 {
                    RcValue::bool(true)
                } else if null_count > 0 {
                    RcValue::null()
                } else {
                    RcValue::bool(false)
                }
            }
            QuantifierType::None => {
                if true_count > 0 {
                    RcValue::bool(false)
                } else if null_count > 0 {
                    RcValue::null()
                } else {
                    RcValue::bool(true)
                }
            }
            QuantifierType::Single => {
                if true_count == 1 && null_count == 0 {
                    RcValue::bool(true)
                } else if true_count > 1 {
                    RcValue::bool(false)
                } else if null_count > 0 {
                    RcValue::null()
                } else {
                    RcValue::bool(false)
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    #[instrument(name = "run", level = "debug", skip(self, idx))]
    fn run(
        &self,
        idx: &NodeIdx<Dyn<IR>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + '_>, String> {
        let child0_idx = self.plan.node(idx).get_child(0).map(|n| n.idx());
        let child1_idx = self.plan.node(idx).get_child(1).map(|n| n.idx());
        match self.plan.node(idx).data() {
            IR::Empty => Ok(Box::new(empty())),
            IR::Optional(vars) => {
                if let Some(child_idx) = child1_idx {
                    let iter = self.run(&child_idx)?.try_flat_map(move |mut env| {
                        for v in vars {
                            env.insert(v, RcValue::null());
                        }
                        self.run(child0_idx.as_ref().unwrap())
                            .unwrap()
                            .lazy_replace(move || Box::new(once(Ok(env))))
                    });
                    return Ok(Box::new(iter));
                }
                if let Some(child_idx) = child0_idx {
                    let iter = self.run(&child_idx)?.lazy_replace(move || {
                        let mut env = Env::default();
                        for v in vars {
                            env.insert(v, RcValue::null());
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
                        .map(|ir| self.run_expr(ir.root(), &Env::default(), false))
                        .collect::<Result<Vec<_>, _>>()?;
                    if !self.write && func.write {
                        return Err(String::from(
                            "graph.RO_QUERY is to be executed only on read-only queries",
                        ));
                    }
                    let res = (func.func)(self, args)?;
                    match &*res {
                        Value::List(arr) => Ok(Box::new(arr.clone().into_iter().map(|v| {
                            let mut env = Env::default();
                            env.insert(
                                &VarId {
                                    name: Some(name.clone()),
                                    id: 0,
                                    ty: Type::Any,
                                },
                                v,
                            );
                            Ok(env)
                        }))),
                        _ => Err(format!("Function '{name}' must return a list")),
                    }
                }
                None => Err(format!("Function '{name}' not found")),
            },
            IR::Unwind(tree, name) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(move |vars| {
                        let value = self.run_iter_expr(tree.root(), &vars);
                        match value {
                            Ok(iter) => iter
                                .map(|v| {
                                    let mut vars = vars.clone();
                                    vars.insert(name, v);
                                    Ok(vars)
                                })
                                .collect::<Vec<_>>()
                                .into_iter(),
                            Err(e) => vec![Err(e)].into_iter(),
                        }
                    })));
                }
                let vars = Env::default();
                let value = self.run_iter_expr(tree.root(), &vars)?;
                return Ok(Box::new(value.map(move |v| {
                    let mut vars = Env::default();
                    vars.insert(name, v);
                    Ok(vars)
                })));
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
                    )));
                }
                let mut vars = Env::default();
                self.create(pattern, &mut vars)?;
                if parent_commit {
                    return Ok(Box::new(empty()));
                }
                Ok(Box::new(once(Ok(vars))))
            }
            IR::Merge(pattern) => {
                if let Some(child_idx) = child1_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(move |vars| {
                        let cvars = vars.clone();
                        let iter =
                            (Box::new(self.run(child0_idx.as_ref().unwrap()).unwrap().try_map(
                                move |v| {
                                    let mut vars = vars.clone();
                                    vars.merge(v);
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
                    })));
                }
                let iter = self.run(child0_idx.as_ref().unwrap())?.lazy_replace(|| {
                    let mut vars = Env::default();
                    match self.create(pattern, &mut vars) {
                        Ok(()) => Box::new(vec![Ok(vars)].into_iter()),
                        Err(e) => Box::new(once(Err(e))),
                    }
                });
                Ok(Box::new(iter))
            }
            IR::Delete(trees, _) => {
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
                    return Ok(Box::new(
                        self.run(&child_idx)?
                            .try_flat_map(move |vars| self.node_scan(node_pattern, vars)),
                    ));
                }
                Ok(self.node_scan(node_pattern, Env::default()))
            }
            IR::RelationshipScan(relationship_pattern) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(move |vars| {
                        self.relationship_scan(relationship_pattern, vars)
                    })));
                }
                Ok(self.relationship_scan(relationship_pattern, Env::default()))
            }
            IR::ExpandInto(relationship_pattern) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_flat_map(move |vars| {
                        self.expand_into(relationship_pattern, vars)
                    })));
                }
                Ok(self.expand_into(relationship_pattern, Env::default()))
            }
            IR::PathBuilder(paths) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.try_map(move |mut vars| {
                        let mut paths = paths.clone();
                        for path in &mut paths {
                            let p = path
                                .vars
                                .iter()
                                .map(|v| {
                                    vars.get(v).map_or_else(
                                        || Err(format!("Variable {} not found", v.as_str())),
                                        Ok,
                                    )
                                })
                                .collect::<Result<_, String>>()?;
                            vars.insert(&path.var, RcValue::path(p));
                        }
                        Ok(vars)
                    })));
                }
                Err(String::from("PathBuilder operator requires a child node"))
            }
            IR::Filter(tree) => {
                if let Some(child_idx) = child0_idx {
                    return Ok(Box::new(self.run(&child_idx)?.filter(move |vars| {
                        let vars = vars.clone().unwrap();
                        self.run_expr(tree.root(), &vars, false) == Ok(RcValue::bool(true))
                    })));
                }
                Err(String::from(
                    "Filter operator requires a boolean expression",
                ))
            }
            IR::Aggregate(_, keys, agg) => {
                let mut cache = std::collections::HashMap::new();
                let mut env = Env::default();
                for (_, t) in agg {
                    if let ExprIR::FuncInvocation(func) = t.root().data() {
                        if let FnType::Aggregation(aggregation_return_type) = &func.fn_type {
                            let ExprIR::Var(key) =
                                t.root().child(t.root().num_children() - 1).data()
                            else {
                                return Err(String::from(
                                    "Aggregation function must end with a variable",
                                ));
                            };
                            env.insert(key, aggregation_return_type.clone());
                        }
                    }
                }
                // in case there are no aggregation keys the aggregator will return
                // default value for empty iterator
                if keys.is_empty() {
                    let key = Ok(Env::default());
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    let k = hasher.finish();
                    cache.insert(k, (key, Ok(env.clone())));
                }
                if let Some(child_idx) = child0_idx {
                    let aggregator = self
                        .run(&child_idx)?
                        .aggregate(
                            move |vars| {
                                let vars = vars.clone()?;
                                let mut return_vars = Env::default();
                                for (name, tree) in keys {
                                    let value = self.run_expr(tree.root(), &vars, false)?;
                                    return_vars.insert(name, value);
                                }
                                Ok::<Env, String>(return_vars)
                            },
                            Ok(env),
                            move |x, acc| {
                                let mut x = x?;
                                let mut acc: Env = acc?;
                                for (_, tree) in agg {
                                    self.run_agg_expr(tree.root(), &mut x, &mut acc)?;
                                }
                                Ok(acc)
                            },
                            cache,
                        )
                        .map(move |(key, v)| {
                            let mut vars = v?;
                            vars.merge(key?);
                            for (name, tree) in agg {
                                vars.insert(name, self.run_expr(tree.root(), &vars, true)?);
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
                        let mut return_vars = Env::default();
                        for (name, tree) in trees {
                            let value = self.run_expr(tree.root(), &vars, false)?;
                            return_vars.insert(name, value);
                        }
                        Ok(return_vars)
                    })))
                } else {
                    Ok(Box::new(once(()).map(move |()| {
                        let vars = Env::default();
                        let mut return_vars = Env::default();
                        for (name, tree) in trees {
                            let value = self.run_expr(tree.root(), &vars, false)?;
                            return_vars.insert(name, value);
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
        let attrs = self
            .run_expr(relationship_pattern.attrs.root(), &vars, false)
            .unwrap();
        let iter = self.g.borrow().get_relationships(
            &relationship_pattern.types,
            &relationship_pattern.from.labels,
            &relationship_pattern.to.labels,
        );
        Box::new(iter.flat_map(move |(src, dst)| {
            let vars = vars.clone();
            let attrs = attrs.clone();
            self.g
                .borrow()
                .get_src_dest_relationships(src, dst, &relationship_pattern.types)
                .into_iter()
                .filter(move |v| {
                    if let Value::Map(attrs) = &*attrs {
                        if !attrs.is_empty() {
                            let g = self.g.borrow();
                            let properties = g.get_relationship_properties(*v);
                            for (key, avalue) in attrs {
                                if let Some(pvalue) =
                                    properties.get(&g.get_relationship_property_id(key).unwrap())
                                {
                                    if avalue == pvalue {
                                        continue;
                                    }
                                    return false;
                                }
                                return false;
                            }
                        }
                    }
                    true
                })
                .map(move |id| {
                    let mut vars = vars.clone();
                    vars.insert(
                        &relationship_pattern.alias,
                        RcValue::relationship(id, src, dst),
                    );
                    vars.insert(&relationship_pattern.from.alias, RcValue::node(src));
                    vars.insert(&relationship_pattern.to.alias, RcValue::node(dst));
                    Ok(vars)
                })
                .collect::<Vec<_>>()
        }))
    }

    fn expand_into(
        &self,
        relationship_pattern: &'a RelationshipPattern,
        vars: Env,
    ) -> Box<dyn Iterator<Item = Result<Env, String>> + '_> {
        let src = vars
            .get(&relationship_pattern.from.alias)
            .and_then(|v| match *v {
                Value::Node(id) => Some(id),
                _ => None,
            })
            .unwrap();
        let dst = vars
            .get(&relationship_pattern.to.alias)
            .and_then(|v| match *v {
                Value::Node(id) => Some(id),
                _ => None,
            })
            .unwrap();
        Box::new(
            self.g
                .borrow()
                .get_src_dest_relationships(src, dst, &relationship_pattern.types)
                .iter()
                .map(move |id| {
                    let mut vars = vars.clone();
                    vars.insert(
                        &relationship_pattern.alias,
                        RcValue::relationship(*id, src, dst),
                    );
                    vars.insert(&relationship_pattern.from.alias, RcValue::node(src));
                    vars.insert(&relationship_pattern.to.alias, RcValue::node(dst));
                    Ok(vars)
                })
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    fn node_scan(
        &self,
        node_pattern: &'a NodePattern,
        vars: Env,
    ) -> Box<dyn Iterator<Item = Result<Env, String>> + '_> {
        let attrs = self
            .run_expr(node_pattern.attrs.root(), &vars, false)
            .unwrap();
        let iter = self.g.borrow().get_nodes(&node_pattern.labels);
        Box::new(iter.filter_map(move |(v, _)| {
            let mut vars = vars.clone();
            if let Value::Map(attrs) = &*attrs {
                if !attrs.is_empty() {
                    let g = self.g.borrow();
                    let properties = g.get_node_properties(v);
                    for (key, avalue) in attrs {
                        if let Some(pvalue) = properties.get(&g.get_node_property_id(key).unwrap())
                        {
                            if avalue == pvalue {
                                continue;
                            }
                            return None;
                        }
                        return None;
                    }
                }
            }
            vars.insert(&node_pattern.alias, RcValue::node(v));
            Some(Ok(vars))
        }))
    }

    fn delete(
        &self,
        trees: &Vec<orx_tree::Tree<Dyn<ExprIR>>>,
        vars: &Env,
    ) -> Result<(), String> {
        for tree in trees {
            let value = self.run_expr(tree.root(), vars, false)?;
            if let Some(value) = self.delete_entity(value) {
                return value;
            }
        }
        Ok(())
    }

    fn delete_entity(
        &self,
        value: RcValue,
    ) -> Option<Result<(), String>> {
        match &*value {
            Value::Node(id) => {
                for (src, dest, id) in self.g.borrow().get_node_relationships(*id) {
                    self.pending
                        .borrow_mut()
                        .deleted_relationships
                        .insert((id, src, dest));
                }
                self.pending.borrow_mut().deleted_nodes.insert(*id);
            }
            Value::Relationship(id, src, dest) => {
                self.pending
                    .borrow_mut()
                    .deleted_relationships
                    .insert((*id, *src, *dest));
            }
            Value::Path(values) => {
                for value in values {
                    let _ = self.delete_entity(value.clone())?;
                }
            }
            Value::Null => {}
            _ => {
                return Some(Err(String::from("Delete operator requires a node")));
            }
        }
        None
    }

    fn create(
        &self,
        pattern: &Pattern,
        vars: &mut Env,
    ) -> Result<(), String> {
        for node in &pattern.nodes {
            let properties = self.run_expr(node.attrs.root(), vars, false)?;
            match &*properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_node();
                    self.pending
                        .borrow_mut()
                        .created_nodes
                        .insert(id, (node.labels.clone(), properties.clone()));
                    vars.insert(&node.alias, RcValue::node(id));
                }
                _ => return Err(String::from("Invalid node properties")),
            }
        }
        for rel in &pattern.relationships {
            let (from_id, to_id) = {
                let Value::Node(from_id) = *vars
                    .get(&rel.from.alias)
                    .ok_or_else(|| format!("Variable {} not found", rel.from.alias.as_str()))?
                else {
                    return Err(String::from("Invalid node id"));
                };
                let Value::Node(to_id) = *vars
                    .get(&rel.to.alias)
                    .ok_or_else(|| format!("Variable {} not found", rel.to.alias.as_str()))?
                else {
                    return Err(String::from("Invalid node id"));
                };
                (from_id, to_id)
            };
            let properties = self.run_expr(rel.attrs.root(), vars, false)?;
            match &*properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_relationship();
                    self.pending.borrow_mut().created_relationships.insert(
                        id,
                        (
                            rel.types.first().unwrap().clone(),
                            from_id,
                            to_id,
                            properties.clone(),
                        ),
                    );
                    vars.insert(&rel.alias, RcValue::relationship(id, from_id, to_id));
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
                .map(|v| match **v {
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
                .map(|v| match **v {
                    Value::Null => 0,
                    _ => 1,
                })
                .sum::<usize>();
            self.g
                .borrow_mut()
                .create_relationships(&self.pending.borrow().created_relationships);
            self.pending.borrow_mut().created_relationships.clear();
        }
        if !self.pending.borrow().deleted_relationships.is_empty() {
            self.stats.borrow_mut().relationships_deleted +=
                self.pending.borrow().deleted_relationships.len();
            for (id, src, dest) in self.pending.borrow().deleted_relationships.clone() {
                self.g.borrow_mut().delete_relationship(id, src, dest);
            }
            self.pending.borrow_mut().deleted_relationships.clear();
        }
        if !self.pending.borrow().deleted_nodes.is_empty() {
            self.stats.borrow_mut().nodes_deleted += self.pending.borrow().deleted_nodes.len();
            for id in self.pending.borrow().deleted_nodes.clone() {
                self.g.borrow_mut().delete_node(id);
            }
            self.pending.borrow_mut().deleted_nodes.clear();
        }
    }
}

#[must_use]
fn evaluate_param(expr: DynNode<ExprIR>) -> RcValue {
    match expr.data() {
        ExprIR::Null => RcValue::null(),
        ExprIR::Bool(x) => RcValue::bool(*x),
        ExprIR::Integer(x) => RcValue::int(*x),
        ExprIR::Float(x) => RcValue::float(*x),
        ExprIR::String(x) => RcValue::string(x.clone()),
        ExprIR::List => RcValue::list(expr.children().map(evaluate_param).collect()),
        ExprIR::Map => RcValue::map(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::String(key) => (key.clone(), evaluate_param(ir.child(0))),
                    _ => todo!(),
                })
                .collect(),
        ),
        ExprIR::Negate => {
            let v = evaluate_param(expr.child(0));
            match *v {
                Value::Int(i) => RcValue::int(-i),
                Value::Float(f) => RcValue::float(-f),
                _ => RcValue::null(),
            }
        }
        _ => todo!(),
    }
}

fn get_elements(
    arr: RcValue,
    start: RcValue,
    end: RcValue,
) -> Result<RcValue, String> {
    match (&*arr, &*start, &*end) {
        (Value::List(values), Value::Int(start), Value::Int(end)) => {
            let mut start = *start;
            let mut end = *end;
            if start < 0 {
                start = (values.len() as i64 + start).max(0);
            }
            if end < 0 {
                end = (values.len() as i64 + end).max(0);
            } else {
                end = end.min(values.len() as i64);
            }
            if start > end {
                return Ok(RcValue::list(vec![]));
            }
            Ok(RcValue::list(values[start as usize..end as usize].to_vec()))
        }
        (_, Value::Null, _) | (_, _, Value::Null) => Ok(RcValue::null()),
        _ => Err(String::from("Invalid array range parameters.")),
    }
}

fn list_contains(
    list: RcValue,
    value: RcValue,
) -> Result<RcValue, String> {
    match &*list {
        Value::List(l) => Ok(Contains::contains(l, value)),
        Value::Null => Ok(RcValue::null()),
        _ => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            list.name()
        )),
    }
}

// the semantic of Eq [1, 2, 3] is: 1 EQ 2 AND 2 EQ 3
fn all_equals<I>(mut iter: I) -> Result<RcValue, String>
where
    I: Iterator<Item = Result<RcValue, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(RcValue::null()),
                Some(Ordering::Less | Ordering::Greater) => return Ok(RcValue::bool(false)),
                Some(Ordering::Equal) => {}
            }
        }
        Ok(RcValue::bool(true))
    } else {
        Err(String::from("Eq operator requires at least two arguments"))
    }
}

fn all_not_equals<I>(mut iter: I) -> Result<RcValue, String>
where
    I: Iterator<Item = Result<RcValue, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(RcValue::null()),
                Some(Ordering::Less | Ordering::Greater) => {}
                Some(Ordering::Equal) => return Ok(RcValue::bool(false)),
            }
        }
        Ok(RcValue::bool(true))
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
