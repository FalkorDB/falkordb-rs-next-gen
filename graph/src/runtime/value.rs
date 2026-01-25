#![allow(clippy::cast_precision_loss)]

use json_escape::escape_str;
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Rem, Sub},
    sync::Arc,
};

use thin_vec::{ThinVec, thin_vec};

use crate::{
    ast::Variable,
    graph::graph::{LabelId, NodeId, RelationshipId, TypeId},
    runtime::{functions::Type, ordermap::OrderMap},
};

/// A trait for formatting values as JSON, similar to Display but for JSON output
pub trait DisplayJson {
    fn fmt_json(
        &self,
        f: &mut fmt::Formatter<'_>,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> fmt::Result;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeletedNode {
    pub labels: HashSet<LabelId>,
    pub attrs: OrderMap<Arc<String>, Value>,
}

impl DeletedNode {
    #[must_use]
    pub const fn new(
        labels: HashSet<LabelId>,
        attrs: OrderMap<Arc<String>, Value>,
    ) -> Self {
        Self { labels, attrs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeletedRelationship {
    pub type_id: TypeId,
    pub attrs: OrderMap<Arc<String>, Value>,
}

impl DeletedRelationship {
    #[must_use]
    pub const fn new(
        type_id: TypeId,
        attrs: OrderMap<Arc<String>, Value>,
    ) -> Self {
        Self { type_id, attrs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    #[must_use]
    pub const fn new(
        latitude: f32,
        longitude: f32,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Validates that the point coordinates are within valid ranges
    pub fn validate(&self) -> Result<(), String> {
        // Check for NaN or infinite values first
        if !self.latitude.is_finite() {
            return Err(format!(
                "latitude must be a finite number, got {}",
                self.latitude
            ));
        }
        if !self.longitude.is_finite() {
            return Err(format!(
                "longitude must be a finite number, got {}",
                self.longitude
            ));
        }
        // Then check range bounds
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(format!(
                "latitude should be within the range -90.0 to 90.0, got {}",
                self.latitude
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(format!(
                "longitude should be within the range -180.0 to 180.0, got {}",
                self.longitude
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Arc<String>),
    List(ThinVec<Self>),
    Map(OrderMap<Arc<String>, Self>),
    Node(NodeId),
    Relationship(Box<(RelationshipId, NodeId, NodeId)>),
    Path(ThinVec<Self>),
    VecF32(ThinVec<f32>),
    Point(Point),
    Datetime(i64), // Unix timestamp in milliseconds
    Date(i64),     // Unix timestamp in milliseconds (midnight UTC)
    Time(i64),     // Unix timestamp in milliseconds (time from epoch)
    Duration(i64), // Duration length in milliseconds
    Arc(Arc<Self>),
}

impl Value {
    #[must_use]
    #[inline]
    pub fn get_numeric(&self) -> f64 {
        match &self {
            Self::Int(i) => *i as f64,
            Self::Float(f) => *f,
            Self::Null => 0.0,
            _ => unreachable!("Expected numeric value, got {}", self.name()),
        }
    }

    #[must_use]
    pub fn format_datetime(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    // Format date as ISO-8601: "2025-04-14"
    #[must_use]
    pub fn format_date(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    // Format time as ISO-8601: "06:08:21"
    #[must_use]
    pub fn format_time(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%H:%M:%S").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    #[must_use]
    pub fn format_duration(duration_ms: i64) -> String {
        let seconds = duration_ms / 1000;
        format!("PT{seconds}S")
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        match self {
            Self::Null => {
                0.hash(state);
            }
            Self::Bool(x) => {
                1.hash(state);
                x.hash(state);
            }
            Self::Int(x) => {
                2.hash(state);
                x.hash(state);
            }
            Self::Float(x) => {
                2.hash(state);
                let casted = *x as i64;
                let diff = *x - casted as f64;
                if diff == 0.0 {
                    casted.hash(state);
                } else {
                    x.to_bits().hash(state);
                }
            }
            Self::String(x) => {
                3.hash(state);
                x.hash(state);
            }
            Self::List(x) => {
                4.hash(state);
                x.hash(state);
            }
            Self::Map(x) => {
                5.hash(state);
                x.hash(state);
            }
            Self::Node(x) => {
                6.hash(state);
                x.hash(state);
            }
            Self::Relationship(rel) => {
                7.hash(state);
                rel.0.hash(state);
            }
            Self::Path(x) => {
                8.hash(state);
                x.hash(state);
            }
            Self::VecF32(x) => {
                9.hash(state);
                for f in x {
                    f.to_bits().hash(state);
                }
            }
            Self::Point(p) => {
                10.hash(state);
                p.latitude.to_bits().hash(state);
                p.longitude.to_bits().hash(state);
            }
            Self::Datetime(x) => {
                11.hash(state);
                x.hash(state);
            }
            Self::Date(x) => {
                12.hash(state);
                x.hash(state);
            }
            Self::Time(x) => {
                13.hash(state);
                x.hash(state);
            }
            Self::Duration(x) => {
                14.hash(state);
                x.hash(state);
            }
            Self::Arc(x) => {
                x.hash(state);
            }
        }
    }
}

#[derive(Default)]
pub struct Env(Vec<Value>);

impl Env {
    pub fn insert(
        &mut self,
        key: &Variable,
        value: Value,
    ) {
        while self.0.len() <= key.id as _ {
            self.0.push(Value::Null);
        }
        self.0[key.id as usize] = value;
    }

    #[must_use]
    pub fn get(
        &self,
        key: &Variable,
    ) -> Option<Value> {
        self.0.get(key.id as usize).cloned()
    }

    /// Takes ownership of a value from the environment, replacing it with `Null`.
    ///
    /// This method is designed for aggregation optimizations where we need to transfer
    /// ownership of large accumulated values (like lists with millions of items) without
    /// cloning them. By replacing the environment entry with `Null`, we ensure the value
    /// can be moved (not cloned) to the aggregation function.
    ///
    /// # Returns
    /// - `Some(value)` if the key exists and contains a non-Null value
    /// - `None` if the key doesn't exist or already contains `Null`
    ///
    /// # Usage
    /// Prefer this over `get()` when:
    /// - You need exclusive ownership of a value
    /// - The value is expensive to clone (e.g., large collections)
    /// - The environment slot won't be read again before being overwritten
    pub fn take(
        &mut self,
        key: &Variable,
    ) -> Option<Value> {
        self.0.get_mut(key.id as usize).and_then(|value| {
            match std::mem::replace(value, Value::Null) {
                Value::Null => None,
                v => Some(v),
            }
        })
    }

    pub fn merge(
        &mut self,
        other: Self,
    ) {
        while self.0.len() < other.0.len() {
            self.0.push(Value::Null);
        }
        for (key, value) in other.0.into_iter().enumerate() {
            if value == Value::Null {
                continue;
            }
            self.0[key] = value;
        }
    }
}

impl AsRef<Vec<Value>> for Env {
    fn as_ref(&self) -> &Vec<Value> {
        &self.0
    }
}

impl Hash for Env {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        for (key, value) in self.0.iter().enumerate() {
            if *value == Value::Null {
                continue;
            }
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_add(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
            (Self::List(a), Self::List(b)) => Ok(Self::List(a.into_iter().chain(b).collect())),
            (Self::List(mut l), rhs) => {
                l.push(rhs);
                Ok(Self::List(l))
            }
            (lhs, Self::List(l)) => {
                let mut new_list = thin_vec![lhs];
                new_list.extend(l);
                Ok(Self::List(new_list))
            }
            (Self::Map(a), Self::Map(b)) => {
                let mut new_map = a;
                for (k, v) in b.iter() {
                    new_map.insert(k.clone(), v.clone());
                }
                Ok(Self::Map(new_map))
            }
            (Self::String(a), Self::String(b)) => Ok(Self::String(Arc::new(format!("{a}{b}")))),
            (Self::String(s), Self::Int(i)) => Ok(Self::String(Arc::new(format!("{s}{i}")))),
            (Self::String(s), Self::Float(f)) => Ok(Self::String(Arc::new(format!("{s}{f:.6}")))),
            (Self::String(s), Self::Bool(b)) => Ok(Self::String(Arc::new(format!("{s}{b}")))),

            (Self::Int(i), Self::String(s)) => Ok(Self::String(Arc::new(format!("{i}{s}")))),
            (Self::Float(f), Self::String(s)) => Ok(Self::String(Arc::new(format!("{f:.6}{s}")))),
            (Self::Bool(b), Self::String(s)) => Ok(Self::String(Arc::new(format!("{b}{s}")))),

            (a, b) => Err(format!(
                "Unexpected types for add operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_sub(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a - b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
            (a, b) => Err(format!(
                "Unexpected types for sub operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_mul(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a * b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
            (a, Self::Int(_) | Self::Float(_)) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                a.name(),
            )),
            (Self::Int(_) | Self::Float(_), b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                b.name(),
            )),
            (a, _) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                a.name(),
            )),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Int(a.wrapping_div(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a / b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a / b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 / b)),
            (a, b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Rem for Value {
    type Output = Result<Self, String>;

    fn rem(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Int(a.wrapping_rem(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                let result = a % b;
                // If result is a whole number (no fractional part) and within i64 range, return Int
                if result.fract() == 0.0 
                    && result.is_finite() 
                    && result >= i64::MIN as f64 
                    && result <= i64::MAX as f64 {
                    #[allow(clippy::cast_possible_truncation)]
                    Ok(Self::Int(result as i64))
                } else {
                    Ok(Self::Float(result))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                let result = a % (b as f64);
                if result.fract() == 0.0 
                    && result.is_finite() 
                    && result >= i64::MIN as f64 
                    && result <= i64::MAX as f64 {
                    #[allow(clippy::cast_possible_truncation)]
                    Ok(Self::Int(result as i64))
                } else {
                    Ok(Self::Float(result))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                let result = (a as f64) % b;
                if result.fract() == 0.0 
                    && result.is_finite() 
                    && result >= i64::MIN as f64 
                    && result <= i64::MAX as f64 {
                    #[allow(clippy::cast_possible_truncation)]
                    Ok(Self::Int(result as i64))
                } else {
                    Ok(Self::Float(result))
                }
            }
            (a, b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

trait OrderedEnum {
    fn order(&self) -> u32;
}

impl OrderedEnum for Value {
    fn order(&self) -> u32 {
        match self {
            Self::Null => 1 << 15,
            Self::Bool(_) => 1 << 12,
            Self::Int(_) => 1 << 13,
            Self::Float(_) => 1 << 14,
            Self::String(_) => 1 << 11,
            Self::List(_) => 1 << 3,
            Self::Map(_) => 1 << 0,
            Self::Node(_) => 1 << 1,
            Self::Relationship(_) => 1 << 2,
            Self::Path(_) => 1 << 4,
            Self::Point(_) => 1 << 5,
            Self::Datetime(_) => 1 << 6,
            Self::Date(_) => 1 << 7,
            Self::Time(_) => 1 << 8,
            Self::Duration(_) => 1 << 10,
            Self::VecF32(_) => 1 << 18,

            Self::Arc(inner) => inner.order(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DisjointOrNull {
    Disjoint,
    ComparedNull,
    NaN,
    None,
}

pub trait CompareValue {
    fn compare_value(
        &self,
        other: &Self,
    ) -> (Ordering, DisjointOrNull);
}

impl CompareValue for Value {
    fn compare_value(
        &self,
        b: &Self,
    ) -> (Ordering, DisjointOrNull) {
        match (self, b) {
            (Self::Bool(a), Self::Bool(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Float(a), Self::Float(b)) => compare_floats(*a, *b),
            (Self::String(a), Self::String(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::List(a), Self::List(b)) | (Self::Path(a), Self::Path(b)) => {
                Self::compare_list(a, b)
            }
            (Self::Map(a), Self::Map(b)) => Self::compare_map(a, b),
            (Self::Node(a), Self::Node(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Relationship(rel_a), Self::Relationship(rel_b)) => {
                (rel_a.0.cmp(&rel_b.0), DisjointOrNull::None)
            }
            (Self::Point(a), Self::Point(b)) => match a.latitude.partial_cmp(&b.latitude) {
                Some(Ordering::Equal) => a
                    .longitude
                    .partial_cmp(&b.longitude)
                    .map_or((Ordering::Less, DisjointOrNull::NaN), |ord| {
                        (ord, DisjointOrNull::None)
                    }),
                Some(ord) => (ord, DisjointOrNull::None),
                None => (Ordering::Less, DisjointOrNull::NaN),
            },
            (Self::Int(a), Self::Int(b))
            | (Self::Datetime(a), Self::Datetime(b))
            | (Self::Date(a), Self::Date(b))
            | (Self::Time(a), Self::Time(b))
            | (Self::Duration(a), Self::Duration(b)) => (a.cmp(b), DisjointOrNull::None),
            // the inputs have different type - compare them if they
            // are both numerics of differing types
            (Self::Int(i), Self::Float(f)) => compare_floats(*i as f64, *f),
            (Self::Float(f), Self::Int(i)) => compare_floats(*f, *i as f64),
            (Self::Null, _) | (_, Self::Null) => {
                (self.order().cmp(&b.order()), DisjointOrNull::ComparedNull)
            }
            _ => (self.order().cmp(&b.order()), DisjointOrNull::Disjoint),
        }
    }
}

pub trait ValueTypeOf {
    fn value_of_type(
        &self,
        arg_type: &Type,
    ) -> Option<(Type, Type)>;
}

impl ValueTypeOf for Value {
    fn value_of_type(
        &self,
        arg_type: &Type,
    ) -> Option<(Type, Type)> {
        match (self, arg_type) {
            (Self::List(vs), Type::List(ty)) => {
                for v in vs {
                    if let Some(res) = v.value_of_type(ty) {
                        return Some(res);
                    }
                }
                None
            }
            (Self::Null, Type::Null)
            | (Self::Bool(_), Type::Bool)
            | (Self::Int(_), Type::Int)
            | (Self::Float(_), Type::Float)
            | (Self::String(_), Type::String)
            | (Self::Map(_), Type::Map)
            | (Self::Node(_), Type::Node)
            | (Self::Relationship(_), Type::Relationship)
            | (Self::Path(_), Type::Path)
            | (Self::Point(_), Type::Point)
            | (Value::Datetime(_), Type::Datetime)
            | (Value::Date(_), Type::Date)
            | (Value::Time(_), Type::Time)
            | (Value::Duration(_), Type::Duration)
            | (_, Type::Any) => None,
            (Self::Arc(inner), ty) => {
                // If the inner value is a Rc, we need to check its type
                inner.value_of_type(ty)
            }
            (v, Type::Optional(ty)) => v.value_of_type(ty),
            (v, Type::Union(tys)) => {
                for ty in tys {
                    v.value_of_type(ty)?;
                }
                Some((v.get_type(), Type::Union(tys.clone())))
            }
            (v, e) => Some((v.get_type(), e.clone())),
        }
    }
}

pub trait ValueGetType {
    fn get_type(&self) -> Type;
}

impl ValueGetType for Value {
    fn get_type(&self) -> Type {
        match self {
            Self::Null => Type::Null,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
            Self::List(_) => Type::List(Box::new(Type::Any)),
            Self::Map(_) => Type::Map,
            Self::Node(_) => Type::Node,
            Self::Relationship(_) => Type::Relationship,
            Self::Path(_) => Type::Path,
            Self::VecF32(_) => Type::VecF32,
            Self::Point(_) => Type::Point,
            Self::Datetime(_) => Type::Datetime,
            Self::Date(_) => Type::Date,
            Self::Time(_) => Type::Time,
            Self::Duration(_) => Type::Duration,
            Self::Arc(inner) => inner.get_type(),
        }
    }
}

impl Value {
    pub(crate) fn name(&self) -> String {
        match self {
            Self::Null => String::from("Null"),
            Self::Bool(_) => String::from("Boolean"),
            Self::Int(_) => String::from("Integer"),
            Self::Float(_) => String::from("Float"),
            Self::String(_) => String::from("String"),
            Self::List(_) => String::from("List"),
            Self::Map(_) => String::from("Map"),
            Self::Node(_) => String::from("Node"),
            Self::Relationship(_) => String::from("Edge"),
            Self::Path(_) => String::from("Path"),
            Self::VecF32(_) => String::from("VecF32"),
            Self::Point(_) => String::from("Point"),
            Self::Datetime(_) => String::from("Datetime"),
            Self::Date(_) => String::from("Date"),
            Self::Time(_) => String::from("Time"),
            Self::Duration(_) => String::from("Duration"),
            Self::Arc(inner) => inner.name(),
        }
    }

    /// Convert Value to JSON string representation
    pub fn to_json_string(
        &self,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> String {
        struct JsonWrapper<'a> {
            value: &'a Value,
            runtime: &'a crate::runtime::runtime::Runtime,
        }

        impl fmt::Display for JsonWrapper<'_> {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                self.value.fmt_json(f, self.runtime)
            }
        }

        JsonWrapper {
            value: self,
            runtime,
        }
        .to_string()
    }

    fn compare_list<T: CompareValue>(
        a: &[T],
        b: &[T],
    ) -> (Ordering, DisjointOrNull) {
        let len_a = a.len();
        let len_b = b.len();
        if len_a == 0 && len_b == 0 {
            return (Ordering::Equal, DisjointOrNull::None);
        }
        let min_len = len_a.min(len_b);

        let mut first_not_equal = Ordering::Equal;
        let mut null_counter: usize = 0;
        let mut not_equal_counter: usize = 0;

        for (a_value, b_value) in a.iter().zip(b) {
            let (compare_result, disjoint_or_null) = a_value.compare_value(b_value);
            if disjoint_or_null != DisjointOrNull::None {
                if disjoint_or_null == DisjointOrNull::ComparedNull {
                    null_counter += 1;
                }
                not_equal_counter += 1;
                if first_not_equal == Ordering::Equal {
                    first_not_equal = compare_result;
                }
            } else if compare_result != Ordering::Equal {
                not_equal_counter += 1;
                if first_not_equal == Ordering::Equal {
                    first_not_equal = compare_result;
                }
            }
        }

        // if all the elements in the shared range yielded false comparisons
        if not_equal_counter == min_len && null_counter < not_equal_counter {
            return (first_not_equal, DisjointOrNull::None);
        }

        // if there was a null comparison on non-disjoint arrays
        if null_counter > 0 && len_a == len_b {
            return (first_not_equal, DisjointOrNull::ComparedNull);
        }

        // if there was a difference in some member, without any null compare
        if first_not_equal != Ordering::Equal {
            return (first_not_equal, DisjointOrNull::None);
        }

        (len_a.cmp(&len_b), DisjointOrNull::None)
    }

    fn compare_map(
        a: &OrderMap<Arc<String>, Self>,
        b: &OrderMap<Arc<String>, Self>,
    ) -> (Ordering, DisjointOrNull) {
        let a_key_count = a.len();
        let b_key_count = b.len();
        if a_key_count != b_key_count {
            return (a_key_count.cmp(&b_key_count), DisjointOrNull::None);
        }

        // sort keys
        let mut a_keys: Vec<&Arc<String>> = a.keys().collect();
        a_keys.sort();
        let mut b_keys: Vec<&Arc<String>> = b.keys().collect();
        b_keys.sort();

        // iterate over keys count
        for (a_key, b_key) in a_keys.iter().zip(b_keys) {
            if *a_key != b_key {
                return ((*a_key).cmp(b_key), DisjointOrNull::None);
            }
        }

        // iterate over values
        for key in a_keys {
            let a_value = &a[key];
            let b_value = &b[key];
            let (compare_result, disjoint_or_null) = a_value.compare_value(b_value);
            if disjoint_or_null == DisjointOrNull::ComparedNull
                || disjoint_or_null == DisjointOrNull::Disjoint
            {
                return (Ordering::Equal, disjoint_or_null);
            } else if compare_result != Ordering::Equal {
                return (compare_result, disjoint_or_null);
            }
        }
        (Ordering::Equal, DisjointOrNull::None)
    }
}

impl DisplayJson for Value {
    fn fmt_json(
        &self,
        f: &mut fmt::Formatter<'_>,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => {
                if fl.is_nan() || fl.is_infinite() {
                    write!(f, "null")
                } else {
                    write!(f, "{fl}")
                }
            }
            Self::String(s) => write_json_string(f, s),
            Self::List(list) => {
                write!(f, "[")?;
                for (i, v) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "]")
            }
            Self::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write_json_string(f, k)?;
                    write!(f, ":")?;
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "}}")
            }
            Self::Node(id) => write_node_json(f, runtime, *id, true),
            Self::Relationship(rel) => {
                let (rel_id, start_id, end_id) = **rel;
                let rel_id_u64 = u64::from(rel_id);
                let properties = runtime.get_relationship_attrs(rel_id);
                let type_name = runtime
                    .get_relationship_type(rel_id)
                    .unwrap_or_else(|| Arc::new(String::new()));

                write!(
                    f,
                    r#"{{"type":"relationship","id":{rel_id_u64},"relationship":"#
                )?;
                write_json_string(f, &type_name)?;
                write!(f, r#","properties":{{"#)?;

                for (i, (k, v)) in properties.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write_json_string(f, k)?;
                    write!(f, ":")?;
                    v.fmt_json(f, runtime)?;
                }

                write!(f, r#"}},"start":"#)?;
                write_node_json(f, runtime, start_id, false)?;
                write!(f, r#","end":"#)?;
                write_node_json(f, runtime, end_id, false)?;
                write!(f, "}}")
            }
            Self::Path(values) => {
                write!(f, "[")?;
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "]")
            }
            Self::VecF32(vec) => {
                write!(f, "[")?;
                for (i, fl) in vec.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    if fl.is_nan() || fl.is_infinite() {
                        write!(f, "null")?;
                    } else {
                        write!(f, "{fl}")?;
                    }
                }
                write!(f, "]")
            }
            Self::Point(point) => {
                write!(f, r#"{{"crs":"wgs-84","latitude":"#)?;
                write!(f, "{:.6}", f64::from(point.latitude))?;
                write!(f, r#","longitude":"#)?;
                write!(f, "{:.6}", f64::from(point.longitude))?;
                write!(f, r#","height": null}}"#)
            }
            Self::Datetime(ts) => {
                let formatted = Self::format_datetime(*ts);
                write_json_string(f, &formatted)
            }
            Self::Date(ts) => {
                let formatted = Self::format_date(*ts);
                write_json_string(f, &formatted)
            }
            Self::Time(ts) => {
                let formatted = Self::format_time(*ts);
                write_json_string(f, &formatted)
            }
            Self::Duration(dur) => {
                let formatted = Self::format_duration(*dur);
                write_json_string(f, &formatted)
            }
            Self::Arc(inner) => inner.fmt_json(f, runtime),
        }
    }
}

/// Write a string in JSON format with proper escaping
fn write_json_string(
    f: &mut fmt::Formatter<'_>,
    s: &str,
) -> fmt::Result {
    write!(f, "\"")?;
    for chunk in escape_str(s) {
        write!(f, "{chunk}")?;
    }
    write!(f, "\"")
}

/// Write a node in JSON format with or without the "type" field
fn write_node_json(
    f: &mut fmt::Formatter<'_>,
    runtime: &crate::runtime::runtime::Runtime,
    id: NodeId,
    include_type: bool,
) -> fmt::Result {
    let node_id = u64::from(id);
    let labels = runtime.get_node_labels(id);
    let properties = runtime.get_node_attrs(id);

    write!(f, "{{")?;

    if include_type {
        write!(f, r#""type":"node","#)?;
    }

    write!(f, r#""id":{node_id},"labels":["#)?;

    for (i, label) in labels.iter().enumerate() {
        if i > 0 {
            write!(f, ",")?;
        }
        write_json_string(f, label)?;
    }

    write!(f, r#"],"properties":{{"#)?;

    for (i, (k, v)) in properties.iter().enumerate() {
        if i > 0 {
            write!(f, ",")?;
        }
        write_json_string(f, k)?;
        write!(f, ":")?;
        v.fmt_json(f, runtime)?;
    }

    write!(f, "}}}}")
}

pub trait Contains {
    fn contains(
        &self,
        value: Value,
    ) -> Value;
}

impl Contains for ThinVec<Value> {
    fn contains(
        &self,
        value: Value,
    ) -> Value {
        let mut is_null = false;
        for item in self {
            let (res, dis) = value.compare_value(item);
            is_null = is_null || dis == DisjointOrNull::ComparedNull;
            if res == Ordering::Equal {
                return if dis == DisjointOrNull::ComparedNull {
                    Value::Null
                } else {
                    Value::Bool(true)
                };
            }
        }
        if is_null {
            Value::Null
        } else {
            Value::Bool(false)
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        let (ordering, disjoint_or_null) = self.compare_value(other);
        if disjoint_or_null == DisjointOrNull::ComparedNull {
            None
        } else {
            Some(ordering)
        }
    }
}

fn compare_floats(
    a: f64,
    b: f64,
) -> (Ordering, DisjointOrNull) {
    match a.partial_cmp(&b) {
        Some(Ordering::Equal) => (Ordering::Equal, DisjointOrNull::None),
        Some(Ordering::Less) => (Ordering::Less, DisjointOrNull::None),
        Some(Ordering::Greater) => (Ordering::Greater, DisjointOrNull::None),
        None => (Ordering::Less, DisjointOrNull::NaN),
    }
}
#[derive(Default, Debug)]
pub struct ValuesDeduper {
    seen: RefCell<HashSet<u64>>,
}

impl ValuesDeduper {
    #[must_use]
    pub fn is_seen(
        &self,
        values: &[Value],
    ) -> bool {
        let mut hasher = DefaultHasher::new();
        values.hash(&mut hasher);
        let hash = hasher.finish();
        self.check_and_insert_hash(hash)
    }

    #[must_use]
    pub fn has_hash(
        &self,
        hash: u64,
    ) -> bool {
        self.check_and_insert_hash(hash)
    }

    fn check_and_insert_hash(
        &self,
        hash: u64,
    ) -> bool {
        let mut seen = self.seen.borrow_mut();
        if seen.contains(&hash) {
            true
        } else {
            seen.insert(hash);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_int_float_whole_result() {
        // 5 % 2.5 = 0.0 → should return Int(0)
        let result = (Value::Int(5) % Value::Float(2.5)).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_modulo_float_int_whole_result() {
        // 10.0 % 5 = 0.0 → should return Int(0)
        let result = (Value::Float(10.0) % Value::Int(5)).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_modulo_float_float_fractional_result() {
        // 5.5 % 2.5 = 0.5 → should return Float(0.5)
        let result = (Value::Float(5.5) % Value::Float(2.5)).unwrap();
        assert_eq!(result, Value::Float(0.5));
    }

    #[test]
    fn test_modulo_int_float_fractional_result() {
        // 5 % 2.0 = 1.0 → should return Int(1)
        let result = (Value::Int(5) % Value::Float(2.0)).unwrap();
        assert_eq!(result, Value::Int(1));
    }

    #[test]
    fn test_modulo_float_float_whole_result() {
        // 10.0 % 2.5 = 0.0 → should return Int(0)
        let result = (Value::Float(10.0) % Value::Float(2.5)).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_modulo_large_float_stays_float() {
        // Very large whole number result should stay as Float if it exceeds i64 range
        let large_value = (i64::MAX as f64) * 2.0;
        let result = (Value::Float(large_value) % Value::Float(1.0)).unwrap();
        // Should remain a Float since it exceeds i64::MAX
        assert!(matches!(result, Value::Float(_)));
    }
}
