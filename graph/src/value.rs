use std::collections::BTreeMap;

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Node(u64),
    Relationship(u64, u64, u64),
}
