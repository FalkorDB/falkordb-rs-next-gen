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
    config::CONFIGURATION_IMPORT_FOLDER,
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
    runtime::{
        pool::Pool,
        runtime::{Runtime, evaluate_param},
    },
    threadpool::spawn,
};
use orx_tree::Collection;
use parking_lot::RwLock;
use redis_module::{Context, raw};
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

pub struct ThreadedGraph {
    pub graph: MvccGraph,
    pub sender: Tx<Array<(BlockedClient, Arc<String>, bool, bool)>>,
    pub receiver: Rx<Array<(BlockedClient, Arc<String>, bool, bool)>>,
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
            write,
            plan,
            false,
            (*CONFIGURATION_IMPORT_FOLDER.lock(ctx)).clone(),
            &env_pool,
        );
        let mut result = runtime.query()?;
        result.stats.cached = cached;
        if compact {
            reply_compact(ctx, &runtime, result);
        } else {
            reply_verbose(ctx, &runtime, result);
        }
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
            let graph = graph.read();
            let bc = bc;
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);

            let res = graph.execute_query(&ctx, &query, compact, write);
            match res {
                Ok((is_write, cached)) => {
                    if is_write {
                        graph.sender.send((bc, query, compact, cached)).unwrap();
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

pub fn process_write_queued_query(graph: &Arc<RwLock<ThreadedGraph>>) {
    let g = graph.read();
    if g.write_loop
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
        .is_ok()
    {
        drop(g);
        let mut graph = graph.write();
        while let Ok((bc, query, compact, cached)) = { graph.receiver.try_recv() } {
            let ctx = unsafe { raw::RedisModule_GetThreadSafeContext.unwrap()(bc.inner) };
            let ctx = Context::new(ctx);
            let res = graph.execute_query_write(&ctx, &query, compact, cached);
            match res {
                Ok(g) => {
                    drop(bc);
                    unsafe { raw::RedisModule_FreeThreadSafeContext.unwrap()(ctx.ctx) };
                    graph.graph.commit(g);
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
        graph.write_loop.store(false, Ordering::Release);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<Arc<RwLock<ThreadedGraph>>>()));
    }
}
