#![allow(clippy::cast_possible_wrap)]

use graph::{
    ast::Variable,
    cypher::Parser,
    graph::{
        graph::{Graph, Plan},
        matrix::init,
    },
    planner::Planner,
    runtime::{
        functions::init_functions,
        runtime::{GetVariables, QueryStatistics, ResultSummary, Runtime, evaluate_param},
        value::Value,
    },
};
use lazy_static::lazy_static;
#[cfg(feature = "zipkin")]
use opentelemetry::global;
#[cfg(feature = "zipkin")]
use opentelemetry::trace::TracerProvider;
#[cfg(feature = "zipkin")]
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor};
#[cfg(feature = "zipkin")]
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
#[cfg(feature = "zipkin")]
use opentelemetry_zipkin::ZipkinExporter;
use orx_tree::{Bfs, NodeRef};
use redis_module::{
    Context, NextArg, REDISMODULE_TYPE_METHOD_VERSION, RedisError, RedisGILGuard,
    RedisModule_Alloc, RedisModule_Calloc, RedisModule_Free, RedisModule_Realloc, RedisModuleIO,
    RedisModuleTypeMethods, RedisResult, RedisString, RedisValue, Status,
    configuration::ConfigurationFlags, native_types::RedisType, raw, redis_module,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    os::raw::{c_char, c_void},
    ptr::null_mut,
};
#[cfg(feature = "fuzz")]
use std::{fs::File, io::Write};
#[cfg(feature = "zipkin")]
use tracing_opentelemetry::OpenTelemetryLayer;
#[cfg(feature = "zipkin")]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(feature = "zipkin")]
use tracing_subscriber::util::SubscriberInitExt;

const EMPTY_KEY_ERR: RedisResult = Err(RedisError::Str("ERR Invalid graph operation on empty key"));

static GRAPH_TYPE: RedisType = RedisType::new(
    "graphdata",
    0,
    RedisModuleTypeMethods {
        version: REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(graph_rdb_load),
        rdb_save: Some(graph_rdb_save),
        aof_rewrite: None,
        free: Some(my_free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save2: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,

        copy2: None,
        free_effort2: None,
        mem_usage2: None,
        unlink2: None,
    },
);

#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_load(
    _: *mut RedisModuleIO,
    _: i32,
) -> *mut c_void {
    null_mut()
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_save(
    _: *mut RedisModuleIO,
    _: *mut c_void,
) {
}

#[unsafe(no_mangle)]
unsafe extern "C" fn my_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<RefCell<Graph>>()));
    }
}

#[allow(clippy::too_many_lines)]
fn reply_compact_value(
    ctx: &Context,
    g: &RefCell<Graph>,
    r: Value,
) {
    match r {
        Value::Null => {
            raw::reply_with_long_long(ctx.ctx, 1);
            raw::reply_with_null(ctx.ctx);
        }
        Value::Bool(x) => {
            raw::reply_with_long_long(ctx.ctx, 4);
            let str = if x { "true" } else { "false" };
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        Value::Int(x) => {
            raw::reply_with_long_long(ctx.ctx, 3);
            raw::reply_with_long_long(ctx.ctx, x as _);
        }
        Value::Float(x) => {
            raw::reply_with_long_long(ctx.ctx, 5);
            let str = format!("{x:.14e}");
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        Value::String(x) => {
            raw::reply_with_long_long(ctx.ctx, 2);
            raw::reply_with_string_buffer(ctx.ctx, x.as_str().as_ptr().cast::<c_char>(), x.len());
        }
        Value::List(values) => {
            raw::reply_with_long_long(ctx.ctx, 6);
            raw::reply_with_array(ctx.ctx, values.len() as _);
            for v in values {
                raw::reply_with_array(ctx.ctx, 2);
                reply_compact_value(ctx, g, v.clone());
            }
        }
        Value::Map(map) => {
            raw::reply_with_long_long(ctx.ctx, 10);
            raw::reply_with_array(ctx.ctx, (map.len() * 2) as _);

            for (key, value) in map.iter() {
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    key.as_str().as_ptr().cast::<c_char>(),
                    key.len(),
                );
                raw::reply_with_array(ctx.ctx, 2);
                reply_compact_value(ctx, g, value.clone());
            }
        }
        Value::Node(id) => {
            raw::reply_with_long_long(ctx.ctx, 8);
            raw::reply_with_array(ctx.ctx, 3);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            let labels = g.borrow().get_node_label_ids(id).collect::<Vec<_>>();
            raw::reply_with_array(ctx.ctx, labels.len() as _);
            for label in labels {
                raw::reply_with_long_long(ctx.ctx, usize::from(label) as _);
            }
            let bg = g.borrow();
            let props = bg.get_node_attrs(id);
            raw::reply_with_array(ctx.ctx, props.len() as _);
            for (key, value) in props {
                raw::reply_with_array(ctx.ctx, 3);
                raw::reply_with_long_long(ctx.ctx, usize::from(*key) as _);
                reply_compact_value(ctx, g, value.clone());
            }
        }
        Value::Relationship(id, from, to) => {
            raw::reply_with_long_long(ctx.ctx, 7);
            raw::reply_with_array(ctx.ctx, 5);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            raw::reply_with_long_long(
                ctx.ctx,
                usize::from(g.borrow().get_relationship_type_id(id)) as _,
            );
            raw::reply_with_long_long(ctx.ctx, u64::from(from) as _);
            raw::reply_with_long_long(ctx.ctx, u64::from(to) as _);
            let bg = g.borrow();
            let props = bg.get_relationship_attrs(id);
            raw::reply_with_array(ctx.ctx, props.len() as _);
            for (key, value) in props {
                raw::reply_with_array(ctx.ctx, 3);
                raw::reply_with_long_long(ctx.ctx, usize::from(*key) as _);
                reply_compact_value(ctx, g, value.clone());
            }
        }
        Value::Path(path) => {
            raw::reply_with_long_long(ctx.ctx, 9);
            raw::reply_with_array(ctx.ctx, 2);

            let mut nodes = 0;
            let mut rels = 0;
            for node in &path {
                match node {
                    Value::Node(_) => nodes += 1,
                    Value::Relationship(_, _, _) => rels += 1,
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }

            raw::reply_with_array(ctx.ctx, 2);
            raw::reply_with_long_long(ctx.ctx, 6);
            raw::reply_with_array(ctx.ctx, nodes);
            for node in &path {
                match node {
                    Value::Node(_) => {
                        raw::reply_with_array(ctx.ctx, 2);
                        reply_compact_value(ctx, g, node.clone());
                    }
                    Value::Relationship(_, _, _) => {}
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }

            raw::reply_with_array(ctx.ctx, 2);
            raw::reply_with_long_long(ctx.ctx, 6);
            raw::reply_with_array(ctx.ctx, rels);
            for node in path {
                match node {
                    Value::Node(_) => {}
                    Value::Relationship(_, _, _) => {
                        raw::reply_with_array(ctx.ctx, 2);
                        reply_compact_value(ctx, g, node.clone());
                    }
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }
        }
        Value::Rc(inner) => {
            reply_compact_value(ctx, g, (*inner).clone());
        }
    }
}

#[allow(clippy::too_many_lines)]
fn reply_verbose_value(
    ctx: &Context,
    g: &RefCell<Graph>,
    r: Value,
) {
    match r {
        Value::Null => {
            raw::reply_with_null(ctx.ctx);
        }
        Value::Bool(x) => {
            let str = if x { "true" } else { "false" };
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        Value::Int(x) => {
            raw::reply_with_long_long(ctx.ctx, x as _);
        }
        Value::Float(x) => {
            let str = format!("{x:.14e}");
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        Value::String(x) => {
            raw::reply_with_string_buffer(ctx.ctx, x.as_str().as_ptr().cast::<c_char>(), x.len());
        }
        Value::List(values) => {
            raw::reply_with_array(ctx.ctx, values.len() as _);
            for v in values {
                reply_verbose_value(ctx, g, v.clone());
            }
        }
        Value::Map(map) => {
            raw::reply_with_array(ctx.ctx, (map.len() * 2) as _);

            for (key, value) in map.iter() {
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    key.as_str().as_ptr().cast::<c_char>(),
                    key.len(),
                );
                reply_verbose_value(ctx, g, value.clone());
            }
        }
        Value::Node(id) => {
            raw::reply_with_array(ctx.ctx, 3);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            let labels = g.borrow().get_node_label_ids(id).collect::<Vec<_>>();
            raw::reply_with_array(ctx.ctx, labels.len() as _);
            for label in labels {
                let label = g.borrow().get_label_by_id(label);
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    label.as_ptr().cast::<c_char>(),
                    label.len(),
                );
            }
            let bg = g.borrow();
            let props = bg.get_node_attrs(id);
            raw::reply_with_array(ctx.ctx, props.len() as _);
            for (key, value) in props {
                raw::reply_with_array(ctx.ctx, 2);
                let key_name = bg.get_node_attribute_string(*key).unwrap();
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    key_name.as_ptr().cast::<c_char>(),
                    key_name.len(),
                );
                reply_verbose_value(ctx, g, value.clone());
            }
        }
        Value::Relationship(id, from, to) => {
            raw::reply_with_array(ctx.ctx, 5);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            let rel_type = g
                .borrow()
                .get_type(g.borrow().get_relationship_type_id(id))
                .unwrap();
            raw::reply_with_string_buffer(
                ctx.ctx,
                rel_type.as_ptr().cast::<c_char>(),
                rel_type.len(),
            );
            raw::reply_with_long_long(ctx.ctx, u64::from(from) as _);
            raw::reply_with_long_long(ctx.ctx, u64::from(to) as _);
            let bg = g.borrow();
            let props = bg.get_relationship_attrs(id);
            raw::reply_with_array(ctx.ctx, props.len() as _);
            for (key, value) in props {
                raw::reply_with_array(ctx.ctx, 2);
                let key_name = bg.get_relationship_attribute_string(*key).unwrap();
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    key_name.as_ptr().cast::<c_char>(),
                    key_name.len(),
                );
                reply_verbose_value(ctx, g, value.clone());
            }
        }
        Value::Path(path) => {
            raw::reply_with_array(ctx.ctx, path.len() as _);

            for node in path {
                match node {
                    Value::Relationship(_, _, _) | Value::Node(_) => {
                        reply_verbose_value(ctx, g, node.clone());
                    }
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }
        }
        Value::Rc(inner) => {
            reply_verbose_value(ctx, g, (*inner).clone());
        }
    }
}

/// This function is used to delete a graph
///
/// See: <https://docs.falkordb.com/commands/graph.delete.html>
///
/// # Example
///
/// ```sh
/// 127.0.0.1:6379> GRAPH.DELETE graph
/// OK
/// ```
fn graph_delete(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let key = ctx.open_key_writable(&key);
    if key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)?.is_some() {
        key.delete()
    } else {
        EMPTY_KEY_ERR
    }
}

#[inline]
fn query_mut(
    ctx: &Context,
    graph: &RefCell<Graph>,
    query: &str,
    compact: bool,
    write: bool,
) -> Result<(), RedisError> {
    // Create a child span for parsing and execution
    tracing::debug_span!("query_execution", query = %query).in_scope(|| {
        let Plan {
            plan,
            cached,
            parameters,
            ..
        } = graph.borrow().get_plan(query).map_err(RedisError::String)?;
        let parameters = parameters
            .into_iter()
            .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
            .collect::<Result<HashMap<_, _>, String>>()
            .map_err(RedisError::String)?;
        let scope = CONFIGURATION_IMPORT_FOLDER.lock(ctx);
        let mut runtime = Runtime::new(graph, parameters, write, plan, false, (*scope).clone());
        let mut result = runtime.query().map_err(RedisError::String)?;
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, graph, &runtime.return_names, result);
        } else {
            reply_verbose(ctx, graph, &runtime.return_names, result);
        }
        Ok(())
    })
}

fn reply_stats(
    ctx: &Context,
    stats: &QueryStatistics,
) {
    let mut stats_len = 2;
    if stats.labels_added > 0 {
        stats_len += 1;
    }
    if stats.nodes_created > 0 {
        stats_len += 1;
    }
    if stats.nodes_deleted > 0 {
        stats_len += 1;
    }
    if stats.properties_set > 0 {
        stats_len += 1;
    }
    if stats.properties_removed > 0 {
        stats_len += 1;
    }
    if stats.relationships_created > 0 {
        stats_len += 1;
    }
    if stats.relationships_deleted > 0 {
        stats_len += 1;
    }

    raw::reply_with_array(ctx.ctx, stats_len.into());
    if stats.labels_added > 0 {
        let str = format!("Labels added: {}", stats.labels_added);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.nodes_created > 0 {
        let str = format!("Nodes created: {}", stats.nodes_created);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.nodes_deleted > 0 {
        let str = format!("Nodes deleted: {}", stats.nodes_deleted);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.properties_set > 0 {
        let str = format!("Properties set: {}", stats.properties_set);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.properties_removed > 0 {
        let str = format!("Properties removed: {}", stats.properties_removed);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.relationships_created > 0 {
        let str = format!("Relationships created: {}", stats.relationships_created);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.relationships_deleted > 0 {
        let str = format!("Relationships deleted: {}", stats.relationships_deleted);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    let str = format!("Cached execution: {}", i32::from(stats.cached));
    raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    let str = format!(
        "Query internal execution time: {} milliseconds",
        stats.execution_time
    );
    raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
}

#[cfg(feature = "fuzz")]
static mut file_id: i32 = 0;

#[allow(static_mut_refs)]
fn graph_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    #[cfg(feature = "fuzz")]
    unsafe {
        //  write the quert to file
        let mut file = File::create(format!(
            "fuzz/corpus/fuzz_target_runtime/output{file_id}.txt"
        ))?;
        file.write_all(query.as_bytes())?;
        drop(file);
        file_id += 1;
    }

    let compact = args.next_str().is_ok_and(|arg| arg == "--compact");
    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)? {
        query_mut(ctx, graph, query, compact, true)?;
    } else {
        let scope = CONFIGURATION_CACHE_SIZE.lock(ctx);
        let graph = RefCell::new(Graph::new(16384, 16384, *scope as usize));
        query_mut(ctx, &graph, query, compact, true)?;
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

#[inline]
fn record_mut(
    ctx: &Context,
    graph: &RefCell<Graph>,
    query: &str,
) -> Result<(), RedisError> {
    // Create a child span for parsing and execution
    let Plan {
        plan, parameters, ..
    } = graph.borrow().get_plan(query).map_err(RedisError::String)?;
    let parameters = parameters
        .into_iter()
        .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
        .collect::<Result<HashMap<_, _>, String>>()
        .map_err(RedisError::String)?;
    let scope = CONFIGURATION_IMPORT_FOLDER.lock(ctx);
    let mut runtime = Runtime::new(
        graph,
        parameters,
        true,
        plan.clone(),
        true,
        (*scope).clone(),
    );
    let _ = runtime.query();
    raw::reply_with_array(ctx.ctx, 2);
    raw::reply_with_array(ctx.ctx, runtime.record.borrow().len() as _);
    for (idx, res) in runtime.record.borrow().iter() {
        let idx_str = format!("{idx:?}");
        raw::reply_with_array(ctx.ctx, 3);
        raw::reply_with_string_buffer(ctx.ctx, idx_str[17..].as_ptr().cast::<c_char>(), 9);
        match res {
            Err(err) => {
                raw::reply_with_long_long(ctx.ctx, 0);
                raw::reply_with_string_buffer(ctx.ctx, err.as_ptr().cast::<c_char>(), err.len());
            }
            Ok(env) => {
                raw::reply_with_long_long(ctx.ctx, 1);
                let vars = plan.node(idx).get_variables();
                raw::reply_with_array(ctx.ctx, vars.len() as _);
                for name in &vars {
                    match env.get(name) {
                        None => {
                            raw::reply_with_null(ctx.ctx);
                        }
                        Some(value) => {
                            reply_verbose_value(ctx, graph, value);
                        }
                    }
                }
            }
        }
    }

    let len = plan.root().indices::<Bfs>().count();
    raw::reply_with_array(ctx.ctx, len as _);
    for idx in plan.root().indices::<Bfs>() {
        raw::reply_with_array(ctx.ctx, 4);
        let idx_str = format!("{idx:?}");
        raw::reply_with_string_buffer(ctx.ctx, idx_str[17..].as_ptr().cast::<c_char>(), 9);
        match plan.node(&idx).parent() {
            Some(parent_idx) => {
                let parent_idx_str = format!("{:?}", parent_idx.idx());
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    parent_idx_str[17..].as_ptr().cast::<c_char>(),
                    9,
                );
            }
            None => {
                raw::reply_with_null(ctx.ctx);
            }
        }
        let node = plan.node(&idx).data().to_string();
        raw::reply_with_string_buffer(ctx.ctx, node.as_ptr().cast::<c_char>(), node.len());
        let vars = plan.node(&idx).get_variables();
        raw::reply_with_array(ctx.ctx, vars.len() as _);
        for var in vars {
            raw::reply_with_string_buffer(
                ctx.ctx,
                var.as_str().as_ptr().cast::<c_char>(),
                var.as_str().len(),
            );
        }
    }
    Ok(())
}

fn graph_record(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)? {
        record_mut(ctx, graph, query)?;
    } else {
        let scope = CONFIGURATION_CACHE_SIZE.lock(ctx);
        let graph = RefCell::new(Graph::new(16384, 16384, *scope as usize));
        record_mut(ctx, &graph, query)?;
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

fn reply_verbose(
    ctx: &Context,
    g: &RefCell<Graph>,
    return_names: &Vec<Variable>,
    result: ResultSummary,
) {
    raw::reply_with_array(ctx.ctx, 3);
    raw::reply_with_array(ctx.ctx, return_names.len() as _);
    for name in return_names {
        raw::reply_with_array(ctx.ctx, 2);
        raw::reply_with_long_long(ctx.ctx, 1);
        raw::reply_with_string_buffer(
            ctx.ctx,
            name.as_str().as_ptr().cast::<c_char>(),
            name.as_str().len(),
        );
    }
    raw::reply_with_array(ctx.ctx, result.result.len() as _);
    for row in result.result {
        raw::reply_with_array(ctx.ctx, return_names.len() as _);
        for name in return_names {
            reply_verbose_value(ctx, g, row.get(name).unwrap());
        }
    }
    reply_stats(ctx, &result.stats);
}

fn reply_compact(
    ctx: &Context,
    g: &RefCell<Graph>,
    return_names: &Vec<Variable>,
    result: ResultSummary,
) {
    raw::reply_with_array(ctx.ctx, 3);
    raw::reply_with_array(ctx.ctx, return_names.len() as _);
    for name in return_names {
        raw::reply_with_array(ctx.ctx, 2);
        raw::reply_with_long_long(ctx.ctx, 1);
        raw::reply_with_string_buffer(
            ctx.ctx,
            name.as_str().as_ptr().cast::<c_char>(),
            name.as_str().len(),
        );
    }
    raw::reply_with_array(ctx.ctx, result.result.len() as _);
    for row in result.result {
        raw::reply_with_array(ctx.ctx, return_names.len() as _);
        for name in return_names {
            raw::reply_with_array(ctx.ctx, 2);
            reply_compact_value(ctx, g, row.get(name).unwrap());
        }
    }
    reply_stats(ctx, &result.stats);
}

/// This function is used to execute a read only query on a graph
///
/// See: <https://docs.falkordb.com/commands/graph.ro_query.html>
///
/// # Example
///
/// ```sh
/// GRAPH.RO_QUERY graph "MATCH (n) RETURN n"
/// ```
fn graph_ro_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;
    let compact = args.next_str().is_ok_and(|arg| arg == "--compact");

    let key = ctx.open_key(&key);

    // We check if the key exists and is of type Graph if wrong type `get_value` return an error
    (key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)?).map_or(
        // If the key does not exist, we return an error
        EMPTY_KEY_ERR,
        |graph| {
            query_mut(ctx, graph, query, compact, false)?;
            RedisResult::Ok(RedisValue::NoReply)
        },
    )
}

/// This function is used to list all the graphs
/// in the database. It returns a list of graphs IDs
/// that are currently stored in the database.
///
/// See: <https://docs.falkordb.com/commands/graph.list.html>
///
/// # Example
///
/// ```sh
/// 127.0.0.1:6379> GRAPH.LIST
/// 2) G
/// 3) resources
/// 4) players
/// ```
#[allow(clippy::needless_pass_by_value)]
fn graph_list(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }

    let mut a = [
        ctx.create_string("0"),
        ctx.create_string("TYPE"),
        ctx.create_string("graphdata"),
    ];
    let mut res = Vec::new();
    loop {
        let call_res = ctx.call("SCAN", a.iter().collect::<Vec<_>>().as_slice())?;
        match call_res {
            RedisValue::Array(mut arr) => {
                if let RedisValue::Array(arr) = arr.remove(1) {
                    res.extend(arr);
                }
                if let RedisValue::SimpleString(i) = arr.remove(0) {
                    if i == "0" {
                        return Ok(RedisValue::Array(res));
                    }
                    a[0] = ctx.create_string(i);
                }
            }
            _ => return Err(RedisError::Str("ERR Failed to list graphs")),
        }
    }
}

fn graph_parse(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let query = args.next_str()?;

    let mut parser = Parser::new(query);
    match parser.parse() {
        Ok(ir) => Ok(RedisValue::BulkString(format!("{ir}"))),
        Err(err) => Err(RedisError::String(err)),
    }
}

fn graph_plan(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let query = args.next_str()?;

    let mut parser = Parser::new(query);
    match parser.parse() {
        Ok(ir) => {
            let mut planner = Planner::default();
            let ir = planner.plan(ir);
            Ok(RedisValue::BulkString(format!("{ir}")))
        }
        Err(err) => Err(RedisError::String(err)),
    }
}

#[cfg(feature = "zipkin")]
fn init_zipkin() {
    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());

    let exporter = ZipkinExporter::builder().build().unwrap();

    let batch = BatchSpanProcessor::builder(exporter)
        .with_batch_config(
            BatchConfigBuilder::default()
                .with_max_queue_size(4096)
                .build(),
        )
        .build();

    let provider = SdkTracerProvider::builder()
        .with_span_processor(batch)
        .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
        .with_resource(
            Resource::builder_empty()
                .with_service_name("falkordb-graph-engine")
                .build(),
        )
        .build();
    let tracer = provider.tracer("falkordb-graph-engine");
    let layer = OpenTelemetryLayer::new(tracer);
    tracing_subscriber::registry().with(layer).init();

    global::set_tracer_provider(provider);
}

fn graph_init(
    _: &Context,
    _: &Vec<RedisString>,
) -> Status {
    #[cfg(feature = "zipkin")]
    init_zipkin();

    unsafe {
        init(
            RedisModule_Alloc,
            RedisModule_Calloc,
            RedisModule_Realloc,
            RedisModule_Free,
        );
    }
    match init_functions() {
        Ok(()) => Status::Ok,
        Err(_) => Status::Err,
    }
}

lazy_static! {
    static ref CONFIGURATION_IMPORT_FOLDER: RedisGILGuard<String> =
        RedisGILGuard::new("/var/lib/FalkorDB/import/".into());
    static ref CONFIGURATION_CACHE_SIZE: RedisGILGuard<i64> = RedisGILGuard::new(25.into());
}

//////////////////////////////////////////////////////

redis_module! {
    name: "falkordb",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [GRAPH_TYPE],
    init: graph_init,
    commands: [
        ["graph.DELETE", graph_delete, "write", 1, 1, 1, ""],
        ["graph.QUERY", graph_query, "write deny-oom", 1, 1, 1, ""],
        ["graph.RO_QUERY", graph_ro_query, "readonly", 1, 1, 1, ""],
        ["graph.LIST", graph_list, "readonly", 0, 0, 0, ""],
        ["graph.PARSE", graph_parse, "readonly", 0, 0, 0, ""],
        ["graph.PLAN", graph_plan, "readonly", 0, 0, 0, ""],
        ["graph.RECORD", graph_record, "write deny-oom", 1, 1, 1, ""],
    ],
    configurations: [
        i64: [
            ["CACHE_SIZE", &*CONFIGURATION_CACHE_SIZE, 25, 0, 1000, ConfigurationFlags::DEFAULT, None],
        ],
        string: [
            ["IMPORT_FOLDER", &*CONFIGURATION_IMPORT_FOLDER, "/var/lib/FalkorDB/import/", ConfigurationFlags::DEFAULT, None],
        ],
        bool: [],
        enum: [],
        module_args_as_configuration: true,
    ]
}
