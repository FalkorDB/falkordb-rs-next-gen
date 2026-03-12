//! Batch-mode index scan operator — retrieves nodes using a secondary index.
//!
//! For each active row in the input batch, evaluates the index query
//! parameters and collects matching nodes.

use std::collections::VecDeque;
use std::sync::Arc;

use crate::graph::graph::NodeId;
use crate::index::indexer::IndexQuery;
use crate::parser::ast::{QueryExpr, QueryNode, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct NodeByIndexScanOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    node_pattern: &'a QueryNode<Arc<String>, Variable>,
    index: &'a Arc<String>,
    query: &'a IndexQuery<QueryExpr<Variable>>,
    pending: VecDeque<(Env<'a>, Box<dyn Iterator<Item = NodeId>>)>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> NodeByIndexScanOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        node_pattern: &'a QueryNode<Arc<String>, Variable>,
        index: &'a Arc<String>,
        query: &'a IndexQuery<QueryExpr<Variable>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            node_pattern,
            index,
            query,
            pending: VecDeque::new(),
            idx,
        }
    }

    fn evaluate_index_query(
        runtime: &Runtime,
        query: &IndexQuery<QueryExpr<Variable>>,
        vars: &Env<'_>,
    ) -> Result<IndexQuery<Value>, String> {
        match query {
            IndexQuery::Equal(key, value) => {
                let value = runtime.run_expr(value, value.root().idx(), vars, None)?;
                Ok(IndexQuery::Equal(key.clone(), value))
            }
            IndexQuery::Range(key, min, max) => {
                let (min, max) = match (min, max) {
                    (Some(min), Some(max)) => {
                        let min = runtime.run_expr(min, min.root().idx(), vars, None)?;
                        let max = runtime.run_expr(max, max.root().idx(), vars, None)?;
                        (Some(min), Some(max))
                    }
                    (Some(min), None) => {
                        let min = runtime.run_expr(min, min.root().idx(), vars, None)?;
                        (Some(min), None)
                    }
                    (None, Some(max)) => {
                        let max = runtime.run_expr(max, max.root().idx(), vars, None)?;
                        (None, Some(max))
                    }
                    (None, None) => (None, None),
                };
                Ok(IndexQuery::Range(key.clone(), min, max))
            }
            IndexQuery::Point { key, point, radius } => {
                let point = runtime.run_expr(point, point.root().idx(), vars, None)?;
                let radius = runtime.run_expr(radius, radius.root().idx(), vars, None)?;
                Ok(IndexQuery::Point {
                    key: key.clone(),
                    point,
                    radius,
                })
            }
            _ => todo!(),
        }
    }

    /// Drains rows from `self.pending` into `envs` until `BATCH_SIZE` is reached
    /// or all pending scans are exhausted.
    fn drain_pending(
        &mut self,
        envs: &mut Vec<Env<'a>>,
    ) {
        while envs.len() < BATCH_SIZE {
            let Some((env, iter)) = self.pending.front_mut() else {
                break;
            };
            if let Some(nid) = iter.next() {
                let mut row = env.clone_pooled(self.runtime.env_pool);
                row.insert(&self.node_pattern.alias, Value::Node(nid));
                envs.push(row);
            } else {
                self.pending.pop_front();
            }
        }
    }
}

impl<'a> Iterator for NodeByIndexScanOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover scans from previous call.
        self.drain_pending(&mut envs);

        while envs.len() < BATCH_SIZE {
            let batch = match self.child.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            };

            for vars in batch.active_env_iter() {
                let q = match Self::evaluate_index_query(self.runtime, self.query, vars) {
                    Ok(q) => q,
                    Err(e) => return Some(Err(e)),
                };

                let iter = Box::new(self.runtime.g.borrow().get_indexed_nodes(self.index, q));
                self.pending
                    .push_back((vars.clone_pooled(self.runtime.env_pool), iter));
            }

            self.drain_pending(&mut envs);
        }

        if envs.is_empty() {
            None
        } else {
            Some(Ok(Batch::from_envs(envs)))
        }
    }
}
