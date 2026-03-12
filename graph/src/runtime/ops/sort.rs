//! Batch-mode sort operator — orders result rows by one or more expressions.
//!
//! This is a *blocking* operator: it consumes all batches from the child on
//! the first `next()` call, evaluates sort-key expressions for each
//! row, sorts in-memory using a stable sort with per-key ascending/descending
//! control, and then yields rows in sorted batches.

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::{CompareValue, Value},
};
use orx_tree::{Dyn, NodeIdx, NodeRef};
use std::cmp::Ordering;

type SortItem<'a> = (Env<'a>, Vec<(Value, bool)>);

pub struct SortOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Option<Box<BatchOp<'a>>>,
    trees: &'a [(QueryExpr<Variable>, bool)],
    /// Sorted results stored in reverse order so we can pop from the end in O(1).
    results: Vec<Env<'a>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> SortOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        trees: &'a [(QueryExpr<Variable>, bool)],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child: Some(child),
            trees,
            results: Vec::new(),
            idx,
        }
    }
}

impl<'a> Iterator for SortOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Consume all input on first call.
        if let Some(child) = self.child.take() {
            let mut items: Vec<SortItem<'a>> = Vec::new();
            for batch_result in child {
                let batch = match batch_result {
                    Ok(b) => b,
                    Err(e) => return Some(Err(e)),
                };
                for env in batch.active_env_iter() {
                    let sort_keys = match self
                        .trees
                        .iter()
                        .map(|(tree, desc)| {
                            Ok((
                                self.runtime.run_expr(tree, tree.root().idx(), env, None)?,
                                *desc,
                            ))
                        })
                        .collect::<Result<Vec<_>, String>>()
                    {
                        Ok(keys) => keys,
                        Err(e) => return Some(Err(e)),
                    };
                    items.push((env.clone_pooled(self.runtime.env_pool), sort_keys));
                }
            }

            items.sort_by(|(_, a), (_, b)| {
                a.iter()
                    .zip(b)
                    .fold(Ordering::Equal, |acc, ((a, desc_a), (b, _))| {
                        if acc != Ordering::Equal {
                            return acc;
                        }
                        let (ordering, _) = a.compare_value(b);
                        if *desc_a {
                            ordering.reverse()
                        } else {
                            ordering
                        }
                    })
            });

            // Reverse so we can pop from the end in O(1) while preserving
            // sorted order, and drop sort keys immediately (no longer needed).
            self.results = items.into_iter().rev().map(|(env, _keys)| env).collect();
        }

        // Emit sorted rows in batches by popping from the end.
        if self.results.is_empty() {
            return None;
        }

        let n = BATCH_SIZE.min(self.results.len());
        let mut envs = self.results.split_off(self.results.len() - n);
        envs.reverse();

        Some(Ok(Batch::from_envs(envs)))
    }
}
