//! Batch-mode semi-apply operator — existence-based filtering via a sub-plan.
//!
//! For each active row in the input batch, runs the right sub-plan. If it
//! produces at least one result, the row is included (or excluded for
//! anti mode).

use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct SemiApplyOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    is_anti: bool,
    right_child_idx: NodeIdx<Dyn<IR>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> SemiApplyOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        is_anti: bool,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        let right_child_idx = runtime.plan.node(idx).child(1).idx();

        Self {
            runtime,
            child,
            is_anti,
            right_child_idx,
            idx,
        }
    }
}

impl<'a> Iterator for SemiApplyOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut batch = match self.child.next()? {
                Ok(b) => b,
                Err(e) => return Some(Err(e)),
            };

            let mut passing = Vec::new();

            for row in batch.active_indices() {
                let env = batch.env_ref(row);
                let has_result = match self.runtime.run_batch(self.right_child_idx) {
                    Ok(mut subtree) => {
                        subtree.set_argument_env(env, self.runtime.env_pool);
                        let mut found = false;
                        'outer: for sub_result in subtree.by_ref() {
                            match sub_result {
                                Ok(sub_batch) => {
                                    if sub_batch.active_len() > 0 {
                                        found = true;
                                        break 'outer;
                                    }
                                }
                                Err(e) => return Some(Err(e)),
                            }
                        }
                        found
                    }
                    Err(e) => return Some(Err(e)),
                };
                if has_result ^ self.is_anti {
                    passing.push(row as u16);
                }
            }

            if !passing.is_empty() {
                batch.set_selection(passing);
                return Some(Ok(batch));
            }
        }
    }
}
