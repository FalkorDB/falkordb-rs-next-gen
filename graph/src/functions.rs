use crate::runtime::Runtime;
use crate::value::Value;
use rand::Rng;
use std::collections::BTreeMap;
use std::sync::OnceLock;

type RuntimeFn = fn(&Runtime, Vec<Value>) -> Result<Value, String>;

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
        let name = name.to_lowercase();
        assert!(
            !self.functions.contains_key(&name),
            "Function '{name}' already exists"
        );
        let graph_fn = GraphFn::new(&name, func, write, min_args, max_args, fn_type);
        self.functions.insert(name, graph_fn);
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
        self.functions
            .get(name.to_lowercase().as_str())
            .and_then(|graph_fn| {
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

    funcs.add("property", property, false, 2, 2, FnType::Internal);

    funcs.add("toInteger", value_to_integer, false, 1, 1, FnType::Function);
    funcs.add("toString", value_to_string, false, 1, 1, FnType::Function);
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
    funcs.add("coalesce", coalesce, false, 1, usize::MAX, FnType::Function);
    funcs.add("keys", keys, false, 1, 1, FnType::Function);


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
    funcs.add("case", internal_case, false, 1, 2, FnType::Internal);

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

fn property(
    runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), Some(Value::String(property)), None) => {
            if let Some(node) = runtime.pending.borrow().created_nodes.get(&node_id) {
                if let Some(value) = node.1.get(&property) {
                    return Ok(value.clone());
                }
            }
            runtime.g.borrow().get_node_property_id(&property).map_or(
                Ok(Value::Null),
                |property_id| {
                    runtime
                        .g
                        .borrow()
                        .get_node_property(node_id, property_id)
                        .map_or(Ok(Value::Null), Ok)
                },
            )
        }
        (Some(Value::Relationship(id, _, _)), Some(Value::String(property)), None) => {
            if let Some(rel) = runtime.pending.borrow().created_relationships.get(&id) {
                if let Some(value) = rel.3.get(&property) {
                    return Ok(value.clone());
                }
            }
            runtime
                .g
                .borrow()
                .get_relationship_property_id(&property)
                .map_or(Ok(Value::Null), |property_id| {
                    runtime
                        .g
                        .borrow()
                        .get_relationship_property(id, property_id)
                        .map_or(Ok(Value::Null), Ok)
                })
        }
        (Some(Value::Map(map)), Some(Value::String(property)), None) => {
            Ok(map.get(&property).unwrap_or(&Value::Null).clone())
        }
        (Some(Value::Null), Some(Value::String(_)), None) => Ok(Value::Null),
        (Some(Value::Map(_)), Some(p), None) => Err(format!(
            "Type mismatch: expected String but was {}",
            p.name()
        )),
        (Some(m), Some(_), None) => Err(format!(
            "Type mismatch: expected Node, Relationship, or Map but was {}",
            m.name()
        )),
        _ => Ok(Value::Null),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn labels(
    runtime: &Runtime,
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
    _runtime: &Runtime,
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
    _runtime: &Runtime,
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
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(a), Some(Value::Null), None) => {
            return Ok(Value::List(vec![a]));
        }
        (Some(a), Some(Value::List(mut l)), None) => {
            l.push(a);
            return Ok(Value::List(l));
        }
        _ => (),
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn count(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(_), Some(Value::Null), None) => {
            return Ok(Value::Int(1));
        }
        (Some(Value::Null), Some(a), None) => {
            return Ok(a);
        }
        (Some(_), Some(Value::Int(a)), None) => {
            return Ok(Value::Int(a + 1));
        }

        _ => (),
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn sum(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::Null), Some(a), None) | (Some(a), Some(Value::Null), None) => {
            return Ok(a);
        }
        (Some(Value::Int(a)), Some(Value::Int(b)), None) => {
            return Ok(Value::Float((a + b) as f64));
        }
        (Some(Value::Float(a)), Some(Value::Float(b)), None) => {
            return Ok(Value::Float(a + b));
        }
        (Some(Value::Int(a)), Some(Value::Float(b)), None) => {
            return Ok(Value::Float(a as f64 + b));
        }
        (Some(Value::Float(a)), Some(Value::Int(b)), None) => {
            return Ok(Value::Float(a + b as f64));
        }
        _ => (),
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn max(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [a, Value::Null] => return Ok(a.clone()),
        [a, b] => {
            if a.partial_cmp(b) == Some(std::cmp::Ordering::Greater) {
                return Ok(a.clone());
            }
            return Ok(b.clone());
        }
        _ => (),
    }
    Ok(Value::Null)
}

#[allow(clippy::unnecessary_wraps)]
fn min(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.as_slice() {
        [a, Value::Null] => return Ok(a.clone()),
        [a, b] => {
            if a.partial_cmp(b) == Some(std::cmp::Ordering::Less) {
                return Ok(a.clone());
            }
            return Ok(b.clone());
        }
        _ => (),
    }
    Ok(Value::Null)
}

fn value_to_integer(
    _runtime: &Runtime,
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
            "Expected one argument for value_to_integer, instead {len}"
        )),
    }
}

fn value_to_string(
    _runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let len = args.len();
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s)),
        Some(arg) => Err(format!(
            "Type mismatch: expected String, Boolean, Integer, Float, or Null but was {}",
            arg.name()
        )),
        _ => Err(format!(
            "Expected one argument for value_to_integer, instead {len}"
        )),
    }
}

fn size(
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(mut v)) => {
            v.reverse();
            Ok(Value::List(v))
        }
        Some(Value::Null) => Ok(Value::Null),
        Some(Value::String(s)) => Ok(Value::String(s.chars().rev().collect())),
        Some(arg) => Err(format!(
            "Type mismatch: expected List, String, or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn substring(
    _: &Runtime,
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
            "Type mismatch: expected Integer but was {}",
            t.name()
        )),
        (Some(t), Some(Value::Int(_)), None | Some(Value::Int(_))) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            t.name()
        )),
        (Some(Value::String(_)), Some(t), Some(Value::Int(_)))
        | (Some(Value::String(_)), Some(Value::Int(_)), Some(t)) => Err(format!(
            "Type mismatch: expected Integer but was {}",
            t.name()
        )),

        _ => unreachable!(),
    }
}

fn split(
    _: &Runtime,
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
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        Some(Value::Null) => Ok(Value::Null),
        Some(arg) => Err(format!(
            "Type mismatch: expected String or Null but was {}",
            arg.name()
        )),
        _ => unreachable!(),
    }
}

fn string_replace(
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    fn to_string_vec(vec: Vec<Value>) -> Result<Vec<String>, String> {
        vec.into_iter()
            .map(|item| {
                if let Value::String(s) = item {
                    Ok(s)
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        None => Ok(Value::Float(std::f64::consts::E)),
        _ => unreachable!(),
    }
}

fn exp(
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
        (Some(Value::Int(_) | Value::Float(_)), Some(v))
        | (Some(v), Some(Value::Int(_) | Value::Float(_))) => Err(format!(
            "Type mismatch: expected Integer, Float, or Null but was {}",
            v.name()
        )),
        _ => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn rand(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    debug_assert!(args.is_empty());
    let mut rng = rand::rng();
    Ok(Value::Float(rng.random_range(0.0..1.0)))
}

fn round(
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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

fn range(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let start = iter.next().ok_or("Missing start value")?;
    let end = iter.next().ok_or("Missing end value")?;
    let step = iter.next().unwrap_or(Value::Int(1));
    match (start, end, step) {
        (Value::Int(start), Value::Int(end), Value::Int(step)) => {
            if start >= end && step < 0 {
                Ok(Value::List(
                    (end..=start)
                        .rev()
                        .step_by(step.abs() as usize)
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


fn coalesce(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let iter = args.into_iter();
    for arg in iter {
        if let Value::Null = arg {
            continue;
        } else {
            return Ok(arg);
        }
    }
    Ok(Value::Null)
}

fn keys(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Map(map)), None) => Ok(Value::List(
            map.keys().map(|k| Value::String(k.to_string())).collect(),
        )),
        (Some(Value::Null), None) => Ok(Value::Null),
        _ => Err("Type mismatch: expected Map or Null".to_string()),
    }
}

//
// Internal functions
//

fn internal_starts_with(
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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
    _: &Runtime,
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

fn internal_case(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::List(alts)), None) => {
            for pair in alts.chunks(2) {
                if let [Value::Bool(true), result] = pair {
                    return Ok(result.clone());
                }
            }
            Ok(Value::Null)
        }
        (Some(value), Some(Value::List(alts))) => {
            for pair in alts.chunks(2) {
                if let [condition, result] = pair {
                    if *condition == value {
                        return Ok(result.clone());
                    }
                }
            }
            Ok(Value::Null)
        }
        _ => unreachable!(),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn db_labels(
    runtime: &Runtime,
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
    runtime: &Runtime,
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
    runtime: &Runtime,
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
