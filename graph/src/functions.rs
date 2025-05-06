use crate::runtime::Runtime;
use crate::value::Value;
use rand::Rng;
use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::OnceLock;

type RuntimeFn = fn(&mut Runtime, Vec<Value>) -> Result<Value, String>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FnType {
    Function,
    Internal,
    Procedure,
    Aggregation,
}

#[derive(Debug)]
pub struct GraphFn {
    pub name: String,
    pub func: RuntimeFn,
    pub write: bool,
    pub min_args: usize,
    pub max_args: usize,
    pub fn_type: FnType,
}

impl GraphFn {
    #[must_use]
    pub fn new(
        name: &str,
        func: RuntimeFn,
        write: bool,
        min_args: usize,
        max_args: usize,
        fn_type: FnType,
    ) -> Self {
        Self {
            name: name.to_string(),
            func,
            write,
            min_args,
            max_args,
            fn_type,
        }
    }
}

#[derive(Default, Debug)]
pub struct Functions {
    functions: BTreeMap<String, GraphFn>,
}

impl Functions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: &str,
        func: RuntimeFn,
        write: bool,
        min_args: usize,
        max_args: usize,
        fn_type: FnType,
    ) {
        assert!(
            !self.functions.contains_key(name),
            "Function '{name}' already exists"
        );
        let graph_fn = GraphFn::new(name, func, write, min_args, max_args, fn_type);
        self.functions.insert(name.to_owned(), graph_fn);
    }

    pub fn validate(
        &self,
        name: &str,
        fn_type: &FnType,
        args: usize,
    ) -> Result<(), String> {
        self.get(name, fn_type).map_or_else(
            || Err(format!("Function {name} not found")),
            |graph_fn| {
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
            },
        )
    }

    #[must_use]
    pub fn get(
        &self,
        name: &str,
        fn_type: &FnType,
    ) -> Option<&GraphFn> {
        self.functions.get(name).and_then(|graph_fn| {
            if &graph_fn.fn_type == fn_type {
                Some(graph_fn)
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn is_aggregate(
        &self,
        name: &str,
    ) -> bool {
        self.functions
            .get(name)
            .is_some_and(|graph_fn| graph_fn.fn_type == FnType::Aggregation)
    }
}

static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

#[allow(clippy::too_many_lines)]
pub fn init_functions() -> Result<(), Functions> {
    let mut funcs = Functions::new();
    funcs.add("create_node", create_node, true, 2, 2, FnType::Internal);
    funcs.add(
        "create_relationship",
        create_relationship,
        true,
        4,
        4,
        FnType::Internal,
    );
    funcs.add(
        "delete_entity",
        delete_entity,
        true,
        1,
        usize::MAX,
        FnType::Internal,
    );

    funcs.add(
        "create_aggregate_ctx",
        create_aggregate_ctx,
        false,
        1,
        usize::MAX,
        FnType::Internal,
    );
    funcs.add(
        "create_node_iter",
        create_node_iter,
        false,
        1,
        1,
        FnType::Internal,
    );
    funcs.add("next_node", next_node, false, 1, 1, FnType::Internal);
    funcs.add(
        "create_relationship_iter",
        create_relationship_iter,
        false,
        1,
        1,
        FnType::Internal,
    );
    funcs.add(
        "next_relationship",
        next_relationship,
        false,
        1,
        1,
        FnType::Internal,
    );
    funcs.add("property", property, false, 2, 2, FnType::Internal);

    funcs.add("toInteger", value_to_integer, false, 1, 1, FnType::Function);
    funcs.add("labels", labels, false, 1, 1, FnType::Function);
    funcs.add("startnode", start_node, false, 1, 1, FnType::Function);
    funcs.add("endnode", end_node, false, 1, 1, FnType::Function);
    funcs.add("size", size, false, 1, 1, FnType::Function);
    funcs.add("head", head, false, 1, 1, FnType::Function);
    funcs.add("last", last, false, 1, 1, FnType::Function);
    funcs.add("tail", tail, false, 1, 1, FnType::Function);
    funcs.add("reverse", reverse, false, 1, 1, FnType::Function);
    funcs.add("substring", substring, false, 2, 3, FnType::Function);
    funcs.add("split", split, false, 2, 2, FnType::Function);
    funcs.add("toLower", string_to_lower, false, 1, 1, FnType::Function);
    funcs.add("toUpper", string_to_upper, false, 1, 1, FnType::Function);
    funcs.add("replace", string_replace, false, 3, 3, FnType::Function);
    funcs.add("left", string_left, false, 2, 2, FnType::Function);
    funcs.add("ltrim", string_ltrim, false, 1, 1, FnType::Function);
    funcs.add("right", string_right, false, 2, 2, FnType::Function);
    funcs.add("string.join", string_join, false, 1, 2, FnType::Function);
    funcs.add(
        "string.matchRegEx",
        string_match_reg_ex,
        false,
        2,
        2,
        FnType::Function,
    );
    funcs.add(
        "string.replaceRegEx",
        string_replace_reg_ex,
        false,
        3,
        3,
        FnType::Function,
    );
    funcs.add("abs", abs, false, 1, 1, FnType::Function);
    funcs.add("ceil", ceil, false, 1, 1, FnType::Function);
    funcs.add("e", e, false, 0, 0, FnType::Function);
    funcs.add("exp", exp, false, 1, 1, FnType::Function);
    funcs.add("floor", floor, false, 1, 1, FnType::Function);
    funcs.add("log", log, false, 1, 1, FnType::Function);
    funcs.add("log10", log10, false, 1, 1, FnType::Function);
    funcs.add("pow", pow, false, 2, 2, FnType::Function);
    funcs.add("rand", rand, false, 0, 0, FnType::Function);
    funcs.add("round", round, false, 1, 1, FnType::Function);
    funcs.add("sign", sign, false, 1, 1, FnType::Function);
    funcs.add("sqrt", sqrt, false, 1, 1, FnType::Function);
    funcs.add("range", range, false, 1, 3, FnType::Function);

    // aggregation functions
    funcs.add("collect", collect, false, 1, 2, FnType::Aggregation);
    funcs.add("count", count, false, 1, 2, FnType::Aggregation);
    funcs.add("sum", sum, false, 1, 2, FnType::Aggregation);
    funcs.add("max", max, false, 1, 2, FnType::Aggregation);
    funcs.add("min", min, false, 1, 2, FnType::Aggregation);

    // Internal functions
    funcs.add(
        "starts_with",
        internal_starts_with,
        false,
        2,
        2,
        FnType::Internal,
    );
    funcs.add(
        "ends_with",
        internal_ends_with,
        false,
        2,
        2,
        FnType::Internal,
    );
    funcs.add("contains", internal_contains, false, 2, 2, FnType::Internal);
    funcs.add(
        "regex_matches",
        internal_regex_matches,
        false,
        2,
        2,
        FnType::Internal,
    );

    // Procedures
    funcs.add("db.labels", db_labels, false, 0, 0, FnType::Procedure);
    funcs.add(
        "db.relationshiptypes",
        db_types,
        false,
        0,
        0,
        FnType::Procedure,
    );
    funcs.add(
        "db.propertykeys",
        db_properties,
        false,
        0,
        0,
        FnType::Procedure,
    );

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
            Ok(runtime.g.borrow_mut().create_node(&labels, attrs))
        }
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn delete_entity(
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    for n in args {
        if let Value::Node(id) = n {
            runtime.nodes_deleted += 1;
            let mut g = runtime.g.borrow_mut();
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
            Ok(runtime
                .g
                .borrow_mut()
                .create_relationship(&relationship_type, from, to, attrs))
        }
        _ => todo!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn create_aggregate_ctx(
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
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::List(raw_labels)), None) => {
            runtime.node_iters.push(
                runtime
                    .g
                    .borrow()
                    .get_nodes(
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
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(raw_type)), None) => {
            runtime
                .relationship_iters
                .push(runtime.g.borrow().get_relationships(&[raw_type]).unwrap());
            Ok(Value::Int(runtime.relationship_iters.len() as i64 - 1))
        }
        _ => todo!(),
    }
}

fn next_relationship(
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
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), Some(Value::String(property)), None) => runtime
            .g
            .borrow()
            .get_node_property_id(&property)
            .map_or(Ok(Value::Null), |property_id| {
                runtime
                    .g
                    .borrow()
                    .get_node_property(node_id, property_id)
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
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), None) => Ok(Value::List(
            runtime
                .g
                .borrow()
                .get_node_label_ids(node_id)
                .map(|label_id| {
                    Value::String(runtime.g.borrow().get_label_by_id(label_id).to_string())
                })
                .collect(),
        )),
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn start_node(
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
    runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Null, _] => {}
        [_, Value::Int(hash)] => {
            runtime.agg_ctxs.entry(*hash as _).and_modify(|v| {
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
    _runtime: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => s.parse::<i64>().map(Value::Int).or_else(|_| {
            s.parse::<f64>()
                .map(|f| Value::Int(f as i64))
                .or(Ok(Value::Null))
        }),
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(*f as i64)),
        [Value::Null] => Ok(Value::Null),
        [Value::Bool(b)] => Ok(Value::Int(i64::from(*b))),
        [arg] => Err(format!(
            "Type mismatch: expected String, Boolean, Integer, Float, or Null but was {}",
            arg.name()
        )),
        args => Err(format!(
            "Expected one argument for value_to_integer, instead {}",
            args.len()
        )),
    }
}

fn size(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => Ok(Value::Int(s.len() as i64)),
        [Value::List(v)] => Ok(Value::Int(v.len() as i64)),
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List, String, or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn head(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(v)] => {
            if v.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(v[0].clone())
            }
        }
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn last(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(v)] => Ok(v.last().unwrap_or(&Value::Null).clone()),
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn tail(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(v)] => {
            if v.is_empty() {
                Ok(Value::List(vec![]))
            } else {
                Ok(Value::List(v[1..].to_vec()))
            }
        }
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn reverse(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(v)] => {
            let mut v = v.clone();
            v.reverse();
            Ok(Value::List(v))
        }
        [Value::Null] => Ok(Value::Null),
        [Value::String(s)] => Ok(Value::String(s.chars().rev().collect())),
        [arg] => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn substring(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        // Handle NULL input case
        [Value::Null, _] | [Value::Null, _, _] => Ok(Value::Null),
        // Two-argument version: (string, start)
        [Value::String(s), Value::Int(start)] => {
            let start = *start;
            if start < 0 {
                return Err("start must be a non-negative integer".into());
            }
            let start = start as usize;

            Ok(Value::String(s[start..].to_string()))
        }

        // Three-argument version: (string, start, length)
        [Value::String(s), Value::Int(start), Value::Int(length)] => {
            let start = *start;
            let length = *length;
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

        [Value::String(_), t] => Err(format!(
            "Type mismatch: expected Integer Or Null but got {}",
            t.name()
        )),
        [t, Value::Int(_)] | [t, Value::Int(_), Value::Int(_)] => Err(format!(
            "Type mismatch: expected String Or Null but got {}",
            t.name()
        )),
        [Value::String(_), t, Value::Int(_)] | [Value::String(_), Value::Int(_), t] => {
            Err(format!(
                "Type mismatch: expected Integer Or Null but got {}",
                t.name()
            ))
        }
        _ => unreachable!(),
    }
}

fn split(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(string), Value::String(delimiter)] => {
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
        [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected 2 String or null arguments, but was {} {}",
            arg1.name(),
            arg2.name()
        )),
        [arg] => Err(format!(
            "Type mismatch: expected 2 String or null arguments, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_to_lower(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => Ok(Value::String(s.to_lowercase())),
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => Ok(Value::String(s.to_uppercase())),
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected List, String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_replace(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [
            Value::String(s),
            Value::String(search),
            Value::String(replacement),
        ] => Ok(Value::String(s.replace(search, replacement))),
        [Value::Null, _, _] | [_, Value::Null, _] | [_, _, Value::Null] => Ok(Value::Null),
        [arg1, arg2, arg3] => Err(format!(
            "Type mismatch: expected (String, String, String) or null, but was: ({}, {}, {})",
            arg1.name(),
            arg2.name(),
            arg3.name()
        )),
        _ => unreachable!(),
    }
}

fn string_left(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::Int(n)] => {
            if *n < 0 {
                Err("length must be a non-negative integer".to_string())
            } else {
                Ok(Value::String(s.chars().take(*n as usize).collect()))
            }
        }
        [Value::Null, _] => Ok(Value::Null),
        [_, Value::Null] => Err("length must be a non-negative integer".to_string()),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn string_ltrim(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => Ok(Value::String(s.trim_start().to_string())),
        [Value::Null] => Ok(Value::Null),
        [arg] => Err(format!(
            "Type mismatch: expected String or null, but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_right(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::Int(n)] => {
            if *n < 0 {
                Err("length must be a non-negative integer".to_string())
            } else {
                let start = s.len().saturating_sub(*n as usize);
                Ok(Value::String(s.chars().skip(start).collect()))
            }
        }
        [Value::Null, _] => Ok(Value::Null),
        [_, Value::Null] => Err("length must be a non-negative integer".to_string()),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}
fn string_join(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    fn to_string_vec(vec: &[Value]) -> Result<Vec<String>, String> {
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

    match args.as_slice() {
        [Value::List(vec), Value::String(s)] => {
            let result = to_string_vec(vec);
            result.map(|strings| Value::String(strings.join(s)))
        }
        [Value::List(vec)] => {
            let result = to_string_vec(vec);
            result.map(|strings| Value::String(strings.join("")))
        }
        [Value::Null, _] => Ok(Value::Null),
        [arg1, _arg2] => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            arg1.name()
        )),
        _ => unreachable!(),
    }
}

fn string_match_reg_ex(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(text), Value::String(pattern)] => match regex::Regex::new(pattern) {
            Ok(re) => {
                let mut all_matches = Vec::new();
                for caps in re.captures_iter(text) {
                    for i in 0..caps.len() {
                        if let Some(m) = caps.get(i) {
                            all_matches.push(Value::String(m.as_str().to_string()));
                        }
                    }
                }
                Ok(Value::List(all_matches))
            }
            Err(e) => Err(format!("Invalid regex, {e}")),
        },
        [Value::Null, _] | [_, Value::Null] => Ok(Value::List(vec![])),
        [Value::String(_), arg2] => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg2.name(),
        )),
        [arg1, _] => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg1.name(),
        )),
        _ => unreachable!(),
    }
}

fn string_replace_reg_ex(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [
            Value::String(text),
            Value::String(pattern),
            Value::String(replacement),
        ] => match regex::Regex::new(pattern) {
            Ok(re) => {
                let replaced_text = re.replace_all(text, replacement).to_string();
                Ok(Value::String(replaced_text))
            }
            Err(e) => Err(format!("Invalid regex, {e}")),
        },
        [Value::Null, _, _] | [_, Value::Null, _] | [_, _, Value::Null] => Ok(Value::Null),
        [Value::String(_), arg2, Value::String(_)] => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg2.name(),
        )),
        [Value::String(_), Value::String(_), arg3] => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg3.name(),
        )),
        [arg1, _, _] => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg1.name(),
        )),
        _ => unreachable!(),
    }
}

fn abs(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Int(n.abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn ceil(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Int(*n)),
        [Value::Float(f)] => Ok(Value::Float(f.ceil())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn e(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [] => Ok(Value::Float(std::f64::consts::E)),
        _ => unreachable!(),
    }
}

fn exp(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Float((*n as f64).exp())),
        [Value::Float(f)] => Ok(Value::Float(f.exp())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn floor(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Int(*n)),
        [Value::Float(f)] => Ok(Value::Float(f.floor())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn log(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Float((*n as f64).ln())),
        [Value::Float(f)] => Ok(Value::Float(f.ln())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn log10(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Float((*n as f64).log10())),
        [Value::Float(f)] => Ok(Value::Float(f.log10())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}
fn pow(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(i1), Value::Int(i2)] => Ok(Value::Float((*i1 as f64).powi(*i2 as i32))),
        [Value::Float(f1), Value::Float(f2)] => Ok(Value::Float(f1.powf(*f2))),
        [Value::Int(i1), Value::Float(f1)] => Ok(Value::Float((*i1 as f64).powf(*f1))),
        [Value::Float(f1), Value::Int(i1)] => Ok(Value::Float(f1.powi(*i1 as i32))),
        [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
        [Value::Int(_) | Value::Float(_), v] | [v, Value::Int(_) | Value::Float(_)] => {
            Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            ))
        }
        _ => unreachable!(),
    }
}

fn rand(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [] => {
            let mut rng = rand::rng();
            Ok(Value::Float(rng.random_range(0.0..1.0)))
        }
        _ => unreachable!(),
    }
}

fn round(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Int(*n)),
        [Value::Float(f)] => Ok(Value::Float(f.round())),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn sign(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => Ok(Value::Int(n.signum())),
        [Value::Float(f)] => Ok(if *f == 0.0 {
            Value::Int(0)
        } else {
            Value::Float(f.signum().round())
        }),
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn sqrt(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Int(n)] => {
            if *n < 0 {
                Ok(Value::Float(f64::NAN))
            } else {
                Ok(Value::Float((*n as f64).sqrt()))
            }
        }
        [Value::Float(f)] => {
            if *f > 0f64 {
                Ok(Value::Float(f.sqrt()))
            } else {
                Ok(Value::Float(f64::NAN))
            }
        }
        [Value::Null] => Ok(Value::Null),
        [v] => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

fn range(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let start = &args[0];
    let end = &args[1];
    let step = args.get(2).unwrap_or(&Value::Int(1));
    match (start, end, step) {
        (Value::Int(start), Value::Int(end), Value::Int(step)) => {
            if start >= end && step < &0 {
                Ok(Value::List(
                    (*end..=*start)
                        .rev()
                        .step_by(step.abs() as usize)
                        .map(Value::Int)
                        .collect(),
                ))
            } else if step < &0 {
                Ok(Value::List(vec![]))
            } else {
                Ok(Value::List(
                    (*start..=*end)
                        .step_by(*step as usize)
                        .map(Value::Int)
                        .collect(),
                ))
            }
        }
        _ => Err("Range operator requires two integers".to_string()),
    }
}

//
// Internal functions
//

fn internal_starts_with(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::String(prefix)] => Ok(Value::Bool(s.starts_with(prefix))),

        [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_ends_with(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::String(suffix)] => Ok(Value::Bool(s.ends_with(suffix))),
        [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
        [arg1, arg2] => Err(format!(
            "Type mismatch: Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_contains(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::String(substring)] => Ok(Value::Bool(s.contains(substring))),
        [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected String or Null but was ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => unreachable!(),
    }
}

fn internal_regex_matches(
    _: &mut Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s), Value::String(pattern)] => {
            // Compile the regex pattern
            match regex::Regex::new(pattern) {
                Ok(re) => Ok(Value::Bool(re.is_match(s))),
                Err(e) => Err(format!("Invalid regex pattern: {e}")),
            }
        }
        [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
        [arg1, arg2] => Err(format!(
            "Type mismatch: expected (String, String) or null, but was: ({}, {})",
            arg1.name(),
            arg2.name()
        )),
        _ => Err("Expected two arguments for regex matching".to_string()),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn db_labels(
    runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_labels()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn db_types(
    runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_types()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn db_properties(
    runtime: &mut Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_properties()
            .map(|n| Value::String(n.to_string()))
            .collect(),
    ))
}
