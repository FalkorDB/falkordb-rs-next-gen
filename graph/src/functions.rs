use crate::graph::Graph;
use crate::runtime::Runtime;
use crate::value::Value;
use rand::Rng;
use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::OnceLock;

#[derive(Default, Debug)]
pub struct Functions {
    functions: BTreeMap<String, GraphFn>,
}

impl Functions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(
        &mut self,
        name: &str,
        fn_type: FnType,
        min_args: usize,
        max_args: usize,
    ) {
        if self.functions.contains_key(name) {
            panic!("Function '{}' already exists", name);
        }
        let graph_fn = GraphFn::new(name, fn_type, min_args, max_args);
        self.functions.insert(name.to_owned(), graph_fn);
    }

    pub fn validate(
        &self,
        name: &str,
        args: usize,
    ) -> Result<(), String> {
        if let Some(graph_fn) = self.functions.get(name) {
            if args < graph_fn.min_args {
                Err(format!(
                    "Received {} arguments to function '{}', expected at least {}",
                    args, name, graph_fn.min_args
                ))
            } else if graph_fn.max_args < args {
                Err(format!(
                    "Received {} arguments to function '{}', expected at most {}",
                    args, name, graph_fn.max_args
                ))
            } else {
                Ok(())
            }
        } else {
            Err(format!("Function {} not found", name))
        }
    }
    pub fn get(
        &self,
        name: &str,
    ) -> Option<&GraphFn> {
        self.functions.get(name)
    }
}

#[derive(Debug)]
pub struct GraphFn {
    pub name: String,
    pub fn_type: FnType,
    pub min_args: usize,
    pub max_args: usize,
}

impl GraphFn {
    pub fn new(
        name: &str,
        fn_type: FnType,
        min_args: usize,
        max_args: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            fn_type,
            min_args,
            max_args,
        }
    }
}

#[derive(Debug)]
pub enum FnType {
    Read(ReadFn),
    Write(WriteFn),
}

type ReadFn = fn(&Graph, &mut Runtime, Vec<Value>) -> Result<Value, String>;
type WriteFn = fn(&mut Graph, &mut Runtime, Vec<Value>) -> Result<Value, String>;

static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

pub fn init_functions() -> Result<(), Functions> {
    let mut funcs = Functions::new();
    funcs.add("create_node", FnType::Write(create_node), 2, 2);
    funcs.add(
        "create_relationship",
        FnType::Write(create_relationship),
        4,
        4,
    );
    funcs.add("delete_entity", FnType::Write(delete_entity), 1, usize::MAX);

    funcs.add(
        "create_aggregate_ctx",
        FnType::Read(create_aggregate_ctx),
        1,
        usize::MAX,
    );
    funcs.add("create_node_iter", FnType::Read(create_node_iter), 1, 1);
    funcs.add("next_node", FnType::Read(next_node), 1, 1);
    funcs.add(
        "create_relationship_iter",
        FnType::Read(create_relationship_iter),
        1,
        1,
    );
    funcs.add("next_relationship", FnType::Read(next_relationship), 1, 1);
    funcs.add("property", FnType::Read(property), 2, 2);
    funcs.add("toInteger", FnType::Read(value_to_integer), 1, 1);
    funcs.add("labels", FnType::Read(labels), 1, 1);
    funcs.add("startnode", FnType::Read(start_node), 1, 1);
    funcs.add("endnode", FnType::Read(end_node), 1, 1);
    funcs.add("size", FnType::Read(size), 1, 1);
    funcs.add("head", FnType::Read(head), 1, 1);
    funcs.add("last", FnType::Read(last), 1, 1);
    funcs.add("tail", FnType::Read(tail), 1, 1);
    funcs.add("reverse", FnType::Read(reverse), 1, 1);
    funcs.add("substring", FnType::Read(substring), 2, 3);
    funcs.add("split", FnType::Read(split), 2, 2);
    funcs.add("toLower", FnType::Read(string_to_lower), 1, 1);
    funcs.add("toUpper", FnType::Read(string_to_upper), 1, 1);
    funcs.add("replace", FnType::Read(string_replace), 3, 3);
    funcs.add("left", FnType::Read(string_left), 2, 2);
    funcs.add("ltrim", FnType::Read(string_ltrim), 1, 1);
    funcs.add("right", FnType::Read(string_right), 2, 2);
    funcs.add("string.join", FnType::Read(string_join), 1, 2);
    funcs.add("string.matchRegEx", FnType::Read(string_match_reg_ex), 2, 2);
    funcs.add(
        "string.replaceRegEx",
        FnType::Read(string_replace_reg_ex),
        3,
        3,
    );
    funcs.add("abs", FnType::Read(abs), 1, 1);
    funcs.add("ceil", FnType::Read(ceil), 1, 1);
    funcs.add("e", FnType::Read(e), 0, 0);
    funcs.add("exp", FnType::Read(exp), 1, 1);
    funcs.add("floor", FnType::Read(floor), 1, 1);
    funcs.add("log", FnType::Read(log), 1, 1);
    funcs.add("log10", FnType::Read(log10), 1, 1);
    funcs.add("pow", FnType::Read(pow), 2, 2);
    funcs.add("rand", FnType::Read(rand), 0, 0);
    funcs.add("round", FnType::Read(round), 1, 1);
    funcs.add("sign", FnType::Read(sign), 1, 1);
    funcs.add("sqrt", FnType::Read(sqrt), 1, 1);

    // aggregation functions
    funcs.add("collect", FnType::Read(collect), 1, 2);
    funcs.add("count", FnType::Read(count), 1, 2);
    funcs.add("sum", FnType::Read(sum), 1, 2);
    funcs.add("max", FnType::Read(max), 1, 2);
    funcs.add("min", FnType::Read(min), 1, 2);

    // Internal functions
    funcs.add("@starts_with", FnType::Read(internal_starts_with), 2, 2);
    funcs.add("@ends_with", FnType::Read(internal_ends_with), 2, 2);
    funcs.add("@contains", FnType::Read(internal_contains), 2, 2);
    funcs.add("@regex_matches", FnType::Read(internal_regex_matches), 2, 2);

    // Procedures
    funcs.add("db.labels", FnType::Read(db_labels), 0, 0);
    funcs.add("db.relationshiptypes", FnType::Read(db_types), 0, 0);
    funcs.add("db.propertykeys", FnType::Read(db_properties), 0, 0);

    FUNCTIONS.set(funcs)
}

pub fn get_functions() -> &'static Functions {
    FUNCTIONS.get().expect("Functions not initialized")
}

///////////////////////////////////
///////////// functions ///////////
///////////////////////////////////

#[allow(clippy::unnecessary_wraps)]
fn create_node(
    g: &mut Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
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
            Ok(g.create_node(&labels, attrs))
        }
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn delete_entity(
    g: &mut Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    for n in args {
        if let Value::Node(id) = n {
            runtime.nodes_deleted += 1;
            for (src, dest, id) in g.get_node_relationships(id).collect::<Vec<_>>() {
                runtime.relationships_deleted += 1;
                g.delete_relationship(id, src, dest);
            }
            g.delete_node(id);
        }
    }

    Ok(Value::Null)
}

fn create_relationship(
    g: &mut Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
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
            Ok(g.create_relationship(&relationship_type, from, to, attrs))
        }
        _ => todo!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn create_aggregate_ctx(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut hasher = DefaultHasher::new();
    args.hash(&mut hasher);
    let key = hasher.finish();
    runtime
        .agg_ctxs
        .entry(key)
        .or_insert_with(|| (Value::List(args), Value::Null));
    Ok(Value::Int(key as i64))
}

fn create_node_iter(
    g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::List(raw_labels)), None) => {
            runtime.node_iters.push(
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

            Ok(Value::Int(runtime.node_iters.len() as i64 - 1))
        }
        _ => todo!(),
    }
}

fn next_node(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Int(iter)), None) => runtime.node_iters[iter as usize]
            .next()
            .map_or_else(|| Ok(Value::Null), |(n, _)| Ok(Value::Node(n))),
        _ => todo!(),
    }
}

fn create_relationship_iter(
    g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(raw_type)), None) => {
            runtime
                .relationship_iters
                .push(g.get_relationships(&[raw_type]).unwrap());
            Ok(Value::Int(runtime.relationship_iters.len() as i64 - 1))
        }
        _ => todo!(),
    }
}

fn next_relationship(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Int(iter)), None) => runtime.relationship_iters[iter as usize]
            .next()
            .map_or(Ok(Value::Null), |(src, dest, id)| {
                Ok(Value::Relationship(id, src, dest))
            }),
        _ => todo!(),
    }
}

fn property(
    g: &Graph,
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), Some(Value::String(property)), None) => g
            .get_node_property_id(&property)
            .map_or(Ok(Value::Null), |property_id| {
                g.get_node_property(node_id, property_id)
                    .map_or(Ok(Value::Null), Ok)
            }),
        (Some(Value::Map(map)), Some(Value::String(property)), None) => {
            Ok(map.get(&property).unwrap_or(&Value::Null).clone())
        }
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn labels(
    g: &Graph,
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), None) => Ok(Value::List(
            g.get_node_label_ids(node_id)
                .map(|label_id| Value::String(g.get_label_by_id(label_id).to_string()))
                .collect(),
        )),
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn start_node(
    _g: &Graph,
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Relationship(_, src, _)), None) => Ok(Value::Node(src)),
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn end_node(
    _g: &Graph,
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Relationship(_, _, dest)), None) => Ok(Value::Node(dest)),
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn collect(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    if let (Some(x), Some(Value::Int(hash)), None) = (iter.next(), iter.next(), iter.next()) {
        runtime.agg_ctxs.entry(hash as _).and_modify(|v| {
            if let (_, Value::List(values)) = v {
                values.push(x.clone());
            } else {
                v.1 = Value::List(vec![x.clone()]);
            }
        });
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn count(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Null), _) => {}
        (_, Some(Value::Int(hash))) => {
            runtime.agg_ctxs.entry(hash as _).and_modify(|v| {
                if let (_, Value::Int(count)) = v {
                    *count += 1;
                } else {
                    v.1 = Value::Int(1);
                }
            });
        }
        _ => (),
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn sum(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    if let [a, Value::Int(hash)] = args.as_slice() {
        runtime.agg_ctxs.entry(*hash as _).and_modify(|v| {
            if let (_, Value::Null) = v {
                v.1 = a.clone();
            } else {
                v.1 = (v.1.clone() + a.clone()).unwrap();
            }
        });
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn max(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    if let [Value::Int(a), Value::Int(hash)] = args.as_slice() {
        runtime.agg_ctxs.entry(*hash as _).and_modify(|v| {
            if let (_, Value::Int(b)) = v {
                if a > b {
                    *b = *a;
                }
            } else {
                v.1 = Value::Int(*a);
            }
        });
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn min(
    _g: &Graph,
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    if let [Value::Int(a), Value::Int(hash)] = args.as_slice() {
        runtime.agg_ctxs.entry(*hash as _).and_modify(|v| {
            if let (_, Value::Int(b)) = v {
                if a < b {
                    *b = *a;
                }
            } else {
                v.1 = Value::Int(*a);
            }
        });
    }
    Ok(Value::Null)
}

fn value_to_integer(
    _g: &Graph,
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let len = args.len();
    match args.into_iter().next() {
        Some(Value::String(s)) => s.parse::<i64>().map(Value::Int).or_else(|_| {
            s.parse::<f64>()
                .map(|f| Value::Int(f as i64))
                .or(Ok(Value::Null))
        }),
        Some(v @ Value::Int(_)) => Ok(v),
        Some(Value::Float(f)) => Ok(Value::Int(f as i64)),
        Some(Value::Null) => Ok(Value::Null),
        Some(Value::Bool(b)) => Ok(Value::Int(i64::from(b))),
        Some(arg) => Err(format!(
            "Type mismatch: expected String, Boolean, Integer, Float, or Null but was {}",
            arg.name()
        )),
        _ => Err(format!(
            "Expected one argument for value_to_integer, instead {}",
            len
        )),
    }
}

fn size(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::Int(s.len() as i64)),
        Some(Value::List(v)) => Ok(Value::Int(v.len() as i64)),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List, String, or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn head(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => {
            if v.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(v[0].clone())
            }
        }
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn last(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => Ok(v.last().unwrap_or(&Value::Null).clone()),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn tail(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => {
            if v.is_empty() {
                Ok(Value::List(vec![]))
            } else {
                Ok(Value::List(v[1..].to_vec()))
            }
        }
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn reverse(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => {
            let mut v = v.clone();
            v.reverse();
            Ok(Value::List(v))
        }
        Some(Value::Null) => Ok(Value::Null),
        Some(Value::String(s)) => Ok(Value::String(s.chars().rev().collect())),
        Some(arg) => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn substring(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        // Handle NULL input case
        (Some(Value::Null), _, _) => Ok(Value::Null),
        // Two-argument version: (string, start)
        (Some(Value::String(s)), Some(Value::Int(start)), None) => {
            if start < 0 {
                return Err("start must be a non-negative integer".into());
            }
            let start = start as usize;

            Ok(Value::String(s[start..].to_string()))
        }

        // Three-argument version: (string, start, length)
        (Some(Value::String(s)), Some(Value::Int(start)), Some(Value::Int(length))) => {
            if length < 0 {
                return Err("length must be a non-negative integer".into());
            }
            if start < 0 {
                return Err("start must be a non-negative integer".into());
            }
            let start = start as usize;
            let length = length as usize;

            let end = start.saturating_add(length).min(s.len());
            Ok(Value::String(s[start..end].to_string()))
        }

        (Some(Value::String(_)), Some(t), None) => Err(format!(
            "Type mismatch: expected Integer Or Null but got {}",
            t.name()
        )),
        (Some(t), Some(Value::Int(_)), None)
        | (Some(t), Some(Value::Int(_)), Some(Value::Int(_))) => Err(format!(
            "Type mismatch: expected String Or Null but got {}",
            t.name()
        )),
        (Some(Value::String(_)), Some(t), Some(Value::Int(_)))
        | (Some(Value::String(_)), Some(Value::Int(_)), Some(t)) => Err(format!(
            "Type mismatch: expected Integer Or Null but got {}",
            t.name()
        )),

        _ => unreachable!(),
    }
}

fn split(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(string)), Some(Value::String(delimiter))) => {
            if delimiter.is_empty() {
                // split string to characters
                let parts: Vec<Value> = string
                    .chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect();
                Ok(Value::List(parts))
            } else {
                let parts: Vec<Value> = string
                    .split(delimiter.as_str())
                    .map(|s| Value::String(s.to_string()))
                    .collect();
                Ok(Value::List(parts))
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::Null),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected 2 String or null arguments, but was {} {}",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn string_to_lower(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_replace(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(search)), Some(Value::String(replacement))) => {
            Ok(Value::String(
                s.replace(search.as_str(), replacement.as_str()),
            ))
        }
        (Some(Value::Null), _, _) | (_, Some(Value::Null), _) | (_, _, Some(Value::Null)) => {
            Ok(Value::Null)
        }
        (Some(arg1), Some(arg2), Some(arg3)) => Err(format!(
            "Type mismatch: expected (String, String, String) or null, but was: ({}, {}, {})",
            arg1.name(),
            arg2.name(),
            arg3.name()
        )),
        _ => unreachable!(),
    }
}

fn string_left(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if n < 0 {
                Err("length must be a non-negative integer".to_string())
            } else {
                Ok(Value::String(s.chars().take(n as usize).collect()))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err("length must be a non-negative integer".to_string()),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn string_ltrim(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s.trim_start().to_string())),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_right(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if n < 0 {
                Err("length must be a non-negative integer".to_string())
            } else {
                let start = s.len().saturating_sub(n as usize);
                Ok(Value::String(s.chars().skip(start).collect()))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err("length must be a non-negative integer".to_string()),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}
fn string_join(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    fn to_string_vec(vec: Vec<Value>) -> Result<Vec<String>, String> {
        vec.iter()
            .map(|item| {
                if let Value::String(s) = item {
                    Ok(s.clone())
                } else {
                    Err(format!(
                        "Type mismatch: expected String but was {}",
                        item.name()
                    ))
                }
            })
            .collect()
    }
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::List(vec)), Some(Value::String(s))) => {
            let result = to_string_vec(vec);
            result.map(|strings| Value::String(strings.join(s.as_str())))
        }
        (Some(Value::List(vec)), None) => {
            let result = to_string_vec(vec);
            result.map(|strings| Value::String(strings.join("")))
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (Some(arg1), Some(_)) => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg1.name()
        )),
        _ => unreachable!(),
    }
}

fn string_match_reg_ex(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(text)), Some(Value::String(pattern))) => {
            match regex::Regex::new(pattern.as_str()) {
                Ok(re) => {
                    let mut all_matches = Vec::new();
                    for caps in re.captures_iter(text.as_str()) {
                        for i in 0..caps.len() {
                            if let Some(m) = caps.get(i) {
                                all_matches.push(Value::String(m.as_str().to_string()));
                            }
                        }
                    }
                    Ok(Value::List(all_matches))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::List(vec![])),
        (Some(Value::String(_)), Some(arg2)) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg2.name(),
        )),
        (Some(arg1), Some(_)) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg1.name(),
        )),
        _ => unreachable!(),
    }
}

fn string_replace_reg_ex(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (
            Some(Value::String(text)),
            Some(Value::String(pattern)),
            Some(Value::String(replacement)),
        ) => match regex::Regex::new(pattern.as_str()) {
            Ok(re) => {
                let replaced_text = re.replace_all(text.as_str(), replacement).to_string();
                Ok(Value::String(replaced_text))
            }
            Err(e) => Err(format!("Invalid regex, {e}")),
        },
        (Some(Value::Null), Some(_), Some(_))
        | (Some(_), Some(Value::Null), Some(_))
        | (Some(_), Some(_), Some(Value::Null)) => Ok(Value::Null),
        (Some(Value::String(_)), Some(arg2), Some(Value::String(_))) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg2.name(),
        )),
        (Some(Value::String(_)), Some(Value::String(_)), Some(arg3)) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg3.name(),
        )),
        (Some(arg1), Some(_), Some(_)) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg1.name(),
        )),
        _ => unreachable!(),
    }
}

fn abs(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Int(n.abs())),
        Some(Value::Float(f)) => Ok(Value::Float(f.abs())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn ceil(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Int(n)),
        Some(Value::Float(f)) => Ok(Value::Float(f.ceil())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn e(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        None => Ok(Value::Float(std::f64::consts::E)),
        _ => unreachable!(),
    }
}

fn exp(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).exp())),
        Some(Value::Float(f)) => Ok(Value::Float(f.exp())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn floor(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Int(n)),
        Some(Value::Float(f)) => Ok(Value::Float(f.floor())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn log(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).ln())),
        Some(Value::Float(f)) => Ok(Value::Float(f.ln())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn log10(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).log10())),
        Some(Value::Float(f)) => Ok(Value::Float(f.log10())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}
fn pow(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Int(i1)), Some(Value::Int(i2))) => {
            Ok(Value::Float((i1 as f64).powi(i2 as i32)))
        }
        (Some(Value::Float(f1)), Some(Value::Float(f2))) => Ok(Value::Float(f1.powf(f2))),
        (Some(Value::Int(i1)), Some(Value::Float(f1))) => Ok(Value::Float((i1 as f64).powf(f1))),
        (Some(Value::Float(f1)), Some(Value::Int(i1))) => Ok(Value::Float(f1.powi(i1 as i32))),
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::Null),
        (Some(Value::Int(_)) | Some(Value::Float(_)), Some(v))
        | (Some(v), Some(Value::Int(_)) | Some(Value::Float(_))) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn rand(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.is_empty() {
        let mut rng = rand::rng();
        Ok(Value::Float(rng.random_range(0.0..1.0)))
    } else {
        unreachable!()
    }
}

fn round(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Int(n)),
        Some(Value::Float(f)) => Ok(Value::Float(f.round())),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn sign(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Int(n.signum())),
        Some(Value::Float(f)) => Ok(if f == 0.0 {
            Value::Int(0)
        } else {
            Value::Float(f.signum().round())
        }),
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn sqrt(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => {
            if n < 0 {
                Ok(Value::Float(f64::NAN))
            } else {
                Ok(Value::Float((n as f64).sqrt()))
            }
        }
        Some(Value::Float(f)) => {
            if f > 0f64 {
                Ok(Value::Float(f.sqrt()))
            } else {
                Ok(Value::Float(f64::NAN))
            }
        }
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

//
// Internal functions
//

fn internal_starts_with(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(prefix))) => {
            Ok(Value::Bool(s.starts_with(prefix.as_str())))
        }

        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(Value::Null),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_ends_with(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(suffix))) => {
            Ok(Value::Bool(s.ends_with(suffix.as_str())))
        }
        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(Value::Null),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_contains(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(substring))) => {
            Ok(Value::Bool(s.contains(substring.as_str())))
        }
        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(Value::Null),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_regex_matches(
    _: &Graph,
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(pattern))) => {
            // Compile the regex pattern
            match regex::Regex::new(pattern.as_str()) {
                Ok(re) => Ok(Value::Bool(re.is_match(s.as_str()))),
                Err(e) => Err(format!("Invalid regex pattern: {e}")),
            }
        }
        (Some(Value::Null), _) | (_, Some(Value::Null)) => Ok(Value::Null),
        (Some(arg1), Some(arg2)) => Err(format!(
            "Type mismatch: expected (String, String) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn db_labels(
    g: &Graph,
    _runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        g.get_labels()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn db_types(
    g: &Graph,
    _runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        g.get_types()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn db_properties(
    g: &Graph,
    _runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        g.get_properties()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}
