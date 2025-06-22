#![allow(clippy::cast_precision_loss)]

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::discriminant;
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};
use std::rc::Rc;

use ordermap::OrderMap;

use crate::ast::Variable;
use crate::functions::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct RcValue(Rc<Value>);

impl RcValue {
    #[must_use]
    pub fn new(value: Value) -> Self {
        Self(Rc::new(value))
    }

    #[must_use]
    pub fn list(vec: Vec<Self>) -> Self {
        Self::new(Value::List(vec))
    }

    #[must_use]
    pub fn string(str: Rc<String>) -> Self {
        Self::new(Value::String(str))
    }

    #[must_use]
    pub fn node(id: u64) -> Self {
        Self::new(Value::Node(id))
    }

    #[must_use]
    pub fn int(i: i64) -> Self {
        Self::new(Value::Int(i))
    }

    #[must_use]
    pub fn float(f: f64) -> Self {
        Self::new(Value::Float(f))
    }

    #[must_use]
    pub fn bool(b: bool) -> Self {
        Self::new(Value::Bool(b))
    }

    #[must_use]
    pub fn map(map: OrderMap<Rc<String>, Self>) -> Self {
        Self::new(Value::Map(map))
    }

    #[must_use]
    pub fn null() -> Self {
        Self::new(Value::Null)
    }

    #[must_use]
    pub fn relationship(
        id: u64,
        from_id: u64,
        to_id: u64,
    ) -> Self {
        Self::new(Value::Relationship(id, from_id, to_id))
    }

    #[must_use]
    pub fn path(path: Vec<Self>) -> Self {
        Self::new(Value::Path(path))
    }
}

impl Hash for RcValue {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.0.hash(state);
    }
}

impl Deref for RcValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Rc<String>),
    List(Vec<RcValue>),
    Map(OrderMap<Rc<String>, RcValue>),
    Node(u64),
    Relationship(u64, u64, u64),
    Path(Vec<RcValue>),
}

impl Value {
    #[must_use]
    #[inline]
    pub fn get_numeric(&self) -> f64 {
        match &self {
            Self::Int(i) => *i as f64,
            Self::Float(f) => *f,
            _ => unreachable!("avg expects numeric value"),
        }
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
            Self::Relationship(x, _, _) => {
                7.hash(state);
                x.hash(state);
            }
            Self::Path(x) => {
                8.hash(state);
                x.hash(state);
            }
        }
    }
}

#[derive(Default)]
pub struct Env(Vec<RcValue>);

impl Env {
    pub fn insert(
        &mut self,
        key: &Variable,
        value: RcValue,
    ) {
        while self.0.len() <= key.id as _ {
            self.0.push(RcValue::null());
        }
        self.0[key.id as usize] = value;
    }

    #[must_use]
    pub fn get(
        &self,
        key: &Variable,
    ) -> Option<RcValue> {
        self.0.get(key.id as usize).cloned()
    }

    pub fn merge(
        &mut self,
        other: Self,
    ) {
        while self.0.len() < other.0.len() {
            self.0.push(RcValue::null());
        }
        for (key, value) in other.0.into_iter().enumerate() {
            if *value == Value::Null {
                continue;
            }
            self.0[key] = value;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = RcValue> {
        self.0.iter().cloned()
    }
}

impl Hash for Env {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        for (key, value) in self.0.iter().enumerate() {
            if **value == Value::Null {
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

impl Add for RcValue {
    type Output = Result<Self, String>;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (&*self, &*rhs) {
            (Value::Null, _) | (_, Value::Null) => Ok(Self::null()),
            (Value::Int(a), Value::Int(b)) => Ok(Self::int(a.wrapping_add(*b))),
            (Value::Float(a), Value::Float(b)) => Ok(Self::float(a + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Self::float(a + *b as f64)),
            (Value::Int(a), Value::Float(b)) => Ok(Self::float(*a as f64 + b)),

            (Value::List(a), Value::List(b)) => {
                Ok(Self::list(a.iter().chain(b).cloned().collect()))
            }
            (Value::List(l), _) => {
                let mut l = l.clone();
                if l.is_empty() {
                    Ok(Self::list(vec![rhs]))
                } else {
                    l.push(rhs);
                    Ok(Self::list(l))
                }
            }
            (_, Value::List(l)) => {
                let mut new_list = vec![self];
                new_list.extend(l.clone());
                Ok(Self::list(new_list))
            }
            (Value::String(a), Value::String(b)) => Ok(Self::string(Rc::new(format!("{a}{b}")))),
            (Value::String(s), Value::Int(i)) => Ok(Self::string(Rc::new(format!("{s}{i}")))),
            (Value::String(s), Value::Float(f)) => Ok(Self::string(Rc::new(format!("{s}{f}")))),
            (Value::String(s), Value::Bool(f)) => Ok(Self::string(Rc::new(format!("{s}{f}")))),
            (a, b) => Err(format!(
                "Unexpected types for add operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Sub for RcValue {
    type Output = Result<Self, String>;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (&*self, &*rhs) {
            (Value::Null, _) | (_, Value::Null) => Ok(Self::null()),
            (Value::Int(a), Value::Int(b)) => Ok(Self::int(a.wrapping_sub(*b))),
            (Value::Float(a), Value::Float(b)) => Ok(Self::float(a - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Self::float(a - *b as f64)),
            (Value::Int(a), Value::Float(b)) => Ok(Self::float(*a as f64 - b)),
            (a, b) => Err(format!(
                "Unexpected types for sub operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Mul for RcValue {
    type Output = Result<Self, String>;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (&*self, &*rhs) {
            (Value::Null, _) | (_, Value::Null) => Ok(Self::null()),
            (Value::Int(a), Value::Int(b)) => Ok(Self::int(a.wrapping_mul(*b))),
            (Value::Float(a), Value::Float(b)) => Ok(Self::float(a * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Self::float(a * *b as f64)),
            (Value::Int(a), Value::Float(b)) => Ok(Self::float(*a as f64 * b)),
            (a, b) => Err(format!(
                "Unexpected types for mul operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Div for RcValue {
    type Output = Result<Self, String>;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (&*self, &*rhs) {
            (Value::Null, _) | (_, Value::Null) => Ok(Self::null()),
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 && *a == 0 {
                    Ok(Self::float(f64::NAN))
                } else if *b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::int(a.wrapping_div(*b)))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 && *a == 0.0 {
                    Ok(Self::float(f64::NAN))
                } else if *b == 0.0 {
                    Ok(Self::float(f64::INFINITY.copysign(*a)))
                } else {
                    Ok(Self::float(a / b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 && *a == 0.0 {
                    Ok(Self::float(f64::NAN))
                } else if *b == 0 {
                    Ok(Self::float(f64::INFINITY.copysign(*a)))
                } else {
                    Ok(Self::float(a / *b as f64))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 && *a == 0 {
                    Ok(Self::float(f64::NAN))
                } else if *b == 0.0 {
                    Ok(Self::float(f64::INFINITY.copysign(*a as _)))
                } else {
                    Ok(Self::float(*a as f64 / b))
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

impl Rem for RcValue {
    type Output = Result<Self, String>;

    fn rem(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (&*self, &*rhs) {
            (Value::Null, _) | (_, Value::Null) => Ok(Self::null()),
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::int(a.wrapping_rem(*b)))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::float(a % b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::float(a % *b as f64))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::float(*a as f64 % b))
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
            Self::Relationship(_, _, _) => 1 << 2,
            Self::Path(_) => 1 << 4,
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
            Self::Relationship(_, _, _) => String::from("Relationship"),
            Self::Path(_) => String::from("Path"),
        }
    }

    #[must_use]
    pub fn compare_value(
        &self,
        b: &Self,
    ) -> (Ordering, DisjointOrNull) {
        match (self, b) {
            (Self::Int(a), Self::Int(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Bool(a), Self::Bool(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Float(a), Self::Float(b)) => compare_floats(*a, *b),
            (Self::String(a), Self::String(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::List(a), Self::List(b)) | (Self::Path(a), Self::Path(b)) => {
                Self::compare_list(a, b)
            }
            (Self::Map(a), Self::Map(b)) => Self::compare_map(a, b),
            (Self::Node(a), Self::Node(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Relationship(a, _, _), Self::Relationship(b, _, _)) => {
                (a.cmp(b), DisjointOrNull::None)
            }
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

    fn compare_list(
        a: &[RcValue],
        b: &[RcValue],
    ) -> (Ordering, DisjointOrNull) {
        let array_a_len = a.len();
        let array_b_len = b.len();
        if array_a_len == 0 && array_b_len == 0 {
            return (Ordering::Equal, DisjointOrNull::None);
        }
        let min_len = array_a_len.min(array_b_len);

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
        if null_counter > 0 && array_a_len == array_b_len {
            return (first_not_equal, DisjointOrNull::ComparedNull);
        }

        // if there was a difference in some member, without any null compare
        if first_not_equal != Ordering::Equal {
            return (first_not_equal, DisjointOrNull::None);
        }

        (array_a_len.cmp(&array_b_len), DisjointOrNull::None)
    }

    fn compare_map(
        a: &OrderMap<Rc<String>, RcValue>,
        b: &OrderMap<Rc<String>, RcValue>,
    ) -> (Ordering, DisjointOrNull) {
        let a_key_count = a.len();
        let b_key_count = b.len();
        if a_key_count != b_key_count {
            return (a_key_count.cmp(&b_key_count), DisjointOrNull::None);
        }

        // sort keys
        let mut a_keys: Vec<&Rc<String>> = a.keys().collect();
        a_keys.sort();
        let mut b_keys: Vec<&Rc<String>> = b.keys().collect();
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

    #[must_use]
    pub fn get_type(&self) -> Type {
        match self {
            Self::Null => Type::Null,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
            Self::List(_) => Type::List(Box::new(Type::Any)),
            Self::Map(_) => Type::Map,
            Self::Node(_) => Type::Node,
            Self::Relationship(_, _, _) => Type::Relationship,
            Self::Path(_) => Type::Path,
        }
    }

    #[must_use]
    pub fn validate_of_type(
        &self,
        arg_type: &Type,
    ) -> Option<(Type, Type)> {
        match (self, arg_type) {
            (Self::List(vs), Type::List(ty)) => {
                for v in vs {
                    if let Some(res) = v.validate_of_type(ty) {
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
            | (Self::Relationship(_, _, _), Type::Relationship)
            | (Self::Path(_), Type::Path)
            | (_, Type::Any) => None,
            (v, Type::Optional(ty)) => v.validate_of_type(ty),
            (v, Type::Union(tys)) => {
                for ty in tys {
                    v.validate_of_type(ty)?;
                }
                Some((v.get_type(), Type::Union(tys.clone())))
            }
            (v, e) => Some((v.get_type(), e.clone())),
        }
    }
}

pub trait Contains {
    fn contains(
        &self,
        value: RcValue,
    ) -> RcValue;
}

impl Contains for Vec<RcValue> {
    fn contains(
        &self,
        value: RcValue,
    ) -> RcValue {
        let mut is_null = false;
        for item in self {
            let (res, dis) = value.compare_value(item);
            is_null = is_null || dis == DisjointOrNull::ComparedNull;
            if res == Ordering::Equal {
                return if dis == DisjointOrNull::ComparedNull {
                    RcValue::null()
                } else {
                    RcValue::bool(true)
                };
            }
        }
        if is_null {
            RcValue::null()
        } else {
            RcValue::bool(false)
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
        values: &[RcValue],
    ) -> bool {
        let mut hasher = DefaultHasher::new();

        // Hash values, replace int with float because 0 and 0.0 should dedup
        // we have to normalize -0.0 to 0.0 and -0 to 0 as well
        // so that we can dedup them
        // for now we do not normalize recursive values
        for v in values {
            match &**v {
                Value::Int(i) => {
                    // Normalize -0 to 0 for integer values
                    let normalized_i = if *i == 0 { 0 } else { *i };
                    RcValue::float(normalized_i as _).hash(&mut hasher);
                }
                Value::Float(f) => {
                    // Normalize -0.0 to 0.0 for float values
                    let normalized_f = if *f == 0.0 { 0.0 } else { *f };
                    RcValue::float(normalized_f).hash(&mut hasher);
                }
                _ => v.hash(&mut hasher),
            }
        }

        let hash = hasher.finish();

        let mut seen = self.seen.borrow_mut();
        if seen.contains(&hash) {
            true
        } else {
            seen.insert(hash);
            false
        }
    }
}
