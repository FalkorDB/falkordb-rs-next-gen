//! Batch-mode filter operator — evaluates a boolean predicate on each row.
//!
//! Pulls a batch from the child operator, evaluates the predicate for each
//! active row, and returns only the passing rows via a selection vector
//! (zero-copy filtering).
//!
//! ```text
//!  Single columnar evaluation entry point (`eval`):
//!
//!  1. Vectorized kernel (simple predicates like `n.age > 30`):
//!     node_ids ──► materialize_node_property ──► compare_i64_column ──► selection
//!
//!  2. Columnar per-row fallback (complex expressions):
//!     for each active row: run_expr(predicate, BatchRow) ──► collect passing
//! ```
//!
//! When the filter expression matches a vectorizable pattern (e.g.,
//! `n.age > 30`), `eval` uses the fast columnar kernel path:
//! extract node IDs → materialize property column → run comparison kernel.
//! For unrecognized patterns it reads each row through a [`BatchRow`] view
//! (no `Env` materialization) and emits the survivors as a selection vector.

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::eval::ExprEval;
use crate::runtime::{
    batch::{Batch, BatchOp, BatchRow, Column},
    runtime::Runtime,
    value::Value,
    vectorized::{
        PredicateTest, SimplePredicate, TriMask, VectorizablePredicate, compare_f64_column,
        compare_i64_column, compare_string_column, match_string_column, null_check_mask,
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

    /// Evaluates the filter for one batch, returning the surviving rows.
    ///
    /// A single columnar entry point: when a vectorizable predicate was
    /// detected it runs the comparison kernels; otherwise (or when the kernel
    /// can't apply to this batch) it evaluates the full predicate per row
    /// through a [`BatchRow`] view. Survivors flow out as a columnar batch
    /// carrying just a selection vector.
    ///
    /// Returns `Ok(Some(batch))` with passing rows, `Ok(None)` when every row
    /// was filtered out, or `Err` on a predicate evaluation error.
    fn eval(
        &mut self,
        mut batch: Batch<'a>,
    ) -> Result<Option<Batch<'a>>, String> {
        // Fast path: vectorized comparison kernels for simple predicates.
        if matches!(self.vectorized, Some(Some(_))) {
            // Scope the immutable borrow of the cached predicate so the kernel
            // can be disabled afterwards if it doesn't apply to this batch.
            let result = {
                let Some(Some(pred)) = &self.vectorized else {
                    unreachable!("guarded by matches! above")
                };
                self.eval_vectorized(&batch, pred)
            };
            match result {
                Ok(Some(sel)) => {
                    batch.set_selection(sel);
                    return Ok(Some(batch));
                }
                Ok(None) => return Ok(None),
                Err(()) => {
                    // Kernel couldn't apply (e.g. variable isn't a node in this
                    // batch). Disable it for future batches and fall through to
                    // the columnar per-row path on this batch.
                    self.vectorized = Some(None);
                }
            }
        }

        // Columnar per-row path: read each active row through a `BatchRow`
        // view and evaluate the full predicate, collecting passing indices.
        let mut passing = Vec::new();
        for row in batch.active_indices() {
            let view = BatchRow::new(&batch, row);
            match ExprEval::from_runtime(self.runtime).eval(
                self.tree,
                self.tree.root().idx(),
                Some(&view),
                None,
            ) {
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

    /// Evaluates a vectorizable predicate with comparison kernels, returning a
    /// selection vector of passing rows.
    ///
    /// Returns `Ok(Some(sel))` with the passing indices, `Ok(None)` when no row
    /// passes, or `Err(())` when the kernel can't apply (caller falls back to
    /// per-row evaluation).
    fn eval_vectorized(
        &self,
        batch: &Batch<'a>,
        pred: &VectorizablePredicate,
    ) -> Result<Option<Vec<u16>>, ()> {
        let mask = self.eval_tri_mask(batch, pred)?;
        let sel = mask.selection();
        Ok((!sel.is_empty()).then_some(sel))
    }

    /// Recursively evaluates a predicate tree into a three-valued row mask,
    /// combining child masks with Kleene `AND`/`OR`/`NOT` — the same NULL
    /// semantics as the per-row evaluator.
    fn eval_tri_mask(
        &self,
        batch: &Batch<'a>,
        pred: &VectorizablePredicate,
    ) -> Result<TriMask, ()> {
        match pred {
            VectorizablePredicate::Leaf(p) => self.eval_leaf_mask(batch, p),
            VectorizablePredicate::And(preds) => {
                let mut iter = preds.iter();
                let first = self.eval_tri_mask(batch, iter.next().ok_or(())?)?;
                iter.try_fold(first, |acc, p| Ok(acc.and(&self.eval_tri_mask(batch, p)?)))
            }
            VectorizablePredicate::Or(preds) => {
                let mut iter = preds.iter();
                let first = self.eval_tri_mask(batch, iter.next().ok_or(())?)?;
                iter.try_fold(first, |acc, p| Ok(acc.or(&self.eval_tri_mask(batch, p)?)))
            }
            VectorizablePredicate::Not(inner) => Ok(!self.eval_tri_mask(batch, inner)?),
        }
    }

    /// Evaluates a single leaf predicate into a three-valued row mask by
    /// materializing the property column and running the matching kernel.
    fn eval_leaf_mask(
        &self,
        batch: &Batch<'a>,
        pred: &SimplePredicate,
    ) -> Result<TriMask, ()> {
        let node_ids = batch.extract_node_ids(pred.var.id).ok_or(())?;
        let (col, nulls) = self
            .runtime
            .materialize_node_property(&node_ids, &pred.attr);
        let len = node_ids.len();
        let null_vec = || (0..len).map(|i| nulls.is_null(i)).collect::<Vec<bool>>();

        let mask = match &pred.test {
            PredicateTest::Cmp { op, constant } => match (&col, constant) {
                (Column::Ints(data), Value::Int(threshold)) => TriMask::new(
                    compare_i64_column(data, *op, *threshold, &nulls),
                    null_vec(),
                ),
                (Column::Ints(data), Value::Float(threshold)) => {
                    // Promote int column to float for comparison.
                    let floats: Vec<f64> = data.iter().map(|&i| i as f64).collect();
                    TriMask::new(
                        compare_f64_column(&floats, *op, *threshold, &nulls),
                        null_vec(),
                    )
                }
                (Column::Floats(data), Value::Float(threshold)) => TriMask::new(
                    compare_f64_column(data, *op, *threshold, &nulls),
                    null_vec(),
                ),
                (Column::Floats(data), Value::Int(threshold)) => TriMask::new(
                    compare_f64_column(data, *op, *threshold as f64, &nulls),
                    null_vec(),
                ),
                (Column::Values(data), Value::String(threshold)) => {
                    compare_string_column(data, *op, threshold)
                }
                _ => return Err(()), // type mismatch, fall back to per-row
            },
            PredicateTest::IsNull { negated } => {
                // IS [NOT] NULL never evaluates to NULL itself.
                TriMask::from_bools(null_check_mask(&nulls, len, *negated))
            }
            PredicateTest::StringMatch { op, pattern } => match &col {
                Column::Values(data) => match_string_column(data, *op, pattern),
                // A typed numeric column can't contain strings, so every row
                // is NULL (`non-string CONTAINS x` evaluates to NULL, which
                // the filter drops but `NOT` must preserve as NULL).
                Column::Ints(_) | Column::Floats(_) => TriMask::all_null(len),
                _ => return Err(()), // fall back to per-row
            },
        };

        Ok(mask)
    }
}

impl<'a> Iterator for FilterOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Lazily analyze the expression on the first call.
        if self.vectorized.is_none() {
            self.vectorized = Some(try_extract_vectorizable_predicate(
                self.tree,
                &self.runtime.parameters,
            ));
        }

        loop {
            let batch = match self.child.next()? {
                Ok(batch) => batch,
                Err(e) => return Some(Err(e)),
            };

            match self.eval(batch) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {} // all rows filtered out — pull next batch
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
