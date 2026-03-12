//! Vectorized block nested loop cartesian product operator.
//!
//! Materializes the right sub-plan(s) once on first use, then cross-joins
//! blocks of left rows with the materialized right rows.

use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct CartesianProductOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    /// Indices of all right children (may be >1 for multi-branch products).
    right_child_indices: Vec<NodeIdx<Dyn<IR>>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
    /// Lazily materialized right-side rows. `None` means not yet computed.
    materialized_right: Option<Vec<Env<'a>>>,
    /// Current block of left-side rows being cross-joined.
    left_envs: Vec<Env<'a>>,
    /// Current position within `left_envs`.
    left_pos: usize,
    /// Current position within `materialized_right`.
    right_pos: usize,
}

impl<'a> CartesianProductOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        let right_child_indices: Vec<_> = runtime
            .plan
            .node(idx)
            .children()
            .skip(1)
            .map(|c| c.idx())
            .collect();

        Self {
            runtime,
            child,
            right_child_indices,
            idx,
            materialized_right: None,
            left_envs: Vec::new(),
            left_pos: 0,
            right_pos: 0,
        }
    }

    /// Materializes all right sub-plans into a single `Vec<Env>`.
    ///
    /// For a single right child, runs the sub-plan once and collects all rows.
    /// For multiple right children, materializes each independently and computes
    /// their cross-product into a single flat vector.
    fn materialize_right(&self) -> Result<Vec<Env<'a>>, String> {
        let pool = self.runtime.env_pool;

        let mut branch_results: Vec<Vec<Env<'a>>> =
            Vec::with_capacity(self.right_child_indices.len());

        for &child_idx in &self.right_child_indices {
            let mut subtree = self.runtime.run_batch(child_idx)?;
            let default_env = Env::new(pool);
            subtree.set_argument_env(&default_env, pool);

            let mut branch_envs = Vec::new();
            for result in &mut subtree {
                let batch = result?;
                for env in batch.active_env_iter() {
                    branch_envs.push(env.clone_pooled(pool));
                }
            }
            branch_results.push(branch_envs);
        }

        // Single branch: no cross-product needed.
        if branch_results.len() == 1 {
            return Ok(branch_results.pop().unwrap());
        }

        // Multi-branch: iteratively cross-product all branches.
        let mut accumulated = branch_results.remove(0);
        for branch in branch_results {
            if accumulated.is_empty() || branch.is_empty() {
                return Ok(Vec::new());
            }
            let mut next = Vec::with_capacity(accumulated.len() * branch.len());
            for left in &accumulated {
                for right in &branch {
                    let mut merged = left.clone_pooled(pool);
                    merged.merge(right);
                    next.push(merged);
                }
            }
            accumulated = next;
        }

        Ok(accumulated)
    }
}

impl<'a> Iterator for CartesianProductOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let pool = self.runtime.env_pool;

        // Lazy materialization of right side (runs once).
        if self.materialized_right.is_none() {
            match self.materialize_right() {
                Ok(right) => {
                    if right.is_empty() {
                        return None;
                    }
                    self.materialized_right = Some(right);
                }
                Err(e) => return Some(Err(e)),
            }
        }

        let right = self.materialized_right.as_ref().unwrap();
        let right_len = right.len();
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        loop {
            // Produce cross-product rows from current (left_pos, right_pos).
            while envs.len() < BATCH_SIZE && self.left_pos < self.left_envs.len() {
                while envs.len() < BATCH_SIZE && self.right_pos < right_len {
                    let mut merged = self.left_envs[self.left_pos].clone_pooled(pool);
                    merged.merge(&right[self.right_pos]);
                    envs.push(merged);
                    self.right_pos += 1;
                }
                if self.right_pos >= right_len {
                    self.left_pos += 1;
                    self.right_pos = 0;
                }
            }

            // If batch is full, return it (positions are preserved for next call).
            if envs.len() >= BATCH_SIZE {
                return Some(Ok(Batch::from_envs(envs)));
            }

            // Current left block exhausted. Load next batch from left child.
            self.left_envs.clear();
            self.left_pos = 0;
            self.right_pos = 0;

            match self.child.next() {
                Some(Ok(batch)) => {
                    for env in batch.active_env_iter() {
                        self.left_envs.push(env.clone_pooled(pool));
                    }
                }
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    if envs.is_empty() {
                        return None;
                    }
                    return Some(Ok(Batch::from_envs(envs)));
                }
            }
        }
    }
}
