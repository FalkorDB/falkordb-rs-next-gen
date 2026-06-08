use crate::redis_type::{create_virtual_keys, delete_stale_virtual_keys, finalize_pending_graphs};
use crate::serializers::DECODE_STATE;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

pub fn graph_debug(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() < 3 {
        return Err(RedisError::WrongArity);
    }
    let mut args_iter = args.into_iter().skip(1);
    let subcmd = args_iter.next_str()?;

    match subcmd.to_uppercase().as_str() {
        "AUX" => debug_aux(ctx, args_iter),
        // `GRAPH.DEBUG COMPACT <key>` — major-compact the native FalkorDB indexes
        // of <key> (collapse each band's LSM segments into one base). Returns the
        // number of indexes compacted. On-demand maintenance / benchmarking lever.
        "COMPACT" => debug_compact(ctx, args_iter),
        _ => Err(RedisError::String(format!(
            "Unknown DEBUG subcommand: {subcmd}"
        ))),
    }
}

#[cfg(feature = "index-falkordb")]
fn debug_compact(
    ctx: &Context,
    mut args: impl Iterator<Item = RedisString>,
) -> RedisResult {
    use crate::{graph_core::ThreadedGraph, redis_type::GRAPH_TYPE};
    use parking_lot::RwLock;
    use std::sync::Arc;

    let key_name = args.next_arg()?;
    let key = ctx.open_key(&key_name);
    let g = key
        .get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?
        .ok_or(RedisError::Str("Graph does not exist"))?;
    let compacted = g.read().graph.read().borrow().falkordb_compact_indexes();
    Ok(RedisValue::Integer(compacted as i64))
}

#[cfg(not(feature = "index-falkordb"))]
fn debug_compact(
    _ctx: &Context,
    _args: impl Iterator<Item = RedisString>,
) -> RedisResult {
    Err(RedisError::Str(
        "GRAPH.DEBUG COMPACT requires the index-falkordb feature",
    ))
}

fn debug_aux(
    ctx: &Context,
    mut args: impl Iterator<Item = RedisString>,
) -> RedisResult {
    let action = args.next_str()?;
    let result = match action.to_uppercase().as_str() {
        "START" => {
            DECODE_STATE.lock().clear();
            unsafe { create_virtual_keys(ctx.ctx) };
            Ok(RedisValue::Integer(1))
        }
        "END" => {
            finalize_pending_graphs();
            unsafe { delete_stale_virtual_keys(ctx.ctx) };
            Ok(RedisValue::Integer(0))
        }
        _ => Err(RedisError::String(format!("Unknown AUX action: {action}"))),
    };
    ctx.replicate_verbatim();
    result
}
