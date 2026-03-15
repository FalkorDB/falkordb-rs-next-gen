//! Batch-mode path builder operator — assembles named path values.
//!
//! Implements Cypher named paths like `p = (a)-[r]->(b)`. For each path,
//! reads the component variable columns via `read_columns`, maps each row
//! into a `Value::Path`, and writes the result column back via `write_column`.

use std::sync::Arc;

use crate::parser::ast::{QueryPath, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx};
use thin_vec::ThinVec;

pub struct PathBuilderOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    paths: &'a [Arc<QueryPath<Variable>>],
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> PathBuilderOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        paths: &'a [Arc<QueryPath<Variable>>],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            paths,
            idx,
        }
    }
}

impl<'a> Iterator for PathBuilderOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut batch = match self.child.next()? {
            Ok(b) => b,
            Err(e) => return Some(Err(e)),
        };

        for path in self.paths {
            let var_ids: Vec<u32> = path.vars.iter().map(|v| v.id).collect();

            // read_columns returns row-major: rows[row][var_index] = &Value
            let rows = batch.read_columns(&var_ids);

            let path_values: Result<Vec<Value>, String> = rows
                .iter()
                .zip(batch.active_indices())
                .map(|(row, row_idx)| {
                    let mut elems = ThinVec::new();
                    let mut skip_next = false;
                    for (i, val) in row.iter().enumerate() {
                        if skip_next {
                            skip_next = false;
                            continue;
                        }
                        let env = batch.env_ref(row_idx);
                        if !env.is_bound(&path.vars[i]) {
                            return Err(format!("Variable {} not found", path.vars[i].as_str()));
                        }
                        // Variable-length relationship: expand the list of
                        // relationships into alternating [rel, intermediate_node, rel, ...].
                        if let Value::List(edges) = val {
                            if !edges.is_empty() {
                                for edge in edges.iter() {
                                    // Determine the next node: whichever endpoint
                                    // differs from the preceding node in the path.
                                    // This handles incoming/bidirectional edges where
                                    // the stored edge direction may oppose the
                                    // traversal direction.
                                    let prev_id = elems.iter().rev().find_map(|v| {
                                        if let Value::Node(id) = v {
                                            Some(*id)
                                        } else {
                                            None
                                        }
                                    });
                                    elems.push(edge.clone());
                                    if let Value::Relationship(rel) = edge {
                                        let next =
                                            if prev_id == Some(rel.1) { rel.2 } else { rel.1 };
                                        elems.push(Value::Node(next));
                                    }
                                }
                            }
                            // 0-hop: skip the following endpoint node since it
                            // duplicates the preceding node already in elems.
                            // The last edge's destination is the same as the
                            // following endpoint node, so skip it.
                            skip_next = true;
                        } else {
                            elems.push((*val).clone());
                        }
                    }
                    Ok(Value::Path(Arc::new(elems)))
                })
                .collect();

            let path_values = match path_values {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };

            batch.write_column(path.var.id, path_values);
        }

        Some(Ok(batch))
    }
}
