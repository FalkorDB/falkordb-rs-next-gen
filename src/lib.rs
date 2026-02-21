//! # FalkorDB Redis Module
//!
//! This crate implements a Redis module that provides graph database functionality,
//! exposing FalkorDB's graph operations as Redis commands. It serves as the entry
//! point for the FalkorDB graph database when running as a Redis module.
//!
//! ## Architecture Overview
//!
//! The module integrates with Redis through the `redis-module` crate, registering
//! custom commands and a native data type (`graphdata`) for storing graph instances.
//!
//! ```text
//! Redis Client
//!     │
//!     ▼
//! ┌─────────────────────────────────────────┐
//! │  Redis Module Interface (this crate)    │
//! │  - Command handlers (GRAPH.QUERY, etc.) │
//! │  - Response serialization               │
//! │  - Thread-safe graph access             │
//! └─────────────────────────────────────────┘
//!     │
//!     ▼
//! ┌─────────────────────────────────────────┐
//! │  Graph Crate (graph/)                   │
//! │  - Cypher parsing & planning            │
//! │  - Query execution runtime              │
//! │  - Graph data structures                │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Supported Commands
//!
//! - `GRAPH.QUERY` - Execute read/write Cypher queries
//! - `GRAPH.RO_QUERY` - Execute read-only Cypher queries
//! - `GRAPH.DELETE` - Delete a graph
//! - `GRAPH.EXPLAIN` - Show query execution plan without executing
//! - `GRAPH.LIST` - List all graphs in the database
//! - `GRAPH.MEMORY` - Get memory usage of a graph
//! - `GRAPH.CONFIG` - Configuration operations
//!
//! ## Threading Model
//!
//! Queries are executed on a thread pool to avoid blocking Redis. Write queries
//! are serialized through a channel to ensure consistency, while read queries
//! can execute concurrently using MVCC (Multi-Version Concurrency Control).
//!
//! ## Configuration
//!
//! - `CACHE_SIZE` - Query plan cache size (default: 25, range: 0-1000)
//! - `IMPORT_FOLDER` - Path for CSV imports (default: `/var/lib/FalkorDB/import/`)

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

/// Error returned when attempting graph operations on a non-existent key.
const EMPTY_KEY_ERR: RedisResult = Err(RedisError::Str("ERR Invalid graph operation on empty key"));

/// Redis native type definition for graph data.
///
/// This registers "graphdata" as a custom Redis type with callbacks for:
/// - RDB persistence (load/save) - currently minimal implementation
/// - Memory cleanup (free) - properly deallocates the graph when key is deleted
///
/// The type is used with `RedisKey::get_value` and `RedisKey::set_value` to
/// store and retrieve `Arc<RwLock<ThreadedGraph>>` instances.
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

/// Thread-safe wrapper around the MVCC graph with write query serialization.
///
/// This struct provides concurrent read access and serialized write access to
/// the underlying graph. It implements a producer-consumer pattern for write
/// queries to ensure consistency.
///
/// # Threading Model
///
/// - **Read queries**: Execute concurrently on the thread pool using MVCC snapshots
/// - **Write queries**: Queued via `sender` channel and processed serially by
///   `process_write_queued_query` to maintain transaction ordering
///
/// # Fields
///
/// - `graph`: The MVCC-enabled graph supporting concurrent reads
/// - `sender`/`receiver`: Channel for queuing write queries
/// - `write_loop`: Atomic flag ensuring only one thread processes writes at a time
struct ThreadedGraph {
    graph: MvccGraph,
    sender: Sender<(BlockedClient, Arc<String>, bool)>,
    receiver: Receiver<(BlockedClient, Arc<String>, bool)>,
    write_loop: AtomicBool,
}

// SAFETY: ThreadedGraph is safe to Send/Sync because:
// - MvccGraph uses internal synchronization (RwLock) for safe concurrent access
// - The sender/receiver channel types are already Send/Sync by design
// - AtomicBool provides atomic synchronization for the write_loop flag
// This allows the graph to be shared across threads safely, with the MVCC layer
// handling concurrent read/write access, and the channel serializing write operations.
unsafe impl Send for ThreadedGraph {}
unsafe impl Sync for ThreadedGraph {}

impl ThreadedGraph {
    /// Creates a new `ThreadedGraph` with default capacity.
    ///
    /// Initializes the MVCC graph with:
    /// - 16384 initial node capacity
    /// - 16384 initial edge capacity
    /// - 1024 relationship type capacity
    pub fn new(
        cache_size: usize,
        name: &str,
    ) -> Self {
        let (sender, receiver) = channel();
        Self {
            graph: MvccGraph::new(16384, 16384, cache_size, name),
            sender,
            receiver,
            write_loop: AtomicBool::new(false),
        }
    }

    /// Executes a query, routing to read or write path based on query type.
    ///
    /// # Arguments
    /// * `ctx` - Redis context for sending responses
    /// * `query` - Cypher query string
    /// * `compact` - If true, use compact Redis protocol format
    /// * `write` - If true, write operations are allowed
    ///
    /// # Returns
    /// * `Ok(true)` - Query contains write operations (needs write path)
    /// * `Ok(false)` - Query executed successfully as read-only
    /// * `Err` - Query parsing or execution failed
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

    /// Executes a write query with exclusive graph access.
    ///
    /// This method acquires write lock on the graph and executes queries that
    /// modify graph state. Called by `process_write_queued_query` to process
    /// serialized write operations.
    ///
    /// # Returns
    /// The graph reference for commit/rollback by the caller
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
        // RwLock::write().unwrap() - Intentional: Lock poisoning indicates a panic during
        // a write operation, which means the graph is in an inconsistent state.
        // Propagating the panic is correct here to prevent using corrupted data.
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

/// RDB load callback for graph persistence (currently no-op).
///
/// TODO: Implement graph deserialization from RDB format.
#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_load(
    _: *mut RedisModuleIO,
    _: i32,
) -> *mut c_void {
    null_mut()
}

/// RDB save callback for graph persistence (currently no-op).
///
/// TODO: Implement graph serialization to RDB format.
#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_save(
    _: *mut RedisModuleIO,
    _: *mut c_void,
) {
}

/// Frees graph memory when Redis key is deleted or expires.
///
/// Called by Redis when the key holding the graph is removed. Properly
/// deallocates the `Arc<RwLock<ThreadedGraph>>` to release all graph resources.
#[unsafe(no_mangle)]
unsafe extern "C" fn graph_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<Arc<RwLock<ThreadedGraph>>>()));
    }
}

/// Serializes a runtime value to Redis protocol in compact format.
///
/// Compact format prefixes each value with a type code (integer) to enable
/// efficient client-side parsing without string parsing. Type codes:
/// - 1: Null, 2: String, 3: Int, 4: Bool, 5: Float
/// - 6: List, 7: Relationship, 8: Node, 9: Path, 10: Map
/// - 11: Point, 12: VecF32, 13: Datetime, 14: Date, 15: Time, 16: Duration
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
                    _ => {
                        // Path should only contain nodes and relationships, but handle gracefully
                        ctx.log(
                            redis_module::logging::RedisLogLevel::Warning,
                            &format!("Unexpected value type in path: {:?}", node.type_of()),
                        );
                    }
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
                    _ => {
                        // Path should only contain nodes and relationships, but handle gracefully
                        ctx.log(
                            redis_module::logging::RedisLogLevel::Warning,
                            &format!("Unexpected value type in path: {:?}", node.type_of()),
                        );
                    }
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
                    _ => {
                        // Path should only contain nodes and relationships, but handle gracefully
                        ctx.log(
                            redis_module::logging::RedisLogLevel::Warning,
                            &format!("Unexpected value type in path: {:?}", node.type_of()),
                        );
                    }
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

/// Serializes a runtime value to Redis protocol in verbose (human-readable) format.
///
/// Unlike compact format, verbose format uses human-readable representations:
/// - Nodes show labels as strings, not IDs
/// - Relationships show type names, not type IDs
/// - Dates/times show formatted strings, not timestamps
///
/// Used by default when `--compact` flag is not specified.
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
                    _ => {
                        // Path should only contain nodes and relationships, but handle gracefully
                        ctx.log(
                            redis_module::logging::RedisLogLevel::Warning,
                            &format!("Unexpected value type in path: {:?}", node.type_of()),
                        );
                    }
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

/// Handle to a Redis blocked client for asynchronous command responses.
///
/// When a query is executed on the thread pool, we block the Redis client
/// and store this handle. When the query completes, we use the handle to
/// send the response back to the waiting client.
///
/// # Safety
/// Implements `Send` and `Sync` because the raw pointer is only accessed
/// through thread-safe Redis module APIs. The `Drop` impl ensures the client
/// is unblocked even if the query panics.
pub struct BlockedClient {
    pub inner: *mut raw::RedisModuleBlockedClient,
}

// SAFETY: BlockedClient is safe to Send/Sync because:
// - The raw pointer is only accessed through thread-safe Redis module APIs
// - Redis module blocked client handles are designed to be passed between threads
// - The Drop impl uses RedisModule_UnblockClient which is thread-safe
// - We never dereference the pointer directly, only pass it to Redis APIs
// The Redis module framework guarantees thread-safety for blocked client operations.
unsafe impl Send for BlockedClient {}
unsafe impl Sync for BlockedClient {}

impl Drop for BlockedClient {
    fn drop(&mut self) {
        unsafe { raw::RedisModule_UnblockClient.unwrap()(self.inner, null_mut()) };
    }
}

/// Executes a query on the thread pool with non-blocking Redis response.
///
/// This is the main query execution entry point. It:
/// 1. Blocks the Redis client to free the main thread
/// 2. Spawns query execution on the thread pool
/// 3. Routes to read or write path based on query analysis
/// 4. Sends response via the blocked client handle
///
/// # Arguments
/// * `ctx` - Redis context
/// * `graph` - The graph to query
/// * `query` - Cypher query string
/// * `compact` - Use compact response format
/// * `write` - Allow write operations
/// * `track_mem` - Log memory allocation statistics
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
            // RwLock::read().unwrap() - Intentional: Lock poisoning indicates a panic during
            // a write operation. Since reads need consistent data, propagating the panic
            // prevents returning corrupted results to clients.
            let graph = graph.read().unwrap();
            let bc = bc;
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);

            let res = graph.execute_query(&ctx, &query, compact, write);
            match res {
                Ok(is_write) => {
                    if is_write {
                        if let Err(e) = graph.sender.send((bc, query, compact)) {
                            // Write receiver has been dropped - graph is shutting down
                            ctx.log(
                                redis_module::logging::RedisLogLevel::Warning,
                                &format!("Failed to queue write query: {:?}", e),
                            );
                            let err_msg = CString::new("ERR Graph write queue unavailable")
                                .expect("hardcoded string is valid");
                            raw::reply_with_error(ctx.ctx, err_msg.as_ptr().cast::<c_char>());
                            drop(bc);
                            unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                        } else {
                            drop(graph);
                            process_write_queued_query(&g);
                        }
                    } else {
                        drop(bc);
                        unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    }
                }
                Err(err) => {
                    let cerr = CString::new(err).unwrap_or_else(|_| {
                        CString::new("ERR Query execution failed (error message contains null byte)")
                            .expect("hardcoded string is valid")
                    });
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

/// Processes queued write queries in serial order.
///
/// This function implements the write serialization pattern:
/// 1. Acquires write lock on the graph
/// 2. Uses atomic compare-exchange to ensure only one thread processes writes
/// 3. Drains the write queue, executing each query with exclusive access
/// 4. Commits or rolls back each transaction based on success/failure
///
/// # Good Practice
/// Uses compare_exchange with Acquire/Relaxed ordering for the write_loop flag.
/// This ensures proper synchronization: Acquire prevents reordering of subsequent
/// reads, while Relaxed is sufficient for the failure case since we just retry.
///
/// RwLock unwraps are intentional: lock poisoning from a panic would indicate
/// the graph is in an inconsistent state, so propagating the panic is correct.
fn process_write_queued_query(graph: &Arc<RwLock<ThreadedGraph>>) {
    let g = graph.read().unwrap();
    if g.write_loop
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
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
                    let cerr = CString::new(err).unwrap_or_else(|_| {
                        CString::new("ERR Query execution failed (error message contains null byte)")
                            .expect("hardcoded string is valid")
                    });
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

/// Sends query execution statistics as part of the response.
///
/// Statistics include counts of modifications (nodes/edges created/deleted,
/// properties set, labels added, indices created) plus execution metadata
/// (cached execution flag, execution time, graph version).
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

/// Handles `GRAPH.QUERY` command - executes read/write Cypher queries.
///
/// This is the primary command for graph operations. It:
/// - Creates the graph if it doesn't exist (implicit graph creation)
/// - Parses and executes the Cypher query
/// - Supports `--compact` flag for compact response format
/// - Supports `--track-memory` flag for allocation logging
///
/// # Arguments (Redis command)
/// - `key` - The Redis key name for the graph
/// - `query` - Cypher query string
/// - `--compact` (optional) - Use compact response format
/// - `--track-memory` (optional) - Log memory allocation stats
///
/// # Fuzz Testing
/// When compiled with `fuzz` feature, queries are saved to corpus files
/// for fuzz testing coverage.
#[allow(static_mut_refs)]
fn graph_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_str = args.next_arg()?;
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

    let key = ctx.open_key_writable(&key_str);

    if let Some(graph) = key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)? {
        query_mut(ctx, graph, query, compact, true, track_memory);
    } else {
        let graph = Arc::new(RwLock::new(ThreadedGraph::new(
            *CONFIGURATION_CACHE_SIZE.lock(ctx) as usize,
            &key_str.to_string(),
        )));
        query_mut(ctx, &graph, query, compact, true, track_memory);
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

/// Records execution trace for debugging/testing purposes.
///
/// Executes the query in recording mode where intermediate results at each
/// plan node are captured. Returns the execution trace along with the plan
/// structure for debugging query execution flow.
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

/// Handles `GRAPH.RECORD` command for execution tracing.
///
/// Similar to GRAPH.QUERY but returns detailed execution trace data
/// instead of query results. Used for debugging and testing.
fn graph_record(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_str = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key_writable(&key_str);

    if let Some(graph) = key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)? {
        record_mut(ctx, graph, query)?;
    } else {
        let graph = Arc::new(RwLock::new(ThreadedGraph::new(
            *CONFIGURATION_CACHE_SIZE.lock(ctx) as usize,
            &key_str.to_string(),
        )));
        record_mut(ctx, &graph, query)?;
        key.set_value(&GRAPH_TYPE, graph)?;
    }

    RedisResult::Ok(RedisValue::NoReply)
}

/// Formats query result in verbose (human-readable) format.
///
/// Response structure:
/// 1. Column headers (array of [type_code, name] pairs)
/// 2. Result rows (array of arrays)
/// 3. Statistics (labels/nodes/edges created, execution time, etc.)
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

/// Formats query result in compact (machine-optimized) format.
///
/// Similar to `reply_verbose` but values are prefixed with type codes
/// for efficient client-side parsing without string inspection.
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

/// Handles `GRAPH.EXPLAIN` command - shows query execution plan without executing.
///
/// Parses the query and returns the execution plan as a list of operations
/// with indentation showing the tree structure. Useful for query optimization
/// and understanding how queries are executed.
///
/// See: <https://docs.falkordb.com/commands/graph.explain.html>
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

/// Handles `GRAPH.MEMORY` command - returns memory usage of a graph in bytes.
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

/// Placeholder for `GRAPH.CONFIG` command - currently returns 0.
///
/// TODO: Implement configuration get/set operations.
#[allow(clippy::unnecessary_wraps)]
fn graph_config(
    _ctx: &Context,
    _args: Vec<RedisString>,
) -> RedisResult {
    Ok(RedisValue::Integer(0))
}

/// Module initialization callback - called when Redis loads the module.
///
/// Performs critical initialization:
/// 1. Optionally starts Pyroscope profiling agent (with `pyro` feature)
/// 2. Initializes RediSearch for full-text indexing support
/// 3. Initializes GraphBLAS with Redis memory allocators
/// 4. Subscribes to Redis FlushDB events
/// 5. Initializes built-in Cypher functions
///
/// # Good Practice
/// Uses Redis's memory allocators for GraphBLAS to ensure all memory
/// is tracked by Redis and properly accounted in memory limits.
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

/// Redis event ID for FlushDB event (database flush/clear).
#[allow(non_upper_case_globals)]
static RedisModuleEvent_FlushDB: RedisModuleEvent = RedisModuleEvent { id: 2, dataver: 1 };

/// Global configuration values protected by Redis GIL.
///
/// These values can be set via `CONFIG SET` and are thread-safe through
/// `RedisGILGuard` which requires holding the Redis Global Interpreter Lock.
lazy_static! {
    /// Path to directory where CSV files can be imported from.
    static ref CONFIGURATION_IMPORT_FOLDER: RedisGILGuard<String> =
        RedisGILGuard::new("/var/lib/FalkorDB/import/".into());
    /// Maximum number of query plans to cache per graph.
    static ref CONFIGURATION_CACHE_SIZE: RedisGILGuard<i64> = RedisGILGuard::new(25.into());
}

/// Callback for Redis FlushDB event - currently no-op.
///
/// TODO: May need to handle graph cleanup on database flush.
const unsafe extern "C" fn on_flush(
    _ctx: *mut RedisModuleCtx,
    _eid: RedisModuleEvent,
    _subevent: u64,
    _data: *mut c_void,
) {
}

//////////////////////////////////////////////////////

/// Redis module registration macro.
///
/// This macro generates the boilerplate code required by Redis to load the module:
/// - Module name: "falkordb"
/// - Uses custom `ThreadCountingAllocator` for memory tracking
/// - Registers `GRAPH_TYPE` as a native Redis data type
/// - Registers all graph commands with their access flags:
///   - `write` - Command may modify data
///   - `readonly` - Command only reads data
///   - `deny-oom` - Reject if Redis is out of memory
///   - `deny-script` - Cannot be called from Lua scripts
///   - `blocking` - Command blocks the client (async execution)
///   - `allow-busy` - Can run even when Redis is busy
/// - Exposes configuration options via Redis CONFIG command
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
