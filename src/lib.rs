//! # FalkorDB Redis Module
//!
//! This crate is the Redis-facing integration layer for FalkorDB. It registers
//! Redis commands, initializes runtime dependencies, and delegates query work to
//! the `graph` crate through focused internal modules.
//!
//! ## High-level flow
//! ```text
//! Redis command
//!     |
//!     v
//! commands/* handler  --->  graph_core::query_mut (async dispatch)
//!     |                              |
//!     |                              +--> read path (concurrent MVCC snapshots)
//!     |                              +--> write path (serialized queue)
//!     v
//! reply::* (Redis protocol serialization)
//! ```
//!
//! ## Module responsibilities
//! - `commands/`: command entrypoints and argument parsing.
//! - `graph_core`: query execution/concurrency primitives.
//! - `reply`: compact + verbose output formatting.
//! - `redis_type`: native Redis value type (`graphdata`).
//! - `module_init`: startup wiring (RediSearch, GraphBLAS, functions).
//! - `config`: runtime configuration state.

#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::non_std_lazy_statics)]

mod allocator;
mod commands;
mod config;
mod graph_core;
mod module_init;
mod redis_type;
mod reply;

use allocator::ThreadCountingAllocator;
use commands::{
    graph_config, graph_delete, graph_explain, graph_info, graph_list, graph_memory, graph_query,
    graph_record, graph_ro_query,
};
use config::{CONFIGURATION_CACHE_SIZE, CONFIGURATION_IMPORT_FOLDER};
use module_init::graph_init;
use redis_module::{configuration::ConfigurationFlags, redis_module};
use redis_type::GRAPH_TYPE;

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
        ["graph.INFO", graph_info, "readonly deny-script allow-busy", 0, 0, 0, ""],
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
