//! Batch-mode optional operator — implements OPTIONAL MATCH semantics.
//!
//! For each active row in the input batch, runs the sub-plan. If it
//! produces results, they are yielded. If not, a single fallback row
//! is emitted with the specified variables set to NULL.

use std::collections::VecDeque;

use crate::parser::ast::Variable;
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

/// Pending sub-plan execution state for a single input row.
struct PendingOptional<'a> {
    /// The input env (for fallback if no results).
    env: Env<'a>,
    /// The sub-plan iterator producing result batches.
    subtree: BatchOp<'a>,
    /// Whether the sub-plan has produced at least one result row.
    had_result: bool,
    /// Remaining rows from the current sub-batch being drained.
    current_batch: Option<(Batch<'a>, usize)>,
}

pub struct OptionalOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    vars: &'a [Variable],
    optional_child_idx: NodeIdx<Dyn<IR>>,
    pending: VecDeque<PendingOptional<'a>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> OptionalOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        vars: &'a [Variable],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        let optional_child_idx = if runtime.plan.node(idx).num_children() == 1 {
            runtime.plan.node(idx).child(0).idx()
        } else {
            runtime.plan.node(idx).child(1).idx()
        };

        Self {
            runtime,
            child,
            vars,
            optional_child_idx,
            pending: VecDeque::new(),
            idx,
        }
    }

    /// Drains rows from `self.pending` into `envs` until `BATCH_SIZE` is reached
    /// or all pending sub-plans are exhausted.
    fn drain_pending(
        &mut self,
        envs: &mut Vec<Env<'a>>,
    ) -> Result<(), String> {
        while envs.len() < BATCH_SIZE {
            let Some(p) = self.pending.front_mut() else {
                break;
            };

            // Try to drain from the current sub-batch first.
            if let Some((batch, pos)) = &mut p.current_batch {
                let active: Vec<usize> = batch.active_indices().collect();
                while *pos < active.len() && envs.len() < BATCH_SIZE {
                    let row = batch.env_ref(active[*pos]);
                    p.had_result = true;
                    envs.push(row.clone_pooled(self.runtime.env_pool));
                    *pos += 1;
                }
                if *pos >= active.len() {
                    p.current_batch = None;
                } else {
                    // Batch not fully drained, wait for next call.
                    return Ok(());
                }
            }

            // Pull next sub-batch from the subtree.
            match p.subtree.next() {
                Some(Ok(sub_batch)) => {
                    p.current_batch = Some((sub_batch, 0));
                    // Loop back to drain from this new sub-batch.
                }
                Some(Err(e)) => return Err(e),
                None => {
                    // Sub-plan exhausted. Emit fallback if no results.
                    if !p.had_result {
                        let mut fallback = p.env.clone_pooled(self.runtime.env_pool);
                        for v in self.vars {
                            fallback.insert(v, Value::Null);
                        }
                        envs.push(fallback);
                    }
                    self.pending.pop_front();
                }
            }
        }
        Ok(())
    }
}

impl<'a> Iterator for OptionalOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover sub-plans from previous call.
        if let Err(e) = self.drain_pending(&mut envs) {
            return Some(Err(e));
        }

        while envs.len() < BATCH_SIZE {
            let batch = match self.child.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            };

            for env in batch.active_env_iter() {
                let mut subtree = match self.runtime.run_batch(self.optional_child_idx) {
                    Ok(iter) => iter,
                    Err(e) => return Some(Err(e)),
                };
                subtree.set_argument_env(env, self.runtime.env_pool);

                self.pending.push_back(PendingOptional {
                    env: env.clone_pooled(self.runtime.env_pool),
                    subtree,
                    had_result: false,
                    current_batch: None,
                });
            }

            if let Err(e) = self.drain_pending(&mut envs) {
                return Some(Err(e));
            }
        }

        if envs.is_empty() {
            None
        } else {
            Some(Ok(Batch::from_envs(envs)))
        }
    }
}
