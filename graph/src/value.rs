use std::collections::HashMap;

pub trait ValueIter: Iterator<Item = u64> + Clone {}

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Node(u64),
    Link(u64),
}
