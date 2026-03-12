//! Union operator — concatenates results from multiple sub-plans.
//!
//! Implements Cypher `UNION` / `UNION ALL`. Iterates through each child
//! sub-plan in order and yields all batches from the first child before
//! moving to the second, and so on.

use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct UnionOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) current: Option<Box<BatchOp<'a>>>,
    current_child: usize,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> UnionOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            current: None,
            current_child: 0,
            idx,
        }
    }
}

impl<'a> Iterator for UnionOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut current) = self.current {
                if let Some(result) = current.next() {
                    return Some(result);
                }
                self.current = None;
                self.current_child += 1;
            }
            let current_node = self.runtime.plan.node(self.idx);
            if self.current_child >= current_node.num_children() {
                return None;
            }
            let child_idx = current_node.child(self.current_child).idx();
            match self.runtime.run_batch(child_idx) {
                Ok(child_op) => {
                    self.current = Some(Box::new(child_op));
                }
                Err(e) => {
                    self.current_child = current_node.num_children();
                    return Some(Err(e));
                }
            }
        }
    }
}
