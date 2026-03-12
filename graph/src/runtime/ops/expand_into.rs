//! Batch-mode expand-into operator — checks for relationships between
//! two already-bound nodes.
//!
//! Unlike `CondTraverse` (which scans all relationships for a label pair),
//! `ExpandInto` only checks edges between two specific already-bound endpoints.
//! This is used when both sides of a relationship pattern have been resolved
//! by prior operators.
//!
//! ```text
//!  Input: row where from=Node(5), to=Node(7)
//!  ──expand_row──►  checks edges between 5→7 (and 7→5 if bidirectional)
//!                   emits one output row per matching edge
//! ```
//!
//! Uses the same `pending` / `current_batch` / `current_pos` state machine
//! as [`CondTraverseOp`] for buffered batch emission.

use std::collections::VecDeque;
use std::sync::Arc;

use crate::parser::ast::{QueryRelationship, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct ExpandIntoOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
    pending: VecDeque<Env<'a>>,
    current_batch: Option<Batch<'a>>,
    current_pos: usize,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> ExpandIntoOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            relationship_pattern,
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
        let runtime = self.runtime;
        let rp = self.relationship_pattern;

        let src = match env.get(&rp.from.alias) {
            Some(Value::Node(id)) => *id,
            Some(Value::Null) | None => return Ok(()),
            _ => {
                return Err(String::from(
                    "Invalid node id for 'from' in relationship pattern",
                ));
            }
        };
        let dst = match env.get(&rp.to.alias) {
            Some(Value::Node(id)) => *id,
            Some(Value::Null) | None => return Ok(()),
            _ => {
                return Err(String::from(
                    "Invalid node id for 'to' in relationship pattern",
                ));
            }
        };

        let filter_attrs = runtime.run_expr(&rp.attrs, rp.attrs.root().idx(), env, None)?;

        let mut edge_pairs = vec![(src, dst)];
        if rp.bidirectional && src != dst {
            edge_pairs.push((dst, src));
        }

        let g = runtime.g.borrow();
        let pending = runtime.pending.borrow();
        for (edge_src, edge_dst) in &edge_pairs {
            for id in g.get_src_dest_relationships(*edge_src, *edge_dst, &rp.types) {
                if pending.is_relationship_deleted(id, *edge_src, *edge_dst) {
                    continue;
                }
                if let Value::Map(ref filter_map) = filter_attrs
                    && !filter_map.is_empty()
                {
                    let mut matches = true;
                    for (attr, avalue) in filter_map.iter() {
                        if let Some(pvalue) = g.get_relationship_attribute(id, attr) {
                            if *avalue == pvalue {
                                continue;
                            }
                            matches = false;
                            break;
                        }
                        matches = false;
                        break;
                    }
                    if !matches {
                        continue;
                    }
                }
                let mut row = env.clone_pooled(runtime.env_pool);
                row.insert(
                    &rp.alias,
                    Value::Relationship(Box::new((id, *edge_src, *edge_dst))),
                );
                row.insert(&rp.from.alias, Value::Node(src));
                row.insert(&rp.to.alias, Value::Node(dst));
                out.push(row);
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

impl<'a> Iterator for ExpandIntoOp<'a> {
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
