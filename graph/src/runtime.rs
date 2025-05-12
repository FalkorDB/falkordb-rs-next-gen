use crate::ast::Pattern;
use crate::functions::{FnType, Functions, GraphFn, get_functions};
use crate::iter::AggregateIter;
use crate::{ast::ExprIR, graph::Graph, planner::IR, value::Contains, value::Value};
use orx_tree::{Dyn, DynNode, DynTree, NodeIdx, NodeRef};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::iter::{empty, once};
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
    pub labels_removed: usize,
    pub nodes_created: usize,
    pub relationships_created: usize,
    pub nodes_deleted: usize,
    pub relationships_deleted: usize,
    pub properties_set: usize,
    pub properties_removed: usize,
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
    pub created_nodes: BTreeMap<u64, (Vec<String>, BTreeMap<String, Value>)>,
    pub created_relationships: BTreeMap<u64, (String, u64, u64, BTreeMap<String, Value>)>,
}

pub struct Runtime<'a> {
    functions: &'static Functions,
    parameters: BTreeMap<String, Value>,
    vars: RefCell<BTreeMap<String, Value>>,
    pub g: &'a RefCell<Graph>,
    write: bool,
    pub pending: RefCell<Pending>,
    pub stats: RefCell<Stats>,
    pub plan: DynTree<IR>,
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
            vars: RefCell::new(BTreeMap::new()),
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
        for v in self.run(&idx)? {
            callback.return_value(self.g, v?);
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
        })
    }

    #[allow(clippy::too_many_lines)]
    fn run_expr(
        &self,
        ir: DynNode<ExprIR>,
    ) -> Result<Value, String> {
        match ir.data() {
            ExprIR::Null => Ok(Value::Null),
            ExprIR::Bool(x) => Ok(Value::Bool(*x)),
            ExprIR::Integer(x) => Ok(Value::Int(*x)),
            ExprIR::Float(x) => Ok(Value::Float(*x)),
            ExprIR::String(x) => Ok(Value::String(x.clone())),
            ExprIR::Var(x) => self.vars.borrow().get(x).map_or_else(
                || Err(format!("Variable {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::Parameter(x) => self.parameters.get(x).map_or_else(
                || Err(format!("Parameter {x} not found")),
                |v| Ok(v.to_owned()),
            ),
            ExprIR::List => Ok(Value::List(
                ir.children()
                    .map(|ir| self.run_expr(ir))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            ExprIR::Length => match self.run_expr(ir.child(0))? {
                Value::List(arr) => Ok(Value::Int(arr.len() as _)),
                _ => Err(String::from("Length operator requires a list")),
            },
            ExprIR::GetElement => {
                let arr = self.run_expr(ir.child(0))?;
                let i = self.run_expr(ir.child(1))?;
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
                let arr = self.run_expr(ir.child(0))?;
                let a = self.run_expr(ir.child(1))?;
                let b = self.run_expr(ir.child(2))?;
                get_elements(arr, a, b)
            }
            ExprIR::IsNull => match self.run_expr(ir.child(0))? {
                Value::Null => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsNode => match self.run_expr(ir.child(0))? {
                Value::Node(_) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::IsRelationship => match self.run_expr(ir.child(0))? {
                Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            ExprIR::Or => {
                let mut is_null = false;
                for ir in ir.children() {
                    match self.run_expr(ir)? {
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
                    match self.run_expr(ir)? {
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
                    match self.run_expr(ir)? {
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
            ExprIR::Not => match self.run_expr(ir.child(0))? {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                Value::Null => Ok(Value::Null),
                _ => Err(String::from(
                    "InvalidArgumentType: Not operator requires a boolean or null",
                )),
            },
            ExprIR::Negate => match self.run_expr(ir.child(0))? {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                Value::Null => Ok(Value::Null),
                _ => Err(String::from(
                    "InvalidArgumentType: Negate operator requires an Integer or Float",
                )),
            },
            ExprIR::Eq => all_equals(ir.children().map(|ir| self.run_expr(ir))),
            ExprIR::Neq => ir
                .children()
                .flat_map(|ir| self.run_expr(ir))
                .reduce(|a, b| Value::Bool(a != b))
                .ok_or_else(|| String::from("Neq operator requires at least one argument")),
            ExprIR::Lt => match (self.run_expr(ir.child(0))?, self.run_expr(ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                _ => Err(String::from("Lt operator requires two integers")),
            },
            ExprIR::Gt => match (self.run_expr(ir.child(0))?, self.run_expr(ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                _ => Err(String::from("Gt operator requires two integers")),
            },
            ExprIR::Le => match (self.run_expr(ir.child(0))?, self.run_expr(ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(String::from("Le operator requires two integers")),
            },
            ExprIR::Ge => match (self.run_expr(ir.child(0))?, self.run_expr(ir.child(1))?) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(String::from("Ge operator requires two integers")),
            },
            ExprIR::In => {
                let value = self.run_expr(ir.child(0))?;
                let list = self.run_expr(ir.child(1))?;
                list_contains(&list, &value)
            }
            ExprIR::Add => ir
                .children()
                .map(|ir| self.run_expr(ir))
                .reduce(|acc, value| acc? + value?)
                .ok_or_else(|| String::from("Add operator requires at least one operand"))?,
            ExprIR::Sub => ir
                .children()
                .map(|ir| self.run_expr(ir))
                .reduce(|acc, value| acc? - value?)
                .ok_or_else(|| String::from("Sub operator requires at least one argument"))?,
            ExprIR::Mul => ir
                .children()
                .map(|ir| self.run_expr(ir))
                .reduce(|acc, value| acc? * value?)
                .ok_or_else(|| String::from("Mul operator requires at least one argument"))?,
            ExprIR::Div => ir
                .children()
                .map(|ir| self.run_expr(ir))
                .reduce(|acc, value| acc? / value?)
                .ok_or_else(|| String::from("Div operator requires at least one argument"))?,
            ExprIR::Modulo => ir
                .children()
                .map(|ir| self.run_expr(ir))
                .reduce(|acc, value| acc? % value?)
                .ok_or_else(|| String::from("Modulo operator requires at least one argument"))?,
            ExprIR::Pow => ir
                .children()
                .flat_map(|ir| self.run_expr(ir))
                .reduce(|a, b| match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                    _ => Value::Null,
                })
                .ok_or_else(|| String::from("Pow operator requires at least one argument")),
            ExprIR::FuncInvocation(name, fn_type) => {
                let args = ir
                    .children()
                    .map(|ir| self.run_expr(ir))
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
                        (
                            if let ExprIR::Var(key) = child.data() {
                                key.clone()
                            } else {
                                todo!();
                            },
                            self.run_expr(child.child(0)).unwrap_or(Value::Null),
                        )
                    })
                    .collect(),
            )),
            ExprIR::Set(x) => {
                let v = self.run_expr(ir.child(0))?;
                self.vars.borrow_mut().insert(x.clone(), v.clone());
                Ok(v)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run(
        &self,
        idx: &NodeIdx<Dyn<IR>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Value, String>> + '_>, String> {
        let child_idx = self.plan.node(idx).get_child(0).map(|n| n.idx());
        match self.plan.node(idx).data() {
            IR::Empty => Ok(Box::new(empty())),
            IR::Call(name, trees) => match self.functions.get(name, &FnType::Procedure) {
                Some(func) => {
                    let args = trees
                        .iter()
                        .map(|ir| self.run_expr(ir.root()))
                        .collect::<Result<Vec<_>, _>>()?;
                    if !self.write && func.write {
                        return Err(String::from(
                            "graph.RO_QUERY is to be executed only on read-only queries",
                        ));
                    }
                    let res = (func.func)(self, args)?;
                    match res {
                        Value::List(arr) => {
                            Ok(Box::new(arr.into_iter().map(|v| Ok(Value::List(vec![v])))))
                        }
                        _ => Err(format!("Function '{name}' must return a list")),
                    }
                }
                None => Err(format!("Function '{name}' not found")),
            },
            IR::Unwind(tree, name) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(self.run(&child_idx)?.flat_map(move |_| {
                        let value = self.run_expr(tree.root());
                        let arr = match value {
                            Ok(Value::List(arr)) => arr.into_iter().map(Ok).collect(),
                            Ok(_) => {
                                vec![Err(String::from("Unwind operator requires a list"))]
                            }
                            Err(e) => {
                                vec![Err(e)]
                            }
                        };
                        arr.into_iter().map(move |v| match v {
                            Ok(v) => {
                                self.vars.borrow_mut().insert(name.clone(), v.clone());
                                Ok(v)
                            }
                            Err(e) => Err(e),
                        })
                    })));
                }
                let value = self.run_expr(tree.root())?;
                if let Value::List(arr) = value {
                    return Ok(Box::new(arr.into_iter().map(move |v| {
                        self.vars.borrow_mut().insert(name.clone(), v.clone());
                        Ok(v)
                    })));
                }
                Ok(Box::new(once(Err(String::from(
                    "Unwind operator requires a list",
                )))))
            }
            IR::UnwindRange(start, stop, step, name) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(self.run(&child_idx)?.flat_map(move |_| {
                        let start = self.run_expr(start.root());
                        let stop = self.run_expr(stop.root());
                        let step = self.run_expr(step.root());
                        match (start, stop, step) {
                            (Ok(Value::Int(start)), Ok(Value::Int(stop)), Ok(Value::Int(step))) => {
                                let iter =
                                    std::iter::repeat_n(start, ((stop - start) / step + 1) as _)
                                        .enumerate()
                                        .map(move |(i, v)| v + i as i64 * step);
                                iter.map(move |v| {
                                    self.vars.borrow_mut().insert(name.clone(), Value::Int(v));
                                    Ok(Value::Int(v))
                                })
                            }
                            _ => {
                                todo!();
                            }
                        }
                    })));
                }
                let start = self.run_expr(start.root())?;
                let stop = self.run_expr(stop.root())?;
                let step = self.run_expr(step.root())?;
                match (start, stop, step) {
                    (Value::Int(start), Value::Int(stop), Value::Int(step)) => {
                        if step == 0 {
                            return Err(String::from("Step cannot be zero"));
                        }
                        let iter = std::iter::repeat_n(start, ((stop - start) / step + 1) as _)
                            .enumerate()
                            .map(move |(i, v)| v + i as i64 * step);
                        Ok(Box::new(iter.map(move |v| {
                            self.vars.borrow_mut().insert(name.clone(), Value::Int(v));
                            Ok(Value::Int(v))
                        })))
                    }
                    _ => Err(String::from("UnwindRange operator requires three integers")),
                }
            }
            IR::Create(pattern) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(self.run(&child_idx)?.map(move |_| {
                        self.create(pattern)?;
                        Ok(Value::List(vec![]))
                    })));
                }
                self.create(pattern)?;
                if let Some(parent) = self.plan.node(idx).parent() {
                    if matches!(parent.data(), IR::Commit) {
                        return Ok(Box::new(empty()));
                    }
                }
                Ok(Box::new(once(Ok(Value::List(vec![])))))
            }
            IR::Delete(trees) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(self.run(&child_idx)?.map(move |_| {
                        self.delete(trees)?;
                        Ok(Value::List(vec![]))
                    })));
                }
                self.delete(trees)?;
                Ok(Box::new(empty()))
            }
            IR::NodeScan(node_pattern) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(
                        self.run(&child_idx)?
                            .flat_map(move |_| self.node_scan(node_pattern)),
                    ));
                }
                Ok(Box::new(self.node_scan(node_pattern)))
            }
            IR::RelationshipScan(relationship_pattern) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(
                        self.run(&child_idx)?
                            .flat_map(move |_| self.relationship_scan(relationship_pattern)),
                    ));
                }
                Ok(Box::new(self.relationship_scan(relationship_pattern)))
            }
            IR::Filter(tree) => {
                if let Some(child_idx) = child_idx {
                    return Ok(Box::new(self.run(&child_idx)?.filter(move |_| {
                        self.run_expr(tree.root()) == Ok(Value::Bool(true))
                    })));
                }
                Err(String::from(
                    "Filter operator requires a boolean expression",
                ))
            }
            IR::Aggregate(name, trees, trees1) => {
                if let Some(child_idx) = child_idx {
                    let aggregator = AggregateIter {
                        iter: self.run(&child_idx)?,
                        key_fn: move |_| {
                            let mut vec = Vec::new();
                            for tree in trees {
                                vec.push(self.run_expr(tree.root()).unwrap());
                            }
                            vec
                        },
                        default_value: Ok(Value::Null),
                        agg_fn: move |x, acc| {
                            self.vars.borrow_mut().insert(name.clone(), acc.unwrap());
                            let mut tmp = trees1.iter();
                            if let Some(tree) = tmp.next() {
                                return self.run_expr(tree.root());
                            }
                            Ok(Value::Null)
                        },
                        cache: HashMap::new(),
                        finished: false,
                    }
                    .map(|(mut key, v)| {
                        key.push(v.unwrap());
                        Ok(Value::List(key))
                    });
                    return Ok(Box::new(aggregator));
                }
                Ok(Box::new(empty()))
            }
            IR::Project(trees) => {
                if let Some(child_idx) = child_idx {
                    Ok(Box::new(self.run(&child_idx)?.map(move |v| {
                        v?;
                        Ok(Value::List(
                            trees
                                .iter()
                                .map(|tree| self.run_expr(tree.root()))
                                .collect::<Result<Vec<Value>, String>>()?,
                        ))
                    })))
                } else {
                    Ok(Box::new(once(()).map(|()| {
                        Ok(Value::List(
                            trees
                                .iter()
                                .map(|tree| self.run_expr(tree.root()))
                                .collect::<Result<Vec<Value>, String>>()?,
                        ))
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
                    .run(&child_idx.unwrap())?
                    .collect::<Result<Vec<Value>, String>>()?
                    .into_iter()
                    .map(Ok);
                self.commit();
                Ok(Box::new(iter))
            }
        }
    }

    fn relationship_scan(
        &self,
        relationship_pattern: &crate::ast::RelationshipPattern,
    ) -> std::iter::Map<crate::tensor::Iter, impl FnMut((u64, u64, u64)) -> Result<Value, String>>
    {
        let iter = self
            .g
            .borrow()
            .get_relationships(&[relationship_pattern.relationship_type.clone()])
            .unwrap();
        iter.map(move |(src, dst, id)| {
            self.vars.borrow_mut().insert(
                relationship_pattern.alias.to_string(),
                Value::Relationship(id, src, dst),
            );
            self.vars
                .borrow_mut()
                .insert(relationship_pattern.from.to_string(), Value::Node(src));
            self.vars
                .borrow_mut()
                .insert(relationship_pattern.to.to_string(), Value::Node(dst));
            Ok(Value::Relationship(id, src, dst))
        })
    }

    fn node_scan(
        &self,
        node_pattern: &crate::ast::NodePattern,
    ) -> std::iter::Map<crate::matrix::Iter<bool>, impl FnMut((u64, u64)) -> Result<Value, String>>
    {
        let iter = self.g.borrow().get_nodes(&node_pattern.labels).unwrap();
        iter.map(move |(v, _)| {
            self.vars
                .borrow_mut()
                .insert(node_pattern.alias.to_string(), Value::Node(v));
            Ok(Value::Node(v))
        })
    }

    fn delete(
        &self,
        trees: &Vec<orx_tree::Tree<Dyn<ExprIR>>>,
    ) -> Result<(), String> {
        for tree in trees {
            let value = self.run_expr(tree.root());
            match value {
                Ok(Value::Node(id)) => {
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
    ) -> Result<(), String> {
        for node in &pattern.nodes {
            let properties = self.run_expr(node.attrs.root())?;
            match properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_node();
                    self.pending
                        .borrow_mut()
                        .created_nodes
                        .insert(id, (node.labels.clone(), properties));
                    self.vars
                        .borrow_mut()
                        .insert(node.alias.to_string(), Value::Node(id));
                }
                _ => return Err(String::from("Invalid node properties")),
            }
        }
        for rel in &pattern.relationships {
            let (from_id, to_id) = {
                let vars = self.vars.borrow();
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
            let properties = self.run_expr(rel.attrs.root())?;
            match properties {
                Value::Map(properties) => {
                    let id = self.g.borrow_mut().reserve_relationship();
                    self.pending.borrow_mut().created_relationships.insert(
                        id,
                        (rel.relationship_type.clone(), from_id, to_id, properties),
                    );
                    self.vars.borrow_mut().insert(
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

#[inline]
const fn logical_xor(
    a: bool,
    b: bool,
) -> bool {
    (a && !b) || (!a && b)
}
