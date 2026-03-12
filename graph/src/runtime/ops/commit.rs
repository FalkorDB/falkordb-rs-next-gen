//! Batch-mode commit operator — materializes pending mutations.
//!
//! This is a *blocking* operator: it drains all child batches first
//! (collecting all result environments), then calls `pending.commit()`
//! to apply batched creates, deletes, and property changes to the
//! underlying graph. After the commit succeeds, the collected
//! environments are yielded as batches.
//!
//! Only allowed in write queries; returns an error for `GRAPH.RO_QUERY`.

use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
};
use orx_tree::{Dyn, NodeIdx};

pub struct CommitOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Option<Box<BatchOp<'a>>>,
    results: Vec<Batch<'a>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> CommitOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Result<Self, String> {
        if !runtime.write {
            return Err(String::from(
                "graph.RO_QUERY is to be executed only on read-only queries",
            ));
        }
        Ok(Self {
            runtime,
            child: Some(child),
            results: Vec::new(),
            idx,
        })
    }
}

impl<'a> Iterator for CommitOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // On first call, drain the entire child and commit.
        if let Some(mut child) = self.child.take() {
            loop {
                match child.next() {
                    Some(Ok(batch)) => {
                        self.results.push(batch);
                    }
                    Some(Err(e)) => return Some(Err(e)),
                    None => break,
                }
            }
            if let Err(e) = self
                .runtime
                .pending
                .borrow_mut()
                .commit(&self.runtime.g, &self.runtime.stats)
            {
                return Some(Err(e));
            }
            // Reverse once so we can pop from the end in O(1) while preserving order.
            self.results.reverse();
        }

        // Yield collected batches one at a time.
        self.results.pop().map(Ok)
    }
}
