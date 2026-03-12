//! Batch-mode distinct operator — deduplicates result rows across batches.
//!
//! Pulls batches from the child operator and filters out rows whose
//! projected return columns have been seen before (by hash). Uses
//! `batch.get()` to read return-name variables and `set_selection`
//! for zero-copy filtering.

use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
    value::ValuesDeduper,
};
use orx_tree::{Dyn, NodeIdx};
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct DistinctOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    deduper: ValuesDeduper,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> DistinctOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            deduper: ValuesDeduper::default(),
            idx,
        }
    }
}

impl<'a> Iterator for DistinctOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut batch = match self.child.next()? {
                Ok(batch) => batch,
                Err(e) => return Some(Err(e)),
            };

            let mut passing = Vec::new();

            for row in batch.active_indices() {
                let mut hasher = DefaultHasher::new();
                for name in &self.runtime.return_names {
                    batch.get(row, name.id).hash(&mut hasher);
                }
                if self.deduper.has_hash(hasher.finish()) {
                    continue;
                }
                passing.push(row as u16);
            }

            if passing.is_empty() {
                continue;
            }

            batch.set_selection(passing);
            return Some(Ok(batch));
        }
    }
}
