//! `GRAPH.INFO` command handler.
//!
//! Returns diagnostic information about internal subsystems.
//! Currently supports `GRAPH.INFO ObjectPool` to query the interned
//! string pool statistics.

use graph::runtime::object_pool::get_object_pool;
use redis_module::{Context, RedisResult, RedisString, RedisValue};

#[allow(clippy::unnecessary_wraps)]
pub fn graph_info(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() < 2 {
        return Err(redis_module::RedisError::WrongArity);
    }

    let section = args[1].to_string_lossy();

    if section == "ObjectPool" {
        let (count, avg) = get_object_pool().stats();
        Ok(RedisValue::Array(vec![
            RedisValue::SimpleStringStatic("Object Pool"),
            RedisValue::Array(vec![
                RedisValue::Array(vec![
                    RedisValue::SimpleStringStatic("Unique Objects in Pool"),
                    RedisValue::Integer(count as i64),
                ]),
                RedisValue::Array(vec![
                    RedisValue::SimpleStringStatic("Average References per Object"),
                    RedisValue::BulkString(format!("{avg}")),
                ]),
            ]),
        ]))
    } else {
        Err(redis_module::RedisError::Str("Unknown info section"))
    }
}
