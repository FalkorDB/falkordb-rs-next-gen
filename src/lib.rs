#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::non_std_lazy_statics)]

mod allocator;
use allocator::ThreadCountingAllocator;
use atomic_refcell::AtomicRefCell;
use graph::{
    graph::{
        graph::{Graph, Plan},
        matrix::init,
        mvcc_graph::MvccGraph,
    },
    planner::IR,
    redisearch::{REDISEARCH_INIT_LIBRARY, RediSearch_Init},
    runtime::{
        functions::init_functions,
        runtime::{GetVariables, QueryStatistics, ResultSummary, Runtime, evaluate_param},
        value::Value,
    },
    threadpool::spawn,
};
use lazy_static::lazy_static;
use orx_tree::{Bfs, Collection, Dfs, NodeRef};
#[cfg(feature = "pyro")]
use pyroscope::PyroscopeAgent;
#[cfg(feature = "pyro")]
use pyroscope_pprofrs::{PprofConfig, pprof_backend};
use redis_module::{
    Context, NextArg, REDISMODULE_OK, REDISMODULE_TYPE_METHOD_VERSION, RedisError, RedisGILGuard,
    RedisModule_Alloc, RedisModule_Calloc, RedisModule_Free, RedisModule_Realloc,
    RedisModule_SubscribeToServerEvent, RedisModuleCtx, RedisModuleEvent, RedisModuleIO,
    RedisModuleTypeMethods, RedisResult, RedisString, RedisValue, Status,
    configuration::ConfigurationFlags, native_types::RedisType, raw, redis_module,
};
#[cfg(feature = "pyro")]
use std::mem;
use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
    ptr::null_mut,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, Sender, channel},
    },
};
#[cfg(feature = "fuzz")]
use std::{fs::File, io::Write};

use crate::allocator::{current_thread_usage, disable_tracking, enable_tracking, reset_counter};

const EMPTY_KEY_ERR: RedisResult = Err(RedisError::Str("ERR Invalid graph operation on empty key"));

static GRAPH_TYPE: RedisType = RedisType::new(
    "graphdata",
    0,
    RedisModuleTypeMethods {
        version: REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(graph_rdb_load),
        rdb_save: Some(graph_rdb_save),
        aof_rewrite: None,
        free: Some(graph_free),

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

struct ThreadedGraph {
    graph: MvccGraph,
    sender: Sender<(BlockedClient, Arc<String>, bool)>,
    receiver: Receiver<(BlockedClient, Arc<String>, bool)>,
    write_loop: AtomicBool,
}

unsafe impl Send for ThreadedGraph {}
unsafe impl Sync for ThreadedGraph {}

impl ThreadedGraph {
    pub fn new(cache_size: usize) -> Self {
        let (sender, receiver) = channel();
        Self {
            graph: MvccGraph::new(16384, 16384, cache_size),
            sender,
            receiver,
            write_loop: AtomicBool::new(false),
        }
    }

    fn execute_query(
        &self,
        ctx: &Context,
        query: &str,
        compact: bool,
        write: bool,
    ) -> Result<bool, String> {
        let Plan {
            plan,
            cached,
            parameters,
            ..
        } = self.graph.read().borrow().get_plan(query)?;
        let parameters = parameters
            .into_iter()
            .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        let is_write = plan.iter().any(|n| matches!(n, IR::Commit));
        let g = if is_write {
            if !write {
                return Err(String::from(
                    "graph.RO_QUERY is to be executed only on read-only queries",
                ));
            }
            return Ok(is_write);
        } else {
            self.graph.read()
        };
        let mut runtime = Runtime::new(
            g,
            parameters,
            write,
            plan,
            false,
            (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
        );
        let mut result = runtime.query()?;
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, &runtime, result);
        } else {
            reply_verbose(ctx, &runtime, result);
        }
        Ok(is_write)
    }

    fn execute_query_write(
        &self,
        ctx: &Context,
        query: &str,
        compact: bool,
    ) -> Result<Arc<AtomicRefCell<Graph>>, String> {
        let Plan {
            plan,
            cached,
            parameters,
            ..
        } = self.graph.read().borrow().get_plan(query)?;
        let parameters = parameters
            .into_iter()
            .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        debug_assert!(plan.iter().any(|n| matches!(n, IR::Commit)));
        let g = self.graph.write().unwrap();
        let mut runtime = Runtime::new(
            g.clone(),
            parameters,
            true,
            plan,
            false,
            (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
        );
        let mut result = runtime.query()?;
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, &runtime, result);
        } else {
            reply_verbose(ctx, &runtime, result);
        }
        Ok(g)
    }
}

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
unsafe extern "C" fn graph_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<Arc<RwLock<ThreadedGraph>>>()));
    }
}

#[allow(clippy::too_many_lines)]
fn reply_compact_value(
    ctx: &Context,
    runtime: &Runtime,
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
        Value::Datetime(ts) => {
            raw::reply_with_long_long(ctx.ctx, 13);
            raw::reply_with_long_long(ctx.ctx, ts as _);
        }
        Value::Date(ts) => {
            raw::reply_with_long_long(ctx.ctx, 14);
            raw::reply_with_long_long(ctx.ctx, ts as _);
        }
        Value::Time(ts) => {
            raw::reply_with_long_long(ctx.ctx, 15);
            raw::reply_with_long_long(ctx.ctx, ts as _);
        }
        Value::Duration(dur) => {
            raw::reply_with_long_long(ctx.ctx, 16);
            raw::reply_with_long_long(ctx.ctx, dur as _);
        }
        Value::List(values) => {
            raw::reply_with_long_long(ctx.ctx, 6);
            raw::reply_with_array(ctx.ctx, values.len() as _);
            for v in values {
                raw::reply_with_array(ctx.ctx, 2);
                reply_compact_value(ctx, runtime, v.clone());
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
                reply_compact_value(ctx, runtime, value.clone());
            }
        }
        Value::Node(id) => {
            raw::reply_with_long_long(ctx.ctx, 8);
            raw::reply_with_array(ctx.ctx, 3);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            let dn = runtime.deleted_nodes.borrow();
            if let Some(x) = dn.get(&id) {
                raw::reply_with_array(ctx.ctx, x.labels.len() as _);
                for label in &x.labels {
                    raw::reply_with_long_long(ctx.ctx, usize::from(*label) as _);
                }
                raw::reply_with_array(ctx.ctx, x.attrs.len() as _);
                for (key, value) in x.attrs.iter() {
                    raw::reply_with_array(ctx.ctx, 3);
                    let key = runtime.g.borrow().get_node_attribute_id(key).unwrap();
                    raw::reply_with_long_long(ctx.ctx, key as _);
                    reply_compact_value(ctx, runtime, value.clone());
                }
            } else {
                let bg = runtime.g.borrow();
                let labels = bg.get_node_label_ids(id).collect::<Vec<_>>();
                raw::reply_with_array(ctx.ctx, labels.len() as _);
                for label in labels {
                    raw::reply_with_long_long(ctx.ctx, usize::from(label) as _);
                }
                let attrs = bg.get_node_attrs(id);
                raw::reply_with_array(ctx.ctx, attrs.len() as _);
                for key in attrs {
                    raw::reply_with_array(ctx.ctx, 3);
                    let attr_id = bg.get_node_attribute_id(&key);
                    raw::reply_with_long_long(ctx.ctx, attr_id.unwrap() as _);
                    reply_compact_value(ctx, runtime, bg.get_node_attribute(id, &key).unwrap());
                }
            }
        }
        Value::Relationship(rel) => {
            raw::reply_with_long_long(ctx.ctx, 7);
            raw::reply_with_array(ctx.ctx, 5);
            raw::reply_with_long_long(ctx.ctx, u64::from(rel.0) as _);
            let dr = runtime.deleted_relationships.borrow();
            if let Some(x) = dr.get(&rel.0) {
                raw::reply_with_long_long(ctx.ctx, usize::from(x.type_id) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.1) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.2) as _);
                raw::reply_with_array(ctx.ctx, x.attrs.len() as _);
                let bg = runtime.g.borrow();
                for (key, value) in x.attrs.iter() {
                    raw::reply_with_array(ctx.ctx, 3);
                    let key = bg.get_relationship_attribute_id(key).unwrap();
                    raw::reply_with_long_long(ctx.ctx, key as _);
                    reply_compact_value(ctx, runtime, value.clone());
                }
            } else {
                let bg = runtime.g.borrow();
                raw::reply_with_long_long(
                    ctx.ctx,
                    usize::from(bg.get_relationship_type_id(rel.0)) as _,
                );
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.1) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.2) as _);
                let attrs = bg.get_relationship_attrs(rel.0);
                raw::reply_with_array(ctx.ctx, attrs.len() as _);
                for key in attrs {
                    raw::reply_with_array(ctx.ctx, 3);
                    let attr_id = bg.get_relationship_attribute_id(&key);
                    raw::reply_with_long_long(ctx.ctx, attr_id.unwrap() as _);
                    reply_compact_value(
                        ctx,
                        runtime,
                        bg.get_relationship_attribute(rel.0, &key).unwrap(),
                    );
                }
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
                    Value::Relationship(_) => rels += 1,
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
                        reply_compact_value(ctx, runtime, node.clone());
                    }
                    Value::Relationship(_) => {}
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }

            raw::reply_with_array(ctx.ctx, 2);
            raw::reply_with_long_long(ctx.ctx, 6);
            raw::reply_with_array(ctx.ctx, rels);
            for node in path {
                match node {
                    Value::Node(_) => {}
                    Value::Relationship(_) => {
                        raw::reply_with_array(ctx.ctx, 2);
                        reply_compact_value(ctx, runtime, node.clone());
                    }
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }
        }
        Value::VecF32(vec) => {
            raw::reply_with_long_long(ctx.ctx, 12);
            raw::reply_with_array(ctx.ctx, vec.len() as _);
            for f in vec {
                raw::reply_with_double(ctx.ctx, f64::from(f));
            }
        }
        Value::Point(point) => {
            raw::reply_with_long_long(ctx.ctx, 11); // VALUE_POINT type code
            raw::reply_with_array(ctx.ctx, 2);

            let lat_str = format!("{:.15}", point.latitude);
            let lat_str = lat_str.trim_end_matches('0').trim_end_matches('.');
            raw::reply_with_string_buffer(
                ctx.ctx,
                lat_str.as_ptr().cast::<c_char>(),
                lat_str.len(),
            );

            let lon_str = format!("{:.15}", point.longitude);
            let lon_str = lon_str.trim_end_matches('0').trim_end_matches('.');
            raw::reply_with_string_buffer(
                ctx.ctx,
                lon_str.as_ptr().cast::<c_char>(),
                lon_str.len(),
            );
        }
        Value::Arc(inner) => {
            reply_compact_value(ctx, runtime, (*inner).clone());
        }
    }
}

#[allow(clippy::too_many_lines)]
fn reply_verbose_value(
    ctx: &Context,
    runtime: &Runtime,
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
        Value::Datetime(ts) => {
            let formatted = Value::format_datetime(ts);
            raw::reply_with_string_buffer(
                ctx.ctx,
                formatted.as_ptr().cast::<c_char>(),
                formatted.len(),
            );
        }
        Value::Date(ts) => {
            let formatted = Value::format_date(ts);
            raw::reply_with_string_buffer(
                ctx.ctx,
                formatted.as_ptr().cast::<c_char>(),
                formatted.len(),
            );
        }
        Value::Time(ts) => {
            let formatted = Value::format_time(ts);
            raw::reply_with_string_buffer(
                ctx.ctx,
                formatted.as_ptr().cast::<c_char>(),
                formatted.len(),
            );
        }
        Value::Duration(dur) => {
            let formatted = Value::format_duration(dur);
            raw::reply_with_string_buffer(
                ctx.ctx,
                formatted.as_ptr().cast::<c_char>(),
                formatted.len(),
            );
        }
        Value::List(values) => {
            raw::reply_with_array(ctx.ctx, values.len() as _);
            for v in values {
                reply_verbose_value(ctx, runtime, v.clone());
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
                reply_verbose_value(ctx, runtime, value.clone());
            }
        }
        Value::Node(id) => {
            raw::reply_with_array(ctx.ctx, 3);
            raw::reply_with_long_long(ctx.ctx, u64::from(id) as _);
            let bg = runtime.g.borrow();
            let dn = runtime.deleted_nodes.borrow();
            if let Some(x) = dn.get(&id) {
                raw::reply_with_array(ctx.ctx, x.labels.len() as _);
                for label in &x.labels {
                    let label = bg.get_label_by_id(*label);
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        label.as_ptr().cast::<c_char>(),
                        label.len(),
                    );
                }
                raw::reply_with_array(ctx.ctx, x.attrs.len() as _);
                for (key, value) in x.attrs.iter() {
                    raw::reply_with_array(ctx.ctx, 2);
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        key.as_ptr().cast::<c_char>(),
                        key.len(),
                    );
                    reply_verbose_value(ctx, runtime, value.clone());
                }
            } else {
                let labels = bg.get_node_labels(id).collect::<Vec<_>>();
                raw::reply_with_array(ctx.ctx, labels.len() as _);
                for label in labels {
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        label.as_ptr().cast::<c_char>(),
                        label.len(),
                    );
                }
                let attrs = bg.get_node_attrs(id);
                raw::reply_with_array(ctx.ctx, attrs.len() as _);
                for key in attrs {
                    raw::reply_with_array(ctx.ctx, 2);
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        key.as_ptr().cast::<c_char>(),
                        key.len(),
                    );
                    reply_verbose_value(ctx, runtime, bg.get_node_attribute(id, &key).unwrap());
                }
            }
        }
        Value::Relationship(rel) => {
            raw::reply_with_array(ctx.ctx, 5);
            raw::reply_with_long_long(ctx.ctx, u64::from(rel.0) as _);
            let dr = runtime.deleted_relationships.borrow();
            if let Some(x) = dr.get(&rel.0) {
                raw::reply_with_long_long(ctx.ctx, usize::from(x.type_id) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.1) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.2) as _);
                raw::reply_with_array(ctx.ctx, x.attrs.len() as _);
                for (key, value) in x.attrs.iter() {
                    raw::reply_with_array(ctx.ctx, 2);
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        key.as_ptr().cast::<c_char>(),
                        key.len(),
                    );
                    reply_verbose_value(ctx, runtime, value.clone());
                }
            } else {
                let bg = runtime.g.borrow();
                let rel_type = bg.get_type(bg.get_relationship_type_id(rel.0)).unwrap();
                raw::reply_with_string_buffer(
                    ctx.ctx,
                    rel_type.as_ptr().cast::<c_char>(),
                    rel_type.len(),
                );
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.1) as _);
                raw::reply_with_long_long(ctx.ctx, u64::from(rel.2) as _);
                let props = bg.get_relationship_attrs(rel.0);
                raw::reply_with_array(ctx.ctx, props.len() as _);
                for key in props {
                    raw::reply_with_array(ctx.ctx, 2);
                    raw::reply_with_string_buffer(
                        ctx.ctx,
                        key.as_ptr().cast::<c_char>(),
                        key.len(),
                    );
                    reply_verbose_value(
                        ctx,
                        runtime,
                        bg.get_relationship_attribute(rel.0, &key).unwrap(),
                    );
                }
            }
        }
        Value::Path(path) => {
            raw::reply_with_array(ctx.ctx, path.len() as _);

            for node in path {
                match node {
                    Value::Relationship(_) | Value::Node(_) => {
                        reply_verbose_value(ctx, runtime, node.clone());
                    }
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }
        }
        Value::VecF32(vec) => {
            raw::reply_with_array(ctx.ctx, vec.len() as _);
            for f in vec {
                raw::reply_with_double(ctx.ctx, f64::from(f));
            }
        }
        Value::Point(point) => {
            // Format:  "point({latitude:%f, longitude:%f})"
            // Match the C implementation format exactly
            let str = format!(
                "point({{latitude:{}, longitude:{}}})",
                point.latitude, point.longitude
            );
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        Value::Arc(inner) => {
            reply_verbose_value(ctx, runtime, (*inner).clone());
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
    if key
        .get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?
        .is_some()
    {
        key.delete()
    } else {
        EMPTY_KEY_ERR
    }
}

pub struct BlockedClient {
    pub inner: *mut raw::RedisModuleBlockedClient,
}

unsafe impl Send for BlockedClient {}
unsafe impl Sync for BlockedClient {}

impl Drop for BlockedClient {
    fn drop(&mut self) {
        unsafe { raw::RedisModule_UnblockClient.unwrap()(self.inner, null_mut()) };
    }
}

#[inline]
fn query_mut(
    ctx: &Context,
    graph: &Arc<RwLock<ThreadedGraph>>,
    query: &str,
    compact: bool,
    write: bool,
    track_mem: bool,
) {
    let bc = BlockedClient {
        inner: unsafe { raw::RedisModule_BlockClient.unwrap()(ctx.ctx, None, None, None, 0) },
    };
    let graph = graph.clone();
    let query = Arc::new(query.to_string());
    spawn(
        move || {
            if track_mem {
                reset_counter();
                enable_tracking();
            }
            let g = graph.clone();
            let graph = graph.clone();
            let graph = graph.read().unwrap();
            let bc = bc;
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);

            let res = graph.execute_query(&ctx, &query, compact, write);
            match res {
                Ok(is_write) => {
                    if is_write {
                        graph.sender.send((bc, query, compact)).unwrap();
                        drop(graph);
                        process_write_queued_query(&g);
                    } else {
                        drop(bc);
                        unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    }
                }
                Err(err) => {
                    let cerr = CString::new(err).unwrap();
                    raw::reply_with_error(ctx.ctx, cerr.as_ptr().cast::<c_char>());
                    drop(bc);
                    unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                }
            }
            if track_mem {
                let (allocated, deallocated) = current_thread_usage();
                disable_tracking();
                ctx.log(
                    redis_module::logging::RedisLogLevel::Notice,
                    &format!(
                        "Allocated: {allocated} bytes, Deallocated: {deallocated} bytes, Net: {}",
                        allocated as isize - deallocated as isize
                    ),
                );
            }
        },
        None,
    );
}

fn process_write_queued_query(graph: &Arc<RwLock<ThreadedGraph>>) {
    let g = graph.read().unwrap();
    if g.write_loop
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        drop(g);
        let mut graph = graph.write().unwrap();
        while let Ok((bc, query, compact)) = { graph.receiver.try_recv() } {
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);
            let res = graph.execute_query_write(&ctx, &query, compact);
            match res {
                Ok(g) => {
                    drop(bc);
                    unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    graph.graph.commit(g);
                }
                Err(err) => {
                    let cerr = CString::new(err).unwrap();
                    raw::reply_with_error(ctx.ctx, cerr.as_ptr().cast::<c_char>());
                    drop(bc);
                    unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    graph.graph.rollback();
                }
            }
        }
        graph.write_loop.store(false, Ordering::Release);
    }
}

fn reply_stats(
    ctx: &Context,
    stats: &QueryStatistics,
    version: u64,
) {
    let mut stats_len = 3;
    if stats.labels_added > 0 {
        stats_len += 1;
    }
    if stats.labels_removed > 0 {
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
    if stats.indexes_created > 0 {
        stats_len += 1;
    }
    if stats.indexes_dropped > 0 {
        stats_len += 1;
    }

    raw::reply_with_array(ctx.ctx, stats_len.into());
    if stats.labels_added > 0 {
        let str = format!("Labels added: {}", stats.labels_added);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.labels_removed > 0 {
        let str = format!("Labels removed: {}", stats.labels_removed);
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
    if stats.indexes_created > 0 {
        let str = format!("Indices created: {}", stats.indexes_created);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    if stats.indexes_dropped > 0 {
        let str = format!("Indices deleted: {}", stats.indexes_dropped);
        raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    }
    let str = format!("Cached execution: {}", i32::from(stats.cached));
    raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    let mut buffer = ryu::Buffer::new();
    let str = buffer.format(stats.execution_time);
    let str = format!("Query internal execution time: {str} milliseconds");
    raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
    let str = format!("Graph version: {version}");
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

    let mut compact = false;
    let mut track_memory = false;
    while let Ok(arg) = args.next_str() {
        if arg == "--compact" {
            compact = true;
        } else if arg == "--track-memory" {
            track_memory = true;
        }
    }

    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)? {
        query_mut(ctx, graph, query, compact, true, track_memory);
    } else {
        let graph = Arc::new(RwLock::new(ThreadedGraph::new(
            *CONFIGURATION_CACHE_SIZE.lock(ctx) as usize,
        )));
        query_mut(ctx, &graph, query, compact, true, track_memory);
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

#[inline]
fn record_mut(
    ctx: &Context,
    graph: &Arc<RwLock<ThreadedGraph>>,
    query: &str,
) -> RedisResult {
    // Create a child span for parsing and execution
    let Plan {
        plan, parameters, ..
    } = graph
        .read()
        .unwrap()
        .graph
        .read()
        .borrow()
        .get_plan(query)
        .map_err(RedisError::String)?;
    let parameters = parameters
        .into_iter()
        .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
        .collect::<Result<HashMap<_, _>, String>>()
        .map_err(RedisError::String)?;
    let mut runtime = Runtime::new(
        graph.read().unwrap().graph.read(),
        parameters,
        true,
        plan.clone(),
        true,
        (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
    );
    let _ = runtime.query();
    let ids = plan.root().indices::<Bfs>().collect::<Vec<_>>();
    raw::reply_with_array(ctx.ctx, 2);
    raw::reply_with_array(ctx.ctx, runtime.record.borrow().len() as _);
    for (idx, res) in runtime.record.borrow().iter() {
        raw::reply_with_array(ctx.ctx, 3);
        raw::reply_with_long_long(ctx.ctx, ids.iter().position(|id| *id == *idx).unwrap() as _);
        match res {
            Err(err) => {
                raw::reply_with_long_long(ctx.ctx, 0);
                raw::reply_with_string_buffer(ctx.ctx, err.as_ptr().cast::<c_char>(), err.len());
            }
            Ok(env) => {
                raw::reply_with_long_long(ctx.ctx, 1);
                let vars = plan.node(*idx).get_variables();
                raw::reply_with_array(ctx.ctx, vars.len() as _);
                for name in &vars {
                    match env.get(name) {
                        None => {
                            raw::reply_with_null(ctx.ctx);
                        }
                        Some(value) => {
                            reply_verbose_value(ctx, &runtime, value);
                        }
                    }
                }
            }
        }
    }

    raw::reply_with_array(ctx.ctx, ids.len() as _);
    for idx in plan.root().indices::<Bfs>() {
        raw::reply_with_array(ctx.ctx, 4);
        raw::reply_with_long_long(ctx.ctx, ids.iter().position(|id| *id == idx).unwrap() as _);
        match plan.node(idx).parent() {
            Some(parent_idx) => {
                raw::reply_with_long_long(
                    ctx.ctx,
                    ids.iter().position(|id| *id == parent_idx.idx()).unwrap() as _,
                );
            }
            None => {
                raw::reply_with_null(ctx.ctx);
            }
        }
        let node = plan.node(idx).data().to_string();
        raw::reply_with_string_buffer(ctx.ctx, node.as_ptr().cast::<c_char>(), node.len());
        let vars = plan.node(idx).get_variables();
        raw::reply_with_array(ctx.ctx, vars.len() as _);
        for var in vars {
            raw::reply_with_string_buffer(
                ctx.ctx,
                var.as_str().as_ptr().cast::<c_char>(),
                var.as_str().len(),
            );
        }
    }
    Ok(RedisValue::NoReply)
}

fn graph_record(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)? {
        record_mut(ctx, graph, query)?;
    } else {
        let graph = Arc::new(RwLock::new(ThreadedGraph::new(
            *CONFIGURATION_CACHE_SIZE.lock(ctx) as usize,
        )));
        record_mut(ctx, &graph, query)?;
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

fn reply_verbose(
    ctx: &Context,
    runtime: &Runtime,
    result: ResultSummary,
) {
    raw::reply_with_array(ctx.ctx, 3);
    raw::reply_with_array(ctx.ctx, runtime.return_names.len() as _);
    for name in &runtime.return_names {
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
        raw::reply_with_array(ctx.ctx, runtime.return_names.len() as _);
        for name in &runtime.return_names {
            reply_verbose_value(ctx, runtime, row.get(name).unwrap());
        }
    }
    reply_stats(ctx, &result.stats, runtime.g.borrow().version);
}

fn reply_compact(
    ctx: &Context,
    runtime: &Runtime,
    result: ResultSummary,
) {
    raw::reply_with_array(ctx.ctx, 3);
    raw::reply_with_array(ctx.ctx, runtime.return_names.len() as _);
    for name in &runtime.return_names {
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
        raw::reply_with_array(ctx.ctx, runtime.return_names.len() as _);
        for name in &runtime.return_names {
            raw::reply_with_array(ctx.ctx, 2);
            reply_compact_value(ctx, runtime, row.get(name).unwrap());
        }
    }
    reply_stats(ctx, &result.stats, runtime.g.borrow().version);
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
    let mut compact = false;
    let mut track_memory = false;
    while let Ok(arg) = args.next_str() {
        if arg == "--compact" {
            compact = true;
        } else if arg == "--track-memory" {
            track_memory = true;
        }
    }

    let key = ctx.open_key(&key);

    // We check if the key exists and is of type Graph if wrong type `get_value` return an error
    (key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?).map_or(
        // If the key does not exist, we return an error
        EMPTY_KEY_ERR,
        |graph| {
            query_mut(ctx, graph, query, compact, false, track_memory);
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

fn graph_explain(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key(&key);

    (key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?).map_or(
        // If the key does not exist, we return an error
        EMPTY_KEY_ERR,
        |graph| {
            let Plan { plan, .. } = graph
                .read()
                .unwrap()
                .graph
                .read()
                .borrow()
                .get_plan(query)
                .map_err(RedisError::String)?;
            let ops = plan.root().indices::<Dfs>().collect::<Vec<_>>();
            raw::reply_with_array(ctx.ctx, ops.len() as _);
            for idx in ops {
                let node = plan.node(idx);
                let depth = node.depth();
                let str = format!("{}{}", " ".repeat(depth * 4), plan.node(idx).data());
                raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
            }
            RedisResult::Ok(RedisValue::NoReply)
        },
    )
}

fn graph_memory(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }

    let key = args.next_arg()?;
    let key = ctx.open_key(&key);

    let g = key
        .get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?
        .expect("Graph does not exist");

    Ok(RedisValue::Integer(
        g.read().unwrap().graph.read().borrow().memory_usage() as i64,
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn graph_config(
    _ctx: &Context,
    _args: Vec<RedisString>,
) -> RedisResult {
    Ok(RedisValue::Integer(0))
}

fn graph_init(
    ctx: &Context,
    _: &Vec<RedisString>,
) -> Status {
    #[cfg(feature = "pyro")]
    {
        let agent = PyroscopeAgent::builder("http://localhost:4040", "falkordb")
            .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
            .build()
            .unwrap();
        let agent_running = agent.start().unwrap();
        mem::forget(agent_running);
    }
    unsafe {
        let result = RediSearch_Init(ctx.ctx.cast(), REDISEARCH_INIT_LIBRARY as c_int);
        if result == REDISMODULE_OK as c_int {
            ctx.log_notice("RediSearch initialized successfully.");
        } else {
            ctx.log_notice("Failed initializing RediSearch.");
            return Status::Err;
        }
        init(
            RedisModule_Alloc,
            RedisModule_Calloc,
            RedisModule_Realloc,
            RedisModule_Free,
        );
        let res = RedisModule_SubscribeToServerEvent.unwrap()(
            ctx.ctx,
            RedisModuleEvent_FlushDB,
            Some(on_flush),
        );
        debug_assert_eq!(res, REDISMODULE_OK as c_int);
    }
    match init_functions() {
        Ok(()) => Status::Ok,
        Err(_) => Status::Err,
    }
}

#[allow(non_upper_case_globals)]
static RedisModuleEvent_FlushDB: RedisModuleEvent = RedisModuleEvent { id: 2, dataver: 1 };

lazy_static! {
    static ref CONFIGURATION_IMPORT_FOLDER: RedisGILGuard<String> =
        RedisGILGuard::new("/var/lib/FalkorDB/import/".into());
    static ref CONFIGURATION_CACHE_SIZE: RedisGILGuard<i64> = RedisGILGuard::new(25.into());
}

const unsafe extern "C" fn on_flush(
    _ctx: *mut RedisModuleCtx,
    _eid: RedisModuleEvent,
    _subevent: u64,
    _data: *mut c_void,
) {
}

//////////////////////////////////////////////////////

redis_module! {
    name: "falkordb",
    version: 1,
    allocator: (ThreadCountingAllocator, ThreadCountingAllocator),
    data_types: [GRAPH_TYPE],
    init: graph_init,
    commands: [
        ["graph.DELETE", graph_delete, "write deny-script", 1, 1, 1, ""],
        ["graph.QUERY", graph_query, "write deny-oom deny-script blocking", 1, 1, 1, ""],
        ["graph.RO_QUERY", graph_ro_query, "readonly deny-script blocking", 1, 1, 1, ""],
        ["graph.EXPLAIN", graph_explain, "write deny-oom deny-script", 1, 1, 1, ""],
        ["graph.LIST", graph_list, "readonly deny-script allow-busy", 0, 0, 0, ""],
        ["graph.RECORD", graph_record, "write deny-oom deny-script blocking", 1, 1, 1, ""],
        ["graph.MEMORY", graph_memory, "readonly deny-script", 1, 1, 1, ""],
        ["graph.CONFIG", graph_config, "readonly deny-script allow-busy", 0, 0, 0, ""],
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
