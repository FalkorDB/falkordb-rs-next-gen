#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

use crate::{
    indexer::{IndexInfo, IndexStatus, IndexType},
    runtime::{
        ordermap::OrderMap,
        runtime::Runtime,
        value::{Point, Value, ValueTypeOf},
    },
};
use rand::Rng;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, OnceLock},
};
use thin_vec::{ThinVec, thin_vec};

type RuntimeFn = fn(&Runtime, ThinVec<Value>) -> Result<Value, String>;

pub enum FnType {
    Function,
    Internal,
    Procedure(Vec<String>),
    Aggregation(Value, Option<Box<dyn Fn(Value) -> Value>>),
}

#[cfg_attr(tarpaulin, skip)]
impl Debug for FnType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Function => write!(f, "Function"),
            Self::Internal => write!(f, "Internal"),
            Self::Procedure(_) => write!(f, "Procedure"),
            Self::Aggregation(_, _) => write!(f, "Aggregation"),
        }
    }
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
                | (Self::Procedure(_), Self::Procedure(_))
                | (Self::Aggregation(_, _), Self::Aggregation(_, _))
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
    List(Box<Self>),
    Map,
    Node,
    Relationship,
    Path,
    VecF32,
    Point,
    Datetime,
    Date,
    Time,
    Duration,
    Any,
    Union(Vec<Self>),
    Optional(Box<Self>),
}

#[cfg_attr(tarpaulin, skip)]
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
            Self::Relationship => write!(f, "Edge"),
            Self::Path => write!(f, "Path"),
            Self::VecF32 => write!(f, "VecF32"),
            Self::Point => write!(f, "Point"),
            Self::Datetime => write!(f, "Datetime"),
            Self::Date => write!(f, "Date"),
            Self::Time => write!(f, "Time"),
            Self::Duration => write!(f, "Duration"),
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
        matches!(self.fn_type, FnType::Aggregation(_, _))
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
                let most = args_type.len();
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
        args: &[Value],
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
                    } else if let Some((actual, expected)) = args[i].value_of_type(arg_type) {
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

impl GraphFn {
    /// Validates domain constraints (e.g., percentile must be in [0.0, 1.0])
    /// This is called AFTER type validation but BEFORE consuming the accumulator
    pub fn validate_args_domain(
        &self,
        args: &[Value],
    ) -> Result<(), String> {
        // Only percentile functions need domain validation currently
        if self.name.to_lowercase().starts_with("percentile") {
            // percentile is at index 1 (after the value argument)
            if args.len() >= 2 {
                if args[1] == Value::Null {
                    return Err("Type mismatch: expected Integer or Float but was Null".to_string());
                }
                let percentile = args[1].get_numeric();
                if !(0.0..=1.0).contains(&percentile) {
                    return Err(format!(
                        "Invalid input - '{percentile}' is not a valid argument, must be a number in the range 0.0 to 1.0"
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Functions {
    functions: HashMap<String, Arc<GraphFn>>,
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
        let graph_fn = Arc::new(GraphFn::new(
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
        let graph_fn = Arc::new(GraphFn::new(
            &name,
            func,
            write,
            FnArguments::VarLength(arg_type),
            fn_type,
        ));
        self.functions.insert(name, graph_fn);
    }

    pub fn get(
        &self,
        name: &str,
        fn_type: &FnType,
    ) -> Result<Arc<GraphFn>, String> {
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
            .is_some_and(|graph_fn| matches!(graph_fn.fn_type, FnType::Aggregation(_, _)))
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
            Type::Union(vec![
                Type::Map,
                Type::Node,
                Type::Relationship,
                Type::Null,
                Type::Point,
                Type::Path,
                Type::Datetime,
                Type::Date,
                Type::Time,
                Type::Duration,
            ]),
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
    funcs.add("typeOf", type_of, false, vec![Type::Any], FnType::Function);
    funcs.add(
        "hasLabels",
        has_labels,
        false,
        vec![
            Type::Union(vec![Type::Node, Type::Null]),
            Type::List(Box::new(Type::Any)),
        ],
        FnType::Function,
    );
    funcs.add(
        "id",
        id,
        false,
        vec![Type::Union(vec![
            Type::Node,
            Type::Relationship,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "properties",
        properties,
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
        "length",
        length,
        false,
        vec![Type::Union(vec![Type::Path, Type::Null])],
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
            Type::Datetime,
            Type::Duration,
            Type::String,
            Type::Bool,
            Type::Int,
            Type::Float,
            Type::Null,
            Type::Point,
        ])],
        FnType::Function,
    );
    funcs.add(
        "tostringornull",
        value_to_string,
        false,
        vec![Type::Any],
        FnType::Function,
    );

    funcs.add("tojson", to_json, false, vec![Type::Any], FnType::Function);
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
        "rtrim",
        string_rtrim,
        false,
        vec![Type::Union(vec![Type::String, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "trim",
        string_trim,
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
            Type::Optional(Box::new(Type::Union(vec![Type::String, Type::Null]))),
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
    funcs.add("randomUUID", random_uuid, false, vec![], FnType::Function);
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
        "sin",
        sin,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "cos",
        cos,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "tan",
        tan,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "cot",
        cot,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "asin",
        asin,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "acos",
        acos,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "atan",
        atan,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "atan2",
        atan2,
        false,
        vec![
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
        ],
        FnType::Function,
    );
    funcs.add(
        "degrees",
        degrees,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "radians",
        radians,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add("pi", pi, false, vec![], FnType::Function);
    funcs.add(
        "haversin",
        haversin,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "isEmpty",
        is_empty,
        false,
        vec![Type::Union(vec![
            Type::Map,
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "toBoolean",
        to_boolean,
        false,
        vec![Type::Union(vec![
            Type::String,
            Type::Bool,
            Type::Int,
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "toBooleanOrNull",
        to_boolean,
        false,
        vec![Type::Any], // Accept ANY type, unlike toBoolean which is restricted
        FnType::Function,
    );
    funcs.add(
        "toFloatOrNull",
        value_to_float, // Reuse the same function
        false,
        vec![Type::Any], // Accept ANY type instead of restricted union
        FnType::Function,
    );
    funcs.add(
        "toIntegerOrNull",
        value_to_integer, // Reuse the same function
        false,
        vec![Type::Any], // Accept ANY type instead of restricted union
        FnType::Function,
    );
    funcs.add(
        "type",
        relationship_type,
        false,
        vec![Type::Union(vec![Type::Relationship, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "nodes",
        nodes,
        false,
        vec![Type::Union(vec![Type::Path, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "relationships",
        relationships,
        false,
        vec![Type::Union(vec![Type::Path, Type::Null])],
        FnType::Function,
    );
    funcs.add(
        "vecf32",
        vecf32,
        false,
        vec![Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        FnType::Function,
    );
    funcs.add(
        "point",
        point,
        false,
        vec![Type::Union(vec![Type::Map, Type::Null])],
        FnType::Function,
    );
    funcs.add("exists", exists, false, vec![Type::Any], FnType::Function);

    // aggregation functions
    funcs.add(
        "collect",
        collect,
        false,
        vec![Type::Any],
        FnType::Aggregation(Value::List(thin_vec![]), None),
    );
    funcs.add(
        "count",
        count,
        false,
        vec![Type::Any],
        FnType::Aggregation(Value::Int(0), None),
    );
    funcs.add(
        "sum",
        sum,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Aggregation(Value::Float(0.0), None),
    );
    funcs.add(
        "max",
        max,
        false,
        vec![Type::Any],
        FnType::Aggregation(Value::Null, None),
    );
    funcs.add(
        "min",
        min,
        false,
        vec![Type::Any],
        FnType::Aggregation(Value::Null, None),
    );
    funcs.add(
        "avg",
        avg,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Aggregation(
            Value::List(thin_vec![
                Value::Float(0.0),
                Value::Int(0),
                Value::Bool(false)
            ]),
            Some(Box::new(finalize_avg)),
        ),
    );
    funcs.add(
        "percentileDisc",
        percentile,
        false,
        vec![
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
            Type::Union(vec![Type::Int, Type::Float]),
        ],
        FnType::Aggregation(
            Value::List(thin_vec![Value::Float(0.0), Value::List(thin_vec![])]),
            Some(Box::new(finalize_percentile_disc)),
        ),
    );
    funcs.add(
        "percentileCont",
        percentile,
        false,
        vec![
            Type::Union(vec![Type::Int, Type::Float, Type::Null]),
            Type::Union(vec![Type::Int, Type::Float]),
        ],
        FnType::Aggregation(
            Value::List(thin_vec![Value::Float(0.0), Value::List(thin_vec![])]),
            Some(Box::new(finalize_percentile_cont)),
        ),
    );
    funcs.add(
        "stDev",
        stdev,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Aggregation(
            Value::List(thin_vec![Value::Float(0.0), Value::List(thin_vec![])]),
            Some(Box::new(finalize_stdev)),
        ),
    );
    funcs.add(
        "stDevP",
        stdev,
        false,
        vec![Type::Union(vec![Type::Int, Type::Float, Type::Null])],
        FnType::Aggregation(
            Value::List(thin_vec![Value::Float(0.0), Value::List(thin_vec![])]),
            Some(Box::new(finalize_stdevp)),
        ),
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
    funcs.add(
        "db.labels",
        db_labels,
        false,
        vec![],
        FnType::Procedure(vec![String::from("label")]),
    );
    funcs.add(
        "db.relationshiptypes",
        db_types,
        false,
        vec![],
        FnType::Procedure(vec![String::from("relationshipType")]),
    );
    funcs.add(
        "db.propertykeys",
        db_properties,
        false,
        vec![],
        FnType::Procedure(vec![String::from("propertyKey")]),
    );
    funcs.add(
        "db.indexes",
        db_indexes,
        false,
        vec![],
        FnType::Procedure(vec![
            String::from("label"),
            String::from("properties"),
            String::from("types"),
            String::from("options"),
            String::from("language"),
            String::from("stopwords"),
            String::from("entitytype"),
            String::from("status"),
            String::from("info"),
        ]),
    );

    funcs.add(
        "db.idx.fulltext.createNodeIndex",
        db_fulltext_create_node_index,
        true,
        vec![Type::Map],
        FnType::Procedure(vec![]),
    );
    funcs.add(
        "db.idx.fulltext.drop",
        db_fulltext_drop_node_index,
        true,
        vec![],
        FnType::Procedure(vec![]),
    );

    funcs.add(
        "db.idx.fulltext.queryNodes",
        db_fulltext_query_nodes,
        false,
        vec![Type::Map],
        FnType::Procedure(vec![String::from("node"), String::from("score")]),
    );

    FUNCTIONS.set(funcs)
}

pub fn get_functions() -> &'static Functions {
    FUNCTIONS.get().expect("Functions not initialized")
}

///////////////////////////////////
///////////// functions ///////////
///////////////////////////////////

// Helper function to extract components from datetime values
fn extract_datetime_component(
    timestamp_ms: i64,
    component: &str,
) -> Result<Value, String> {
    use chrono::{Datelike, TimeZone, Timelike, Utc};

    match Utc.timestamp_millis_opt(timestamp_ms) {
        chrono::LocalResult::Single(dt) => match component {
            "year" => Ok(Value::Int(i64::from(dt.year()))),
            "month" => Ok(Value::Int(i64::from(dt.month()))),
            "day" => Ok(Value::Int(i64::from(dt.day()))),
            "hour" => Ok(Value::Int(i64::from(dt.hour()))),
            "minute" => Ok(Value::Int(i64::from(dt.minute()))),
            "second" => Ok(Value::Int(i64::from(dt.second()))),
            "millisecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_millis()))),
            "microsecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_micros()))),
            "nanosecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_nanos()))),
            _ => Err(format!("unknown datetime component {component}")),
        },
        _ => Err(format!("invalid timestamp: {timestamp_ms}")),
    }
}

// Helper function to extract components from date values
fn extract_date_component(
    timestamp_ms: i64,
    component: &str,
) -> Result<Value, String> {
    use chrono::{Datelike, TimeZone, Utc};

    match Utc.timestamp_millis_opt(timestamp_ms) {
        chrono::LocalResult::Single(dt) => match component {
            "year" => Ok(Value::Int(i64::from(dt.year()))),
            "month" => Ok(Value::Int(i64::from(dt.month()))),
            "day" => Ok(Value::Int(i64::from(dt.day()))),
            _ => Err(format!("unknown date component {component}")),
        },
        _ => Err(format!("invalid timestamp: {timestamp_ms}")),
    }
}

// Helper function to extract components from time values
fn extract_time_component(
    timestamp_ms: i64,
    component: &str,
) -> Result<Value, String> {
    use chrono::{TimeZone, Timelike, Utc};

    match Utc.timestamp_millis_opt(timestamp_ms) {
        chrono::LocalResult::Single(dt) => match component {
            "hour" => Ok(Value::Int(i64::from(dt.hour()))),
            "minute" => Ok(Value::Int(i64::from(dt.minute()))),
            "second" => Ok(Value::Int(i64::from(dt.second()))),
            "millisecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_millis()))),
            "microsecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_micros()))),
            "nanosecond" => Ok(Value::Int(i64::from(dt.timestamp_subsec_nanos()))),
            _ => Err(format!("unknown time component {component}")),
        },
        _ => Err(format!("invalid timestamp: {timestamp_ms}")),
    }
}

// Helper function to extract components from duration values
fn extract_duration_component(
    duration_ms: i64,
    component: &str,
) -> Result<Value, String> {
    // Duration is stored as milliseconds
    const MS_PER_SECOND: i64 = 1000;
    const MS_PER_MINUTE: i64 = 60 * MS_PER_SECOND;
    const MS_PER_HOUR: i64 = 60 * MS_PER_MINUTE;
    const MS_PER_DAY: i64 = 24 * MS_PER_HOUR;

    match component {
        "days" => Ok(Value::Int(duration_ms / MS_PER_DAY)),
        "hours" => Ok(Value::Int(duration_ms / MS_PER_HOUR)),
        "minutes" => Ok(Value::Int(duration_ms / MS_PER_MINUTE)),
        "seconds" => Ok(Value::Int(duration_ms / MS_PER_SECOND)),
        "milliseconds" => Ok(Value::Int(duration_ms)),
        "microseconds" => duration_ms
            .checked_mul(1000)
            .map(Value::Int)
            .ok_or_else(|| "duration overflow".to_string()),
        "nanoseconds" => duration_ms
            .checked_mul(1_000_000)
            .map(Value::Int)
            .ok_or_else(|| "duration overflow".to_string()),
        _ => Err(format!("unknown duration component {component}")),
    }
}

fn property(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(id)), Some(Value::String(attr))) => runtime
            .get_node_attribute(id, &attr)
            .map_or(Ok(Value::Null), Ok),
        (Some(Value::Relationship(rel)), Some(Value::String(attr))) => runtime
            .get_relationship_attribute(rel.0, &attr)
            .map_or(Ok(Value::Null), Ok),
        (Some(Value::Map(map)), Some(Value::String(attr))) => {
            Ok(map.get(&attr).cloned().unwrap_or(Value::Null))
        }
        (Some(Value::Point(point)), Some(Value::String(attr))) => match attr.as_str() {
            "latitude" => Ok(Value::Float(f64::from(point.latitude))),
            "longitude" => Ok(Value::Float(f64::from(point.longitude))),
            "crs" => Ok(Value::String(Arc::new(String::from("wgs-84")))),
            _ => Ok(Value::Null),
        },
        (Some(Value::Null), Some(Value::String(_))) => Ok(Value::Null),
        (Some(Value::Datetime(ts)), Some(Value::String(attr))) => {
            extract_datetime_component(ts, &attr)
        },
        (Some(Value::Date(ts)), Some(Value::String(attr))) => {
            extract_date_component(ts, &attr)
        },
        (Some(Value::Time(ts)), Some(Value::String(attr))) => {
            extract_time_component(ts, &attr)
        },
        (Some(Value::Duration(dur)), Some(Value::String(attr))) => {
            extract_duration_component(dur, &attr)
        },
        (Some(Value::Path(_)), Some(Value::String(_))) => {
            Err("Type mismatch: expected Map, Node, Edge, Datetime, Date, Time, Duration, Null, or Point but was Path".to_string())
        }
        _ => unreachable!(),
    }
}

fn labels(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Node(id)) => {
            let labels = runtime.get_node_labels(id);
            Ok(Value::List(labels.into_iter().map(Value::String).collect()))
        }
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn type_of(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let type_name = match args.into_iter().next() {
        Some(Value::Null) => "Null",
        Some(Value::Bool(_)) => "Boolean",
        Some(Value::Int(_)) => "Integer",
        Some(Value::Float(_)) => "Float",
        Some(Value::String(_)) => "String",
        Some(Value::List(_)) => "List",
        Some(Value::Arc(v)) => {
            // Handle Arc-wrapped values
            match &*v {
                Value::List(_) => "List",
                Value::Map(_) => "Map",
                _ => "Unknown",
            }
        }
        Some(Value::Map(_)) => "Map",
        Some(Value::Node(_)) => "Node",
        Some(Value::Relationship(_)) => "Edge",
        Some(Value::Path(_)) => "Path",
        Some(Value::VecF32(_)) => "Vectorf32",
        Some(Value::Point(_)) => "Point",
        Some(Value::Datetime(_)) => "Datetime",
        Some(Value::Date(_)) => "Date",
        Some(Value::Time(_)) => "Time",
        Some(Value::Duration(_)) => "Duration",
        None => unreachable!(),
    };
    Ok(Value::String(Arc::new(String::from(type_name))))
}

fn has_labels(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Node(id)), Some(Value::List(required_labels))) => {
            // Validate that all items in the list are strings
            for label_value in &required_labels {
                match label_value {
                    Value::String(_) => {}
                    Value::Int(_) => {
                        return Err("Type mismatch: expected String but was Integer".to_string());
                    }
                    Value::Float(_) => {
                        return Err("Type mismatch: expected String but was Float".to_string());
                    }
                    Value::Bool(_) => {
                        return Err("Type mismatch: expected String but was Boolean".to_string());
                    }
                    _ => return Err("Type mismatch: expected String".to_string()),
                }
            }

            // Get the actual labels of the node
            let node_labels = runtime.get_node_labels(id);
            // Check if all required labels are present
            let has_all = required_labels.iter().all(|req_label| {
                if let Value::String(req_str) = req_label {
                    node_labels.iter().any(|node_label| node_label == req_str)
                } else {
                    false
                }
            });

            Ok(Value::Bool(has_all))
        }
        (Some(Value::Null), _) | (_, Some(Value::Null)) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn id(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Node(id)) => Ok(Value::Int(u64::from(id) as i64)),
        Some(Value::Relationship(rel)) => Ok(Value::Int(u64::from(rel.0) as i64)),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn properties(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Map(map)) => Ok(Value::Map(map)),
        Some(Value::Node(id)) => Ok(Value::Map(runtime.get_node_attrs(id))),
        Some(Value::Relationship(rel)) => Ok(Value::Map(runtime.get_relationship_attrs(rel.0))),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn start_node(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Relationship(rel)) => Ok(Value::Node(rel.1)),

        _ => unreachable!(),
    }
}

fn end_node(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Relationship(rel)) => Ok(Value::Node(rel.2)),

        _ => unreachable!(),
    }
}

fn length(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Path(path)) => Ok(Value::Int(path.len() as i64)),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn collect(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(Value::Null)) => Ok(Value::List(thin_vec![a])),
        (Some(a), Some(Value::List(mut l))) => {
            if a == Value::Null {
                return Ok(Value::List(l));
            }
            l.push(a);
            Ok(Value::List(l))
        }

        _ => unreachable!(),
    }
}

fn count(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let first = iter.next();
    let sec = iter.next();
    match (first, sec) {
        (Some(Value::Null), Some(sec)) => Ok(sec),
        (Some(_), Some(Value::Int(a))) | (Some(Value::Int(a)), None) => Ok(Value::Int(a + 1)),

        _ => unreachable!(),
    }
}

fn sum(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let first = iter.next();
    let second = iter.next();

    match (first, second) {
        // Skip null values - return accumulator unchanged
        (Some(Value::Null), Some(acc)) => Ok(acc),

        // Numeric value + Int accumulator (cast before adding to avoid i64 overflow)
        (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(Value::Float(a as f64 + b as f64)),
        (Some(Value::Int(a)), Some(Value::Float(b))) => Ok(Value::Float(a as f64 + b)),

        // Numeric value + Float accumulator
        (Some(Value::Float(a)), Some(Value::Float(b))) => Ok(Value::Float(a + b)),
        (Some(Value::Float(a)), Some(Value::Int(b))) => Ok(Value::Float(a + b as f64)),

        _ => unreachable!("sum expects Integer, Float, or Null (validation done before call)"),
    }
}

fn max(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => {
            if b == Value::Null {
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => {
            if b == Value::Null {
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

fn avg(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let val = iter.next().unwrap();
    let ctx = iter.next().unwrap();
    match (val, ctx) {
        // distinct may pass null as a way to skip the value
        (Value::Null, ctx) => {
            // If the first value is null, return the accumulator unchanged
            Ok(ctx)
        }
        (val @ (Value::Int(_) | Value::Float(_)), Value::List(mut vec)) => {
            let val = val.get_numeric();

            // Use split_at_mut to get mutable references to all three elements safely
            // vec = [sum, count, had_overflow]
            let (first, rest) = vec.split_at_mut(1);
            let (second, third) = rest.split_at_mut(1);

            let (sum, count, had_overflow) = match (&mut first[0], &mut second[0], &mut third[0]) {
                (Value::Float(sum), Value::Int(count), Value::Bool(had_overflow)) => {
                    (sum, count, had_overflow)
                }
                _ => unreachable!("avg accumulator should be [sum, count, overflow]"),
            };

            *count += 1;

            // Check for overflow condition
            if *had_overflow || about_to_overflow(*sum, val) {
                // Use incremental averaging algorithm
                // Divide the total by the new count (in-place mutation like C)
                *sum /= *count as f64;

                // If we were already in overflow mode, multiply back by previous count
                if *had_overflow {
                    *sum *= (*count - 1) as f64;
                }

                // Add the new value contribution
                *sum += val / *count as f64;

                // Mark that we're now in overflow mode
                *had_overflow = true;
            } else {
                // Normal accumulation - sum stores total
                *sum += val;
            }

            Ok(Value::List(vec))
        }
        _ => unreachable!("avg expects Integer, Float, or Null (validation done before call)"),
    }
}

fn about_to_overflow(
    a: f64,
    b: f64,
) -> bool {
    a.signum() == b.signum() && a.abs() >= (f64::MAX - b.abs())
}

fn finalize_avg(value: Value) -> Value {
    let Value::List(vec) = value else {
        unreachable!("finalize_avg expects a list");
    };
    let (Value::Float(sum), Value::Int(count), Value::Bool(overflow)) = (&vec[0], &vec[1], &vec[2])
    else {
        unreachable!("avg function should have [sum, count, overflow] format");
    };
    if *count == 0 {
        Value::Null
    } else if *overflow {
        Value::Float(*sum)
    } else {
        Value::Float(sum / *count as f64)
    }
}

fn percentile(
    _: &Runtime,
    mut args: ThinVec<Value>,
) -> Result<Value, String> {
    let val = args.remove(0);
    let percentile_val = args.remove(0);

    // Domain validation is now done in PHASE 3.5, so these checks are removed
    // (Or kept as defensive programming - they should never fail)

    let percentile = percentile_val.get_numeric();

    let ctx = args.remove(0);
    if matches!(val, Value::Null) {
        return Ok(ctx);
    }

    let Value::List(mut state) = ctx else {
        unreachable!("Context must be a List");
    };

    let Value::List(mut collected_values) = std::mem::take(&mut state[1]) else {
        unreachable!("Second element of state must be a List")
    };

    collected_values.push(Value::Float(val.get_numeric()));

    Ok(Value::List(thin_vec![
        Value::Float(percentile),
        Value::List(collected_values),
    ]))
}

#[allow(clippy::needless_pass_by_value)]
fn finalize_percentile_disc(ctx: Value) -> Value {
    let Value::List(mut state) = ctx else {
        unreachable!()
    };

    let [Value::Float(percentile), Value::List(values)] = state.as_mut_slice() else {
        unreachable!()
    };

    if values.is_empty() {
        return Value::Null;
    }

    values.sort_by(|a, b| {
        a.get_numeric()
            .partial_cmp(&b.get_numeric())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let index = if *percentile > 0.0 {
        (values.len() as f64 * *percentile).ceil() as usize - 1
    } else {
        0
    };

    Value::Float(values[index].get_numeric())
}

#[allow(clippy::needless_pass_by_value)]
fn finalize_percentile_cont(ctx: Value) -> Value {
    let Value::List(mut state) = ctx else {
        unreachable!()
    };

    let [Value::Float(percentile), Value::List(values)] = state.as_mut_slice() else {
        unreachable!()
    };

    if values.is_empty() {
        return Value::Null;
    }

    values.sort_by(|a, b| {
        a.get_numeric()
            .partial_cmp(&b.get_numeric())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    #[allow(clippy::float_cmp)]
    if *percentile == 1.0 || values.len() == 1 {
        return Value::Float(values[values.len() - 1].get_numeric());
    }

    let float_idx = (values.len() - 1) as f64 * *percentile;

    let (fraction_val, int_val) = modf(float_idx);
    let index = int_val as usize;

    if fraction_val == 0.0 {
        return Value::Float(values[index].get_numeric());
    }
    let lhs = values[index].get_numeric() * (1.0 - fraction_val);
    let rhs = values[index + 1].get_numeric() * fraction_val;
    Value::Float(lhs + rhs)
}

const fn modf(x: f64) -> (f64, f64) {
    let int_part = x.trunc();
    let frac_part = x.fract();
    (frac_part, int_part)
}

fn stdev(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let val = iter.next().unwrap();
    let ctx = iter.next().unwrap();
    match (val, ctx) {
        (Value::Null, ctx) => Ok(ctx),
        (val @ (Value::Int(_) | Value::Float(_)), Value::List(mut vec)) => {
            let val = val.get_numeric();

            // Use split_at_mut to get mutable references to both elements safely
            let (first, rest) = vec.split_at_mut(1);
            let (Value::Float(sum), Value::List(values)) = (&mut first[0], &mut rest[0]) else {
                unreachable!("stdev accumulator should be [sum, values]")
            };

            // Mutate in-place:  update sum and push value to list (avoids O(n²) cloning)
            *sum += val;
            values.push(Value::Float(val));

            Ok(Value::List(vec))
        }
        _ => unreachable!("stdev expects Integer, Float, or Null (validation done before call)"),
    }
}

fn finalize_stdev(ctx: Value) -> Value {
    let Value::List(vec) = ctx else {
        unreachable!("finalize_stdev expects a list");
    };
    let (Value::Float(sum), Value::List(values)) = (&vec[0], &vec[1]) else {
        unreachable!("stdev function should have [sum, values] format");
    };
    if values.is_empty() || values.len() == 1 {
        return Value::Float(0.0);
    }
    let mean = sum / values.len() as f64;
    let variance: f64 = values
        .iter()
        .map(|v| {
            let diff = v.get_numeric() - mean;
            diff * diff
        })
        .sum::<f64>()
        / (values.len() - 1) as f64;
    Value::Float(variance.sqrt())
}

fn finalize_stdevp(ctx: Value) -> Value {
    let Value::List(vec) = ctx else {
        unreachable!("finalize_stdev expects a list");
    };
    let (Value::Float(sum), Value::List(values)) = (&vec[0], &vec[1]) else {
        unreachable!("stdev function should have [sum, values] format");
    };
    if values.is_empty() {
        return Value::Float(0.0);
    }
    let mean = sum / values.len() as f64;
    let variance: f64 = values
        .iter()
        .map(|v| {
            let diff = v.get_numeric() - mean;
            diff * diff
        })
        .sum::<f64>()
        / values.len() as f64;
    Value::Float(variance.sqrt())
}

fn value_to_integer(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(Value::Null);
            }

            // Match C behavior: strtoll() with errno == ERANGE check
            // First try parsing as i64 for simple integers
            if let Ok(i) = s.parse::<i64>() {
                return Ok(Value::Int(i));
            }

            // If that fails, try parsing as f64 to handle decimals and scientific notation
            // C does: if (strchr(arg.stringval, '.') == NULL) { strtoll } else { strtod }
            // Then: if (errno == ERANGE) return SI_NullVal();
            // This means: return None for ANY overflow, no saturation
            match s.parse::<f64>() {
                Ok(f) if f.is_finite() => {
                    let floored = f.floor();

                    #[allow(clippy::cast_precision_loss)]
                    let i64_max_as_f64 = i64::MAX as f64;
                    #[allow(clippy::cast_precision_loss)]
                    let i64_min_as_f64 = i64::MIN as f64;

                    // C behavior: errno == ERANGE means return None
                    // For string path, if value is out of i64 range, return None (no saturation)
                    // Use >= because i64::MAX+1 rounds to same f64 as i64::MAX
                    // Use <= because i64::MIN-1 rounds to same f64 as i64::MIN
                    // Since exact i64::MAX/MIN were caught by parse::<i64>() above,
                    // any value equal to the boundary must have rounded from out-of-range
                    if floored >= i64_max_as_f64 || floored <= i64_min_as_f64 {
                        return Ok(Value::Null);
                    }

                    #[allow(clippy::cast_possible_truncation)]
                    Ok(Value::Int(floored as i64))
                }
                _ => Ok(Value::Null),
            }
        }
        Some(Value::Int(i)) => Ok(Value::Int(i)),
        Some(Value::Float(f)) => {
            if !f.is_finite() {
                return Ok(Value::Null);
            }

            // Match C behavior: return SI_LongVal(floor(arg.doubleval));
            // C does NOT check for overflow on float->int conversion
            // This is undefined behavior in C, but typically saturates
            // Rust's `as i64` cast has defined saturation behavior, which matches typical C behavior
            let floored = f.floor();

            #[allow(clippy::cast_possible_truncation)]
            Ok(Value::Int(floored as i64))
        }
        Some(Value::Bool(b)) => Ok(Value::Int(i64::from(b))),
        _ => Ok(Value::Null),
    }
}

fn value_to_float(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => s.parse::<f64>().map(Value::Float).or(Ok(Value::Null)),
        Some(Value::Float(f)) => Ok(Value::Float(f)),
        Some(Value::Int(i)) => Ok(Value::Float(i as f64)),
        _ => Ok(Value::Null),
    }
}

// Single implementation - returns Null for non-stringable types
fn value_to_string(
    _runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(s)),
        Some(Value::Int(i)) => Ok(Value::String(Arc::new(i.to_string()))),
        Some(Value::Float(f)) => Ok(Value::String(Arc::new(format!("{f:.6}")))),
        Some(Value::Bool(b)) => Ok(Value::String(Arc::new(b.to_string()))),
        Some(Value::Point(p)) => Ok(Value::String(Arc::new(format!(
            "Point(latitude: {}, longitude: {})",
            p.latitude, p.longitude
        )))),
        Some(Value::Datetime(ts)) => Ok(Value::String(Arc::new(Value::format_datetime(ts)))),
        Some(Value::Date(ts)) => Ok(Value::String(Arc::new(Value::format_date(ts)))),
        Some(Value::Time(ts)) => Ok(Value::String(Arc::new(Value::format_time(ts)))),
        Some(Value::Duration(dur)) => Ok(Value::String(Arc::new(Value::format_duration(dur)))),
        // All other types return Null (matches C behavior)
        Some(_) => Ok(Value::Null),

        None => unreachable!(),
    }
}
fn to_json(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    args.into_iter().next().map_or_else(
        || unreachable!(),
        |v| {
            let json_string = v.to_json_string(runtime);
            Ok(Value::String(Arc::new(json_string)))
        },
    )
}

fn size(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::Int(s.chars().count() as i64)),
        Some(Value::List(v)) => Ok(Value::Int(v.len() as i64)),
        Some(Value::Arc(v)) => {
            if let Value::List(v) = &*v {
                Ok(Value::Int(v.len() as i64))
            } else {
                unreachable!()
            }
        }
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn head(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => Ok(v.last().cloned().unwrap_or(Value::Null)),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn tail(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(v)) => {
            if v.is_empty() {
                Ok(Value::List(thin_vec![]))
            } else {
                Ok(Value::List(v[1..].iter().cloned().collect::<ThinVec<_>>()))
            }
        }
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn reverse(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::List(mut v)) => {
            v.reverse();
            Ok(Value::List(v))
        }
        Some(Value::String(s)) => Ok(Value::String(Arc::new(s.chars().rev().collect()))),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn substring(
    _: &Runtime,
    args: ThinVec<Value>,
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
            if start >= s.len() as _ {
                return Ok(Value::String(Arc::new(String::new())));
            }
            let start = start as usize;

            Ok(Value::String(Arc::new(s.chars().skip(start).collect())))
        }

        // Three-argument version: (string, start, length)
        (Some(Value::String(s)), Some(Value::Int(start)), Some(Value::Int(length))) => {
            if start < 0 {
                return Err("start must be a non-negative integer".into());
            }

            let start = start as usize;
            if start >= s.len() {
                return Ok(Value::String(Arc::new(String::new())));
            }

            if length < 0 {
                return Err("length must be a non-negative integer".into());
            }

            let length = length as usize;

            Ok(Value::String(Arc::new(
                s.chars().skip(start).take(length).collect(),
            )))
        }

        _ => unreachable!(),
    }
}

fn split(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(string)), Some(Value::String(delimiter))) => {
            if string.is_empty() {
                Ok(Value::List(thin_vec![Value::String(Arc::new(
                    String::new()
                ))]))
            } else if delimiter.is_empty() {
                // split string to characters
                let parts = string
                    .chars()
                    .map(|c| Value::String(Arc::new(String::from(c))))
                    .collect();
                Ok(Value::List(parts))
            } else {
                let parts = string
                    .split(delimiter.as_str())
                    .map(|s| Value::String(Arc::new(String::from(s))))
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => {
            // Match C behavior: detect replacement character which indicates invalid UTF-8
            // In the C version, str_tolower returns NULL on invalid UTF-8 (c == -1)
            // In Rust, we check for the replacement character
            if s.contains('\u{FFFD}') {
                return Err(String::from("Invalid UTF8 string"));
            }
            Ok(Value::String(Arc::new(s.to_lowercase())))
        }
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn string_to_upper(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => {
            // Match C behavior: detect replacement character which indicates invalid UTF-8
            // In the C version, str_toupper returns NULL on invalid UTF-8 (c == -1)
            // In Rust, we check for the replacement character
            if s.contains('\u{FFFD}') {
                return Err(String::from("Invalid UTF8 string"));
            }
            Ok(Value::String(Arc::new(s.to_uppercase())))
        }
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn string_replace(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::String(search)), Some(Value::String(replacement))) => {
            Ok(Value::String(Arc::new(
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if n < 0 {
                Err(String::from("length must be a non-negative integer"))
            } else {
                Ok(Value::String(Arc::new(
                    s.chars().take(n as usize).collect(),
                )))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),

        _ => unreachable!(),
    }
}

fn string_ltrim(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Arc::new(String::from(
            s.trim_start_matches(' '),
        )))),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn string_rtrim(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Arc::new(String::from(
            s.trim_end_matches(' '),
        )))),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn string_trim(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::String(s)) => Ok(Value::String(Arc::new(String::from(s.trim_matches(' '))))),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn string_right(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(s)), Some(Value::Int(n))) => {
            if n < 0 {
                Err(String::from("length must be a non-negative integer"))
            } else {
                let start = s.chars().count().saturating_sub(n as usize);
                Ok(Value::String(Arc::new(s.chars().skip(start).collect())))
            }
        }
        (Some(Value::Null), _) => Ok(Value::Null),
        (_, Some(Value::Null)) => Err(String::from("length must be a non-negative integer")),

        _ => unreachable!(),
    }
}

fn string_join(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    /// Convert Value list to Arc<String> vector, checking types
    fn to_string_vec(values: &[Value]) -> Result<Vec<Arc<String>>, String> {
        values
            .iter()
            .map(|value| match value {
                Value::String(s) => Ok(Arc::clone(s)),
                _ => Err(format!(
                    "Type mismatch: expected String but was {}",
                    value.name()
                )),
            })
            .collect()
    }

    /// Compute the total length needed, with overflow detection
    /// Uses i32 for calculations to match C implementation behavior
    fn compute_join_length(
        strings: &[Arc<String>],
        delimiter: &str,
    ) -> Result<usize, String> {
        if strings.is_empty() {
            return Ok(0);
        }

        let delimiter_len =
            i32::try_from(delimiter.len()).map_err(|_| String::from("String overflow"))?;
        let n = i32::try_from(strings.len()).map_err(|_| String::from("String overflow"))?;
        let mut str_len: i32 = 0;

        if n >= 2 {
            let delimiter_contribution = delimiter_len
                .checked_mul(n - 1)
                .ok_or_else(|| String::from("String overflow"))?;

            str_len = str_len
                .checked_add(delimiter_contribution)
                .ok_or_else(|| String::from("String overflow"))?;
        }

        for s in strings.iter() {
            let s_len = i32::try_from(s.len()).map_err(|_| String::from("String overflow"))?;

            str_len = str_len
                .checked_add(s_len)
                .ok_or_else(|| String::from("String overflow"))?;
        }

        str_len = str_len
            .checked_add(1)
            .ok_or_else(|| String::from("String overflow"))?;

        let capacity = (str_len - 1) as usize;
        Ok(capacity)
    }

    /// Join strings with pre-allocated buffer
    fn join_with_preallocate(
        strings: &[Arc<String>],
        delimiter: &str,
        capacity: usize,
    ) -> String {
        if strings.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(capacity);
        let mut first = true;

        for s in strings {
            if !first {
                result.push_str(delimiter);
            }
            result.push_str(s.as_str());
            first = false;
        }

        debug_assert_eq!(result.len(), capacity, "String join calculation mismatch");
        result
    }

    let mut iter = args.into_iter();

    // Unwrap Arc if present (handles range() returning Arc-wrapped lists)
    let first = match iter.next().unwrap() {
        Value::Arc(arc) => Arc::unwrap_or_clone(arc),
        v => v,
    };

    match (first, iter.next()) {
        (Value::List(vec), Some(Value::String(s))) => {
            let strings = to_string_vec(&vec)?;
            let size = compute_join_length(&strings, s.as_str())?;
            let joined = join_with_preallocate(&strings, s.as_str(), size);
            Ok(Value::String(Arc::new(joined)))
        }
        (Value::List(vec), None) => {
            let strings = to_string_vec(&vec)?;
            let size = compute_join_length(&strings, "")?;
            let joined = join_with_preallocate(&strings, "", size);
            Ok(Value::String(Arc::new(joined)))
        }
        (Value::List(_), Some(Value::Null)) | (Value::Null, _) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn string_match_reg_ex(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::String(text)), Some(Value::String(pattern))) => {
            match regex::Regex::new(pattern.as_str()) {
                Ok(re) => {
                    let mut all_matches = thin_vec![];
                    // For each match, create a sub-list containing the full match and all capture groups
                    for caps in re.captures_iter(text.as_str()) {
                        let mut match_list = thin_vec![];
                        // Iterate through all capture groups (0 = full match, 1+ = capture groups)
                        // Include NULL for non-participating optional groups to maintain index consistency
                        for i in 0..caps.len() {
                            if let Some(m) = caps.get(i) {
                                match_list.push(Value::String(Arc::new(String::from(m.as_str()))));
                            } else {
                                // Non-participating optional group - use NULL to preserve position
                                match_list.push(Value::Null);
                            }
                        }
                        // Add this match's captures as a sub-list
                        all_matches.push(Value::List(match_list));
                    }
                    Ok(Value::List(all_matches))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            }
        }
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::List(thin_vec![])),

        _ => unreachable!(),
    }
}

fn string_replace_reg_ex(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let text = iter.next();
    let pattern = iter.next();
    let replacement = iter.next(); // May be None (optional third argument)

    match (text, pattern) {
        // NULL text or pattern returns NULL
        (Some(Value::Null), _) | (_, Some(Value::Null)) => Ok(Value::Null),

        // Valid text and pattern
        (Some(Value::String(text)), Some(Value::String(pattern))) => {
            // Compile the regex first (before handling replacement)
            let re = match regex::Regex::new(pattern.as_str()) {
                Ok(re) => re,
                Err(e) => return Err(format!("Invalid regex, {e}")),
            };

            // Now handle replacement and perform replacement in one step
            match replacement {
                // No third argument provided, default to empty string
                None => {
                    let replaced_text = re.replace_all(text.as_str(), "").into_owned();
                    Ok(Value::String(Arc::new(replaced_text)))
                }
                // NULL replacement returns NULL
                Some(Value::Null) => Ok(Value::Null),
                // Use provided string
                Some(Value::String(repl)) => {
                    let replaced_text = re.replace_all(text.as_str(), repl.as_str()).into_owned();
                    Ok(Value::String(Arc::new(replaced_text)))
                }
                // All other types should have been caught by type checking
                Some(v) => Err(format!(
                    "Type mismatch: expected String or Null but was {}",
                    v.name()
                )),
            }
        }

        // All other type combinations should have been caught by type checking
        _ => unreachable!(),
    }
}

fn abs(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        None => Ok(Value::Float(std::f64::consts::E)),

        _ => unreachable!(),
    }
}

fn exp(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).log10())),
        Some(Value::Float(f)) => Ok(Value::Float(f.log10())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn random_uuid(
    _: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    use rand::Rng;

    // Generate 16 random bytes (128 bits)
    let mut rng = rand::rng();
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    // Set version to 4 (random UUID) - set bits 12-15 of byte 6 to 0100
    bytes[6] = (bytes[6] & 0x0F) | 0x40;

    // Set variant to RFC 4122 - set bits 6-7 of byte 8 to 10
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    // Format as UUID string:  xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    let uuid = format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}",
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u16::from_be_bytes([bytes[4], bytes[5]]),
        u16::from_be_bytes([bytes[6], bytes[7]]),
        u16::from_be_bytes([bytes[8], bytes[9]]),
        u16::from_be_bytes([bytes[10], bytes[11]]),
        u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
    );

    Ok(Value::String(Arc::new(uuid)))
}

// called from fn pow and expr pow (^)
#[inline]
#[must_use]
pub(crate) fn apply_pow(
    base: Value,
    exponent: Value,
) -> Value {
    match (base, exponent) {
        // Convert all numeric types to f64 and use powf
        // This matches C's behavior:  pow(SI_GET_NUMERIC(base), SI_GET_NUMERIC(exp))
        (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as f64)),
        (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
        (Value::Int(a), Value::Float(b)) => Value::Float((a as f64).powf(b)),
        (Value::Float(a), Value::Int(b)) => Value::Float(a.powf(b as f64)),
        _ => Value::Null,
    }
}

fn pow(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => Ok(apply_pow(a, b)),
        _ => unreachable!(),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn rand(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    debug_assert!(args.is_empty());
    let mut rng = rand::rng();
    Ok(Value::Float(rng.random_range(0.0..1.0)))
}

fn round(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    let start = iter.next().ok_or("Missing start value")?;
    let end = iter.next().ok_or("Missing end value")?;
    let step = iter.next().unwrap_or_else(|| Value::Int(1));
    match (start, end, step) {
        (Value::Int(start), Value::Int(end), Value::Int(step)) => {
            if step == 0 {
                return Err(String::from(
                    "ArgumentError: step argument to range() can't be 0",
                ));
            }
            if (start > end && step > 0) || (start < end && step < 0) {
                return Ok(Value::List(thin_vec![]));
            }

            let length = (end - start) / step + 1;
            #[allow(clippy::cast_lossless)]
            if length > u32::MAX as i64 {
                return Err(String::from("Range too large"));
            }
            if step > 0 {
                return Ok(Value::Arc(Arc::new(Value::List(
                    (start..=end)
                        .step_by(step as usize)
                        .map(Value::Int)
                        .collect(),
                ))));
            }
            Ok(Value::Arc(Arc::new(Value::List(
                (end..=start)
                    .rev()
                    .step_by((-step) as usize)
                    .map(Value::Int)
                    .collect(),
            ))))
        }

        _ => unreachable!(),
    }
}

fn coalesce(
    _: &Runtime,
    args: ThinVec<Value>,
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
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Map(map)) => Ok(Value::List(
            map.keys().cloned().map(Value::String).collect(),
        )),
        Some(Value::Node(id)) => Ok(Value::List(
            runtime
                .get_node_attrs(id)
                .keys()
                .cloned()
                .map(Value::String)
                .collect::<ThinVec<_>>(),
        )),
        Some(Value::Relationship(rel)) => Ok(Value::List(
            runtime
                .get_relationship_attrs(rel.0)
                .keys()
                .cloned()
                .map(Value::String)
                .collect::<ThinVec<_>>(),
        )),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn sin(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).sin())),
        Some(Value::Float(f)) => Ok(Value::Float(f.sin())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn cos(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).cos())),
        Some(Value::Float(f)) => Ok(Value::Float(f.cos())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn tan(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).tan())),
        Some(Value::Float(f)) => Ok(Value::Float(f.tan())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn cot(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => {
            let val = n as f64;
            Ok(Value::Float(val.cos() / val.sin()))
        }
        Some(Value::Float(f)) => Ok(Value::Float(f.cos() / f.sin())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn asin(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).asin())),
        Some(Value::Float(f)) => Ok(Value::Float(f.asin())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn acos(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).acos())),
        Some(Value::Float(f)) => Ok(Value::Float(f.acos())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn atan(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).atan())),
        Some(Value::Float(f)) => Ok(Value::Float(f.atan())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn atan2(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Int(y)), Some(Value::Int(x))) => Ok(Value::Float((y as f64).atan2(x as f64))),
        (Some(Value::Float(y)), Some(Value::Float(x))) => Ok(Value::Float(y.atan2(x))),
        (Some(Value::Int(y)), Some(Value::Float(x))) => Ok(Value::Float((y as f64).atan2(x))),
        (Some(Value::Float(y)), Some(Value::Int(x))) => Ok(Value::Float(y.atan2(x as f64))),
        (Some(Value::Null), Some(_)) | (Some(_), Some(Value::Null)) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn degrees(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).to_degrees())),
        Some(Value::Float(f)) => Ok(Value::Float(f.to_degrees())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn radians(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => Ok(Value::Float((n as f64).to_radians())),
        Some(Value::Float(f)) => Ok(Value::Float(f.to_radians())),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn pi(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    debug_assert!(args.is_empty());
    Ok(Value::Float(std::f64::consts::PI))
}

fn haversin(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Int(n)) => {
            let val = n as f64;
            Ok(Value::Float((1.0 - val.cos()) / 2.0))
        }
        Some(Value::Float(f)) => Ok(Value::Float((1.0 - f.cos()) / 2.0)),
        Some(Value::Null) => Ok(Value::Null),

        _ => unreachable!(),
    }
}

fn is_empty(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
        Some(Value::Null) => Ok(Value::Null),
        Some(Value::String(s)) => Ok(Value::Bool(s.is_empty())),
        Some(Value::List(v)) => Ok(Value::Bool(v.is_empty())),
        Some(Value::Map(m)) => Ok(Value::Bool(m.is_empty())),

        _ => unreachable!(),
    }
}

fn to_boolean(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    match args.into_iter().next() {
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
        _ => Ok(Value::Null),
    }
}

fn relationship_type(
    runtime: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Relationship(rel)) => runtime
            .get_relationship_type(rel.0)
            .map_or_else(|| Ok(Value::Null), |type_name| Ok(Value::String(type_name))),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn nodes(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Path(values)) => Ok(Value::List(
            values
                .iter()
                .filter_map(|v| {
                    if let Value::Node(_) = v {
                        Some(v.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        )),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn relationships(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Path(values)) => Ok(Value::List(
            values
                .iter()
                .filter_map(|v| {
                    if let Value::Relationship(_) = v {
                        Some(v.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        )),
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn vecf32(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    fn build_vecf32(vec: &[Value]) -> Result<Value, String> {
        // Validate that all elements are numeric (Int or Float)
        // Matching C implementation:  SIArray_AllOfType(arr, SI_NUMERIC)
        for v in vec {
            if !matches!(v, Value::Int(_) | Value::Float(_)) {
                return Err("vectorf32 expects an array of numbers".to_string());
            }
        }

        // All elements are numeric, convert to f32 vector
        Ok(Value::VecF32(
            vec.iter().map(|v| v.get_numeric() as f32).collect(),
        ))
    }

    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::List(vec)) => build_vecf32(&vec),
        Some(Value::Arc(arc)) => match &*arc {
            Value::List(vec) => build_vecf32(vec),
            _ => unreachable!(),
        },
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn point(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Map(map)) => {
            // Extract latitude
            let latitude = map
                .get_str("latitude")
                .ok_or_else(|| String::from("point() requires 'latitude' field"))?;

            let latitude = match latitude {
                Value::Float(f) => *f as f32,
                Value::Int(i) => *i as f32,
                _ => {
                    return Err(format!(
                        "Type mismatch: 'latitude' must be a number, got {}",
                        latitude.name()
                    ));
                }
            };

            // Extract longitude
            let longitude = map
                .get_str("longitude")
                .ok_or_else(|| String::from("point() requires 'longitude' field"))?;

            let longitude = match longitude {
                Value::Float(f) => *f as f32,
                Value::Int(i) => *i as f32,
                _ => {
                    return Err(format!(
                        "Type mismatch: 'longitude' must be a number, got {}",
                        longitude.name()
                    ));
                }
            };

            let point = Point::new(latitude, longitude);
            point.validate()?;
            Ok(Value::Point(point))
        }
        Some(Value::Null) => Ok(Value::Null),
        _ => unreachable!(),
    }
}

fn exists(
    _: &Runtime,
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match iter.next() {
        Some(Value::Null) => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
    }
}

//
// Internal functions
//

fn internal_starts_with(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next()) {
        (Some(Value::Bool(is_not)), Some(Value::Null)) => Ok(Value::Bool(!is_not)),
        (Some(Value::Bool(is_not)), Some(_)) => Ok(Value::Bool(is_not)),

        _ => unreachable!(),
    }
}

fn internal_regex_matches(
    _: &Runtime,
    args: ThinVec<Value>,
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
    args: ThinVec<Value>,
) -> Result<Value, String> {
    let mut iter = args.into_iter();
    match (iter.next(), iter.next(), iter.next()) {
        (Some(Value::List(alts)), Some(else_), None) => {
            for pair in alts.chunks(2) {
                match (&pair[0], &pair[1]) {
                    (Value::Bool(false) | Value::Null, _) => {}
                    (_, result) => return Ok(result.clone()),
                }
            }
            Ok(else_)
        }
        (Some(value), Some(alt), Some(else_)) => {
            let Value::List(alts) = alt else {
                unreachable!()
            };
            for pair in alts.chunks(2) {
                if let [condition, result] = pair
                    && *condition == value
                {
                    return Ok(result.clone());
                }
            }
            Ok(else_)
        }

        _ => unreachable!(),
    }
}

fn db_labels(
    runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_labels()
            .into_iter()
            .map(|l| {
                let mut map = OrderMap::default();
                map.insert(Arc::new(String::from("label")), Value::String(l));
                Value::Map(map)
            })
            .collect(),
    ))
}

fn db_types(
    runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_types()
            .into_iter()
            .map(|t| {
                let mut map = OrderMap::default();
                map.insert(Arc::new(String::from("relationshipType")), Value::String(t));
                Value::Map(map)
            })
            .collect(),
    ))
}

fn db_properties(
    runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .get_attrs()
            .into_iter()
            .map(|p| {
                let mut map = OrderMap::default();
                map.insert(Arc::new(String::from("propertyKey")), Value::String(p));
                Value::Map(map)
            })
            .collect(),
    ))
}

fn db_indexes(
    runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(
        runtime
            .g
            .borrow()
            .index_info()
            .into_iter()
            .map(
                |IndexInfo {
                     label,
                     status,
                     fields,
                 }| {
                    let mut map = OrderMap::default();
                    map.insert(Arc::new(String::from("label")), Value::String(label));
                    map.insert(
                        Arc::new(String::from("properties")),
                        Value::List(fields.keys().map(|f| Value::String(f.clone())).collect()),
                    );
                    let mut types_map = OrderMap::default();
                    for (attr, fields) in fields {
                        let mut types = thin_vec![];
                        for field in fields {
                            match field.ty {
                                IndexType::Range => {
                                    types.push(Value::String(Arc::new(String::from("RANGE"))));
                                }
                                IndexType::Fulltext => {
                                    types.push(Value::String(Arc::new(String::from("FULLTEXT"))));
                                }
                                IndexType::Vector => {
                                    types.push(Value::String(Arc::new(String::from("VECTOR"))));
                                }
                            }
                        }
                        types_map.insert(attr, Value::List(types));
                    }
                    map.insert(Arc::new(String::from("types")), Value::Map(types_map));
                    map.insert(Arc::new(String::from("options")), Value::Null);
                    map.insert(Arc::new(String::from("language")), Value::Null);
                    map.insert(Arc::new(String::from("stopwords")), Value::Null);
                    map.insert(
                        Arc::new(String::from("entitytype")),
                        Value::String(Arc::new(String::from("NODE"))),
                    );
                    map.insert(
                        Arc::new(String::from("status")),
                        if let IndexStatus::UnderConstruction(current, total) = status {
                            Value::String(Arc::new(format!(
                                "[Indexing] {current}/{total}: UNDER CONSTRUCTION"
                            )))
                        } else {
                            Value::String(Arc::new(String::from("OPERATIONAL")))
                        },
                    );
                    map.insert(Arc::new(String::from("info")), Value::Null);

                    Value::Map(map)
                },
            )
            .collect(),
    ))
}

fn db_fulltext_create_node_index(
    _runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(thin_vec![]))
}

fn db_fulltext_drop_node_index(
    _runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(thin_vec![]))
}

fn db_fulltext_query_nodes(
    _runtime: &Runtime,
    _args: ThinVec<Value>,
) -> Result<Value, String> {
    Ok(Value::List(thin_vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::runtime::Runtime;
    use std::sync::Arc;
    use thin_vec::thin_vec;

    /// Get a Runtime reference for testing.
    ///
    /// SAFETY: All functions being tested have `_runtime` parameters (never dereferenced).
    /// We use a dangling pointer that's properly aligned but never accessed.
    fn get_test_runtime() -> &'static Runtime {
        unsafe {
            // Create a dangling but aligned pointer
            // SAFETY: This is safe because _runtime is never dereferenced in any function
            &*(std::ptr::NonNull::dangling().as_ptr())
        }
    }

    #[test]
    fn test_value_to_integer_scientific_notation() {
        let runtime = get_test_runtime();

        // THE PRIMARY BUG FIX: Scientific notation strings without decimal point
        // This was failing before because the code checked for '.' which doesn't exist in '1e-13'
        let result = value_to_integer(
            runtime,
            thin_vec![Value::String(Arc::new("1e-13".to_string()))],
        )
        .unwrap();
        assert_eq!(result, Value::Int(0));

        let result = value_to_integer(
            runtime,
            thin_vec![Value::String(Arc::new("1.5e-10".to_string()))],
        )
        .unwrap();
        assert_eq!(result, Value::Int(0));

        // Positive exponents
        let result = value_to_integer(
            runtime,
            thin_vec![Value::String(Arc::new("1e2".to_string()))],
        )
        .unwrap();
        assert_eq!(result, Value::Int(100));

        let result = value_to_integer(
            runtime,
            thin_vec![Value::String(Arc::new("1.5e2".to_string()))],
        )
        .unwrap();
        assert_eq!(result, Value::Int(150));

        // Uppercase E
        let result = value_to_integer(
            runtime,
            thin_vec![Value::String(Arc::new("1E2".to_string()))],
        )
        .unwrap();
        assert_eq!(result, Value::Int(100));
    }

    #[test]
    fn test_value_to_integer_boundary_values() {
        let runtime = get_test_runtime();

        // Test i64::MAX - should parse successfully as string
        let args = thin_vec![Value::String(Arc::new("9223372036854775807".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(i64::MAX));

        // Test i64::MIN - should parse successfully as string
        let args = thin_vec![Value::String(Arc::new("-9223372036854775808".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(i64::MIN));

        // Test i64::MAX + 1 - should return NULL (out of range)
        let args = thin_vec![Value::String(Arc::new("9223372036854775808".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Test i64::MIN - 1 - should return NULL (out of range)
        let args = thin_vec![Value::String(Arc::new("-9223372036854775809".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Test way out of range (u64::MAX+1) - should return NULL
        let args = thin_vec![Value::String(Arc::new("18446744073709551616".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Test way out of range negative - should return NULL
        let args = thin_vec![Value::String(Arc::new("-18446744073709551616".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_value_to_integer_large_valid_numbers() {
        let runtime = get_test_runtime();

        // Large positive number within range
        let args = thin_vec![Value::String(Arc::new("1790460441484152222".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1_790_460_441_484_152_222));

        // Large negative number within range
        let args = thin_vec![Value::String(Arc::new("-1790460441484152222".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(-1_790_460_441_484_152_222));
    }

    #[test]
    fn test_value_to_integer_basic_types() {
        let runtime = get_test_runtime();

        // Integer passthrough
        let args = thin_vec![Value::Int(42)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(42));

        // Float floor
        let args = thin_vec![Value::Float(1.9)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        let args = thin_vec![Value::Float(1.1)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        // Boolean true
        let args = thin_vec![Value::Bool(true)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        // Boolean false
        let args = thin_vec![Value::Bool(false)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(0));

        // Null
        let args = thin_vec![Value::Null];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_value_to_integer_string_conversions() {
        let runtime = get_test_runtime();

        // Simple integer string
        let args = thin_vec![Value::String(Arc::new("1".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        // String with decimal (floor)
        let args = thin_vec![Value::String(Arc::new("1.1".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        let args = thin_vec![Value::String(Arc::new("1.9".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1));

        // Empty string (NULL)
        let args = thin_vec![Value::String(Arc::new(String::new()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Invalid string (NULL)
        let args = thin_vec![Value::String(Arc::new("z".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Invalid string (NULL)
        let args = thin_vec![Value::String(Arc::new("abc".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_value_to_integer_negative_numbers() {
        let runtime = get_test_runtime();

        // Negative integer string
        let args = thin_vec![Value::String(Arc::new("-1".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(-1));

        // Negative float string - floor(-1.9) = -2 (matches C behavior)
        let args = thin_vec![Value::String(Arc::new("-1.9".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(-2)); // Changed from -1 to -2

        // Negative float value - floor(-1.9) = -2
        let args = thin_vec![Value::Float(-1.9)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(-2)); // Changed from -1 to -2
    }
    #[test]
    fn test_value_to_integer_edge_cases() {
        let runtime = get_test_runtime();

        // Zero
        let args = thin_vec![Value::Int(0)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(0));

        // Zero float
        let args = thin_vec![Value::Float(0.0)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(0));

        // Zero string
        let args = thin_vec![Value::String(Arc::new("0".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(0));

        // Negative zero float
        let args = thin_vec![Value::Float(-0.0)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_value_to_integer_special_floats() {
        let runtime = get_test_runtime();

        // Infinity should return NULL (non-finite)
        let args = thin_vec![Value::Float(f64::INFINITY)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // Negative infinity should return NULL (non-finite)
        let args = thin_vec![Value::Float(f64::NEG_INFINITY)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);

        // NaN should return NULL (non-finite)
        let args = thin_vec![Value::Float(f64::NAN)];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_value_to_integer_ci_failing_case() {
        let runtime = get_test_runtime();

        // THIS IS THE EXACT FAILING CASE FROM CI
        // x = Decimal(1) becomes 1.0 as float, formatted as "1.0"
        // The query becomes: RETURN toInteger(1.0), toInteger('1.0')
        // Expected: [[1, 1]]
        // Got:      [[1, None]]

        // Test toInteger('1.0')
        let args = thin_vec![Value::String(Arc::new("1.0".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(
            result,
            Value::Int(1),
            "toInteger('1.0') should return 1, not None"
        );

        // Test toInteger('2.0')
        let args = thin_vec![Value::String(Arc::new("2.0".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(
            result,
            Value::Int(2),
            "toInteger('2.0') should return 2, not None"
        );

        // Test toInteger('0.0')
        let args = thin_vec![Value::String(Arc::new("0.0".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(
            result,
            Value::Int(0),
            "toInteger('0.0') should return 0, not None"
        );

        // Test toInteger('-1.0')
        let args = thin_vec![Value::String(Arc::new("-1.0".to_string()))];
        let result = value_to_integer(runtime, args).unwrap();
        assert_eq!(
            result,
            Value::Int(-1),
            "toInteger('-1.0') should return -1, not None"
        );
    }

    #[test]
    fn test_datetime_component_extraction() {
        let runtime = get_test_runtime();

        // Create a datetime: 1984-10-11T12:31:14.645Z (466345874645 ms from epoch)
        let datetime_ms = 466_345_874_645_i64;
        let datetime = Value::Datetime(datetime_ms);

        // Test year
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("year".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1984));

        // Test month
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("month".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(10));

        // Test day
        let args = thin_vec![datetime.clone(), Value::String(Arc::new("day".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(11));

        // Test hour
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("hour".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(12));

        // Test minute
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("minute".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(31));

        // Test second
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("second".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(14));

        // Test millisecond
        let args = thin_vec![
            datetime.clone(),
            Value::String(Arc::new("millisecond".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(645));

        // Test unknown component
        let args = thin_vec![datetime, Value::String(Arc::new("invalid".to_string()))];
        let result = property(runtime, args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown datetime component"));
    }

    #[test]
    fn test_date_component_extraction() {
        let runtime = get_test_runtime();

        // Create a date: 1984-10-11 (466300800000 ms from epoch - midnight UTC)
        let date_ms = 466_300_800_000_i64;
        let date = Value::Date(date_ms);

        // Test year
        let args = thin_vec![date.clone(), Value::String(Arc::new("year".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(1984));

        // Test month
        let args = thin_vec![date.clone(), Value::String(Arc::new("month".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(10));

        // Test day
        let args = thin_vec![date.clone(), Value::String(Arc::new("day".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(11));

        // Test unknown component
        let args = thin_vec![date, Value::String(Arc::new("hour".to_string()))];
        let result = property(runtime, args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown date component"));
    }

    #[test]
    fn test_time_component_extraction() {
        let runtime = get_test_runtime();

        // Create a time: 12:31:14.645 (45074645 ms from epoch)
        let time_ms = 45_074_645_i64;
        let time = Value::Time(time_ms);

        // Test hour
        let args = thin_vec![time.clone(), Value::String(Arc::new("hour".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(12));

        // Test minute
        let args = thin_vec![time.clone(), Value::String(Arc::new("minute".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(31));

        // Test second
        let args = thin_vec![time.clone(), Value::String(Arc::new("second".to_string()))];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(14));

        // Test millisecond
        let args = thin_vec![
            time.clone(),
            Value::String(Arc::new("millisecond".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(645));

        // Test unknown component
        let args = thin_vec![time.clone(), Value::String(Arc::new("year".to_string()))];
        let result = property(runtime, args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown time component"));
    }

    #[test]
    fn test_duration_component_extraction() {
        let runtime = get_test_runtime();

        // Create a duration: 2 days, 3 hours, 15 minutes, 30 seconds (189330000 ms)
        let duration_ms = 189_330_000_i64;
        let duration = Value::Duration(duration_ms);

        // Test days
        let args = thin_vec![
            duration.clone(),
            Value::String(Arc::new("days".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(2)); // 189330000 / (24*60*60*1000) = 2

        // Test hours
        let args = thin_vec![
            duration.clone(),
            Value::String(Arc::new("hours".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(52)); // 189330000 / (60*60*1000) = 52

        // Test minutes
        let args = thin_vec![
            duration.clone(),
            Value::String(Arc::new("minutes".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(3155)); // 189330000 / (60*1000) = 3155

        // Test seconds
        let args = thin_vec![
            duration.clone(),
            Value::String(Arc::new("seconds".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(189_330)); // 189330000 / 1000 = 189330

        // Test milliseconds
        let args = thin_vec![
            duration.clone(),
            Value::String(Arc::new("milliseconds".to_string()))
        ];
        let result = property(runtime, args).unwrap();
        assert_eq!(result, Value::Int(189_330_000));

        // Test unknown component
        let args = thin_vec![duration, Value::String(Arc::new("years".to_string()))];
        let result = property(runtime, args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown duration component"));
    }
}
