//! Batch-mode fulltext scan operator — retrieves nodes via a fulltext index query.
//!
//! Implements `CALL db.idx.fulltext.queryNodes(label, query)`. For each
//! active row in each input batch, evaluates the label and query expressions,
//! delegates to the graph's fulltext index, and expands matching nodes into
//! output rows accumulated into batches of up to `BATCH_SIZE`.

use std::collections::VecDeque;

use crate::graph::graph::NodeId;
use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct NodeByFulltextScanOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    pending: VecDeque<(Env<'a>, Box<dyn Iterator<Item = (NodeId, f64)>>)>,
    node: &'a Variable,
    label: &'a QueryExpr<Variable>,
    query: &'a QueryExpr<Variable>,
    score: &'a Option<Variable>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> NodeByFulltextScanOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        node: &'a Variable,
        label: &'a QueryExpr<Variable>,
        query: &'a QueryExpr<Variable>,
        score: &'a Option<Variable>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            pending: VecDeque::new(),
            node,
            label,
            query,
            score,
            idx,
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
            if let Some((node_id, s)) = iter.next() {
                let mut row = env.clone_pooled(self.runtime.env_pool);
                row.insert(self.node, Value::Node(node_id));
                if let Some(score) = self.score {
                    row.insert(score, Value::Float(s));
                }
                envs.push(row);
            } else {
                self.pending.pop_front();
            }
        }
    }
}

impl<'a> Iterator for NodeByFulltextScanOp<'a> {
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
                let label_str =
                    match self
                        .runtime
                        .run_expr(self.label, self.label.root().idx(), vars, None)
                    {
                        Ok(Value::String(s) | Value::InternedString(s)) => s,
                        Ok(_) => {
                            return Some(Err("fulltext query expects a string label".into()));
                        }
                        Err(e) => return Some(Err(e)),
                    };
                let query_str =
                    match self
                        .runtime
                        .run_expr(self.query, self.query.root().idx(), vars, None)
                    {
                        Ok(Value::String(s) | Value::InternedString(s)) => s,
                        Ok(_) => {
                            return Some(Err("fulltext query expects a string query".into()));
                        }
                        Err(e) => return Some(Err(e)),
                    };
                let iter = match self
                    .runtime
                    .g
                    .borrow()
                    .fulltext_query_nodes(&label_str, &query_str)
                {
                    Ok(iter) => Box::new(iter),
                    Err(e) => return Some(Err(e)),
                };

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
