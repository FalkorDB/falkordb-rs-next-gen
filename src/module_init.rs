//! Redis module initialization and startup wiring.
//!
//! Handles RediSearch bootstrap, GraphBLAS allocator setup, Redis event
//! subscription, and function registry initialization.
//!
//! ## Startup sequence
//! ```text
//! Redis loads module
//!      |
//!      v
//! graph_init()
//!   1) (optional) start profiler backend
//!   2) initialize RediSearch API bindings
//!   3) install Redis allocators into GraphBLAS layer
//!   4) subscribe to FlushDB event hook
//!   5) register built-in runtime functions
//!      |
//!      v
//! module ready to accept GRAPH.* commands
//! ```
//!
//! Any hard failure during critical init steps returns `Status::Err` so Redis
//! can reject loading an incomplete module.

use graph::{
    graph::graphblas::matrix::init,
    index::redisearch::{REDISEARCH_INIT_LIBRARY, RediSearch_Init},
    runtime::{
        functions::init_functions,
        object_pool::{get_object_pool, init_object_pool},
    },
};
#[cfg(feature = "pyro")]
use pyroscope::PyroscopeAgent;
#[cfg(feature = "pyro")]
use pyroscope_pprofrs::{PprofConfig, pprof_backend};
use redis_module::{
    Context, REDISMODULE_OK, RedisModule_Alloc, RedisModule_Calloc, RedisModule_Free,
    RedisModule_Realloc, RedisModule_SubscribeToServerEvent, RedisModuleCtx, RedisModuleEvent,
    Status,
};
#[cfg(feature = "pyro")]
use std::mem;
use std::{os::raw::c_int, os::raw::c_void};

/// Redis event ID for FlushDB event (database flush/clear).
#[allow(non_upper_case_globals)]
static RedisModuleEvent_FlushDB: RedisModuleEvent = RedisModuleEvent { id: 2, dataver: 1 };

pub fn graph_init(
    ctx: &Context,
    _: &Vec<redis_module::RedisString>,
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
    init_object_pool();
    match init_functions() {
        Ok(()) => Status::Ok,
        Err(_) => Status::Err,
    }
}

unsafe extern "C" fn on_flush(
    _ctx: *mut RedisModuleCtx,
    _eid: RedisModuleEvent,
    _subevent: u64,
    _data: *mut c_void,
) {
    get_object_pool().clear();
}
