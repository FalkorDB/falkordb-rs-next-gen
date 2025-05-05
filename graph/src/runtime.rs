use crate::graph::ReturnCallback;
use crate::planner::IR;
use crate::{ast::ExprIR, graph::Graph, value::Contains, value::Value};
use crate::{matrix, tensor};
use orx_tree::{Dyn, DynNode, NodeMut, NodeRef};
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};

type ReadFn = fn(&mut Runtime, &Graph, Vec<Value>) -> Result<Value, String>;
type WriteFn = fn(&mut Runtime, &mut Graph, Vec<Value>) -> Result<Value, String>;

pub struct Runtime {
    read_functions: BTreeMap<String, ReadFn>,
    write_functions: BTreeMap<String, WriteFn>,
    agg_ctxs: BTreeMap<u64, (Value, Value)>,
    node_iters: Vec<matrix::Iter<bool>>,
    relationship_iters: Vec<tensor::Iter>,
    parameters: BTreeMap<String, Value>,
    vars: BTreeMap<String, Value>,
    pub created_nodes: Vec<(u64, Vec<String>, BTreeMap<String, Value>)>,
    pub created_relationships: Vec<(u64, String, u64, u64, BTreeMap<String, Value>)>,
    pub nodes_created: usize,
    pub relationships_created: usize,
    pub nodes_deleted: usize,
    pub relationships_deleted: usize,
    pub properties_set: usize,
    pub properties_removed: usize,
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
        read_functions.insert(
            "create_aggregate_ctx".to_string(),
            Self::create_aggregate_ctx,
        );
        read_functions.insert("create_node_iter".to_string(), Self::create_node_iter);
        read_functions.insert("next_node".to_string(), Self::next_node);
        read_functions.insert(
            "create_relationship_iter".to_string(),
            Self::create_relationship_iter,
        );
        read_functions.insert("next_relationship".to_string(), Self::next_relationship);
        read_functions.insert("property".to_string(), Self::property);
        read_functions.insert("toInteger".to_string(), Self::value_to_integer);
        read_functions.insert("labels".to_string(), Self::labels);
        read_functions.insert("startnode".to_string(), Self::start_node);
        read_functions.insert("endnode".to_string(), Self::end_node);
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
        read_functions.insert("string.join".to_string(), Self::string_join);
        read_functions.insert("string.matchRegEx".to_string(), Self::string_match_reg_ex);
        read_functions.insert(
            "string.replaceRegEx".to_string(),
            Self::string_replace_reg_ex,
        );
        read_functions.insert("abs".to_string(), Self::abs);
        read_functions.insert("ceil".to_string(), Self::ceil);
        read_functions.insert("e".to_string(), Self::e);
        read_functions.insert("exp".to_string(), Self::exp);
        read_functions.insert("floor".to_string(), Self::floor);
        read_functions.insert("log".to_string(), Self::log);
        read_functions.insert("log10".to_string(), Self::log10);
        read_functions.insert("pow".to_string(), Self::pow);
        read_functions.insert("rand".to_string(), Self::rand);
        read_functions.insert("round".to_string(), Self::round);
        read_functions.insert("sign".to_string(), Self::sign);
        read_functions.insert("sqrt".to_string(), Self::sqrt);
        read_functions.insert("range".to_string(), Self::range);

        // aggregation functions
        read_functions.insert("collect".to_string(), Self::collect);
        read_functions.insert("count".to_string(), Self::count);
        read_functions.insert("sum".to_string(), Self::sum);
        read_functions.insert("max".to_string(), Self::max);
        read_functions.insert("min".to_string(), Self::min);

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
            read_functions,
            write_functions,
            agg_ctxs: BTreeMap::new(),
            node_iters: Vec::new(),
            relationship_iters: Vec::new(),
            parameters,
            vars: BTreeMap::new(),
            created_nodes: Vec::new(),
            created_relationships: Vec::new(),
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
        &mut self,
        g: &mut Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
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
                let id = g.reserve_node();
                self.created_nodes.push((id, labels, attrs));
                Ok(Value::Node(id))
            }
            _ => Ok(Value::Null),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn delete_entity(
        &mut self,
        g: &mut Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        for n in args {
            if let Value::Node(id) = n {
                self.nodes_deleted += 1;
                for (src, dest, id) in g.get_node_relationships(id).collect::<Vec<_>>() {
                    self.relationships_deleted += 1;
                    g.delete_relationship(id, src, dest);
                }
                g.delete_node(id);
            }
        }

        Ok(Value::Null)
    }

    fn create_relationship(
        &mut self,
        g: &mut Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
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
                let id = g.reserve_relationship();
                self.created_relationships
                    .push((id, relationship_type, from, to, attrs));
                Ok(Value::Relationship(id, from, to))
            }
            _ => todo!(),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn create_aggregate_ctx(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut hasher = DefaultHasher::new();
        args.hash(&mut hasher);
        let key = hasher.finish();
        self.agg_ctxs
            .entry(key)
            .or_insert_with(|| (Value::List(args), Value::Null));
        Ok(Value::Int(key as i64))
    }

    fn create_node_iter(
        &mut self,
        g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::List(raw_labels)), None) => {
                self.node_iters.push(
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

                Ok(Value::Int(self.node_iters.len() as i64 - 1))
            }
            _ => todo!(),
        }
    }

    fn next_node(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::Int(iter)), None) => self.node_iters[iter as usize]
                .next()
                .map_or_else(|| Ok(Value::Null), |(n, _)| Ok(Value::Node(n))),
            _ => todo!(),
        }
    }

    fn create_relationship_iter(
        &mut self,
        g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::String(raw_type)), None) => {
                self.relationship_iters
                    .push(g.get_relationships(&[raw_type]).unwrap());
                Ok(Value::Int(self.relationship_iters.len() as i64 - 1))
            }
            _ => todo!(),
        }
    }

    fn next_relationship(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::Int(iter)), None) => self.relationship_iters[iter as usize]
                .next()
                .map_or(Ok(Value::Null), |(src, dest, id)| {
                    Ok(Value::Relationship(id, src, dest))
                }),
            _ => todo!(),
        }
    }

    fn property(
        &mut self,
        g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(Value::Node(node_id)), Some(Value::String(property)), None) => g
                .get_node_property_id(&property)
                .map_or(Ok(Value::Null), |property_id| {
                    g.get_node_property(node_id, property_id)
                        .map_or(Ok(Value::Null), Ok)
                }),
            (Some(Value::Map(map)), Some(Value::String(property)), None) => {
                Ok(map.get(&property).unwrap_or(&Value::Null).clone())
            }
            _ => Ok(Value::Null),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn labels(
        &mut self,
        g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::Node(node_id)), None) => Ok(Value::List(
                g.get_node_label_ids(node_id)
                    .map(|label_id| Value::String(g.get_label_by_id(label_id).to_string()))
                    .collect(),
            )),
            _ => Ok(Value::Null),
        }
    }

    fn args_size_error(
        args: &[Value],
        function_name: &str,
        min: usize,
        max: usize,
    ) -> Result<Value, String> {
        if max < args.len() {
            Err(format!(
                "Received {} arguments to function '{}', expected at most {}",
                args.len(),
                function_name,
                max
            ))
        } else {
            Err(format!(
                "Received {} arguments to function '{}', expected at least {}",
                args.len(),
                function_name,
                min
            ))
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn start_node(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::Relationship(_, src, _)), None) => Ok(Value::Node(src)),
            _ => Ok(Value::Null),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn end_node(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        match (iter.next(), iter.next()) {
            (Some(Value::Relationship(_, _, dest)), None) => Ok(Value::Node(dest)),
            _ => Ok(Value::Null),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn collect(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let mut iter = args.into_iter();
        if let (Some(x), Some(Value::Int(hash)), None) = (iter.next(), iter.next(), iter.next()) {
            self.agg_ctxs.entry(hash as _).and_modify(|v| {
                if let (_, Value::List(values)) = v {
                    values.push(x.clone());
                } else {
                    v.1 = Value::List(vec![x.clone()]);
                }
            });
        }
        Ok(Value::Null)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn count(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Null, _] => {}
            [_, Value::Int(hash)] => {
                self.agg_ctxs.entry(*hash as _).and_modify(|v| {
                    if let (_, Value::Int(count)) = v {
                        *count += 1;
                    } else {
                        v.1 = Value::Int(1);
                    }
                });
            }
            _ => (),
        }
        Ok(Value::Null)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn sum(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        if let [a, Value::Int(hash)] = args.as_slice() {
            self.agg_ctxs.entry(*hash as _).and_modify(|v| {
                if let (_, Value::Null) = v {
                    v.1 = a.clone();
                } else {
                    v.1 = (v.1.clone() + a.clone()).unwrap();
                }
            });
        }
        Ok(Value::Null)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn max(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        if let [Value::Int(a), Value::Int(hash)] = args.as_slice() {
            self.agg_ctxs.entry(*hash as _).and_modify(|v| {
                if let (_, Value::Int(b)) = v {
                    if a > b {
                        *b = *a;
                    }
                } else {
                    v.1 = Value::Int(*a);
                }
            });
        }
        Ok(Value::Null)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn min(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        if let [Value::Int(a), Value::Int(hash)] = args.as_slice() {
            self.agg_ctxs.entry(*hash as _).and_modify(|v| {
                if let (_, Value::Int(b)) = v {
                    if a < b {
                        *b = *a;
                    }
                } else {
                    v.1 = Value::Int(*a);
                }
            });
        }
        Ok(Value::Null)
    }

    fn value_to_integer(
        &mut self,
        _g: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s)] => s.parse::<i64>().map(Value::Int).or_else(|_| {
                s.parse::<f64>()
                    .map(|f| Value::Int(f as i64))
                    .or(Ok(Value::Null))
            }),
            [Value::Int(i)] => Ok(Value::Int(*i)),
            [Value::Float(f)] => Ok(Value::Int(*f as i64)),
            [Value::Null] => Ok(Value::Null),
            [Value::Bool(b)] => Ok(Value::Int(i64::from(*b))),
            [arg] => Err(format!(
                "Type mismatch: expected String, Boolean, Integer, Float, or Null but was {}",
                arg.name()
            )),
            args => Err(format!(
                "Expected one argument for value_to_integer, instead {}",
                args.len()
            )),
        }
    }

    fn size(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s)] => Ok(Value::Int(s.len() as i64)),
            [Value::List(v)] => Ok(Value::Int(v.len() as i64)),
            [Value::Null] => Ok(Value::Null),
            [arg] => Err(format!(
                "Type mismatch: expected List, String, or Null but was {}",
                arg.name()
            )),
            args => Self::args_size_error(args, "size", 1, 1),
        }
    }

    fn head(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "head", 1, 1),
        }
    }

    fn last(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::List(v)] => Ok(v.last().unwrap_or(&Value::Null).clone()),
            [Value::Null] => Ok(Value::Null),
            [arg] => Err(format!(
                "Type mismatch: expected List or Null but was {}",
                arg.name()
            )),
            args => Self::args_size_error(args, "last", 1, 1),
        }
    }

    fn tail(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "tail", 1, 1),
        }
    }

    fn reverse(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "reverse", 1, 1),
        }
    }

    fn substring(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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

            [Value::String(_), t] => Err(format!(
                "Type mismatch: expected Integer Or Null but got {}",
                t.name()
            )),
            [t, Value::Int(_)] | [t, Value::Int(_), Value::Int(_)] => Err(format!(
                "Type mismatch: expected String Or Null but got {}",
                t.name()
            )),
            [Value::String(_), t, Value::Int(_)] | [Value::String(_), Value::Int(_), t] => {
                Err(format!(
                    "Type mismatch: expected Integer Or Null but got {}",
                    t.name()
                ))
            }

            // Type mismatch handling
            args => Self::args_size_error(args, "substring", 2, 3),
        }
    }

    fn split(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "split", 2, 2),
        }
    }

    fn string_to_lower(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s)] => Ok(Value::String(s.to_lowercase())),
            [Value::Null] => Ok(Value::Null),
            [arg] => Err(format!(
                "Type mismatch: expected List, String or null, but was {}",
                arg.name()
            )),
            args => Self::args_size_error(args, "toLower", 1, 1),
        }
    }

    fn string_to_upper(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s)] => Ok(Value::String(s.to_uppercase())),
            [Value::Null] => Ok(Value::Null),
            [arg] => Err(format!(
                "Type mismatch: expected List, String or null, but was {}",
                arg.name()
            )),
            args => Self::args_size_error(args, "toUpper", 1, 1),
        }
    }

    fn string_replace(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "replace", 3, 3),
        }
    }

    fn string_left(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "left", 2, 2),
        }
    }

    fn string_ltrim(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s)] => Ok(Value::String(s.trim_start().to_string())),
            [Value::Null] => Ok(Value::Null),
            [arg] => Err(format!(
                "Type mismatch: expected String or null, but was {}",
                arg.name()
            )),
            args => Self::args_size_error(args, "ltrim", 1, 1),
        }
    }

    fn string_right(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
            args => Self::args_size_error(args, "right", 2, 2),
        }
    }
    fn string_join(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        fn to_string_vec(vec: &[Value]) -> Result<Vec<String>, String> {
            vec.iter()
                .map(|item| {
                    if let Value::String(s) = item {
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

        match args.as_slice() {
            [Value::List(vec), Value::String(s)] => {
                let result = to_string_vec(vec);
                result.map(|strings| Value::String(strings.join(s)))
            }
            [Value::List(vec)] => {
                let result = to_string_vec(vec);
                result.map(|strings| Value::String(strings.join("")))
            }
            [Value::Null, _] => Ok(Value::Null),
            [arg1, _arg2] => Err(format!(
                "Type mismatch: expected List or Null but was {}",
                arg1.name()
            )),
            args => Self::args_size_error(args, "string.join", 1, 2),
        }
    }

    fn string_match_reg_ex(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(text), Value::String(pattern)] => match regex::Regex::new(pattern) {
                Ok(re) => {
                    let mut all_matches = Vec::new();
                    for caps in re.captures_iter(text) {
                        for i in 0..caps.len() {
                            if let Some(m) = caps.get(i) {
                                all_matches.push(Value::String(m.as_str().to_string()));
                            }
                        }
                    }
                    Ok(Value::List(all_matches))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            },
            [Value::Null, _] | [_, Value::Null] => Ok(Value::List(vec![])),
            [Value::String(_), arg2] => Err(format!(
                "Type mismatch: expected String or Null but was {}",
                arg2.name(),
            )),
            [arg1, _] => Err(format!(
                "Type mismatch: expected String or Null but was {}",
                arg1.name(),
            )),
            args => Self::args_size_error(args, "string.matchRegEx", 2, 2),
        }
    }

    fn string_replace_reg_ex(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [
                Value::String(text),
                Value::String(pattern),
                Value::String(replacement),
            ] => match regex::Regex::new(pattern) {
                Ok(re) => {
                    let replaced_text = re.replace_all(text, replacement).to_string();
                    Ok(Value::String(replaced_text))
                }
                Err(e) => Err(format!("Invalid regex, {e}")),
            },
            [Value::Null, _, _] | [_, Value::Null, _] | [_, _, Value::Null] => Ok(Value::Null),
            [Value::String(_), arg2, Value::String(_)] => Err(format!(
                "Type mismatch: expected String or Null but was {}",
                arg2.name(),
            )),
            [Value::String(_), Value::String(_), arg3] => Err(format!(
                "Type mismatch: expected String or Null but was {}",
                arg3.name(),
            )),
            [arg1, _, _] => Err(format!(
                "Type mismatch: expected String or Null but was {}",
                arg1.name(),
            )),
            args => Self::args_size_error(args, "string.replaceRegEx", 3, 3),
        }
    }

    fn abs(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Int(n.abs())),
            [Value::Float(f)] => Ok(Value::Float(f.abs())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "abs", 1, 1),
        }
    }

    fn ceil(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Int(*n)),
            [Value::Float(f)] => Ok(Value::Float(f.ceil())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "ceil", 1, 1),
        }
    }

    fn e(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [] => Ok(Value::Float(std::f64::consts::E)),
            args => Self::args_size_error(args, "e", 0, 0),
        }
    }

    fn exp(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Float((*n as f64).exp())),
            [Value::Float(f)] => Ok(Value::Float(f.exp())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "exp", 1, 1),
        }
    }

    fn floor(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Int(*n)),
            [Value::Float(f)] => Ok(Value::Float(f.floor())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "floor", 1, 1),
        }
    }

    fn log(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Float((*n as f64).ln())),
            [Value::Float(f)] => Ok(Value::Float(f.ln())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "log", 1, 1),
        }
    }

    fn log10(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Float((*n as f64).log10())),
            [Value::Float(f)] => Ok(Value::Float(f.log10())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "log10", 1, 1),
        }
    }
    fn pow(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(i1), Value::Int(i2)] => Ok(Value::Float((*i1 as f64).powi(*i2 as i32))),
            [Value::Float(f1), Value::Float(f2)] => Ok(Value::Float(f1.powf(*f2))),
            [Value::Int(i1), Value::Float(f1)] => Ok(Value::Float((*i1 as f64).powf(*f1))),
            [Value::Float(f1), Value::Int(i1)] => Ok(Value::Float(f1.powi(*i1 as i32))),
            [Value::Null, _] | [_, Value::Null] => Ok(Value::Null),
            [Value::Int(_) | Value::Float(_), v] | [v, Value::Int(_) | Value::Float(_)] => {
                Err(format!(
                    "Type mismatch: expected Integer, Float, or Null but was {}",
                    v.name()
                ))
            }
            args => Self::args_size_error(args, "pow", 2, 2),
        }
    }

    fn rand(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [] => {
                let mut rng = rand::rng();
                Ok(Value::Float(rng.random_range(0.0..1.0)))
            }
            args => Self::args_size_error(args, "rand", 0, 0),
        }
    }

    fn round(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Int(*n)),
            [Value::Float(f)] => Ok(Value::Float(f.round())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "round", 1, 1),
        }
    }

    fn sign(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => Ok(Value::Int(n.signum())),
            [Value::Float(f)] => Ok(if *f == 0.0 {
                Value::Int(0)
            } else {
                Value::Float(f.signum().round())
            }),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "sign", 1, 1),
        }
    }

    fn sqrt(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::Int(n)] => {
                if *n < 0 {
                    Ok(Value::Float(f64::NAN))
                } else {
                    Ok(Value::Float((*n as f64).sqrt()))
                }
            }
            [Value::Float(f)] => {
                if *f > 0f64 {
                    Ok(Value::Float(f.sqrt()))
                } else {
                    Ok(Value::Float(f64::NAN))
                }
            }
            [Value::Null] => Ok(Value::Null),
            [v] => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            )),
            args => Self::args_size_error(args, "sqrt", 1, 1),
        }
    }

    fn range(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let start = &args[0];
        let end = &args[1];
        let step = args.get(2).unwrap_or(&Value::Int(1));
        match (start, end, step) {
            (Value::Int(start), Value::Int(end), Value::Int(step)) => {
                Ok(Value::List(if step < &0 {
                    (*end..=*start)
                        .step_by((-step) as usize)
                        .map(Value::Int)
                        .collect()
                } else {
                    (*start..=*end)
                        .step_by(*step as usize)
                        .map(Value::Int)
                        .collect()
                }))
            }
            _ => Err("Range operator requires two integers".to_string()),
        }
    }

    //
    // Internal functions
    //

    fn internal_starts_with(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s), Value::String(prefix)] => Ok(Value::Bool(s.starts_with(prefix))),

            [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
            [arg1, arg2] => Err(format!(
                "Type mismatch: expected String or Null but was ({}, {})",
                arg1.name(),
                arg2.name()
            )),
            _ => unreachable!(),
        }
    }

    fn internal_ends_with(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s), Value::String(suffix)] => Ok(Value::Bool(s.ends_with(suffix))),
            [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
            [arg1, arg2] => Err(format!(
                "Type mismatch: Type mismatch: expected String or Null but was ({}, {})",
                arg1.name(),
                arg2.name()
            )),
            _ => unreachable!(),
        }
    }

    fn internal_contains(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
            [Value::String(s), Value::String(substring)] => Ok(Value::Bool(s.contains(substring))),
            [_, Value::Null] | [Value::Null, _] => Ok(Value::Null),
            [arg1, arg2] => Err(format!(
                "Type mismatch: expected String or Null but was ({}, {})",
                arg1.name(),
                arg2.name()
            )),
            _ => unreachable!(),
        }
    }

    fn internal_regex_matches(
        &mut self,
        _: &Graph,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        match args.as_slice() {
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
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_labels(
        &mut self,
        g: &Graph,
        _args: Vec<Value>,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_labels()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_types(
        &mut self,
        g: &Graph,
        _args: Vec<Value>,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_types()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn db_properties(
        &mut self,
        g: &Graph,
        _args: Vec<Value>,
    ) -> Result<Value, String> {
        Ok(Value::List(
            g.get_properties()
                .map(|n| Value::String(n.to_string()))
                .collect(),
        ))
    }
}

#[allow(clippy::too_many_lines)]
pub fn ro_run_expr(
    g: &Graph,
    runtime: &mut Runtime,
    ir: DynNode<ExprIR>,
) -> Result<Value, String> {
    match ir.data() {
        ExprIR::Null => Ok(Value::Null),
        ExprIR::Bool(x) => Ok(Value::Bool(*x)),
        ExprIR::Integer(x) => Ok(Value::Int(*x)),
        ExprIR::Float(x) => Ok(Value::Float(*x)),
        ExprIR::String(x) => Ok(Value::String(x.to_string())),
        ExprIR::Var(x) => runtime.vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        ExprIR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        ExprIR::List => Ok(Value::List(
            ir.children()
                .map(|ir| ro_run_expr(g, runtime, ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        ExprIR::Length => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        ExprIR::GetElement => {
            let arr = ro_run_expr(g, runtime, ir.child(0))?;
            let i = ro_run_expr(g, runtime, ir.child(1))?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected List but was {v:?}")),
            }
        }
        ExprIR::GetElements => {
            let arr = ro_run_expr(g, runtime, ir.child(0))?;
            let a = ro_run_expr(g, runtime, ir.child(1))?;
            let b = ro_run_expr(g, runtime, ir.child(2))?;
            get_elements(arr, a, b)
        }
        ExprIR::IsNull => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::IsNode => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::IsRelationship => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::Or => {
            let mut is_null = false;
            for ir in ir.children() {
                match ro_run_expr(g, runtime, ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        ExprIR::Xor => {
            let mut last = None;
            for ir in ir.children() {
                match ro_run_expr(g, runtime, ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }

        ExprIR::And => {
            let mut is_null = false;
            for ir in ir.children() {
                match ro_run_expr(g, runtime, ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        ExprIR::Not => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        ExprIR::Negate => match ro_run_expr(g, runtime, ir.child(0))? {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Null => Ok(Value::Null),
            _ => {
                Err("InvalidArgumentType: Negate operator requires an Integer or Float".to_string())
            }
        },
        ExprIR::Eq => all_equals(ir.children().map(|ir| ro_run_expr(g, runtime, ir))),
        ExprIR::Neq => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        ExprIR::Lt => match (
            ro_run_expr(g, runtime, ir.child(0))?,
            ro_run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        ExprIR::Gt => match (
            ro_run_expr(g, runtime, ir.child(0))?,
            ro_run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        ExprIR::Le => match (
            ro_run_expr(g, runtime, ir.child(0))?,
            ro_run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        ExprIR::Ge => match (
            ro_run_expr(g, runtime, ir.child(0))?,
            ro_run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        ExprIR::In => {
            let value = ro_run_expr(g, runtime, ir.child(0))?;
            let list = ro_run_expr(g, runtime, ir.child(1))?;
            list_contains(&list, &value)
        }
        ExprIR::Add => ir
            .children()
            .map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|acc, value| acc? + value?)
            .ok_or_else(|| "Add operator requires at least one operand".to_string())?,
        ExprIR::Sub => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        ExprIR::Mul => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        ExprIR::Div => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        ExprIR::Pow => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        ExprIR::Modulo => ir
            .children()
            .flat_map(|ir| ro_run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        ExprIR::FuncInvocation(name) => {
            let args = ir
                .children()
                .map(|ir| ro_run_expr(g, runtime, ir))
                .collect::<Result<Vec<_>, _>>()?;
            #[allow(clippy::option_if_let_else)]
            if let Some(func) = runtime.read_functions.get(name) {
                func(runtime, g, args)
            } else {
                Err(format!("Function {name} not found"))
            }
        }
        ExprIR::Map => Ok(Value::Map(
            ir.children()
                .map(|child| {
                    (
                        child.data().to_string(),
                        ro_run_expr(g, runtime, child.child(0)).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        ExprIR::Set(x) => {
            let v = ro_run_expr(g, runtime, ir.child(0))?;
            runtime.vars.insert(x.to_string(), v.clone());
            Ok(v)
        }
    }
}

fn ro_consume<CB: ReturnCallback>(
    g: &Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<'_, Dyn<IR>>,
    res: Option<()>,
) -> Result<Option<()>, String> {
    let child_num = ir.num_children();
    match (ir.data_mut(), res) {
        (IR::Empty, _) => todo!(),
        (IR::Call(_, trees), _) => todo!(),
        (IR::Unwind(list, name, iter @ None), _) => {
            let list = ro_run_expr(g, runtime, list.root())?;
            if let Value::List(vs) = list {
                let mut it = vs.into_iter();
                if let Some(v) = it.next() {
                    runtime.vars.insert(name.to_string(), v);
                    *iter = Some(it);
                    return Ok(Some(()));
                }
                return Ok(None);
            }
            Err("err".to_string())
        }
        (IR::Unwind(list, name, Some(iter)), _) => {
            if let Some(v) = iter.next() {
                runtime.vars.insert(name.to_string(), v);
                return Ok(Some(()));
            }
            Ok(None)
        }
        (IR::Create(pattern), _) => todo!(),
        (IR::Delete(trees), _) => todo!(),
        (IR::NodeScan(node_pattern), _) => todo!(),
        (IR::RelationshipScan(relationship_pattern), _) => todo!(),
        (IR::Filter(tree), _) => todo!(),
        (IR::Aggregate(_, trees, trees1), _) => todo!(),
        (IR::With(trees, _, run @ false), Some(())) => {
            *run = child_num == 0;
            for e in trees {
                ro_run_expr(g, runtime, e.root())?;
            }
            Ok(Some(()))
        }
        (IR::With(_, _, true) | IR::Return(_, _, true), _)
        | (IR::With(_, _, _) | IR::Return(_, _, _), None) => Ok(None),
        (IR::Return(trees, _, run @ false), Some(())) => {
            *run = child_num == 0;
            let vs = trees
                .iter()
                .flat_map(|e| ro_run_expr(g, runtime, e.root()))
                .collect();
            callback.return_value(g, Value::List(vs));
            Ok(Some(()))
        }
        (IR::Commit, _) => todo!(),
    }
}

fn ro_run_inner<CB: ReturnCallback>(
    g: &Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<Dyn<IR>>,
) -> Result<Option<()>, String> {
    if ir.num_children() == 1 {
        let res = ro_run_inner(g, runtime, callback, &mut ir.child_mut(0))?;
        return ro_consume(g, runtime, callback, ir, res);
    }

    ro_consume(g, runtime, callback, ir, Some(()))
}

pub fn ro_run<CB: ReturnCallback>(
    g: &Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<Dyn<IR>>,
) -> Result<(), String> {
    while ro_run_inner(g, runtime, callback, ir)?.is_some() {}
    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn run_expr(
    g: &mut Graph,
    runtime: &mut Runtime,
    ir: DynNode<ExprIR>,
) -> Result<Value, String> {
    match ir.data() {
        ExprIR::Null => Ok(Value::Null),
        ExprIR::Bool(x) => Ok(Value::Bool(*x)),
        ExprIR::Integer(x) => Ok(Value::Int(*x)),
        ExprIR::Float(x) => Ok(Value::Float(*x)),
        ExprIR::String(x) => Ok(Value::String(x.to_string())),
        ExprIR::Var(x) => runtime.vars.get(x).map_or_else(
            || Err(format!("Variable {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        ExprIR::Parameter(x) => runtime.parameters.get(x).map_or_else(
            || Err(format!("Parameter {x} not found")),
            |v| Ok(v.to_owned()),
        ),
        ExprIR::List => Ok(Value::List(
            ir.children()
                .map(|ir| run_expr(g, runtime, ir))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        ExprIR::Length => match run_expr(g, runtime, ir.child(0))? {
            Value::List(arr) => Ok(Value::Int(arr.len() as _)),
            _ => Err("Length operator requires a list".to_string()),
        },
        ExprIR::GetElement => {
            let arr = run_expr(g, runtime, ir.child(0))?;
            let i = run_expr(g, runtime, ir.child(1))?;
            match (arr, i) {
                (Value::List(values), Value::Int(i)) => {
                    if i >= 0 && i < values.len() as _ {
                        Ok(values[i as usize].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::List(_), v) => Err(format!("Type mismatch: expected Bool but was {v:?}")),
                v => Err(format!("Type mismatch: expected List but was {v:?}")),
            }
        }
        ExprIR::GetElements => {
            let arr = run_expr(g, runtime, ir.child(0))?;
            let a = run_expr(g, runtime, ir.child(1))?;
            let b = run_expr(g, runtime, ir.child(2))?;
            get_elements(arr, a, b)
        }
        ExprIR::IsNull => match run_expr(g, runtime, ir.child(0))? {
            Value::Null => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::IsNode => match run_expr(g, runtime, ir.child(0))? {
            Value::Node(_) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::IsRelationship => match run_expr(g, runtime, ir.child(0))? {
            Value::Relationship(_, _, _) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        ExprIR::Or => {
            let mut is_null = false;
            for ir in ir.children() {
                match run_expr(g, runtime, ir)? {
                    Value::Bool(true) => return Ok(Value::Bool(true)),
                    Value::Bool(false) => {}
                    Value::Null => is_null = true,
                    ir => return Err(format!("Type mismatch: expected Bool but was {:?}", ir)),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(false))
        }
        ExprIR::Xor => {
            let mut last = None;
            for ir in ir.children() {
                match run_expr(g, runtime, ir)? {
                    Value::Bool(b) => last = Some(last.map_or(b, |l| logical_xor(l, b))),
                    Value::Null => return Ok(Value::Null),
                    ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            Ok(Value::Bool(last.unwrap_or(false)))
        }
        ExprIR::And => {
            let mut is_null = false;
            for ir in ir.children() {
                match run_expr(g, runtime, ir)? {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => {}
                    Value::Null => is_null = true,
                    ir => return Err(format!("Type mismatch: expected Bool but was {ir:?}")),
                }
            }
            if is_null {
                return Ok(Value::Null);
            }

            Ok(Value::Bool(true))
        }
        ExprIR::Not => match run_expr(g, runtime, ir.child(0))? {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Null => Ok(Value::Null),
            _ => Err("InvalidArgumentType: Not operator requires a boolean or null".to_string()),
        },
        ExprIR::Negate => match run_expr(g, runtime, ir.child(0))? {
            Value::Int(i) => Ok(Value::Int(-i)),
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Null => Ok(Value::Null),
            _ => {
                Err("InvalidArgumentType: Negate operator requires an Integer or Float".to_string())
            }
        },
        ExprIR::Eq => all_equals(ir.children().map(|ir| run_expr(g, runtime, ir))),
        ExprIR::Neq => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| Value::Bool(a != b))
            .ok_or_else(|| "Neq operator requires at least one argument".to_string()),
        ExprIR::Lt => match (
            run_expr(g, runtime, ir.child(0))?,
            run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err("Lt operator requires two integers".to_string()),
        },
        ExprIR::Gt => match (
            run_expr(g, runtime, ir.child(0))?,
            run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            _ => Err("Gt operator requires two integers".to_string()),
        },
        ExprIR::Le => match (
            run_expr(g, runtime, ir.child(0))?,
            run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            _ => Err("Le operator requires two integers".to_string()),
        },
        ExprIR::Ge => match (
            run_expr(g, runtime, ir.child(0))?,
            run_expr(g, runtime, ir.child(1))?,
        ) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err("Ge operator requires two integers".to_string()),
        },
        ExprIR::In => {
            let value = run_expr(g, runtime, ir.child(0))?;
            let list = run_expr(g, runtime, ir.child(1))?;
            list_contains(&list, &value)
        }
        ExprIR::Add => ir
            .children()
            .map(|ir| run_expr(g, runtime, ir))
            .reduce(|acc, value| acc? + value?)
            .ok_or_else(|| "Add operator requires at least one operand".to_string())?,
        ExprIR::Sub => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Sub operator requires at least one argument".to_string()),
        ExprIR::Mul => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Mul operator requires at least one argument".to_string()),
        ExprIR::Div => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Div operator requires at least one argument".to_string()),
        ExprIR::Pow => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Float((a as f64).powf(b as _)),
                _ => Value::Null,
            })
            .ok_or_else(|| "Pow operator requires at least one argument".to_string()),
        ExprIR::Modulo => ir
            .children()
            .flat_map(|ir| run_expr(g, runtime, ir))
            .reduce(|a, b| match (a, b) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                _ => Value::Null,
            })
            .ok_or_else(|| "Modulo operator requires at least one argument".to_string()),
        ExprIR::FuncInvocation(name) => {
            let args = ir
                .children()
                .map(|ir| run_expr(g, runtime, ir))
                .collect::<Result<Vec<_>, _>>()?;
            if let Some(func) = runtime.write_functions.get(name) {
                func(runtime, g, args)
            } else if let Some(func) = runtime.read_functions.get(name) {
                func(runtime, g, args)
            } else {
                Err(format!("Function {name} not found"))
            }
        }
        ExprIR::Map => Ok(Value::Map(
            ir.children()
                .map(|child| {
                    (
                        child.data().to_string(),
                        run_expr(g, runtime, child.child(0)).unwrap_or(Value::Null),
                    )
                })
                .collect(),
        )),
        ExprIR::Set(x) => {
            let v = run_expr(g, runtime, ir.child(0))?;
            runtime.vars.insert(x.to_string(), v.clone());
            Ok(v)
        } // ExprIR::Commit(befor) => {
          //     if !befor {
          //         run( g, runtime, &ir.child(0))?;
          //     }
          //     if !runtime.created_nodes.is_empty() {
          //         runtime.nodes_created += runtime.created_nodes.len();
          //         runtime.properties_set += runtime
          //             .created_nodes
          //             .iter()
          //             .flat_map(|v| v.2.values())
          //             .map(|v| match v {
          //                 Value::Null => 0,
          //                 _ => 1,
          //             })
          //             .sum::<usize>();
          //         g.create_nodes(&runtime.created_nodes);
          //         runtime.created_nodes.clear();
          //     }
          //     if !runtime.created_relationships.is_empty() {
          //         runtime.relationships_created += runtime.created_relationships.len();
          //         runtime.properties_set += runtime
          //             .created_relationships
          //             .iter()
          //             .flat_map(|v| v.4.values())
          //             .map(|v| match v {
          //                 Value::Null => 0,
          //                 _ => 1,
          //             })
          //             .sum::<usize>();
          //         g.create_relationships(&runtime.created_relationships);
          //         runtime.created_relationships.clear();
          //     }
          //     if *befor {
          //         run(vars, g, runtime, &ir.child(0))?;
          //     }
          //     Ok(Value::Null)
          // }
    }
}

fn consume<CB: ReturnCallback>(
    g: &mut Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<'_, Dyn<IR>>,
    res: Option<()>,
) -> Result<Option<()>, String> {
    let child_num = ir.num_children();
    match (ir.data_mut(), res) {
        (IR::Empty, _) => todo!(),
        (IR::Call(_, trees), _) => todo!(),
        (IR::Unwind(list, name, iter @ None), _) => {
            let list = run_expr(g, runtime, list.root())?;
            if let Value::List(vs) = list {
                let mut it = vs.into_iter();
                if let Some(v) = it.next() {
                    runtime.vars.insert(name.to_string(), v);
                    *iter = Some(it);
                    return Ok(Some(()));
                }
                return Ok(None);
            }
            Err("err".to_string())
        }
        (IR::Unwind(list, name, Some(iter)), _) => {
            if let Some(v) = iter.next() {
                runtime.vars.insert(name.to_string(), v);
                return Ok(Some(()));
            }
            Ok(None)
        }
        (IR::Create(pattern), _) => todo!(),
        (IR::Delete(trees), _) => todo!(),
        (IR::NodeScan(node_pattern), _) => todo!(),
        (IR::RelationshipScan(relationship_pattern), _) => todo!(),
        (IR::Filter(tree), _) => todo!(),
        (IR::Aggregate(_, trees, trees1), _) => todo!(),
        (IR::With(trees, _, run @ false), Some(())) => {
            *run = child_num == 0;
            for e in trees {
                run_expr(g, runtime, e.root())?;
            }
            Ok(Some(()))
        }
        (IR::With(_, _, true) | IR::Return(_, _, true), _)
        | (IR::With(_, _, _) | IR::Return(_, _, _), None) => Ok(None),
        (IR::Return(trees, _, run @ false), Some(())) => {
            *run = child_num == 0;
            let vs = trees
                .iter()
                .flat_map(|e| run_expr(g, runtime, e.root()))
                .collect();
            callback.return_value(g, Value::List(vs));
            Ok(Some(()))
        }
        (IR::Commit, _) => todo!(),
    }
}

fn run_inner<CB: ReturnCallback>(
    g: &mut Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<Dyn<IR>>,
) -> Result<Option<()>, String> {
    if ir.num_children() == 1 {
        let res = run_inner(g, runtime, callback, &mut ir.child_mut(0))?;
        return consume(g, runtime, callback, ir, res);
    }

    consume(g, runtime, callback, ir, Some(()))
}

pub fn run<CB: ReturnCallback>(
    g: &mut Graph,
    runtime: &mut Runtime,
    callback: &CB,
    ir: &mut NodeMut<Dyn<IR>>,
) -> Result<(), String> {
    while run_inner(g, runtime, callback, ir)?.is_some() {}
    Ok(())
}

#[must_use]
pub fn evaluate_param(expr: DynNode<ExprIR>) -> Value {
    match expr.data() {
        ExprIR::Null => Value::Null,
        ExprIR::Bool(x) => Value::Bool(*x),
        ExprIR::Integer(x) => Value::Int(*x),
        ExprIR::Float(x) => Value::Float(*x),
        ExprIR::String(x) => Value::String(x.to_string()),
        ExprIR::List => Value::List(expr.children().map(evaluate_param).collect()),
        ExprIR::Map => Value::Map(
            expr.children()
                .map(|ir| match ir.data() {
                    ExprIR::Var(key) => (key.to_string(), evaluate_param(ir.child(0))),
                    _ => todo!(),
                })
                .collect(),
        ),
        ExprIR::Negate => {
            let v = evaluate_param(expr.child(0));
            match v {
                Value::Int(i) => Value::Int(-i),
                Value::Float(f) => Value::Float(-f),
                _ => Value::Null,
            }
        }
        _ => todo!(),
    }
}

fn get_elements(
    arr: Value,
    start: Value,
    end: Value,
) -> Result<Value, String> {
    match (arr, start, end) {
        (Value::List(values), Value::Int(mut start), Value::Int(mut end)) => {
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
        (_, Value::Null, _) | (_, _, Value::Null) => Ok(Value::Null),
        _ => Err("Invalid array range parameters.".to_string()),
    }
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
