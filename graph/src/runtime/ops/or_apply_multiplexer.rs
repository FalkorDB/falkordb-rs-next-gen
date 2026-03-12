//! Batch-mode OR-apply multiplexer operator — evaluates multiple existence-check branches.
//!
//! Implements disjunctive patterns like `WHERE EXISTS {...} OR EXISTS {...}`.
//! For each active row in each input batch, iterates through branch sub-plans.
//! A row is emitted when any branch produces a result (or, for `anti` branches,
//! when a branch produces NO results). The `anti_flags` array controls whether
//! each branch uses normal or anti-semi-join semantics.

use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct OrApplyMultiplexerOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    anti_flags: &'a [bool],
    branch_indices: Vec<NodeIdx<Dyn<IR>>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> OrApplyMultiplexerOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        anti_flags: &'a [bool],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        let num_branches = runtime.plan.node(idx).num_children() - 1;
        let mut branch_indices = Vec::with_capacity(num_branches);

        for i in 1..=num_branches {
            let branch_idx = runtime.plan.node(idx).child(i).idx();
            branch_indices.push(branch_idx);
        }

        Self {
            runtime,
            child,
            anti_flags,
            branch_indices,
            idx,
        }
    }
}

impl<'a> Iterator for OrApplyMultiplexerOp<'a> {
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
                let mut matched = false;

                for (branch_num, branch_idx) in self.branch_indices.iter().enumerate() {
                    let has_result = match self.runtime.run_batch(*branch_idx) {
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
                    let is_anti = self.anti_flags[branch_num];
                    if has_result ^ is_anti {
                        matched = true;
                        break;
                    }
                }

                if matched {
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
