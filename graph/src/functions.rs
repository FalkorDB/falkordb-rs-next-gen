#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

use crate::runtime::{PendingRelationship, Runtime};
use crate::value::{RcValue, Value};
use hashbrown::HashMap;
use ordermap::OrderSet;
use rand::Rng;
use std::fmt::Display;
use std::iter::repeat_with;
use std::rc::Rc;
use std::sync::OnceLock;

type RuntimeFn = fn(&Runtime, Vec<RcValue>) -> Result<RcValue, String>;

#[derive(Debug)]
pub enum FnType {
    Function,
    Internal,
    Procedure,
    Aggregation(RcValue),
}

impl PartialEq for FnType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        matches!(
            (self, other),
            (Self::Function, Self::Function)
                | (Self::Internal, Self::Internal)
                | (Self::Procedure, Self::Procedure)
                | (Self::Aggregation(_), Self::Aggregation(_))
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
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

    #[must_use]
    pub const fn is_aggregate(&self) -> bool {
        matches!(self.fn_type, FnType::Aggregation(_))
    }

    pub fn validate(
        &self,
        args: usize,
    ) -> Result<(), String> {
        match &self.args_type {
            FnArguments::Fixed(args_type) => {
                let least = args_type
                    .iter()
                    .filter(|x| !matches!(x, Type::Optional(_)))
                    .count();
                if args < least {
                    return Err(format!(
                        "Received {args} arguments to function '{}', expected at least {least}",
                        self.name
                    ));
                }
                let most = match self.fn_type {
                    FnType::Aggregation(_) => args_type.len() + 1,
                    _ => args_type.len(),
                };
                if args > most {
                    return Err(format!(
                        "Received {} arguments to function '{}', expected at most {}",
                        args,
                        self.name,
                        args_type.len()
                    ));
                }
            }
            FnArguments::VarLength(_) => {}
        }
        Ok(())
    }
}

impl GraphFn {
    pub fn validate_args_type(
        &self,
        args: &[RcValue],
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
    functions: HashMap<String, Rc<GraphFn>>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Functions {}
unsafe impl Sync for Functions {}

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
        let graph_fn = Rc::new(GraphFn::new(
            name,
            func,
            write,
            FnArguments::Fixed(args_type),
            fn_type,
        ));
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
        let graph_fn = Rc::new(GraphFn::new(
            &name,
            func,
            write,
            FnArguments::VarLength(arg_type),
            fn_type,
        ));
        self.functions.insert(name, graph_fn);
    }

    #[must_use]
    pub fn get(
        &self,
        name: &str,
        fn_type: &FnType,
    ) -> Result<Rc<GraphFn>, String> {
        self.functions
            .get(name.to_lowercase().as_str())
            .and_then(|graph_fn| {
                if &graph_fn.fn_type == fn_type {
                    Some(graph_fn.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| format!("Unknown function '{name}'"))
    }

    #[must_use]
    pub fn is_aggregate(
        &self,
        name: &str,
    ) -> bool {
        self.functions
            .get(name)
            .is_some_and(|graph_fn| matches!(graph_fn.fn_type, FnType::Aggregation(_)))
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
            Type::Union(vec![Type::List(Box::new(Type::String)), Type::Null]),
            Type::Optional(Box::new(Type::String)),
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
        vec![Type::Union(vec![
            Type::Map,
            Type::Node,
            Type::Relationship,
            Type::Null,
        ])],
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
    funcs.add(
        "type",
        relationship_type,
        false,
        vec![Type::Relationship],
        FnType::Function,
    );
    funcs.add("nodes", nodes, false, vec![Type::Path], FnType::Function);
    // aggregation functions
    funcs.add(
        "collect",
        collect,
        false,
        vec![Type::Any],
        FnType::Aggregation(RcValue::list(vec![])),
    );
    funcs.add(
        "count",
        count,
        false,
        vec![Type::Optional(Box::new(Type::Any))],
        FnType::Aggregation(RcValue::int(0)),
    );
    funcs.add(
        "sum",
        sum,
        false,
        vec![Type::Any],
        FnType::Aggregation(RcValue::float(0.0)),
    );
    funcs.add(
        "max",
        max,
        false,
        vec![Type::Any],
        FnType::Aggregation(RcValue::null()),
    );
    funcs.add(
        "min",
        min,
        false,
        vec![Type::Any],
        FnType::Aggregation(RcValue::null()),
    );

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
        "node_set_labels",
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
        vec![
            Type::Any,
            Type::Optional(Box::new(Type::Any)),
            Type::Optional(Box::new(Type::Any)),
        ],
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
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::Node(node_id)), Some(Value::String(property))) => {
            if let Some(attrs) = runtime.pending.borrow().set_nodes_attrs.get(node_id) {
                if let Some(value) = attrs.get(property) {
                    return Ok(value.clone());
                }
            }
            runtime.g.borrow().get_node_attribute_id(property).map_or(
                Ok(RcValue::null()),
                |property_id| {
                    runtime
                        .g
                        .borrow()
                        .get_node_attribute(*node_id, property_id)
                        .map_or(Ok(RcValue::null()), Ok)
                },
            )
        }
        (Some(Value::Relationship(id, _, _)), Some(Value::String(property))) => {
            if let Some(attrs) = runtime.pending.borrow().set_relationships_attrs.get(id) {
                if let Some(value) = attrs.get(property) {
                    return Ok(value.clone());
                }
            }
            runtime
                .g
                .borrow()
                .get_relationship_attribute_id(property)
                .map_or(Ok(RcValue::null()), |property_id| {
                    runtime
                        .g
                        .borrow()
                        .get_relationship_attribute(*id, property_id)
                        .map_or(Ok(RcValue::null()), Ok)
                })
        }
        (Some(Value::Map(map)), Some(Value::String(property))) => {
            Ok(map.get(property).cloned().unwrap_or_else(RcValue::null))
        }
        (Some(Value::Null), Some(Value::String(_))) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn labels(
    runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Node(node_id)) => {
            let mut labels = runtime
                .g
                .borrow()
                .get_node_labels(*node_id)
                .collect::<OrderSet<_>>();
            labels.extend(
                runtime
                    .pending
                    .borrow()
                    .set_node_labels
                    .get(node_id)
                    .cloned()
                    .unwrap_or_else(OrderSet::new),
            );
            if let Some(remove_labels) = runtime.pending.borrow().remove_node_labels.get(node_id) {
                for label in remove_labels {
                    labels.remove(label);
                }
            }
            Ok(RcValue::list(
                labels.into_iter().map(RcValue::string).collect(),
            ))
        }
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn start_node(
    _runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match iter.next().as_deref() {
        Some(Value::Relationship(_, src, _)) => Ok(RcValue::node(*src)),
        _ => unreachable!(),
    }
}

fn end_node(
    _runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match iter.next().as_deref() {
        Some(Value::Relationship(_, _, dest)) => Ok(RcValue::node(*dest)),
        _ => unreachable!(),
    }
}

fn collect(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next().as_deref()) {
        (Some(a), Some(Value::Null)) => Ok(RcValue::list(vec![a])),
        (Some(a), Some(Value::List(l))) => {
            let mut l = l.clone();
            if *a == Value::Null {
                return Ok(RcValue::list(l));
            }
            l.push(a);
            Ok(RcValue::list(l))
        }
        _ => unreachable!(),
    }
}

fn count(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    let first = iter.next();
    let sec = iter.next();
    match (first.as_deref(), sec.as_deref()) {
        (Some(Value::Null), Some(_)) => Ok(sec.unwrap()),
        (Some(_), Some(Value::Int(a))) | (Some(Value::Int(a)), None) => Ok(RcValue::int(a + 1)),
        _ => unreachable!(),
    }
}

fn sum(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::Null), Some(Value::Int(a))) => Ok(RcValue::float(*a as f64)),
        (Some(Value::Null), Some(Value::Float(a))) => Ok(RcValue::float(*a)),
        (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(RcValue::float((a + b) as f64)),
        (Some(Value::Float(a)), Some(Value::Float(b))) => Ok(RcValue::float(a + b)),
        (Some(Value::Int(a)), Some(Value::Float(b))) => Ok(RcValue::float(*a as f64 + b)),
        (Some(Value::Float(a)), Some(Value::Int(b))) => Ok(RcValue::float(a + *b as f64)),
        _ => unreachable!(),
    }
}

fn max(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => {
            if *b == Value::Null {
                return Ok(a);
            }
            if a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater) {
                return Ok(a);
            }
            Ok(b)
        }
        _ => unreachable!(),
    }
}

fn min(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => {
            if *b == Value::Null {
                return Ok(a);
            }
            if a.partial_cmp(&b) == Some(std::cmp::Ordering::Less) {
                return Ok(a);
            }
            Ok(b)
        }
        _ => unreachable!(),
    }
}

fn value_to_integer(
    _runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => s.parse::<i64>().map(RcValue::int).or_else(|_| {
            s.parse::<f64>()
                .map(|f| RcValue::int(f.floor() as i64))
                .or(Ok(RcValue::null()))
        }),
        Some(Value::Int(i)) => Ok(RcValue::int(*i)),
        Some(Value::Float(f)) => Ok(RcValue::int(f.floor() as i64)),
        Some(Value::Bool(b)) => Ok(RcValue::int(i64::from(*b))),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn value_to_float(
    _runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => s.parse::<f64>().map(RcValue::float).or(Ok(RcValue::null())),
        Some(Value::Float(f)) => Ok(RcValue::float(*f)),
        Some(Value::Int(i)) => Ok(RcValue::float(*i as f64)),
        Some(Value::Null) => Ok(RcValue::null()),
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
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Null) => Ok(RcValue::null()),
        Some(v) => Ok(RcValue::string(value_string(v)?)),
        _ => unreachable!(),
    }
}

fn size(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => Ok(RcValue::int(s.len() as i64)),
        Some(Value::List(v)) => Ok(RcValue::int(v.len() as i64)),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn head(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::List(v)) => {
            if v.is_empty() {
                Ok(RcValue::null())
            } else {
                Ok(v[0].clone())
            }
        }
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn last(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::List(v)) => Ok(v.last().cloned().unwrap_or_else(RcValue::null)),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn tail(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::List(v)) => {
            if v.is_empty() {
                Ok(RcValue::list(vec![]))
            } else {
                Ok(RcValue::list(v[1..].to_vec()))
            }
        }
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn reverse(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::List(v)) => {
            let mut v = v.clone();
            v.reverse();
            Ok(RcValue::list(v))
        }
        Some(Value::String(s)) => Ok(RcValue::string(Rc::new(s.chars().rev().collect()))),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn substring(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (
        iter.next().as_deref(),
        iter.next().as_deref(),
        iter.next().as_deref(),
    ) {
        // Handle NULL input case
        (Some(Value::Null), _, _) => Ok(RcValue::null()),
        // Two-argument version: (string, start)
        (Some(Value::String(s)), Some(Value::Int(start)), None) => {
            if *start < 0 {
                return Err("start must be a non-negative integer".into());
            }
            if start >= &(s.len() as _) {
                return Ok(RcValue::string(Rc::new(String::new())));
            }
            let start = *start as usize;

            Ok(RcValue::string(Rc::new(String::from(&s[start..]))))
        }

        // Three-argument version: (string, start, length)
        (Some(Value::String(s)), Some(Value::Int(start)), Some(Value::Int(length))) => {
            if *start < 0 {
                return Err("start must be a non-negative integer".into());
            }

            let start = *start as usize;
            if start >= s.len() {
                return Ok(RcValue::string(Rc::new(String::new())));
            }

            if *length < 0 {
                return Err("length must be a non-negative integer".into());
            }

            let length = *length as usize;

            let end = start.saturating_add(length).min(s.len());
            Ok(RcValue::string(Rc::new(String::from(&s[start..end]))))
        }
        _ => unreachable!(),
    }
}

fn split(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(string)), Some(Value::String(delimiter))) => {
            if string.is_empty() {
                Ok(RcValue::list(vec![RcValue::string(Rc::new(String::new()))]))
            } else if delimiter.is_empty() {
                // split string to characters
                let parts = string
                    .chars()
                    .map(|c| RcValue::string(Rc::new(String::from(c))))
                    .collect();
                Ok(RcValue::list(parts))
            } else {
                let parts = string
                    .split(delimiter.as_str())
                    .map(|s| RcValue::string(Rc::new(String::from(s))))
                    .collect();
                Ok(RcValue::list(parts))
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn string_to_lower(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => Ok(RcValue::string(Rc::new(s.to_lowercase()))),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => Ok(RcValue::string(Rc::new(s.to_uppercase()))),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn string_replace(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (
        iter.next().as_deref(),
        iter.next().as_deref(),
        iter.next().as_deref(),
    ) {
        (Some(Value::String(s)), Some(Value::String(search)), Some(Value::String(replacement))) => {
            Ok(RcValue::string(Rc::new(
                s.replace(search.as_str(), replacement.as_str()),
            )))
        }
        (Some(Value::Null), _, _) | (_, Some(Value::Null), _) | (_, _, Some(Value::Null)) => {
            Ok(RcValue::null())
        }
        _ => unreachable!(),
    }
}

fn string_left(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if *n < 0 {
                Err(String::from("length must be a non-negative integer"))
            } else {
                Ok(RcValue::string(Rc::new(
                    s.chars().take(*n as usize).collect(),
                )))
            }
        }
        (Some(Value::Null), _) => Ok(RcValue::null()),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),
        _ => unreachable!(),
    }
}

fn string_ltrim(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::String(s)) => Ok(RcValue::string(Rc::new(String::from(
            s.trim_start_matches(' '),
        )))),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn string_right(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if *n < 0 {
                Err(String::from("length must be a non-negative integer"))
            } else {
                let start = s.len().saturating_sub(*n as usize);
                Ok(RcValue::string(Rc::new(s.chars().skip(start).collect())))
            }
        }
        (Some(Value::Null), _) => Ok(RcValue::null()),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),
        _ => unreachable!(),
    }
}

fn string_join(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    fn to_string_vec(vec: &[RcValue]) -> Result<Vec<Rc<String>>, String> {
        vec.iter()
            .map(|item| {
                if let Value::String(s) = &**item {
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
    let first = iter.next().unwrap();
    match (&*first, iter.next()) {
        (Value::List(vec), Some(a)) => match &*a {
            Value::String(s) => {
                let result = to_string_vec(vec);
                result.map(|strings| {
                    RcValue::string(Rc::new(
                        strings
                            .iter()
                            .map(|label| label.as_str())
                            .collect::<Vec<_>>()
                            .join(s.as_str()),
                    ))
                })
            }
            _ => unreachable!(),
        },
        (Value::List(vec), None) => {
            let result = to_string_vec(vec);
            result.map(|strings| {
                RcValue::string(Rc::new(
                    strings
                        .iter()
                        .map(|label| label.as_str())
                        .collect::<Vec<_>>()
                        .join(""),
                ))
            })
        }
        (Value::Null, _) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn string_match_reg_ex(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(text)), Some(Value::String(pattern))) => {
            match regex::Regex::new(pattern.as_str()) {
                Ok(re) => {
                    let mut all_matches = Vec::new();
                    for caps in re.captures_iter(text.as_str()) {
                        for i in 0..caps.len() {
                            if let Some(m) = caps.get(i) {
                                all_matches
                                    .push(RcValue::string(Rc::new(String::from(m.as_str()))));
                            }
                        }
                    }
                    Ok(RcValue::list(all_matches))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(RcValue::list(vec![])),
        _ => unreachable!(),
    }
}

fn string_replace_reg_ex(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (
        iter.next().as_deref(),
        iter.next().as_deref(),
        iter.next().as_deref(),
    ) {
        (
            Some(Value::String(text)),
            Some(Value::String(pattern)),
            Some(Value::String(replacement)),
        ) => match regex::Regex::new(pattern.as_str()) {
            Ok(re) => {
                let replaced_text = re
                    .replace_all(text.as_str(), replacement.as_str())
                    .into_owned();
                Ok(RcValue::string(Rc::new(replaced_text)))
            }
            Err(e) => Err(format!("Invalid regex, {e}")),
        },
        (Some(Value::Null), Some(_), Some(_))
        | (Some(_), Some(Value::Null), Some(_))
        | (Some(_), Some(_), Some(Value::Null)) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn abs(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::int(n.abs())),
        Some(Value::Float(f)) => Ok(RcValue::float(f.abs())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn ceil(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::int(*n)),
        Some(Value::Float(f)) => Ok(RcValue::float(f.ceil())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn e(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        None => Ok(RcValue::float(std::f64::consts::E)),
        _ => unreachable!(),
    }
}

fn exp(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::float((*n as f64).exp())),
        Some(Value::Float(f)) => Ok(RcValue::float(f.exp())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn floor(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::int(*n)),
        Some(Value::Float(f)) => Ok(RcValue::float(f.floor())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn log(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::float((*n as f64).ln())),
        Some(Value::Float(f)) => Ok(RcValue::float(f.ln())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn log10(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::float((*n as f64).log10())),
        Some(Value::Float(f)) => Ok(RcValue::float(f.log10())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn pow(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::Int(i1)), Some(Value::Int(i2))) => {
            Ok(RcValue::float((*i1 as f64).powi(*i2 as i32)))
        }
        (Some(Value::Float(f1)), Some(Value::Float(f2))) => Ok(RcValue::float(f1.powf(*f2))),
        (Some(Value::Int(i1)), Some(Value::Float(f1))) => {
            Ok(RcValue::float((*i1 as f64).powf(*f1)))
        }
        (Some(Value::Float(f1)), Some(Value::Int(i1))) => Ok(RcValue::float(f1.powi(*i1 as i32))),
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn rand(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    debug_assert!(args.is_empty());
    let mut rng = rand::rng();
    Ok(RcValue::float(rng.random_range(0.0..1.0)))
}

fn round(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::int(*n)),
        Some(Value::Float(f)) => Ok(RcValue::float(f.round())),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn sign(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => Ok(RcValue::int(n.signum())),
        Some(Value::Float(f)) => Ok(if *f == 0.0 {
            RcValue::int(0)
        } else {
            RcValue::float(f.signum().round())
        }),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn sqrt(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Int(n)) => {
            if *n < 0 {
                Ok(RcValue::float(f64::NAN))
            } else {
                Ok(RcValue::float((*n as f64).sqrt()))
            }
        }
        Some(Value::Float(f)) => {
            if *f > 0f64 {
                Ok(RcValue::float(f.sqrt()))
            } else {
                Ok(RcValue::float(f64::NAN))
            }
        }
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn range(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    let start = iter.next().ok_or("Missing start value")?;
    let end = iter.next().ok_or("Missing end value")?;
    let step = iter.next().unwrap_or_else(|| RcValue::int(1));
    match (&*start, &*end, &*step) {
        (Value::Int(start), Value::Int(end), Value::Int(step)) => {
            if *step == 0 {
                return Err(String::from(
                    "ArgumentError: step argument to range() can't be 0",
                ));
            }
            if (start > end && step > &0) || (start < end && step < &0) {
                return Ok(RcValue::list(vec![]));
            }
            let mut curr = *start;
            let step = *step;
            Ok(RcValue::list(
                repeat_with(move || {
                    let tmp = curr;
                    curr += step;
                    RcValue::int(tmp)
                })
                .take(((end - start) / step + 1) as usize)
                .collect(),
            ))
        }
        _ => unreachable!(),
    }
}

fn coalesce(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let iter = args.into_iter();
    for arg in iter {
        if *arg == Value::Null {
            continue;
        }
        return Ok(arg);
    }
    Ok(RcValue::null())
}

fn keys(
    runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Map(map)) => Ok(RcValue::list(
            map.keys().map(|k| RcValue::string(k.clone())).collect(),
        )),
        Some(Value::Node(id)) => {
            let g = runtime.g.borrow();
            let mut actual: OrderSet<Rc<String>> = g
                .get_node_attrs(*id)
                .keys()
                .map(|k| g.get_node_attribute_string(*k).unwrap())
                .collect();
            if let Some(attrs) = runtime.pending.borrow().set_nodes_attrs.get(id) {
                for (k, v) in attrs {
                    if matches!(&**v, Value::Null) {
                        actual.remove(k);
                    } else {
                        actual.insert(k.clone());
                    }
                }
            }
            Ok(RcValue::list(
                actual.into_iter().map(|s| RcValue::string(s)).collect(),
            ))
        }
        Some(Value::Relationship(id, _, _)) => {
            let g = runtime.g.borrow();
            let mut actual: OrderSet<Rc<String>> = g
                .get_relationship_attrs(*id)
                .keys()
                .map(|k| g.get_relationship_attribute_string(*k).unwrap())
                .collect();
            if let Some(attrs) = runtime.pending.borrow().set_relationships_attrs.get(id) {
                for (k, v) in attrs {
                    if matches!(&**v, Value::Null) {
                        actual.remove(k);
                    } else {
                        actual.insert(k.clone());
                    }
                }
            }
            Ok(RcValue::list(
                actual.into_iter().map(|s| RcValue::string(s)).collect(),
            ))
        }

        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn to_boolean(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    match args.into_iter().next().as_deref() {
        Some(Value::Bool(b)) => Ok(RcValue::bool(*b)),
        Some(Value::String(s)) => {
            if s.eq_ignore_ascii_case("true") {
                Ok(RcValue::bool(true))
            } else if s.eq_ignore_ascii_case("false") {
                Ok(RcValue::bool(false))
            } else {
                Ok(RcValue::null())
            }
        }
        Some(Value::Int(n)) => Ok(RcValue::bool(*n != 0)),
        Some(Value::Null) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn relationship_type(
    runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match iter.next().as_deref() {
        Some(Value::Relationship(id, _from, _to)) => {
            if let Some(PendingRelationship { type_name, .. }) =
                runtime.pending.borrow().created_relationships.get(id)
            {
                return Ok(RcValue::string(type_name.clone()));
            }
            let relation_type_id = runtime.g.borrow().get_relationship_type_id(*id);
            runtime
                .g
                .borrow()
                .get_types()
                .nth(relation_type_id as usize)
                .map(|type_name| RcValue::string(type_name.clone()))
                .ok_or_else(|| String::from("Relationship type not found"))
        }
        _ => unreachable!(),
    }
}

fn nodes(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match iter.next().as_deref() {
        Some(Value::Path(_values)) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

//
// Internal functions
//

fn internal_starts_with(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::String(prefix))) => {
            Ok(RcValue::bool(s.starts_with(prefix.as_str())))
        }

        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn internal_ends_with(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::String(suffix))) => {
            Ok(RcValue::bool(s.ends_with(suffix.as_str())))
        }
        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn internal_contains(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::String(substring))) => {
            Ok(RcValue::bool(s.contains(substring.as_str())))
        }
        (_, Some(Value::Null)) | (Some(Value::Null), _) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn internal_is_null(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::Bool(is_not)), Some(Value::Null)) => Ok(RcValue::bool(!is_not)),
        (Some(Value::Bool(is_not)), Some(_)) => Ok(RcValue::bool(*is_not)),
        _ => unreachable!(),
    }
}

fn internal_node_has_labels(
    runtime: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::Node(node_id)), Some(Value::List(required_labels))) => {
            let mut labels = runtime
                .g
                .borrow()
                .get_node_labels(*node_id)
                .collect::<OrderSet<_>>();
            labels.extend(
                runtime
                    .pending
                    .borrow()
                    .set_node_labels
                    .get(node_id)
                    .cloned()
                    .unwrap_or_else(OrderSet::new),
            );
            if let Some(remove_labels) = runtime.pending.borrow().remove_node_labels.get(node_id) {
                for label in remove_labels {
                    labels.remove(label);
                }
            }
            let all_labels_present = required_labels.iter().all(|label| {
                if let Value::String(label_str) = &**label {
                    labels.contains(label_str)
                } else {
                    false
                }
            });

            Ok(RcValue::bool(all_labels_present))
        }
        _ => unreachable!(),
    }
}

fn internal_regex_matches(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next().as_deref()) {
        (Some(Value::String(s)), Some(Value::String(pattern))) => {
            // Compile the regex pattern
            match regex::Regex::new(pattern.as_str()) {
                Ok(re) => Ok(RcValue::bool(re.is_match(s.as_str()))),
                Err(e) => Err(format!("Invalid regex pattern: {e}")),
            }
        }
        (Some(Value::Null), _) | (_, Some(Value::Null)) => Ok(RcValue::null()),
        _ => unreachable!(),
    }
}

fn internal_case(
    _: &Runtime,
    args: Vec<RcValue>,
) -> Result<RcValue, String> {
    let mut iter = args.into_iter();
    match (iter.next().as_deref(), iter.next(), iter.next()) {
        (Some(Value::List(alts)), Some(else_), None) => {
            for pair in alts.chunks(2) {
                match (&*pair[0], &pair[1]) {
                    (Value::Bool(false) | Value::Null, _) => {}
                    (_, result) => return Ok(result.clone()),
                }
            }
            Ok(else_)
        }
        (Some(value), Some(alt), Some(else_)) => {
            let Value::List(alts) = &*alt else {
                unreachable!()
            };
            for pair in alts.chunks(2) {
                if let [condition, result] = pair {
                    if &**condition == value {
                        return Ok(result.clone());
                    }
                }
            }
            Ok(else_)
        }
        _ => unreachable!(),
    }
}

fn db_labels(
    runtime: &Runtime,
    _args: Vec<RcValue>,
) -> Result<RcValue, String> {
    Ok(RcValue::list(
        runtime
            .g
            .borrow()
            .get_labels()
            .map(|n| RcValue::string(n.clone()))
            .collect(),
    ))
}

fn db_types(
    runtime: &Runtime,
    _args: Vec<RcValue>,
) -> Result<RcValue, String> {
    Ok(RcValue::list(
        runtime
            .g
            .borrow()
            .get_types()
            .map(|n| RcValue::string(n.clone()))
            .collect(),
    ))
}

fn db_properties(
    runtime: &Runtime,
    _args: Vec<RcValue>,
) -> Result<RcValue, String> {
    Ok(RcValue::list(
        runtime
            .g
            .borrow()
            .get_attrs()
            .map(|n| RcValue::string(n.clone()))
            .collect(),
    ))
}
