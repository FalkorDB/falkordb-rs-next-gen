//! Batch-mode unwind operator — expands a list expression into individual rows.
//!
//! For each active row in each input batch, evaluates the list expression and
//! expands it into individual rows. Output rows are accumulated into batches
//! of up to `BATCH_SIZE`.
//!
//! Uses `batch.env_ref()` to evaluate expressions without cloning input rows,
//! and only clones when producing output rows.

use std::collections::VecDeque;
use std::sync::Arc;

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct UnwindOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    list: &'a QueryExpr<Variable>,
    name: &'a Variable,
    pending: VecDeque<Env<'a>>,
    current_batch: Option<Batch<'a>>,
    current_pos: usize,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> UnwindOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        list: &'a QueryExpr<Variable>,
        name: &'a Variable,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            list,
            name,
            pending: VecDeque::new(),
            current_batch: None,
            current_pos: 0,
            idx,
        }
    }

    fn expand_row(
        &self,
        env: &Env<'a>,
        out: &mut Vec<Env<'a>>,
    ) -> Result<(), String> {
        let pool = self.runtime.env_pool;
        let value = self
            .runtime
            .run_expr(self.list, self.list.root().idx(), env, None)?;

        match value {
            Value::Null => {
                // Null produces zero output rows — skip without cloning.
            }
            Value::List(list) => {
                let items: Vec<Value> = Arc::unwrap_or_clone(list).into();
                for item in items {
                    let mut out_row = env.clone_pooled(pool);
                    out_row.insert(self.name, item);
                    out.push(out_row);
                }
            }
            other => {
                // Scalar value — produces exactly one output row.
                let mut out_row = env.clone_pooled(pool);
                out_row.insert(self.name, other);
                out.push(out_row);
            }
        }

        Ok(())
    }

    /// Drains rows from `self.pending` into `envs` until `BATCH_SIZE` is reached
    /// or all pending rows are exhausted.
    fn drain_pending(
        &mut self,
        envs: &mut Vec<Env<'a>>,
    ) {
        while envs.len() < BATCH_SIZE {
            if let Some(row) = self.pending.pop_front() {
                envs.push(row);
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for UnwindOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover rows from previous call.
        self.drain_pending(&mut envs);

        loop {
            if envs.len() >= BATCH_SIZE {
                break;
            }

            if self.current_batch.is_none() {
                match self.child.next() {
                    Some(Ok(b)) => {
                        self.current_batch = Some(b);
                        self.current_pos = 0;
                    }
                    Some(Err(e)) => return Some(Err(e)),
                    None => break,
                }
            }

            {
                let batch = self.current_batch.as_ref().unwrap();
                let active: Vec<usize> = batch.active_indices().collect();

                while self.current_pos < active.len() {
                    let row_idx = active[self.current_pos];
                    self.current_pos += 1;
                    let env = batch.env_ref(row_idx);
                    let mut expanded = Vec::new();
                    if let Err(e) = self.expand_row(env, &mut expanded) {
                        return Some(Err(e));
                    }
                    self.pending.extend(expanded);

                    if self.pending.len() >= BATCH_SIZE {
                        break;
                    }
                }
            }

            self.drain_pending(&mut envs);

            // Check if batch is exhausted.
            if let Some(ref batch) = self.current_batch
                && self.current_pos >= batch.active_len()
            {
                self.current_batch = None;
            }
        }

        if envs.is_empty() {
            None
        } else {
            Some(Ok(Batch::from_envs(envs)))
        }
    }
}
