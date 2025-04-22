use crate::{
    ast::QueryExprIR, graph::Graph, matrix::Iter, planner::IR, value::Contains, value::Value,
};
use std::cmp::Ordering;
use std::collections::BTreeMap;

type ReadFn = fn(&Graph, &mut Runtime, Value) -> Result<Value, String>;
type WriteFn = fn(&mut Graph, &mut Runtime, Value) -> Result<Value, String>;

pub struct Runtime {
    read_functions: BTreeMap<String, ReadFn>,
    write_functions: BTreeMap<String, WriteFn>,
    iters: Vec<Iter<bool>>,
    parameters: BTreeMap<String, Value>,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}

impl Runtime {
    #[must_use]
    pub fn new(parameters: BTreeMap<String, Value>) -> Self {
        let mut write_functions: BTreeMap<String, WriteFn> = BTreeMap::new();
        let mut read_functions: BTreeMap<String, ReadFn> = BTreeMap::new();

        // write functions
        write_functions.insert("create_node".to_string(), Self::create_node);
        write_functions.insert("create_relationship".to_string(), Self::create_relationship);
        write_functions.insert("delete_entity".to_string(), Self::delete_entity);

        // read functions
        read_functions.insert("create_node_iter".to_string(), Self::create_node_iter);
        read_functions.insert("next_node".to_string(), Self::next_node);
        read_functions.insert("property".to_string(), Self::property);
        read_functions.insert("toInteger".to_string(), Self::value_to_integer);
        read_functions.insert("labels".to_string(), Self::labels);
        read_functions.insert("size".to_string(), Self::size);
        read_functions.insert("head".to_string(), Self::head);
        read_functions.insert("last".to_string(), Self::last);
        read_functions.insert("tail".to_string(), Self::tail);
        read_functions.insert("reverse".to_string(), Self::reverse);
        read_functions.insert("substring".to_string(), Self::substring);
        read_functions.insert("split".to_string(), Self::split);
        read_functions.insert("toLower".to_string(), Self::string_to_lower);
        read_functions.insert("toUpper".to_string(), Self::string_to_upper);
        read_functions.insert("replace".to_string(), Self::string_replace);
        read_functions.insert("left".to_string(), Self::string_left);
        read_functions.insert("ltrim".to_string(), Self::string_ltrim);
        read_functions.insert("right".to_string(), Self::string_right);

        // internal functions are not accessible from Cypher
        read_functions.insert("@starts_with".to_string(), Self::internal_starts_with);
        read_functions.insert("@ends_with".to_string(), Self::internal_ends_with);
        read_functions.insert("@contains".to_string(), Self::internal_contains);
        read_functions.insert("@regex_matches".to_string(), Self::internal_regex_matches);

        // procedures
        read_functions.insert("db.labels".to_string(), Self::db_labels);
        read_functions.insert("db.relationshiptypes".to_string(), Self::db_types);
        read_functions.insert("db.propertykeys".to_string(), Self::db_properties);

        Self {
            write_functions,
            read_functions,
            iters: Vec::new(),
            parameters,
            nodes_created: 0,
            relationships_created: 0,
            nodes_deleted: 0,
            relationships_deleted: 0,
            properties_set: 0,
            properties_removed: 0,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn create_node(
        g: &mut Graph,
        runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (iter.next(), iter.next(), iter.next()) {
                    (Some(Value::List(raw_labels)), Some(Value::Map(attrs)), None) => {
                        let labels = raw_labels
                            .into_iter()
                            .filter_map(|label| {
                                if let Value::String(label) = label {
                                    Some(label)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();
                        runtime.nodes_created += 1;
                        runtime.properties_set += attrs
                            .values()
                            .map(|v| match v {
                                Value::Null => 0,
                                _ => 1,
                            })
                            .sum::<i32>();
                        Ok(g.create_node(&labels, attrs))
                    }
                    _ => Ok(Value::Null),
                }
            }
            _ => Ok(Value::Null),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn delete_entity(
        g: &mut Graph,
        runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(nodes) => {
                for n in nodes {
                    if let Value::Node(id) = n {
                        runtime.nodes_deleted += 1;
                        g.delete_node(id);
                    }
                }
            }
            _ => todo!(),
        }

        Ok(Value::Null)
    }

    fn create_relationship(
        g: &mut Graph,
        runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                    iter.next(),
                ) {
                    (
                        Some(Value::String(relationship_type)),
                        Some(Value::Node(from)),
                        Some(Value::Node(to)),
                        Some(Value::Map(attrs)),
                        None,
                    ) => {
                        runtime.relationships_created += 1;
                        runtime.properties_set += attrs
                            .values()
                            .map(|v| match v {
                                Value::Null => 0,
                                _ => 1,
                            })
                            .sum::<i32>();
                        Ok(g.create_relationship(&relationship_type, from, to, attrs))
                    }
                    _ => todo!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn create_node_iter(
        g: &Graph,
        runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(args) => {
                let mut iter = args.into_iter();
                match (iter.next(), iter.next()) {
                    (Some(Value::List(raw_labels)), None) => {
                        runtime.iters.push(
                            g.get_nodes(
                                raw_labels
                                    .into_iter()
                                    .filter_map(|label| {
                                        if let Value::String(label) = label {
                                            Some(label)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                            )
                            .unwrap(),
                        );
                        Ok(Value::Int(runtime.iters.len() as i64 - 1))
                    }
                    _ => todo!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn next_node(
        _g: &Graph,
        runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(args) => match args.as_slice() {
                [Value::Int(iter)] => runtime.iters[*iter as usize]
                    .next()
                    .map_or_else(|| Ok(Value::Null), |(n, _)| Ok(Value::Node(n))),
                _ => todo!(),
            },
            _ => unreachable!(),
        }
    }

    fn property(
        g: &Graph,
        _runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::Node(node_id), Value::String(property)] => g
                    .get_node_property_id(property)
                    .map_or(Ok(Value::Null), |property_id| {
                        g.get_node_property(*node_id, property_id)
                            .map_or(Ok(Value::Null), |n| Ok(n))
                    }),
                [Value::Map(map), Value::String(property)] => {
                    Ok(map.get(property).unwrap_or(&Value::Null).clone())
                }
                _ => Ok(Value::Null),
            },
            _ => unreachable!(),
        }
    }

    fn labels(
        g: &Graph,
        _runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::Node(node_id)] => Ok(Value::List(
                    g.get_node_label_ids(*node_id)
                        .map(|label_id| Value::String(g.get_label_by_id(label_id).to_string()))
                        .collect(),
                )),
                _ => Ok(Value::Null),
            },
            _ => unreachable!(),
        }
    }

    fn value_to_integer(
        _g: &Graph,
        _runtime: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(params) => match params.as_slice() {
                [Value::String(s)] => s.parse::<i64>().map(Value::Int).or_else(|_| {
                    s.parse::<f64>()
                        .map(|f| Value::Int(f as i64))
                        .or(Ok(Value::Null))
                }),
                [Value::Int(i)] => Ok(Value::Int(*i)),
                [Value::Float(f)] => Ok(Value::Int(*f as i64)),
                [Value::Null] => Ok(Value::Null),
                [Value::Bool(b)] => Ok(Value::Int(if *b { 1 } else { 0 })),
                [arg] => Err(format!(
                    "Type mismatch: expected String, Boolean, Integer, Float, or Null but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for value_to_integer, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn size(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s)] => Ok(Value::Int(s.len() as i64)),
                [Value::List(v)] => Ok(Value::Int(v.len() as i64)),
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List, String, or Null but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for size, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn head(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::List(v)] => {
                    if v.is_empty() {
                        Ok(Value::Null)
                    } else {
                        Ok(v[0].clone())
                    }
                }
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List or Null but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for head, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn last(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::List(v)] => Ok(v.last().unwrap_or(&Value::Null).clone()),
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List or Null but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for last, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn tail(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::List(v)] => {
                    if v.is_empty() {
                        Ok(Value::List(vec![]))
                    } else {
                        Ok(Value::List(v[1..].to_vec()))
                    }
                }
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List or Null but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for tail, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn reverse(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::List(v)] => {
                    let mut v = v.clone();
                    v.reverse();
                    Ok(Value::List(v))
                }
                [Value::Null] => Ok(Value::Null),
                [Value::String(s)] => Ok(Value::String(s.chars().rev().collect())),
                [arg] => Err(format!(
                    "Type mismatch: expected List, String or null, but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for reverse, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn substring(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                // Handle NULL input case
                [Value::Null, _] | [Value::Null, _, _] => Ok(Value::Null),
                // Two-argument version: (string, start)
                [Value::String(s), Value::Int(start)] => {
                    let start = *start;
                    if start < 0 {
                        return Err("start must be a non-negative integer".into());
                    }
                    let start = start as usize;

                    Ok(Value::String(s[start..].to_string()))
                }

                // Three-argument version: (string, start, length)
                [Value::String(s), Value::Int(start), Value::Int(length)] => {
                    let start = *start;
                    let length = *length;
                    if length < 0 {
                        return Err("length must be a non-negative integer".into());
                    }
                    if start < 0 {
                        return Err("start must be a non-negative integer".into());
                    }
                    let start = start as usize;
                    let length = length as usize;

                    let end = start.saturating_add(length).min(s.len());
                    Ok(Value::String(s[start..end].to_string()))
                }

                // Type mismatch handling
                args => Err(format!(
                    "Type mismatch: expected substring(String, Integer) [+ Integer] but got {:?}",
                    args.iter().map(Value::name).collect::<Vec<_>>()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn split(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(string), Value::String(delimiter)] => {
                    if delimiter.is_empty() {
                        // split string to characters
                        let parts: Vec<Value> = string
                            .chars()
                            .map(|c| Value::String(c.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    } else {
                        let parts: Vec<Value> = string
                            .split(delimiter.as_str())
                            .map(|s| Value::String(s.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    }
                }
                [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected 2 String or null arguments, but was {} {}",
                    arg1.name(),
                    arg2.name()
                )),
                [arg] => Err(format!(
                    "Type mismatch: expected 2 String or null arguments, but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected two arguments for split, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_to_lower(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s)] => Ok(Value::String(s.to_lowercase())),
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List, String or null, but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for toLower, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_to_upper(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s)] => Ok(Value::String(s.to_uppercase())),
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected List, String or null, but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for toUpper, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_replace(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [
                    Value::String(s),
                    Value::String(search),
                    Value::String(replacement),
                ] => Ok(Value::String(s.replace(search, replacement))),
                [Value::Null, _, _] | [_, Value::Null, _] | [_, _, Value::Null] => Ok(Value::Null),
                [arg1, arg2, arg3] => Err(format!(
                    "Type mismatch: expected (String, String, String) or null, but was: ({}, {}, {})",
                    arg1.name(),
                    arg2.name(),
                    arg3.name()
                )),
                args => Err(format!(
                    "Expected three arguments for replace, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_left(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::Int(n)] => {
                    if *n < 0 {
                        Err("length must be a non-negative integer".to_string())
                    } else {
                        Ok(Value::String(s.chars().take(*n as usize).collect()))
                    }
                }
                [Value::Null, _] => Ok(Value::Null),
                [_, Value::Null] => Err("length must be a non-negative integer".to_string()),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                args => Err(format!(
                    "Expected two arguments for function 'left', instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_ltrim(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s)] => Ok(Value::String(s.trim_start().to_string())),
                [Value::Null] => Ok(Value::Null),
                [arg] => Err(format!(
                    "Type mismatch: expected String or null, but was {}",
                    arg.name()
                )),
                args => Err(format!(
                    "Expected one argument for ltrim, instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    fn string_right(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::Int(n)] => {
                    if *n < 0 {
                        Err("length must be a non-negative integer".to_string())
                    } else {
                        let start = s.len().saturating_sub(*n as usize);
                        Ok(Value::String(s.chars().skip(start).collect()))
                    }
                }
                [Value::Null, _] => Ok(Value::Null),
                [_, Value::Null] => Err("length must be a non-negative integer".to_string()),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected (String, Integer) or null, but was: ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                args => Err(format!(
                    "Expected two arguments for function 'right', instead {}",
                    args.len()
                )),
            },
            _ => unreachable!(),
        }
    }

    //
    // Internal functions
    //

    fn internal_starts_with(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::String(prefix)] => Ok(Value::Bool(s.starts_with(prefix))),

                [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected String or Null but was ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn internal_ends_with(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::String(suffix)] => Ok(Value::Bool(s.ends_with(suffix))),
                [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: Type mismatch: expected String or Null but was ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn internal_contains(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::String(substring)] => {
                    Ok(Value::Bool(s.contains(substring)))
                }
                [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected String or Null but was ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn internal_regex_matches(
        _: &Graph,
        _: &mut Self,
        args: Value,
    ) -> Result<Value, String> {
        match args {
            Value::List(arr) => match arr.as_slice() {
                [Value::String(s), Value::String(pattern)] => {
                    // Compile the regex pattern
                    match regex::Regex::new(pattern) {
                        Ok(re) => Ok(Value::Bool(re.is_match(s))),
                        Err(e) => Err(format!("Invalid regex pattern: {e}")),
                    }
                }
                [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
                [arg1, arg2] => Err(format!(
                    "Type mismatch: expected (String, String) or null, but was: ({}, {})",
                    arg1.name(),
                    arg2.name()
                )),
                _ => Err("Expected two arguments for regex matching".to_string()),
            },
            _ => unreachable!(),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_labels(
        g: &Graph,
        _runtime: &mut Self,
        _args: Value,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_labels()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_types(
        g: &Graph,
        _runtime: &mut Self,
        _args: Value,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_types()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_properties(
        g: &Graph,
        _runtime: &mut Self,
        _args: Value,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_properties()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }
}

#[allow(clippy::too_many_lines)]
pub fn ro_run(
    vars: &mut BTreeMap<String, Value>,
    g: &Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
    ir: &IR,
) -> Result<Value, String> {
    match ir {
        IR::Null => Ok(Value::Null),
        IR::Bool(x) => Ok(Value::Bool(*x)),
        IR::Integer(x) => Ok(Value::Int(*x)),
        IR::Float(x) => Ok(Value::Float(*x)),
        IR::String(x) => Ok(Value::String(x.to_string())),
        IR::Var(x) => vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::List(irs) => Ok(Value::List(
            irs.iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        IR::Length(ir) => match ro_run(vars, g, runtime, result_fn, ir)? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        IR::GetElement(op) => {
            let arr = ro_run(vars, g, runtime, result_fn, &op.0)?;
            let i = ro_run(vars, g, runtime, result_fn, &op.1)?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected Lust but was {v:?}")),
            }
        }
        IR::GetElements(op) => {
            let arr = ro_run(vars, g, runtime, result_fn, &op.0)?;
            match (&op.1, &op.2) {
                (None, None) => get_elements(arr, None, None),
                (None, Some(b)) => {
                    get_elements(arr, None, Some(ro_run(vars, g, runtime, result_fn, b)?))
                }
                (Some(a), None) => {
                    get_elements(arr, Some(ro_run(vars, g, runtime, result_fn, a)?), None)
                }
                (Some(a), Some(b)) => get_elements(
                    arr,
                    Some(ro_run(vars, g, runtime, result_fn, a)?),
                    Some(ro_run(vars, g, runtime, result_fn, b)?),
                ),
            }
        }
        IR::Range(_) => Err("Range operator not implemented".to_string()),
        IR::IsNull(ir) => match ro_run(vars, g, runtime, result_fn, ir)? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsNode(ir) => match ro_run(vars, g, runtime, result_fn, ir)? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::Or(irs) => {
            let mut is_null = false;
            for ir in irs {
                match ro_run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        IR::Xor(irs) => {
            let mut last = None;
            for ir in irs {
                match ro_run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }

        IR::And(irs) => {
            let mut is_null = false;
            for ir in irs {
                match ro_run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        IR::Not(ir) => match ro_run(vars, g, runtime, result_fn, ir)? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        IR::Eq(irs) => {
            let iter = irs.iter().map(|ir| ro_run(vars, g, runtime, result_fn, ir));
            all_equals(iter)
        }
        IR::Neq(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        IR::Lt(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0)?,
            ro_run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        IR::Gt(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0)?,
            ro_run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        IR::Le(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0)?,
            ro_run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        IR::Ge(op) => match (
            ro_run(vars, g, runtime, result_fn, &op.0)?,
            ro_run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        IR::In(op) => {
            let value = ro_run(vars, g, runtime, result_fn, &op.0)?;
            let list = ro_run(vars, g, runtime, result_fn, &op.1)?;
            list_contains(&list, &value)
        }
        IR::Add(irs) => add_all(
            irs.iter()
                .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect::<Vec<Value>>(),
        ),
        IR::Sub(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        IR::Mul(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        IR::Div(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        IR::Pow(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        IR::Modulo(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        IR::FuncInvocation(name, irs) => {
            let args = irs
                .iter()
                .map(|ir| ro_run(vars, g, runtime, result_fn, ir))
                .collect::<Result<Vec<_>, _>>()?;
            #[allow(clippy::option_if_let_else)]
            if let Some(func) = runtime.read_functions.get(name) {
                func(g, runtime, Value::List(args))
            } else {
                Err(format!("Function {name} not found"))
            }
        }
        IR::Map(items) => Ok(Value::Map(
            items
                .iter()
                .map(|(key, ir)| {
                    (
                        key.to_string(),
                        ro_run(vars, g, runtime, result_fn, ir).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        IR::Set(x, ir) => {
            let v = ro_run(vars, g, runtime, result_fn, ir)?;
            vars.insert(x.to_string(), v.clone());
            Ok(v)
        }
        IR::If(op) => match ro_run(vars, g, runtime, result_fn, &op.0)? {
            Value::Bool(true) => ro_run(vars, g, runtime, result_fn, &op.1),
            _ => Ok(Value::Null),
        },
        IR::For(op) => {
            ro_run(vars, g, runtime, result_fn, &op.0)?;
            while ro_run(vars, g, runtime, result_fn, &op.1)? == Value::Bool(true) {
                ro_run(vars, g, runtime, result_fn, &op.3)?;
                ro_run(vars, g, runtime, result_fn, &op.2)?;
            }
            Ok(Value::Null)
        }
        IR::Return(ir) => {
            let v = ro_run(vars, g, runtime, result_fn, ir)?;
            result_fn(g, v);
            Ok(Value::Null)
        }
        IR::Block(irs) => {
            for ir in irs {
                ro_run(vars, g, runtime, result_fn, ir)?;
            }
            Ok(Value::Null)
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn run(
    vars: &mut BTreeMap<String, Value>,
    g: &mut Graph,
    runtime: &mut Runtime,
    result_fn: &mut dyn FnMut(&Graph, Value),
    ir: &IR,
) -> Result<Value, String> {
    match ir {
        IR::Null => Ok(Value::Null),
        IR::Bool(x) => Ok(Value::Bool(*x)),
        IR::Integer(x) => Ok(Value::Int(*x)),
        IR::Float(x) => Ok(Value::Float(*x)),
        IR::String(x) => Ok(Value::String(x.to_string())),
        IR::Var(x) => vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        IR::List(irs) => Ok(Value::List(
            irs.iter()
                .map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        IR::Length(ir) => match run(vars, g, runtime, result_fn, ir)? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        IR::GetElement(op) => {
            let arr = run(vars, g, runtime, result_fn, &op.0)?;
            let i = run(vars, g, runtime, result_fn, &op.1)?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected Lust but was {v:?}")),
            }
        }
        IR::GetElements(op) => {
            let arr = run(vars, g, runtime, result_fn, &op.0)?;
            match (&op.1, &op.2) {
                (None, None) => get_elements(arr, None, None),
                (None, Some(b)) => {
                    get_elements(arr, None, Some(run(vars, g, runtime, result_fn, b)?))
                }
                (Some(a), None) => {
                    get_elements(arr, Some(run(vars, g, runtime, result_fn, a)?), None)
                }
                (Some(a), Some(b)) => get_elements(
                    arr,
                    Some(run(vars, g, runtime, result_fn, a)?),
                    Some(run(vars, g, runtime, result_fn, b)?),
                ),
            }
        }
        IR::Range(_) => Err("Range operator not implemented".to_string()),
        IR::IsNull(ir) => match run(vars, g, runtime, result_fn, ir)? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::IsNode(ir) => match run(vars, g, runtime, result_fn, ir)? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        IR::Or(irs) => {
            let mut is_null = false;
            for ir in irs {
                match ro_run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        IR::Xor(irs) => {
            let mut last = None;
            for ir in irs {
                match run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }
        IR::And(irs) => {
            let mut is_null = false;
            for ir in irs {
                match run(vars, g, runtime, result_fn, ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    _ => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        IR::Not(ir) => match run(vars, g, runtime, result_fn, ir)? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        IR::Eq(irs) => {
            let iter = irs.iter().map(|ir| run(vars, g, runtime, result_fn, ir));
            all_equals(iter)
        }
        IR::Neq(irs) => irs
            .iter()
            .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        IR::Lt(op) => match (
            run(vars, g, runtime, result_fn, &op.0)?,
            run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        IR::Gt(op) => match (
            run(vars, g, runtime, result_fn, &op.0)?,
            run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        IR::Le(op) => match (
            run(vars, g, runtime, result_fn, &op.0)?,
            run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        IR::Ge(op) => match (
            run(vars, g, runtime, result_fn, &op.0)?,
            run(vars, g, runtime, result_fn, &op.1)?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        IR::In(op) => {
            let value = run(vars, g, runtime, result_fn, &op.0)?;
            let list = run(vars, g, runtime, result_fn, &op.1)?;
            list_contains(&list, &value)
        }
        IR::Add(irs) => add_all(
            irs.iter()
                .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect::<Vec<Value>>(),
        ),
        IR::Sub(irs) => irs
            .iter()
            .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        IR::Mul(irs) => irs
            .iter()
            .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        IR::Div(irs) => irs
            .iter()
            .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        IR::Pow(irs) => irs
            .iter()
            .flat_map(|ir| run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        IR::Modulo(irs) => irs
            .iter()
            .flat_map(|ir| ro_run(vars, g, runtime, result_fn, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        IR::FuncInvocation(name, irs) => {
            let args = irs
                .iter()
                .map(|ir| run(vars, g, runtime, result_fn, ir))
                .collect::<Result<Vec<_>, _>>()?;
            if let Some(func) = runtime.write_functions.get(name) {
                func(g, runtime, Value::List(args))
            } else if let Some(func) = runtime.read_functions.get(name) {
                func(g, runtime, Value::List(args))
            } else {
                Err(format!("Function {name} not found"))
            }
        }
        IR::Map(items) => Ok(Value::Map(
            items
                .iter()
                .map(|(key, ir)| {
                    (
                        key.to_string(),
                        run(vars, g, runtime, result_fn, ir).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        IR::Set(x, ir) => {
            let v = run(vars, g, runtime, result_fn, ir)?;
            vars.insert(x.to_string(), v.clone());
            Ok(v)
        }
        IR::If(op) => match run(vars, g, runtime, result_fn, &op.0)? {
            Value::Bool(true) => run(vars, g, runtime, result_fn, &op.1),
            _ => Ok(Value::Null),
        },
        IR::For(op) => {
            run(vars, g, runtime, result_fn, &op.0)?;
            while run(vars, g, runtime, result_fn, &op.1)? == Value::Bool(true) {
                run(vars, g, runtime, result_fn, &op.3)?;
                run(vars, g, runtime, result_fn, &op.2)?;
            }
            Ok(Value::Null)
        }
        IR::Return(ir) => {
            let v = run(vars, g, runtime, result_fn, ir)?;
            result_fn(g, v);
            Ok(Value::Null)
        }
        IR::Block(irs) => {
            for ir in irs {
                run(vars, g, runtime, result_fn, ir)?;
            }
            Ok(Value::Null)
        }
    }
}

#[must_use]
pub fn evaluate_param(expr: QueryExprIR) -> Value {
    match expr {
        QueryExprIR::Null => Value::Null,
        QueryExprIR::Bool(x) => Value::Bool(x),
        QueryExprIR::Integer(x) => Value::Int(x),
        QueryExprIR::Float(x) => Value::Float(x),
        QueryExprIR::String(x) => Value::String(x),
        QueryExprIR::List(irs) => Value::List(irs.into_iter().map(evaluate_param).collect()),
        QueryExprIR::Map(irs) => Value::Map(
            irs.into_iter()
                .map(|(key, ir)| (key, evaluate_param(ir)))
                .collect(),
        ),
        _ => todo!(),
    }
}

fn get_elements(
    arr: Value,
    start: Option<Value>,
    end: Option<Value>,
) -> Result<Value, String> {
    match (arr, start, end) {
        (Value::List(values), Some(Value::Int(mut start)), Some(Value::Int(mut end))) => {
            if start < 0 {
                start = (values.len() as i64 + start).max(0);
            }
            if end < 0 {
                end = (values.len() as i64 + end).max(0);
            } else {
                end = end.min(values.len() as i64);
            }
            if start > end {
                return Ok(Value::List(vec![]));
            }
            Ok(Value::List(values[start as usize..end as usize].to_vec()))
        }
        (Value::List(values), None, Some(Value::Int(mut end))) => {
            if end < 0 {
                end = (values.len() as i64 + end).max(0);
            } else {
                end = end.min(values.len() as i64);
            }
            Ok(Value::List(values[..end as usize].to_vec()))
        }
        (Value::List(values), Some(Value::Int(mut start)), None) => {
            if start < 0 {
                start = (values.len() as i64 + start).max(0);
            }
            start = start.min(values.len() as i64);
            Ok(Value::List(values[start as usize..].to_vec()))
        }
        (_, Some(Value::Null), _) | (_, _, Some(Value::Null)) => Ok(Value::Null),
        (Value::List(values), None, None) => Ok(Value::List(values)),

        _ => Err("Invalid array range parameters.".to_string()),
    }
}

fn add_list_scalar(
    mut l: Vec<Value>,
    scalar: Value,
) -> Value {
    if l.is_empty() {
        return Value::List(vec![scalar]);
    }

    l.push(scalar);
    Value::List(l)
}

fn list_contains(
    list: &Value,
    value: &Value,
) -> Result<Value, String> {
    match list {
        Value::List(l) => Ok(Contains::contains(l, value)),
        Value::Null => Ok(Value::Null),
        _ => Err(format!(
            "Type mismatch: expected List or Null but was {}",
            list.name()
        )),
    }
}

// the semantic of Eq [1, 2, 3] is: 1 EQ 2 AND 2 EQ 3
fn all_equals<I>(mut iter: I) -> Result<Value, String>
where
    I: Iterator<Item = Result<Value, String>>,
{
    if let Some(first) = iter.next() {
        let prev = first?;
        for next in iter {
            let next = next?;
            match prev.partial_cmp(&next) {
                None => return Ok(Value::Null),
                Some(Ordering::Less | Ordering::Greater) => return Ok(Value::Bool(false)),
                Some(Ordering::Equal) => {}
            }
        }
        Ok(Value::Bool(true))
    } else {
        Err("Eq operator requires at least two arguments".to_string())
    }
}

#[inline]
const fn logical_xor(
    a: bool,
    b: bool,
) -> bool {
    (a && !b) || (!a && b)
}

fn add_all(values: Vec<Value>) -> Result<Value, String> {
    let mut iter = values.into_iter();
    let mut result = iter
        .next()
        .ok_or_else(|| "Add operator requires at least one argument".to_string())?;

    for value in iter {
        result = match (result, value) {
            (Value::Null, _) | (_, Value::Null) => Value::Null,
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::List(a), Value::List(b)) => Value::List(a.into_iter().chain(b).collect()),
            (Value::List(a), b) => add_list_scalar(a, b),
            (s, Value::List(l)) => {
                let mut new_list = vec![s];
                new_list.extend(l);
                Value::List(new_list)
            }
            (Value::String(a), Value::String(b)) => Value::String(a + &b),
            (Value::String(s), Value::Int(i)) => Value::String(s + &i.to_string()),
            (Value::String(s), Value::Float(f)) => Value::String(s + &f.to_string()),
            (Value::String(s), Value::Bool(f)) => Value::String(s + &f.to_string().to_lowercase()),
            (a, b) => {
                return Err(format!(
                    "Unexpected types for add operator ({}, {})",
                    a.name(),
                    b.name()
                ));
            }
        };
    }

    Ok(result)
}
