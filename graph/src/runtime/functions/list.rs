//! List operations.
//!
//! Functions that inspect or transform `List` values.  `Null` inputs
//! propagate as `Null` outputs.
//!
//! ```text
//!  Cypher         Function    Returns            Notes
//! ──────────────────────────────────────────────────────────
//!  size(x)        size()      Int                works on List, String, Arc<List>
//!  head(list)     head()      first element      Null on empty
//!  last(list)     last()      last element       Null on empty
//!  tail(list)     tail()      list[1..]          empty on empty
//!  reverse(x)     reverse()   reversed list/str  works on both List and String
//! ```

#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::cast_possible_wrap)]

use super::{FnType, Functions, Type};
use crate::runtime::{runtime::Runtime, value::Value};
use std::sync::Arc;
use thin_vec::{ThinVec, thin_vec};

pub fn register(funcs: &mut Functions) {
    cypher_fn!(funcs, "size",
        args: [Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ])],
        ret: Type::Union(vec![Type::Int, Type::Null]),
        fn size(_, args) {
            match args.into_iter().next() {
                Some(Value::String(s) | Value::InternedString(s)) => Ok(Value::Int(s.chars().count() as i64)),
                Some(Value::List(v)) => Ok(Value::Int(v.len() as i64)),
                Some(Value::Null) => Ok(Value::Null),
                _ => unreachable!(),
            }
        }
    );

    cypher_fn!(funcs, "head",
        args: [Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        ret: Type::Any,
        fn head(_, args) {
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
    );

    cypher_fn!(funcs, "last",
        args: [Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        ret: Type::Any,
        fn last(_, args) {
            match args.into_iter().next() {
                Some(Value::List(v)) => Ok(v.last().cloned().unwrap_or(Value::Null)),
                Some(Value::Null) => Ok(Value::Null),
                _ => unreachable!(),
            }
        }
    );

    cypher_fn!(funcs, "tail",
        args: [Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::Null,
        ])],
        ret: Type::Any,
        fn tail(_, args) {
            match args.into_iter().next() {
                Some(Value::List(v)) => {
                    if v.is_empty() {
                        Ok(Value::List(Arc::new(thin_vec![])))
                    } else {
                        Ok(Value::List(Arc::new(v[1..].iter().cloned().collect::<ThinVec<_>>())))
                    }
                }
                Some(Value::Null) => Ok(Value::Null),
                _ => unreachable!(),
            }
        }
    );

    cypher_fn!(funcs, "reverse",
        args: [Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ])],
        ret: Type::Union(vec![
            Type::List(Box::new(Type::Any)),
            Type::String,
            Type::Null,
        ]),
        fn reverse(_, args) {
            match args.into_iter().next() {
                Some(Value::List(mut v)) => {
                    Arc::make_mut(&mut v).reverse();
                    Ok(Value::List(v))
                }
                Some(Value::String(s) | Value::InternedString(s)) => Ok(Value::String(Arc::new(s.chars().rev().collect()))),
                Some(Value::Null) => Ok(Value::Null),
                _ => unreachable!(),
            }
        }
    );
}
