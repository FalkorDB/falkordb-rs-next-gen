use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Node(u64),
    Relationship(u64, u64, u64),
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
        }
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
            (Self::String(a), Self::String(b)) => Ok(Self::String(a + &b)),
            (Self::String(s), Self::Int(i)) => Ok(Self::String(s + &i.to_string())),
            (Self::String(s), Self::Float(f)) => Ok(Self::String(s + &f.to_string())),
            (Self::String(s), Self::Bool(f)) => Ok(Self::String(s + &f.to_string().to_lowercase())),
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
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Int(a.wrapping_div(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a / b))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a / b as f64))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a as f64 / b))
                }
            }
            (a, b) => Err(format!(
                "Unexpected types for div operator ({}, {})",
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
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Int(a.wrapping_rem(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a % b))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a % b as f64))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Self::Float(a as f64 % b))
                }
            }
            (a, b) => Err(format!(
                "Unexpected types for rem operator ({}, {})",
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
            Self::Null => 0,
            Self::Bool(_) => 1,
            Self::Int(_) => 2,
            Self::Float(_) => 3,
            Self::String(_) => 4,
            Self::List(_) => 5,
            Self::Map(_) => 6,
            Self::Node(_) => 7,
            Self::Relationship(_, _, _) => 8,
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
            Self::Null => "Null".to_string(),
            Self::Bool(_) => "Boolean".to_string(),
            Self::Int(_) => "Integer".to_string(),
            Self::Float(_) => "Float".to_string(),
            Self::String(_) => "String".to_string(),
            Self::List(_) => "List".to_string(),
            Self::Map(_) => "Map".to_string(),
            Self::Node(_) => "Node".to_string(),
            Self::Relationship(_, _, _) => "Relationship".to_string(),
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
        a: &BTreeMap<String, Self>,
        b: &BTreeMap<String, Self>,
    ) -> (Ordering, DisjointOrNull) {
        let a_key_count = a.len();
        let b_key_count = b.len();
        if a_key_count != b_key_count {
            return (a_key_count.cmp(&b_key_count), DisjointOrNull::None);
        }

        // sort keys
        let mut a_keys: Vec<&String> = a.keys().collect();
        a_keys.sort();
        let mut b_keys: Vec<&String> = b.keys().collect();
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
