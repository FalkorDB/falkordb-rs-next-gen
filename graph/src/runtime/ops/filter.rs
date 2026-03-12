//! Batch-mode filter operator — evaluates a boolean predicate on each row.
//!
//! Pulls a batch from the child operator, evaluates the predicate for each
//! active row, and returns only the passing rows via a selection vector
//! (zero-copy filtering).
//!
//! ```text
//!  Evaluation paths (chosen on first batch, cached thereafter):
//!
//!  1. Vectorized (simple predicates like `n.age > 30`):
//!     node_ids ──► materialize_node_property ──► compare_i64_column ──► selection
//!
//!  2. Per-row fallback (complex expressions):
//!     for each active row: run_expr(predicate, env) ──► collect passing indices
//! ```
//!
//! When the filter expression matches a vectorizable pattern (e.g.,
//! `n.age > 30`), the operator uses the fast columnar path:
//! extract node IDs → materialize property column → run comparison kernel.
//! For unrecognized patterns it falls back to per-row `run_expr` evaluation.

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp, Column},
    runtime::Runtime,
    value::Value,
    vectorized::{
        SimplePredicate, VectorizablePredicate, compare_f64_column, compare_i64_column,
        compare_string_column, mask_intersect_selection, mask_to_selection,
        try_extract_vectorizable_predicate,
    },
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

/// Cached predicate analysis result.
/// `None` means the expression has not been analyzed yet.
/// `Some(None)` means it was analyzed and is not vectorizable.
/// `Some(Some(..))` means it is vectorizable.
type CachedPredicate = Option<Option<VectorizablePredicate>>;

pub struct FilterOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    tree: &'a QueryExpr<Variable>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
    /// Lazily-initialized vectorizable predicate cache.
    vectorized: CachedPredicate,
}

impl<'a> FilterOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        tree: &'a QueryExpr<Variable>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            tree,
            idx,
            vectorized: None,
        }
    }

    /// Evaluates the filter using vectorized comparison kernels.
    ///
    /// Returns:
    /// - `Ok(Some(batch))` — filtered batch with passing rows
    /// - `Ok(None)` — all rows filtered out, caller should pull next batch
    /// - `Err(batch)` — vectorized path not applicable, fall back to per-row
    fn eval_vectorized(
        &self,
        mut batch: Batch<'a>,
        pred: &VectorizablePredicate,
    ) -> Result<Option<Batch<'a>>, Batch<'a>> {
        match pred {
            VectorizablePredicate::Single(p) => {
                let Ok(sel) = self.eval_single_predicate(&batch, p) else {
                    return Err(batch);
                };
                if sel.is_empty() {
                    Ok(None)
                } else {
                    batch.set_selection(sel);
                    Ok(Some(batch))
                }
            }
            VectorizablePredicate::Conjunction(preds) => {
                let mut sel: Option<Vec<u16>> = None;
                for p in preds {
                    let Ok(mask) = self.eval_single_mask(&batch, p) else {
                        return Err(batch);
                    };
                    sel = Some(sel.map_or_else(
                        || mask_to_selection(&mask),
                        |existing| mask_intersect_selection(&mask, &existing),
                    ));
                    // Early exit if nothing passes.
                    if sel.as_ref().is_some_and(Vec::is_empty) {
                        return Ok(None);
                    }
                }
                match sel {
                    Some(s) if s.is_empty() => Ok(None),
                    Some(s) => {
                        batch.set_selection(s);
                        Ok(Some(batch))
                    }
                    None => Ok(None), // empty conjunction
                }
            }
        }
    }

    /// Evaluates a single predicate and returns a selection vector of passing row indices.
    fn eval_single_predicate(
        &self,
        batch: &Batch<'a>,
        pred: &SimplePredicate,
    ) -> Result<Vec<u16>, ()> {
        let mask = self.eval_single_mask(batch, pred)?;
        Ok(mask_to_selection(&mask))
    }

    /// Evaluates a single predicate and returns a boolean mask.
    fn eval_single_mask(
        &self,
        batch: &Batch<'a>,
        pred: &SimplePredicate,
    ) -> Result<Vec<bool>, ()> {
        let node_ids = batch.extract_node_ids(pred.var.id).ok_or(())?;
        let (col, nulls) = self
            .runtime
            .materialize_node_property(&node_ids, &pred.attr);

        let mask = match (&col, &pred.constant) {
            (Column::Ints(data), Value::Int(threshold)) => {
                compare_i64_column(data, pred.op, *threshold, &nulls)
            }
            (Column::Ints(data), Value::Float(threshold)) => {
                // Promote int column to float for comparison.
                let floats: Vec<f64> = data.iter().map(|&i| i as f64).collect();
                compare_f64_column(&floats, pred.op, *threshold, &nulls)
            }
            (Column::Floats(data), Value::Float(threshold)) => {
                compare_f64_column(data, pred.op, *threshold, &nulls)
            }
            (Column::Floats(data), Value::Int(threshold)) => {
                compare_f64_column(data, pred.op, *threshold as f64, &nulls)
            }
            (Column::Values(data), Value::String(threshold)) => {
                compare_string_column(data, pred.op, threshold)
            }
            _ => return Err(()), // type mismatch, fall back to per-row
        };

        Ok(mask)
    }

    /// Per-row fallback: evaluates the filter expression for each active row.
    /// Uses `set_selection` for zero-copy filtering instead of rebuilding
    /// via `Batch::from_envs`.
    fn eval_per_row(
        &self,
        mut batch: Batch<'a>,
    ) -> Result<Option<Batch<'a>>, String> {
        let mut passing = Vec::new();
        for row in batch.active_indices() {
            let env = batch.env_ref(row);
            match self
                .runtime
                .run_expr(self.tree, self.tree.root().idx(), env, None)
            {
                Ok(Value::Bool(true)) => passing.push(row as u16),
                Ok(Value::Bool(false) | Value::Null) => {}
                Err(e) => return Err(e),
                Ok(value) => {
                    return Err(format!(
                        "Type mismatch: expected Boolean but was {}",
                        value.name()
                    ));
                }
            }
        }

        if passing.is_empty() {
            Ok(None)
        } else {
            batch.set_selection(passing);
            Ok(Some(batch))
        }
    }
}

impl<'a> Iterator for FilterOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Lazily analyze the expression on the first call.
        if self.vectorized.is_none() {
            self.vectorized = Some(try_extract_vectorizable_predicate(self.tree));
        }

        loop {
            let batch = match self.child.next()? {
                Ok(batch) => batch,
                Err(e) => return Some(Err(e)),
            };

            // Try vectorized path if we detected a vectorizable predicate.
            if let Some(Some(pred)) = &self.vectorized {
                match self.eval_vectorized(batch, pred) {
                    Ok(Some(result)) => return Some(Ok(result)),
                    Ok(None) => continue, // all rows filtered out
                    Err(batch) => {
                        // Vectorized path couldn't apply (e.g., variable isn't
                        // a node in this batch). Disable for future batches and
                        // fall through to per-row eval on the same batch.
                        self.vectorized = Some(None);
                        match self.eval_per_row(batch) {
                            Ok(Some(result)) => return Some(Ok(result)),
                            Ok(None) => continue,
                            Err(e) => return Some(Err(e)),
                        }
                    }
                }
            }

            // Per-row fallback path.
            match self.eval_per_row(batch) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
