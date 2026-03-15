//! Batch-mode variable-length traverse operator — multi-hop BFS relationship expansion.
//!
//! Implements Cypher patterns like `(a)-[*2..5]->(b)`. For each active row
//! in each input batch, performs a breadth-first search from the source node
//! up to `max_hops` away, yielding result rows for destinations reached at
//! or beyond `min_hops`. Output rows are accumulated into batches of up to
//! `BATCH_SIZE`.

use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::graph::graph::NodeId;
use crate::parser::ast::{QueryRelationship, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx};
use thin_vec::ThinVec;

pub struct CondVarLenTraverseOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    pending: VecDeque<Env<'a>>,
    relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> CondVarLenTraverseOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        relationship_pattern: &'a QueryRelationship<Arc<String>, Arc<String>, Variable>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            pending: VecDeque::new(),
            relationship_pattern,
            idx,
        }
    }

    fn expand_row(
        &self,
        vars: &Env<'a>,
        out: &mut Vec<Env<'a>>,
    ) {
        let relationship_pattern = self.relationship_pattern;

        let from_id = vars
            .get(&relationship_pattern.from.alias)
            .and_then(|v| match v {
                Value::Node(id) => Some(id),
                _ => None,
            });
        if from_id.is_none() && vars.is_bound(&relationship_pattern.from.alias) {
            return;
        }
        let to_id = vars
            .get(&relationship_pattern.to.alias)
            .and_then(|v| match v {
                Value::Node(id) => Some(*id),
                _ => None,
            });
        if to_id.is_none() && vars.is_bound(&relationship_pattern.to.alias) {
            return;
        }

        let min_hops = relationship_pattern.min_hops.unwrap_or(1);
        let max_hops = relationship_pattern.max_hops.unwrap_or(u32::MAX);
        let bidirectional = relationship_pattern.bidirectional;

        // Get starting nodes
        let start_nodes: Vec<NodeId> = from_id.map_or_else(
            || {
                self.runtime
                    .g
                    .borrow()
                    .get_nodes(&relationship_pattern.from.labels, 0)
                    .collect()
            },
            |id| vec![*id],
        );

        // Each frontier entry tracks: (current_node, path_of_edges)
        // where path_of_edges is a list of Value::Relationship for path building.
        for start_node in start_nodes {
            // Handle 0-hop case: start node itself is a valid result.
            if min_hops == 0 && (to_id.is_none() || to_id == Some(start_node)) {
                let mut env = vars.clone_pooled(self.runtime.env_pool);
                env.insert(&relationship_pattern.from.alias, Value::Node(start_node));
                env.insert(&relationship_pattern.to.alias, Value::Node(start_node));
                env.insert(
                    &relationship_pattern.alias,
                    Value::List(Arc::new(ThinVec::new())),
                );
                out.push(env);
            }

            let mut frontier: Vec<(NodeId, ThinVec<Value>)> = vec![(start_node, ThinVec::new())];
            let mut visited: HashSet<NodeId> = HashSet::new();
            visited.insert(start_node);

            let g = self.runtime.g.borrow();
            let to_labels = &relationship_pattern.to.labels;

            for hop in 1..=max_hops {
                let mut next_frontier_set: HashSet<NodeId> = HashSet::new();
                let mut next_frontier: Vec<(NodeId, ThinVec<Value>)> = Vec::new();
                for (current, path) in &frontier {
                    for (edge_src, edge_dst, edge_id) in g.get_node_relationships(*current) {
                        let neighbor = if edge_src == *current {
                            Some(edge_dst)
                        } else if bidirectional && edge_dst == *current {
                            Some(edge_src)
                        } else {
                            None
                        };
                        if let Some(dest) = neighbor
                            && !visited.contains(&dest)
                        {
                            let mut new_path = path.clone();
                            new_path
                                .push(Value::Relationship(Box::new((edge_id, edge_src, edge_dst))));

                            if hop >= min_hops
                                && (to_id.is_none() || to_id == Some(dest))
                                && (to_labels.is_empty()
                                    || to_labels
                                        .iter()
                                        .all(|l| g.get_node_labels(dest).any(|nl| nl == *l)))
                            {
                                let mut env = vars.clone_pooled(self.runtime.env_pool);
                                env.insert(
                                    &relationship_pattern.from.alias,
                                    Value::Node(start_node),
                                );
                                env.insert(&relationship_pattern.to.alias, Value::Node(dest));
                                env.insert(
                                    &relationship_pattern.alias,
                                    Value::List(Arc::new(new_path.clone())),
                                );
                                out.push(env);
                            }
                            if next_frontier_set.insert(dest) {
                                next_frontier.push((dest, new_path));
                            }
                        }
                    }
                }
                if next_frontier.is_empty() {
                    break;
                }
                for (node, _) in &next_frontier {
                    visited.insert(*node);
                }
                frontier = next_frontier;
            }
        }
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

impl<'a> Iterator for CondVarLenTraverseOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover rows from previous call.
        self.drain_pending(&mut envs);

        while envs.len() < BATCH_SIZE {
            let batch = match self.child.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            };

            for vars in batch.active_env_iter() {
                let mut expanded = Vec::new();
                self.expand_row(vars, &mut expanded);
                self.pending.extend(expanded);

                self.drain_pending(&mut envs);

                if envs.len() >= BATCH_SIZE {
                    break;
                }
            }
        }

        if envs.is_empty() {
            None
        } else {
            Some(Ok(Batch::from_envs(envs)))
        }
    }
}
