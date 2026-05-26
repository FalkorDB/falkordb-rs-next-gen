//! `GRAPH.EXPLAIN` command handler.
//!
//! Parses a query and returns the execution plan tree without running the query.
//!
//! ## Output shape
//! The response is a linearized DFS traversal of the plan tree, where each
//! operator string is left-indented by depth to preserve hierarchy.
//!
//! ```text
//! Results
//!  ├─ Project
//!  │   └─ Filter
//!  │       └─ NodeByLabelScan
//! ```

use crate::{commands::EMPTY_KEY_ERR, graph_core::ThreadedGraph, redis_type::GRAPH_TYPE};
use graph::graph::graph::Plan;
use orx_tree::{Dfs, NodeRef};
use parking_lot::RwLock;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue, raw};
use std::{os::raw::c_char, sync::Arc};

pub fn graph_explain(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key(&key);

    (key.get_value::<Arc<RwLock<ThreadedGraph>>>(&GRAPH_TYPE)?).map_or(EMPTY_KEY_ERR, |graph| {
        let graph_read = graph.read();
        let Plan { plan, .. } = graph_read
            .graph
            .read()
            .borrow()
            .get_plan(query)
            .map_err(RedisError::String)?;
        let ops = plan.root().indices::<Dfs>().collect::<Vec<_>>();
        raw::reply_with_array(ctx.ctx, ops.len() as i64 + 1);
        let root = "Results";
        raw::reply_with_string_buffer(ctx.ctx, root.as_ptr().cast::<c_char>(), root.len());
        for idx in ops {
            let node = plan.node(idx);
            let depth = node.depth();
            let str = format!("{}{}", " ".repeat((depth + 1) * 4), plan.node(idx).data());
            raw::reply_with_string_buffer(ctx.ctx, str.as_ptr().cast::<c_char>(), str.len());
        }
        RedisResult::Ok(RedisValue::NoReply)
    })
}
