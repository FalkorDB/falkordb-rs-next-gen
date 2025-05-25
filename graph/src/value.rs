use hashbrown::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;

use ordermap::OrderMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Rc<String>),
    List(Vec<Value>),
    Map(OrderMap<Rc<String>, Value>),
    Node(u64),
    Relationship(u64, u64, u64),
    Path(Vec<Value>),
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        match self {
            Self::Null => todo!(),
            Self::Bool(x) => x.hash(state),
            Self::Int(x) => x.hash(state),
            Self::Float(x) => x.to_string().hash(state),
            Self::String(x) => x.hash(state),
            Self::List(x) => x.hash(state),
            Self::Map(x) => x.hash(state),
            Self::Node(x) | Self::Relationship(x, _, _) => x.hash(state),
            Self::Path(x) => x.hash(state),
        }
    }
}

pub struct Env(HashMap<Rc<String>, Value>);

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(
        &mut self,
        key: Rc<String>,
        value: Value,
    ) {
        self.0.insert(key, value);
    }

    #[must_use]
    pub fn get(
        &self,
        key: &Rc<String>,
    ) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn take(
        &mut self,
        key: &Rc<String>,
    ) -> Option<Value> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Rc<String>, &Value)> {
        self.0.iter()
    }
}

impl Hash for Env {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by_key(|(k, _)| *k);
        for (key, value) in entries {
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
            (Self::List(mut l), scalar) => {
                if l.is_empty() {
                    Ok(Self::List(vec![scalar]))
                } else {
                    l.push(scalar);
                    Ok(Self::List(l))
                }
            }
            (s, Self::List(l)) => {
                let mut new_list = vec![s];
                new_list.extend(l);
                Ok(Self::List(new_list))
            }
            (Self::String(a), Self::String(b)) => {
                Ok(Self::String(Rc::new(String::from(format!("{}{}", a, b)))))
            }
            (Self::String(s), Self::Int(i)) => Ok(Self::String(Rc::new(format!("{}{}", s, i)))),
            (Self::String(s), Self::Float(f)) => Ok(Self::String(Rc::new(format!("{}{}", s, f)))),
            (Self::String(s), Self::Bool(f)) => Ok(Self::String(Rc::new(format!("{}{}", s, f)))),
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
            (a, b) => Err(format!(
                "Unexpected types for mul operator ({}, {})",
                a.name(),
                b.name()
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
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a / b))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a / b as f64))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a as f64 / b))
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
                if b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a % b))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a % b as f64))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Float(a as f64 % b))
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

#[derive(Debug, PartialEq)]
enum DisjointOrNull {
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

    fn compare_value(
        &self,
        b: &Self,
    ) -> (Ordering, DisjointOrNull) {
        match (self, b) {
            (Self::Int(a), Self::Int(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Bool(a), Self::Bool(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Float(a), Self::Float(b)) => compare_floats(*a, *b),
            (Self::String(a), Self::String(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::List(a), Self::List(b)) => Self::compare_list(a, b),
            (Self::Map(a), Self::Map(b)) => Self::compare_map(a, b),
            (Self::Node(a), Self::Node(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Relationship(a, b, c), Self::Relationship(a1, b1, c1)) => {
                ((a, b, c).cmp(&(a1, b1, c1)), DisjointOrNull::None)
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
        a: &[Self],
        b: &[Self],
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
        a: &OrderMap<Rc<String>, Self>,
        b: &OrderMap<Rc<String>, Self>,
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
}

pub trait Contains {
    fn contains(
        &self,
        value: &Value,
    ) -> Value;
}

impl Contains for Vec<Value> {
    fn contains(
        &self,
        value: &Value,
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
