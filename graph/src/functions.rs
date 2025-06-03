#![allow(clippy::unnecessary_wraps)]

use crate::runtime::Runtime;
use crate::value::Value;
use hashbrown::{HashMap, HashSet};
use rand::Rng;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::OnceLock;

type RuntimeFn = fn(&Runtime, Vec<Value>) -> Result<Value, String>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FnType {
    Function,
    Internal,
    Procedure,
    Aggregation,
}

#[derive(Clone, Debug)]
pub enum Type {
    Null,
    Bool,
    Int,
    Float,
    String,
    List(Box<Type>),
    Map,
    Node,
    Relationship,
    Path,
    Any,
    Union(Vec<Type>),
    Optional(Box<Type>),
}

impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Bool => write!(f, "Boolean"),
            Self::Int => write!(f, "Integer"),
            Self::Float => write!(f, "Float"),
            Self::String => write!(f, "String"),
            Self::List(_) => write!(f, "List"),
            Self::Map => write!(f, "Map"),
            Self::Node => write!(f, "Node"),
            Self::Relationship => write!(f, "Relationship"),
            Self::Path => write!(f, "Path"),
            Self::Any => write!(f, "Any"),
            Self::Union(types) => {
                let mut iter = types.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;
                }
                for _ in 0..types.len().saturating_sub(2) {
                    if let Some(next) = iter.next() {
                        write!(f, ", {next}")?;
                    }
                }
                if let Some(last) = iter.next() {
                    if types.len() > 2 {
                        write!(f, ",")?;
                    }
                    write!(f, " or {last}")?;
                }
                Ok(())
            }
            Self::Optional(inner) => write!(f, "{inner}"),
        }
    }
}

#[derive(Debug)]
pub enum FnArguments {
    Fixed(Vec<Type>),
    VarLength(Type),
}

#[derive(Debug)]
pub struct GraphFn {
    pub name: String,
    pub func: RuntimeFn,
    pub write: bool,
    pub args_type: FnArguments,
    pub fn_type: FnType,
}

impl GraphFn {
    #[must_use]
    pub fn new(
        name: &str,
        func: RuntimeFn,
        write: bool,
        args_type: FnArguments,
        fn_type: FnType,
    ) -> Self {
        Self {
            name: String::from(name),
            func,
            write,
            args_type,
            fn_type,
        }
    }
}

impl GraphFn {
    pub fn validate_args_type(
        &self,
        args: &Vec<Value>,
    ) -> Result<(), String> {
        match &self.args_type {
            FnArguments::Fixed(args_type) => {
                for (i, arg_type) in args_type.iter().enumerate() {
                    if i >= args.len() {
                        if !matches!(arg_type, Type::Optional(_)) {
                            return Err(format!(
                                "Missing argument {} for function '{}', expected type {:?}",
                                i + 1,
                                self.name,
                                arg_type
                            ));
                        }
                    } else if let Some((actual, expected)) = args[i].validate_of_type(arg_type) {
                        return Err(format!(
                            "Type mismatch: expected {expected} but was {actual}"
                        ));
                    }
                }
            }
            FnArguments::VarLength(_) => {}
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Functions {
    functions: HashMap<String, GraphFn>,
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
        args_type: Vec<Type>,
        fn_type: FnType,
    ) {
        let lower_name = name.to_lowercase();
        assert!(
            !self.functions.contains_key(&lower_name),
            "Function '{name}' already exists"
        );
        let graph_fn = GraphFn::new(name, func, write, FnArguments::Fixed(args_type), fn_type);
        self.functions.insert(lower_name, graph_fn);
    }

    pub fn add_var_len(
        &mut self,
        name: &str,
        func: RuntimeFn,
        write: bool,
        arg_type: Type,
        fn_type: FnType,
    ) {
        let name = name.to_lowercase();
        assert!(
            !self.functions.contains_key(&name),
            "Function '{name}' already exists"
        );
        let graph_fn = GraphFn::new(
            &name,
            func,
            write,
            FnArguments::VarLength(arg_type),
            fn_type,
        );
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
                match &graph_fn.args_type {
                    FnArguments::Fixed(args_type) => {
                        let least = args_type
                            .iter()
                            .filter(|x| !matches!(x, Type::Optional(_)))
                            .count();
                        if args < least {
                            return Err(format!(
                                "Received {args} arguments to function '{}', expected at least {least}", graph_fn.name
                            ));
                        }
                        let most = if fn_type == &FnType::Aggregation {
                            args_type.len() + 1 // aggregation functions have one more argument for the temporary result
                        } else {
                            args_type.len()
                        };
                        if args > most {
                            return Err(format!(
                                "Received {} arguments to function '{}', expected at most {}",
                                args,
                                graph_fn.name,
                                args_type.len()
                            ));
                        }
                    }
                    FnArguments::VarLength(_) => {}
                }
                Ok(())
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

    funcs.add(
        "property",
        property,
        false,
        vec![
            Type::Union(vec![Type::Node, Type::Relationship, Type::Map, Type::Null]),
            Type::String,
        ],
        FnType::Internal,
    );

    funcs.add(
        "labels",
        labels,
        false,
        vec![Type::Union(vec![Type::Node, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "startnode",
        start_node,
        false,
        vec![Type::Relationship],
        FnType::Function,
    );
    funcs.add(
        "endnode",
        end_node,
        false,
        vec![Type::Relationship],
        FnType::Function,
    );
    funcs.add(
        "tointeger",
        value_to_integer,
        false,
        vec![Type::Union(vec![
            Type::String,
            Type::Bool,
            Type::Int,
            Type::Float,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "tofloat",
        value_to_float,
        false,
        vec![Type::Union(vec![
            Type::String,
            Type::Float,
            Type::Int,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "tostring",
        value_to_string,
        false,
        vec![Type::Union(vec![
            Type::String,
            Type::Int,
            Type::Bool,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "size",
        size,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "head",
        head,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "last",
        last,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "tail",
        tail,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "reverse",
        reverse,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "substring",
        substring,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Int,
            Type::Optional(Box::new(Type::Int)),
        ],
        FnType::Function,
    );
    funcs.add(
        "split",
        split,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "tolower",
        string_to_lower,
        false,
        vec![Type::Union(vec![Type::String, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "toupper",
        string_to_upper,
        false,
        vec![Type::Union(vec![Type::String, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "replace",
        string_replace,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "left",
        string_left,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::Int, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "ltrim",
        string_ltrim,
        false,
        vec![Type::Union(vec![Type::String, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "right",
        string_right,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::Int, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "string.join",
        string_join,
        false,
        vec![
            Type::Union(vec![Type::List(Box::new(Type::Any)), Type::Null]),
            Type::Optional(Box::new(Type::Union(vec![Type::String, Type::Null]))),
        ],
        FnType::Function,
    );
    funcs.add(
        "string.matchRegEx",
        string_match_reg_ex,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "string.replaceRegEx",
        string_replace_reg_ex,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "abs",
        abs,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "ceil",
        ceil,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add("e", e, false, vec![], FnType::Function);
    funcs.add(
        "exp",
        exp,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "floor",
        floor,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "log",
        log,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "log10",
        log10,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "pow",
        pow,
        false,
        vec![
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add("rand", rand, false, vec![], FnType::Function);
    funcs.add(
        "round",
        round,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "sign",
        sign,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "sqrt",
        sqrt,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "range",
        range,
        false,
        vec![Type::Int, Type::Int, Type::Optional(Box::new(Type::Int))],
        FnType::Function,
    );
    funcs.add_var_len("coalesce", coalesce, false, Type::Any, FnType::Function);
    funcs.add(
        "keys",
        keys,
        false,
        vec![Type::Union(vec![Type::Map, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "toBoolean",
        to_boolean,
        false,
        vec![Type::Union(vec![
            Type::Bool,
            Type::String,
            Type::Int,
            Type::Null,
        ])],
        FnType::Function,
    );

    // aggregation functions
    funcs.add(
        "collect",
        collect,
        false,
        vec![Type::Any],
        FnType::Aggregation,
    );
    funcs.add(
        "count",
        count,
        false,
        vec![Type::Optional(Box::new(Type::Any))],
        FnType::Aggregation,
    );
    funcs.add("sum", sum, false, vec![Type::Any], FnType::Aggregation);
    funcs.add("max", max, false, vec![Type::Any], FnType::Aggregation);
    funcs.add("min", min, false, vec![Type::Any], FnType::Aggregation);

    // Internal functions
    funcs.add(
        "starts_with",
        internal_starts_with,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Internal,
    );
    funcs.add(
        "ends_with",
        internal_ends_with,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Internal,
    );
    funcs.add(
        "contains",
        internal_contains,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Internal,
    );
    funcs.add(
        "is_null",
        internal_is_null,
        false,
        vec![Type::Union(vec![Type::Bool]), Type::Any],
        FnType::Internal,
    );
    funcs.add(
        "node_has_labels",
        internal_node_has_labels,
        false,
        vec![Type::Node, Type::List(Box::new(Type::Any))],
        FnType::Internal,
    );
    funcs.add(
        "regex_matches",
        internal_regex_matches,
        false,
        vec![
            Type::Union(vec![Type::String, Type::Null]),
            Type::Union(vec![Type::String, Type::Null]),
        ],
        FnType::Internal,
    );
    funcs.add(
        "case",
        internal_case,
        false,
        vec![Type::Any, Type::Optional(Box::new(Type::Any))],
        FnType::Internal,
    );

    // Procedures
    funcs.add("db.labels", db_labels, false, vec![], FnType::Procedure);
    funcs.add(
        "db.relationshiptypes",
        db_types,
        false,
        vec![],
        FnType::Procedure,
    );
    funcs.add(
        "db.propertykeys",
        db_properties,
        false,
        vec![],
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
        _ => unreachable!(),
    }
}

fn labels(
    runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), None) => {
            if runtime
                .pending
                .borrow()
                .created_nodes
                .contains_key(&node_id)
            {
                return Ok(Value::List(
                    runtime
                        .pending
                        .borrow()
                        .created_nodes
                        .get(&node_id)
                        .unwrap()
                        .0
                        .iter()
                        .map(|label| Value::String(label.clone()))
                        .collect(),
                ));
            }
            Ok(Value::List(
                runtime
                    .g
                    .borrow()
                    .get_node_label_ids(node_id)
                    .map(|label_id| Value::String(runtime.g.borrow().get_label_by_id(label_id)))
                    .collect(),
            ))
        }
        (Some(Value::Null), None) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn start_node(
    _runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Relationship(_, src, _)), None) => Ok(Value::Node(src)),
        _ => unreachable!(),
    }
}

fn end_node(
    _runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Relationship(_, _, dest)), None) => Ok(Value::Node(dest)),
        _ => unreachable!(),
    }
}

fn collect(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(a), Some(Value::Null), None) => {
            return Ok(Value::List(vec![a]));
        }
        (Some(Value::Null), Some(Value::List(a)), None) => {
            return Ok(Value::List(a));
        }
        (Some(a), Some(Value::List(mut l)), None) => {
            l.push(a);
            return Ok(Value::List(l));
        }
        _ => (),
    }
    Ok(Value::Null)
}

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
        (Some(Value::Null), None, None) => {
            return Ok(Value::Int(1));
        }
        (Some(Value::Int(a)), None, None) => {
            return Ok(Value::Int(a + 1));
        }
        _ => (),
    }
    Ok(Value::Null)
}

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
    match args.into_iter().next() {
        Some(Value::String(s)) => s.parse::<i64>().map(Value::Int).or_else(|_| {
            s.parse::<f64>()
                .map(|f| Value::Int(f as i64))
                .or(Ok(Value::Null))
        }),
        Some(v @ Value::Int(_)) => Ok(v),
        Some(Value::Float(f)) => Ok(Value::Int(f as i64)),
        Some(Value::Bool(b)) => Ok(Value::Int(i64::from(b))),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn value_to_float(
    _runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => s.parse::<f64>().map(Value::Float).or(Ok(Value::Null)),
        Some(v @ Value::Float(_)) => Ok(v),
        Some(Value::Int(i)) => Ok(Value::Float(i as f64)),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn value_string(value: &Value) -> Result<Rc<String>, String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Int(i) => Ok(Rc::new(i.to_string())),
        Value::Bool(b) => Ok(Rc::new(String::from(if *b { "true" } else { "false" }))),
        _ => unreachable!(),
    }
}

fn value_to_string(
    _runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Null) => Ok(Value::Null),
        Some(v) => Ok(Value::String(value_string(&v)?)),
        _ => unreachable!(),
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
        Some(Value::String(s)) => Ok(Value::String(Rc::new(s.chars().rev().collect()))),
        Some(Value::Null) => Ok(Value::Null),
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

            Ok(Value::String(Rc::new(String::from(&s[start..]))))
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
            Ok(Value::String(Rc::new(String::from(&s[start..end]))))
        }
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
                    .map(|c| Value::String(Rc::new(String::from(c))))
                    .collect();
                Ok(Value::List(parts))
            } else {
                let parts: Vec<Value> = string
                    .split(delimiter.as_str())
                    .map(|s| Value::String(Rc::new(String::from(s))))
                    .collect();
                Ok(Value::List(parts))
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn string_to_lower(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Rc::new(s.to_lowercase()))),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Rc::new(s.to_uppercase()))),
        Some(Value::Null) => Ok(Value::Null),
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
            Ok(Value::String(Rc::new(
                s.replace(search.as_str(), replacement.as_str()),
            )))
        }
        (Some(Value::Null), _, _) | (_, Some(Value::Null), _) | (_, _, Some(Value::Null)) => {
            Ok(Value::Null)
        }
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
                Err(String::from("length must be a non-negative integer"))
            } else {
                Ok(Value::String(Rc::new(s.chars().take(n as usize).collect())))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),
        _ => unreachable!(),
    }
}

fn string_ltrim(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Rc::new(String::from(s.trim_start())))),
        Some(Value::Null) => Ok(Value::Null),
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
                Err(String::from("length must be a non-negative integer"))
            } else {
                let start = s.len().saturating_sub(n as usize);
                Ok(Value::String(Rc::new(s.chars().skip(start).collect())))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),
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
                    Ok((*s).clone())
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
            result.map(|strings| Value::String(Rc::new(strings.join(s.as_str()))))
        }
        (Some(Value::List(vec)), None) => {
            let result = to_string_vec(vec);
            result.map(|strings| Value::String(Rc::new(strings.join(""))))
        }
        (Some(Value::Null), _) => Ok(Value::Null),
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
                                all_matches.push(Value::String(Rc::new(String::from(m.as_str()))));
                            }
                        }
                    }
                    Ok(Value::List(all_matches))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::List(vec![])),
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
                let replaced_text = re
                    .replace_all(text.as_str(), replacement.as_str())
                    .into_owned();
                Ok(Value::String(Rc::new(replaced_text)))
            }
            Err(e) => Err(format!("Invalid regex, {e}")),
        },
        (Some(Value::Null), Some(_), Some(_))
        | (Some(_), Some(Value::Null), Some(_))
        | (Some(_), Some(_), Some(Value::Null)) => Ok(Value::Null),
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
        _ => unreachable!(),
    }
}

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
        _ => unreachable!(),
    }
}
fn coalesce(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let iter = args.into_iter();
    for arg in iter {
        if arg == Value::Null {
            continue;
        }
        return Ok(arg);
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
            map.keys().map(|k| Value::String(k.clone())).collect(),
        )),
        (Some(Value::Null), None) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn to_boolean(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Bool(b)) => Ok(Value::Bool(b)),
        Some(Value::String(s)) => {
            if s.eq_ignore_ascii_case("true") {
                Ok(Value::Bool(true))
            } else if s.eq_ignore_ascii_case("false") {
                Ok(Value::Bool(false))
            } else {
                Ok(Value::Null)
            }
        }
        Some(Value::Int(n)) => Ok(Value::Bool(n != 0)),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
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
        _ => unreachable!(),
    }
}

fn internal_is_null(
    _: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Bool(is_not)), Some(Value::Null)) => Ok(Value::Bool(!is_not)),
        (Some(Value::Bool(is_not)), Some(_)) => Ok(Value::Bool(is_not)),
        _ => unreachable!(),
    }
}

fn internal_node_has_labels(
    runtime: &Runtime,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(node_id)), Some(Value::List(required_labels))) => {
            let actual_labels = runtime
                .g
                .borrow()
                .get_node_labels(node_id)
                .collect::<HashSet<_>>();
            let all_labels_present = required_labels.iter().all(|label| {
                if let Value::String(label) = label {
                    actual_labels.contains(label)
                } else {
                    false
                }
            });

            Ok(Value::Bool(all_labels_present))
        }
        (Some(n), Some(l)) => Err(format!(
            "Type mismatch: expected Node and Labels Null but was ({}, {})",
            n.name(),
            l.name()
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
                match pair {
                    [Value::Bool(false) | Value::Null, _] => {}
                    [_, result] => return Ok(result.clone()),
                    _ => unreachable!(),
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

fn db_labels(
    runtime: &Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_labels()
            .map(|n| Value::String(n.clone()))
            .collect(),
    ))
}

fn db_types(
    runtime: &Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_types()
            .map(|n| Value::String(n.clone()))
            .collect(),
    ))
}

fn db_properties(
    runtime: &Runtime,
    _args: Vec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_properties()
            .map(|n| Value::String(n.clone()))
            .collect(),
    ))
}
