//! Core graph execution and concurrency primitives.
//!
//! This module owns the execution model used by Redis command handlers.
//!
//! ## Concurrency model
//! ```text
//! Client query
//!    |
//!    v
//! query_mut -> threadpool worker
//!    |
//!    +--> execute_query() detects write IR?
//!            |
//!            +-- no --> run on MVCC read snapshot (parallel reads)
//!            |
//!            +-- yes -> enqueue blocked client + query
//!                        |
//!                        v
//!                  process_write_queued_query()
//!                        |
//!                        +--> single writer loop
//!                              +--> execute_query_write()
//!                              +--> commit() on success / rollback() on error
//! ```
//!
//! The key design goal is predictable write ordering with high read throughput.
//! Reads are lock-light and concurrent; writes are serialized through an explicit
//! queue guarded by `write_loop`.

use crate::{
    config::{CONFIGURATION_IMPORT_FOLDER, MAX_QUEUED_QUERIES, RESULTSET_SIZE, TIMEOUT_DEFAULT},
    reply::{reply_compact, reply_verbose},
};
use atomic_refcell::AtomicRefCell;
use crossfire::{
    Rx, Tx,
    spsc::{Array, bounded_blocking},
};
use graph::{
    graph::{
        graph::{Graph, Plan},
        mvcc_graph::MvccGraph,
    },
    planner::IR,
    runtime::{eval::evaluate_param, pool::Pool, runtime::Runtime},
    threadpool::{pending_count, spawn},
};
use orx_tree::Collection;
use parking_lot::RwLock;
use redis_module::{Context, ContextFlags, RedisResult, RedisValue, raw};
use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::c_void,
    ptr::null_mut,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::allocator::{current_thread_usage, disable_tracking, enable_tracking, reset_counter};

type WriteMessage = (BlockedClient, Arc<String>, bool, bool, Arc<String>);

pub struct ThreadedGraph {
    pub graph: MvccGraph,
    pub sender: Tx<Array<WriteMessage>>,
    pub receiver: Rx<Array<WriteMessage>>,
    pub write_loop: AtomicBool,
}

unsafe impl Send for ThreadedGraph {}
unsafe impl Sync for ThreadedGraph {}

impl ThreadedGraph {
    pub fn new(
        cache_size: usize,
        name: &str,
    ) -> Self {
        let (sender, receiver) = bounded_blocking(1024);
        Self {
            graph: MvccGraph::new(16384, 16384, cache_size, name),
            sender,
            receiver,
            write_loop: AtomicBool::new(false),
        }
    }

    pub fn execute_query(
        &self,
        ctx: &Context,
        query: &str,
        compact: bool,
        write: bool,
    ) -> Result<(bool, bool), String> {
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
        let is_write = plan.iter().any(|n| {
            matches!(
                n,
                IR::Commit | IR::CreateIndex { .. } | IR::DropIndex { .. }
            )
        });
        let g = if is_write {
            if !write {
                return Err(String::from(
                    "graph.RO_QUERY is to be executed only on read-only queries",
                ));
            }
            return Ok((is_write, cached));
        } else {
            self.graph.read()
        };
        let env_pool = Pool::new();
        let runtime = Runtime::new(
            g,
            parameters,
            is_write,
            plan,
            false,
            (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
            &env_pool,
            RESULTSET_SIZE.load(Ordering::Relaxed),
        );
        let mut result = runtime.query()?;
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, &runtime, &result);
        } else {
            reply_verbose(ctx, &runtime, &result);
        }
        drop(result);
        drop(runtime);
        Ok((is_write, cached))
    }

    pub fn execute_query_write(
        &self,
        ctx: &Context,
        query: &str,
        compact: bool,
        first_cached: bool,
    ) -> Result<Arc<AtomicRefCell<Graph>>, String> {
        let Plan {
            plan, parameters, ..
        } = self.graph.read().borrow().get_plan(query)?;
        let cached = first_cached;
        let parameters = parameters
            .into_iter()
            .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        debug_assert!(plan.iter().any(|n| matches!(
            n,
            IR::Commit | IR::CreateIndex { .. } | IR::DropIndex { .. }
        )));
        let g = self.graph.write().unwrap();
        let env_pool = Pool::new();
        let runtime = Runtime::new(
            g.clone(),
            parameters,
            true,
            plan,
            false,
            (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
            &env_pool,
            RESULTSET_SIZE.load(Ordering::Relaxed),
        );
        let mut result = match runtime.query() {
            Ok(r) => r,
            Err(err) => {
                // Clean up dirty cache entries before the graph is dropped.
                g.borrow_mut().rollback_cache();
                return Err(err);
            }
        };
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, &runtime, &result);
        } else {
            reply_verbose(ctx, &runtime, &result);
        }
        Ok(g)
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
pub fn query_mut(
    ctx: &Context,
    graph: &Arc<RwLock<ThreadedGraph>>,
    query: &str,
    compact: bool,
    write: bool,
    track_mem: bool,
    key_name: Arc<String>,
) -> RedisResult {
    // Inside MULTI/EXEC: execute synchronously (blocking commands not allowed).
    if ctx.get_flags().contains(ContextFlags::MULTI) {
        return query_sync(ctx, graph, query, compact, write);
    }

    // Check pending queries limit before dispatching.
    let max = MAX_QUEUED_QUERIES.load(Ordering::Relaxed) as usize;
    if pending_count() >= max {
        let bc = BlockedClient {
            inner: unsafe { raw::RedisModule_BlockClient.unwrap()(ctx.ctx, None, None, None, 0) },
        };
        let err_ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
        let err_ctx = Context::new(err_ctx);
        let cerr = CString::new("Max pending queries exceeded").unwrap();
        raw::reply_with_error(err_ctx.ctx, cerr.as_ptr());
        drop(bc);
        unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(err_ctx.ctx) };
        return Ok(RedisValue::NoReply);
    }

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
            // Sync query timeout to UDF JS runtime
            graph::udf::js_context::JS_TIMEOUT_MS
                .store(TIMEOUT_DEFAULT.load(Ordering::Relaxed), Ordering::Relaxed);

            let g = graph.clone();
            let binding = graph.clone();
            let graph = binding.read();
            let bc = bc;
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);

            let res = graph.execute_query(&ctx, &query, compact, write);

            // Log memory tracking BEFORE freeing the context.
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

            match res {
                Ok((is_write, cached)) => {
                    if is_write {
                        graph
                            .sender
                            .send((bc, query, compact, cached, key_name))
                            .unwrap();
                        drop(graph);
                        process_write_queued_query(&g);
                    } else {
                        drop(bc);
                        unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    }
                }
                Err(err) => {
                    let cerr = CString::new(err).unwrap();
                    raw::reply_with_error(ctx.ctx, cerr.as_ptr());
                    drop(bc);
                    unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                }
            }
        },
        None,
    );
    Ok(RedisValue::NoReply)
}

/// Execute a query synchronously on the calling thread.
/// Used when inside MULTI/EXEC where blocking is not allowed.
fn query_sync(
    ctx: &Context,
    graph: &Arc<RwLock<ThreadedGraph>>,
    query: &str,
    compact: bool,
    write: bool,
) -> RedisResult {
    // First pass: parse + detect if write, execute reads inline.
    // Sync query timeout to UDF JS runtime
    graph::udf::js_context::JS_TIMEOUT_MS
        .store(TIMEOUT_DEFAULT.load(Ordering::Relaxed), Ordering::Relaxed);

    let res = {
        let g = graph.read();
        g.execute_query(ctx, query, compact, write)
    };
    match res {
        Ok((is_write, cached)) => {
            if is_write {
                // Write path: acquire exclusive lock and execute.
                let mut g = graph.write();
                let res = g.execute_query_write(ctx, query, compact, cached);
                match res {
                    Ok(new_graph) => {
                        g.graph.commit(new_graph);
                        // Flush dirty cache entries to fjall if over budget.
                        let value = g.graph.read().borrow().maybe_flush_caches();
                        if let Err(e) = value {
                            eprintln!("FalkorDB: cache flush failed: {e}");
                        }
                    }
                    Err(err) => {
                        g.graph.rollback();
                        return Err(redis_module::RedisError::String(err));
                    }
                }
                drop(g);
            }
        }
        Err(err) => {
            return Err(redis_module::RedisError::String(err));
        }
    }
    Ok(RedisValue::NoReply)
}

pub fn process_write_queued_query(graph: &Arc<RwLock<ThreadedGraph>>) {
    let g = graph.read();
    if g.write_loop
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        drop(g);
        let mut graph = graph.write();
        loop {
            while let Ok((bc, query, compact, cached, key_name)) = { graph.receiver.try_recv() } {
                execute_and_commit_write(&mut graph, bc, query, compact, cached, key_name);
            }
            // Tentatively mark the write loop as idle.
            graph.write_loop.store(false, Ordering::Release);
            // Re-check: a message may have been enqueued between the last
            // try_recv (which saw nothing) and the store(false) above.
            // Without this, the sender's CAS would fail and the message
            // would be stranded until another write arrives.
            match graph.receiver.try_recv() {
                Ok((bc, query, compact, cached, key_name)) => {
                    // Reclaim the write loop via CAS so we don't race with
                    // another thread that may have already acquired it.
                    if graph
                        .write_loop
                        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                        .is_err()
                    {
                        // Another thread won the CAS — it will process this
                        // message (it's back in the channel... except we
                        // consumed it). We must still handle it ourselves
                        // since try_recv already dequeued it.
                        execute_and_commit_write(&mut graph, bc, query, compact, cached, key_name);
                        break;
                    }
                    execute_and_commit_write(&mut graph, bc, query, compact, cached, key_name);
                    // Continue the loop to drain any further messages.
                }
                Err(_) => break,
            }
        }
    }
}

/// Process a single dequeued write message: execute the query, commit or
/// rollback, signal the key as modified, and free the thread-safe context.
fn execute_and_commit_write(
    graph: &mut ThreadedGraph,
    bc: BlockedClient,
    query: Arc<String>,
    compact: bool,
    cached: bool,
    key_name: Arc<String>,
) {
    let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
    let ctx = Context::new(ctx);
    let res = graph.execute_query_write(&ctx, &query, compact, cached);
    match res {
        Ok(g) => {
            // Signal the key as modified so WATCH gets triggered.
            unsafe {
                raw::RedisModule_ThreadSafeContextLock.unwrap()(ctx.ctx);
                let rstr = raw::RedisModule_CreateString.unwrap()(
                    ctx.ctx,
                    key_name.as_ptr().cast(),
                    key_name.len(),
                );
                raw::RedisModule_SignalModifiedKey.unwrap()(ctx.ctx, rstr);
                raw::RedisModule_FreeString.unwrap()(ctx.ctx, rstr);
                raw::RedisModule_ThreadSafeContextUnlock.unwrap()(ctx.ctx);
                raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx);
            };
            drop(bc);
            graph.graph.commit(g);
            // Flush dirty cache entries to fjall if over budget.
            let value = graph.graph.read().borrow().maybe_flush_caches();
            if let Err(e) = value {
                eprintln!("FalkorDB: cache flush failed: {e}");
            }
        }
        Err(err) => {
            let cerr = CString::new(err).unwrap();
            raw::reply_with_error(ctx.ctx, cerr.as_ptr());
            drop(bc);
            unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
            graph.graph.rollback();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<Arc<RwLock<ThreadedGraph>>>()));
    }
}
