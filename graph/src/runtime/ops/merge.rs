//! Batch-mode merge operator — implements Cypher `MERGE` (match-or-create).
//!
//! For each active row in each input batch, executes the match sub-plan.
//! If matches are found, applies `ON MATCH SET` clauses; otherwise creates
//! the pattern and applies `ON CREATE SET` clauses. A pattern hash cache
//! prevents duplicate creates within the same query.

use std::cell::OnceCell;
use std::collections::VecDeque;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use crate::graph::graph::LabelId;
use crate::parser::ast::{QueryGraph, SetItem, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

/// Pending merge sub-plan state for a single input row.
/// We already know this subtree produced at least one match,
/// so we lazily drain remaining matches applying ON MATCH SET.
struct PendingMerge<'a> {
    /// The input env to merge with each match result.
    env: Env<'a>,
    /// The sub-plan iterator producing match batches.
    subtree: BatchOp<'a>,
    /// Remaining rows from the current sub-batch being drained.
    current_batch: Option<(Batch<'a>, usize)>,
}

pub struct MergeOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    pending: VecDeque<PendingMerge<'a>>,
    merge_child_idx: NodeIdx<Dyn<IR>>,
    pattern: &'a QueryGraph<Arc<String>, Arc<String>, Variable>,
    resolved_pattern: OnceCell<QueryGraph<Arc<String>, LabelId, Variable>>,
    on_create_set_items: &'a [SetItem<Arc<String>, Variable>],
    resolved_on_create_set_items: OnceCell<Vec<SetItem<LabelId, Variable>>>,
    on_match_set_items: &'a [SetItem<Arc<String>, Variable>],
    resolved_on_match_set_items: OnceCell<Vec<SetItem<LabelId, Variable>>>,
    is_error: bool,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> MergeOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        pattern: &'a QueryGraph<Arc<String>, Arc<String>, Variable>,
        on_create_set_items: &'a [SetItem<Arc<String>, Variable>],
        on_match_set_items: &'a [SetItem<Arc<String>, Variable>],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        let merge_child_idx = if runtime.plan.node(idx).num_children() == 1 {
            runtime.plan.node(idx).child(0).idx()
        } else {
            runtime.plan.node(idx).child(1).idx()
        };

        Self {
            runtime,
            child,
            pending: VecDeque::new(),
            merge_child_idx,
            pattern,
            resolved_pattern: OnceCell::new(),
            on_create_set_items,
            resolved_on_create_set_items: OnceCell::new(),
            on_match_set_items,
            resolved_on_match_set_items: OnceCell::new(),
            is_error: false,
            idx,
        }
    }

    fn resolve_pattern(&self) -> &QueryGraph<Arc<String>, LabelId, Variable> {
        self.resolved_pattern.get_or_init(|| {
            let resolved = self.runtime.resolve_pattern(self.pattern);
            self.runtime.pending.borrow_mut().resize(
                self.runtime.g.borrow().node_cap(),
                self.runtime.g.borrow().labels_count(),
            );
            resolved
        })
    }

    fn resolve_on_create_set_items(&self) -> &Vec<SetItem<LabelId, Variable>> {
        self.resolved_on_create_set_items.get_or_init(|| {
            let resolved = self.runtime.resolve_set_items(self.on_create_set_items);
            self.runtime.pending.borrow_mut().resize(
                self.runtime.g.borrow().node_cap(),
                self.runtime.g.borrow().labels_count(),
            );
            resolved
        })
    }

    fn resolve_on_match_set_items(&self) -> &Vec<SetItem<LabelId, Variable>> {
        self.resolved_on_match_set_items.get_or_init(|| {
            let resolved = self.runtime.resolve_set_items(self.on_match_set_items);
            self.runtime.pending.borrow_mut().resize(
                self.runtime.g.borrow().node_cap(),
                self.runtime.g.borrow().labels_count(),
            );
            resolved
        })
    }

    fn do_create_fallback(
        &self,
        vars: Env<'a>,
    ) -> Result<Env<'a>, String> {
        let resolved_pattern = self.resolve_pattern();
        let pattern_hash = self.compute_merge_pattern_hash(resolved_pattern, &vars)?;

        let merge_cache = self.runtime.merge_pattern_cache.borrow_mut();

        if let Some(cached_vars) = merge_cache.get(&pattern_hash) {
            // Pattern already created, apply ON MATCH and return cached vars
            let mut vars = vars;
            for (id, value) in cached_vars {
                vars.insert_by_id(*id, value.clone());
            }
            drop(merge_cache);

            let resolved = self.resolve_on_match_set_items();
            let batch = Batch::from_envs(vec![vars]);
            self.runtime.set_batch(resolved, &batch)?;
            let mut envs_vec = batch.into_envs();
            Ok(envs_vec.pop().unwrap())
        } else {
            // Pattern not yet created, create it
            drop(merge_cache);

            let mut batch = Batch::from_envs(vec![vars]);
            self.runtime.create_batch(resolved_pattern, &mut batch)?;

            // Cache only the created entity bindings (node/relationship IDs)
            let env_ref = batch.env_ref(0);
            let pattern_vars: Vec<(u32, Value)> = resolved_pattern
                .nodes()
                .iter()
                .map(|n| {
                    (
                        n.alias.id,
                        env_ref.get(&n.alias).cloned().unwrap_or(Value::Null),
                    )
                })
                .chain(resolved_pattern.relationships().iter().map(|r| {
                    (
                        r.alias.id,
                        env_ref.get(&r.alias).cloned().unwrap_or(Value::Null),
                    )
                }))
                .collect();
            self.runtime
                .merge_pattern_cache
                .borrow_mut()
                .insert(pattern_hash, pattern_vars);

            let resolved = self.resolve_on_create_set_items();
            self.runtime.set_batch(resolved, &batch)?;
            let mut envs_vec = batch.into_envs();
            Ok(envs_vec.pop().unwrap())
        }
    }

    fn compute_merge_pattern_hash(
        &self,
        pattern: &QueryGraph<Arc<String>, LabelId, Variable>,
        vars: &Env<'a>,
    ) -> Result<u64, String> {
        let mut hasher = DefaultHasher::new();

        // Hash nodes in the pattern
        for node in pattern.nodes() {
            if let Some(value) = vars.get(&node.alias) {
                value.hash(&mut hasher);
            } else {
                for label in node.labels.iter() {
                    label.hash(&mut hasher);
                }
                let attrs =
                    self.runtime
                        .run_expr(&node.attrs, node.attrs.root().idx(), vars, None)?;

                if let Value::Map(ref map) = attrs {
                    for (key, value) in map.iter() {
                        if *value == Value::Null {
                            return Err(format!(
                                "Cannot merge node using null property value for key '{key}'"
                            ));
                        }
                    }
                }

                attrs.hash(&mut hasher);
            }
        }

        // Hash relationships in the pattern
        for rel in pattern.relationships() {
            rel.types.hash(&mut hasher);

            if let Some(value) = vars.get(&rel.from.alias) {
                value.hash(&mut hasher);
            }
            if let Some(value) = vars.get(&rel.to.alias) {
                value.hash(&mut hasher);
            }

            let attrs = self
                .runtime
                .run_expr(&rel.attrs, rel.attrs.root().idx(), vars, None)?;

            if let Value::Map(ref map) = attrs {
                for (key, value) in map.iter() {
                    if *value == Value::Null {
                        return Err(format!(
                            "Cannot merge relationship using null property value for key '{key}'"
                        ));
                    }
                }
            }

            attrs.hash(&mut hasher);
        }

        Ok(hasher.finish())
    }

    /// Drains rows from `self.pending` into `envs` until `BATCH_SIZE` is reached
    /// or all pending sub-plans are exhausted.
    fn drain_pending(
        &mut self,
        envs: &mut Vec<Env<'a>>,
    ) -> Result<(), String> {
        while envs.len() < BATCH_SIZE && !self.pending.is_empty() {
            // Take current_batch out to avoid borrow conflicts.
            let mut current_batch = self.pending[0].current_batch.take();

            if let Some((ref batch, ref mut pos)) = current_batch {
                let active: Vec<usize> = batch.active_indices().collect();
                while *pos < active.len() && envs.len() < BATCH_SIZE {
                    let match_env = batch
                        .env_ref(active[*pos])
                        .clone_pooled(self.runtime.env_pool);
                    let mut vars = self.pending[0].env.clone_pooled(self.runtime.env_pool);
                    vars.merge(&match_env);
                    let resolved = self.resolve_on_match_set_items();
                    let result_batch = Batch::from_envs(vec![vars]);
                    self.runtime.set_batch(resolved, &result_batch)?;
                    let mut envs_vec = result_batch.into_envs();
                    envs.push(envs_vec.pop().unwrap());
                    *pos += 1;
                }
                if *pos < active.len() {
                    // Not fully drained, put it back and wait for next call.
                    self.pending[0].current_batch = current_batch;
                    return Ok(());
                }
                // Fully drained, fall through to pull next sub-batch.
            }

            // Pull next sub-batch from the subtree.
            match self.pending[0].subtree.next() {
                Some(Ok(sub_batch)) => {
                    self.pending[0].current_batch = Some((sub_batch, 0));
                }
                Some(Err(e)) => return Err(e),
                None => {
                    self.pending.pop_front();
                }
            }
        }
        Ok(())
    }
}

impl<'a> Iterator for MergeOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_error {
            return None;
        }

        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover match sub-plans from previous call.
        if let Err(e) = self.drain_pending(&mut envs) {
            self.is_error = true;
            return Some(Err(e));
        }

        while envs.len() < BATCH_SIZE {
            let batch = match self.child.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => {
                    self.is_error = true;
                    return Some(Err(e));
                }
                None => break,
            };

            for env in batch.active_env_iter() {
                // Check if all nodes in the pattern are already bound
                let all_nodes_bound = self
                    .resolve_pattern()
                    .nodes()
                    .iter()
                    .all(|node| env.get(&node.alias).is_some());

                let mut subtree = match self.runtime.run_batch(self.merge_child_idx) {
                    Ok(iter) => iter,
                    Err(e) => {
                        self.is_error = true;
                        return Some(Err(e));
                    }
                };
                subtree.set_argument_env(env, self.runtime.env_pool);

                // Try to get the first match to determine match vs create.
                let first_match = 'first: {
                    for sub_result in subtree.by_ref() {
                        match sub_result {
                            Ok(sub_batch) => {
                                let mut active = sub_batch.active_env_iter();
                                if let Some(first) = active.next() {
                                    let first = first.clone_pooled(self.runtime.env_pool);
                                    // If all_nodes_bound, we only need the first match.
                                    if all_nodes_bound {
                                        break 'first Some((first, None, None));
                                    }
                                    // Collect remaining from this batch, store subtree for lazy drain.
                                    break 'first Some((first, Some(sub_batch), Some(subtree)));
                                }
                            }
                            Err(e) => {
                                self.is_error = true;
                                return Some(Err(e));
                            }
                        }
                    }
                    None
                };

                match first_match {
                    None => {
                        // No matches found, do create fallback.
                        match self.do_create_fallback(env.clone_pooled(self.runtime.env_pool)) {
                            Ok(result_env) => envs.push(result_env),
                            Err(e) => {
                                self.is_error = true;
                                return Some(Err(e));
                            }
                        }
                    }
                    Some((first, remaining_batch, remaining_subtree)) => {
                        // Apply ON MATCH to the first result.
                        let mut vars = env.clone_pooled(self.runtime.env_pool);
                        vars.merge(&first);
                        let resolved = self.resolve_on_match_set_items();
                        let result_batch = Batch::from_envs(vec![vars]);
                        match self.runtime.set_batch(resolved, &result_batch) {
                            Ok(()) => {
                                let mut envs_vec = result_batch.into_envs();
                                envs.push(envs_vec.pop().unwrap());
                            }
                            Err(e) => {
                                self.is_error = true;
                                return Some(Err(e));
                            }
                        }

                        // If there are remaining matches, store them for lazy drain.
                        if let Some(subtree) = remaining_subtree {
                            // The remaining_batch still has rows after the first.
                            // We need to figure the position: first was index 0 of active,
                            // so start from position 1.
                            let batch = remaining_batch.unwrap();
                            let active_len = batch.active_env_iter().len();
                            let current_batch = if active_len > 1 {
                                Some((batch, 1))
                            } else {
                                None
                            };

                            self.pending.push_back(PendingMerge {
                                env: env.clone_pooled(self.runtime.env_pool),
                                subtree,
                                current_batch,
                            });
                        }
                    }
                }
            }

            if let Err(e) = self.drain_pending(&mut envs) {
                self.is_error = true;
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
