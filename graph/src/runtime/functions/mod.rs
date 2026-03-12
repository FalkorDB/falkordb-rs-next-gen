//! Built-in Cypher function registry and type definitions.
//!
//! This module is the root of the functions subsystem. It owns the global
//! function registry and the core types that every other module in the
//! crate depends on (`FnType`, `Type`, `GraphFn`, ...).  The actual
//! function implementations live in categorised submodules:
//!
//! ```text
//! functions/
//! +----- mod.rs          Registry, types, init_functions()
//! |
//! +----- entity.rs       Graph-entity inspection (labels, id, properties, ...)
//! +----- string.rs       String manipulation (toLower, substring, replace, ...)
//! +----- math.rs         Numeric / general math (abs, ceil, sqrt, range, ...)
//! +----- trig.rs         Trigonometric functions (sin, cos, atan2, ...)
//! +----- conversion.rs   Type casting (toInteger, toFloat, toString, ...)
//! +----- list.rs         List operations (head, tail, size, reverse, ...)
//! +----- aggregation.rs  Aggregation functions (count, sum, avg, collect, ...)
//! +----- spatial.rs      Spatial / vector (point, distance, vecf32)
//! +----- path.rs         Path decomposition (nodes, relationships)
//! +----- internal.rs     Internal-only operators (starts_with, case, ...)
//! +----- procedures.rs   CALL procedures (db.labels, db.indexes, ...)
//! ```
//!
//! ## Function registry
//!
//! [`init_functions`] populates a `OnceLock<Functions>` singleton at startup.
//! Each entry maps a **lowercase** function name to an `Arc<GraphFn>` that
//! stores the implementation pointer, argument types, return type, and whether
//! the function is scalar / aggregation / procedure / internal.
//!
//! ```text
//! +--------------------------+
//! |  OnceLock<Functions>     |
//! |  HashMap<String,         |
//! |          Arc<GraphFn>>   |
//! +------+-------------------+
//!        |
//!        |  "tolower" --> GraphFn { func: string::string_to_lower, ... }
//!        |  "count"   --> GraphFn { func: aggregation::count,      ... }
//!        |  "db.labels" -> GraphFn { func: procedures::db_labels,  ... }
//!        |  ...
//! ```
//!
//! Callers obtain a reference via [`get_functions`] and look up entries
//! with `Functions::get(name, &FnType)`.
//!
//! ## Key types
//!
//! | Type          | Purpose                                              |
//! |---------------|------------------------------------------------------|
//! | [`FnType`]    | Discriminates Function / Internal / Procedure / Agg  |
//! | [`Type`]      | Cypher type system (Int, String, Union, Optional ...) |
//! | [`GraphFn`]   | A registered function: name + pointer + metadata     |
//! | [`Functions`] | The registry itself (wraps a `HashMap`)               |

/// Defines a Cypher runtime function and registers it in a single declaration.
///
/// Invoke inside a `register(funcs: &mut Functions)` function.  The macro
/// defines a nested function with the `RuntimeFn` signature and emits the
/// matching `funcs.add()` or `funcs.add_var_len()` call.
///
/// # Arms
///
/// | Discriminator           | Pattern               | `FnType` produced       |
/// |-------------------------|-----------------------|-------------------------|
/// | *(none)*                | scalar, fixed args    | `Function`              |
/// | `var_arg:`              | scalar, var-len args  | `Function`              |
/// | `agg_init:`             | aggregation, no final | `Aggregation(v, None)`  |
/// | `agg_init:` `finalizer:`| aggregation + final   | `Aggregation(v, Some)`  |
/// | `internal,`             | internal operator     | `Internal`              |
/// | `procedure:`            | read-only procedure   | `Procedure(yields)`     |
/// | `write procedure:`      | write procedure       | `Procedure(yields)`     |
macro_rules! cypher_fn {
    // ── Scalar function (FnType::Function, write=false, fixed args) ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            false,
            vec![$($arg),*],
            FnType::Function,
            $ret,
        );
    };

    // ── Variable-length argument function (add_var_len) ──
    ($funcs:ident, $name:expr,
     var_arg: $arg_type:expr,
     ret: $ret:expr,
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add_var_len(
            $name,
            $fn_name,
            false,
            $arg_type,
            FnType::Function,
            $ret,
        );
    };

    // ── Aggregation without finalizer ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     agg_init: $init:expr,
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            false,
            vec![$($arg),*],
            FnType::Aggregation($init, None),
            $ret,
        );
    };

    // ── Aggregation with finalizer ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     agg_init: $init:expr,
     finalizer: $finalizer:expr,
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            false,
            vec![$($arg),*],
            FnType::Aggregation($init, Some(Box::new($finalizer))),
            $ret,
        );
    };

    // ── Internal function (FnType::Internal) ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     internal,
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            false,
            vec![$($arg),*],
            FnType::Internal,
            $ret,
        );
    };

    // ── Read-only procedure ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     procedure: [$($yield_col:expr),* $(,)?],
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            false,
            vec![$($arg),*],
            FnType::Procedure(vec![$(String::from($yield_col)),*]),
            $ret,
        );
    };

    // ── Write procedure ──
    ($funcs:ident, $name:expr,
     args: [$($arg:expr),* $(,)?],
     ret: $ret:expr,
     write procedure: [$($yield_col:expr),* $(,)?],
     $(#[$attr:meta])*
     fn $fn_name:ident($rt:pat, $args:pat) $body:block
    ) => {
        $(#[$attr])*
        fn $fn_name(
            $rt: &Runtime,
            $args: ThinVec<Value>,
        ) -> Result<Value, String>
        $body

        $funcs.add(
            $name,
            $fn_name,
            true,
            vec![$($arg),*],
            FnType::Procedure(vec![$(String::from($yield_col)),*]),
            $ret,
        );
    };
}

mod aggregation;
mod conversion;
mod entity;
mod internal;
mod list;
mod math;
mod path;
mod procedures;
mod spatial;
mod string;
mod trig;

pub use math::apply_pow;

use crate::runtime::{
    runtime::Runtime,
    value::{Value, ValueTypeOf},
};
use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, OnceLock},
};
use thin_vec::ThinVec;

/// Function pointer type for runtime function implementations.
type RuntimeFn = fn(&Runtime, ThinVec<Value>) -> Result<Value, String>;

/// Classification of function types.
pub enum FnType {
    /// Regular scalar function (e.g., `toUpper()`)
    Function,
    /// Internal function not exposed to users
    Internal,
    /// Procedure that returns a result set (e.g., `db.labels()`)
    Procedure(Vec<String>),
    /// Aggregation function with initial value and optional finalizer
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

impl Type {
    /// Returns true if this type can include a boolean value.
    /// This mirrors `AR_EXP_ReturnsBoolean` in the C implementation:
    /// returns true if the type is Bool, Null, Any, or a Union/Optional
    /// containing Bool/Null/Any.
    #[must_use]
    pub fn can_return_boolean(&self) -> bool {
        match self {
            Self::Bool | Self::Null | Self::Any => true,
            Self::Union(types) => types.iter().any(Self::can_return_boolean),
            Self::Optional(inner) => inner.can_return_boolean(),
            _ => false,
        }
    }

    #[must_use]
    pub fn can_return_entity(&self) -> bool {
        match self {
            Self::Node | Self::Relationship | Self::Path | Self::Any => true,
            Self::Union(types) => types.iter().any(Self::can_return_entity),
            Self::Optional(inner) => inner.can_return_entity(),
            _ => false,
        }
    }
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
    pub ret_type: Type,
}

impl GraphFn {
    #[must_use]
    pub fn new(
        name: &str,
        func: RuntimeFn,
        write: bool,
        args_type: FnArguments,
        fn_type: FnType,
        ret_type: Type,
    ) -> Self {
        Self {
            name: String::from(name),
            func,
            write,
            args_type,
            fn_type,
            ret_type,
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
    pub fn validate_args_type<V: Borrow<Value>>(
        &self,
        args: &[V],
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
                    } else if let Some((actual, expected)) =
                        args[i].borrow().value_of_type(arg_type)
                    {
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
        ret_type: Type,
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
            ret_type,
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
        ret_type: Type,
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
            ret_type,
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

pub fn init_functions() -> Result<(), Functions> {
    let mut funcs = Functions::new();

    entity::register(&mut funcs);
    string::register(&mut funcs);
    math::register(&mut funcs);
    trig::register(&mut funcs);
    conversion::register(&mut funcs);
    list::register(&mut funcs);
    aggregation::register(&mut funcs);
    spatial::register(&mut funcs);
    path::register(&mut funcs);
    internal::register(&mut funcs);
    procedures::register(&mut funcs);

    FUNCTIONS.set(funcs)
}

pub fn get_functions() -> &'static Functions {
    FUNCTIONS.get().expect("Functions not initialized")
}
