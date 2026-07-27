use crate::{
    config::CONFIGURATION_CACHE_SIZE, graph_core::ThreadedGraph, redis_type::GRAPH_TYPE,
    serializers,
};
use graph::graph::mvcc_graph::MvccGraph;
use parking_lot::RwLock;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};
use std::sync::Arc;

/// First byte of a `DUMP` payload for a module value (`RDB_TYPE_MODULE_2`).
///
/// The internal `vec_save_graph` format always starts with a type tag in the
/// range 0..=6 (see `buffered_io`), so this byte unambiguously distinguishes a
/// Redis `DUMP` payload from the internal replication payload.
const RDB_TYPE_MODULE_2: u8 = 7;

pub fn graph_restore(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let dest_key_name = args.next_arg()?;
    let data_arg = args.next_arg()?;

    let dest_name = std::str::from_utf8(dest_key_name.as_slice())
        .map_err(|_| RedisError::Str("ERR destination key is not valid UTF-8"))?;

    // Verify dest key does not already exist. The key handle is scoped so it is
    // closed before any nested RM_Call re-opens the same key below.
    {
        let dest_key = ctx.open_key_writable(&dest_key_name);
        if dest_key
            .get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?
            .is_some()
        {
            return Err(RedisError::Str("restore graph failed, key already exists"));
        }
        if dest_key.key_type() != redis_module::KeyType::Empty {
            return Err(RedisError::Str("restore graph failed, key already exists"));
        }
    }

    // A payload produced by the Redis `DUMP` command carries the full RDB
    // framing (module id, opcode-tagged chunks, optional LZF compression, RDB
    // version + CRC64 footer). Delegate it to Redis' own `RESTORE`, which
    // validates the footer and drives the module's `rdb_load` callback --
    // that callback registers the graph under the destination key name.
    if data_arg.as_slice().first() == Some(&RDB_TYPE_MODULE_2) {
        let ttl = ctx.create_string("0");
        ctx.call("RESTORE", &[&dest_key_name, &ttl, &data_arg][..])?;

        // RESTORE accepts any module payload; make sure what landed on the key
        // is actually a graph, otherwise undo it rather than leaving a foreign
        // value behind under a GRAPH.* command.
        let is_graph = {
            let key = ctx.open_key_writable(&dest_key_name);
            key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)
                .is_ok_and(|v| v.is_some())
        };
        if !is_graph {
            ctx.call("DEL", &[&dest_key_name][..])?;
            return Err(RedisError::Str(
                "restore graph failed, payload is not a graph",
            ));
        }

        ctx.replicate_verbatim();
        return Ok(RedisValue::SimpleStringStatic("OK"));
    }

    let cache_size = *CONFIGURATION_CACHE_SIZE.lock(ctx) as usize;

    let data = data_arg.as_slice();
    let new_graph = serializers::decoder::vec_load_graph(data, cache_size, dest_name)
        .map_err(RedisError::String)?;

    // Wrap the decoded graph and set on dest key.
    let mvcc = MvccGraph::from_graph(new_graph);
    let graph_arc = mvcc.read();
    graph_arc.borrow().set_indexer_graph(graph_arc.clone());
    let tg = ThreadedGraph::from_mvcc(mvcc);
    let boxed = Arc::new(RwLock::new(tg));

    let dest_key = ctx.open_key_writable(&dest_key_name);
    dest_key.set_value(&GRAPH_TYPE, boxed.clone())?;
    crate::graph_core::register_graph(dest_name.to_string(), boxed);

    // Replicate verbatim so sub-replicas also receive GRAPH.RESTORE.
    ctx.replicate_verbatim();

    Ok(RedisValue::SimpleStringStatic("OK"))
}
