//! Batch-mode delete operator — marks nodes and relationships for deletion.
//!
//! For each active row in each input batch, evaluates the delete expressions
//! and records deletions in the pending batch. Node deletions cascade: all
//! connected relationships are also marked for deletion.

use crate::parser::ast::{ExprIR, QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    runtime::Runtime,
    value::{DeletedNode, DeletedRelationship, Value},
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct DeleteOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    trees: &'a Vec<QueryExpr<Variable>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> DeleteOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        trees: &'a Vec<QueryExpr<Variable>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            trees,
            idx,
        }
    }
}

impl<'a> Iterator for DeleteOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let batch = match self.child.next()? {
            Ok(b) => b,
            Err(e) => return Some(Err(e)),
        };

        if let Err(e) = self.runtime.delete_batch(self.trees, &batch) {
            return Some(Err(e));
        }

        Some(Ok(batch))
    }
}
impl Runtime<'_> {
    pub fn delete_batch(
        &self,
        trees: &Vec<QueryExpr<Variable>>,
        batch: &Batch<'_>,
    ) -> Result<(), String> {
        // Partition trees: collect var IDs for simple variable references (fast path),
        // and keep references to non-variable trees (slow path).
        let mut var_ids = Vec::new();
        let mut expr_trees = Vec::new();

        for tree in trees {
            match tree.root().data() {
                ExprIR::Variable(var) => var_ids.push(var.id),
                _ => expr_trees.push(tree),
            }
        }

        // Fast path: read all simple variable columns at once, no env needed
        if !var_ids.is_empty() {
            let rows = batch.read_columns(&var_ids);
            for row in rows {
                for val in row {
                    self.delete_entity(val)?;
                }
            }
        }

        // Slow path: evaluate remaining expression trees via env_ref
        if !expr_trees.is_empty() {
            for row in batch.active_indices() {
                let env = batch.env_ref(row);
                for tree in &expr_trees {
                    let value = self.run_expr(tree, tree.root().idx(), env, None)?;
                    self.delete_entity(&value)?;
                }
            }
        }

        Ok(())
    }

    pub fn delete_entity(
        &self,
        value: &Value,
    ) -> Result<(), String> {
        match value {
            Value::Node(id) => {
                let id = *id;
                if !self.g.borrow().is_node_deleted(id) {
                    for (src, dest, id) in self.g.borrow().get_node_relationships(id) {
                        self.pending
                            .borrow_mut()
                            .deleted_relationship(id, src, dest);
                    }
                    self.pending.borrow_mut().deleted_node(id);
                    let labels = self.g.borrow().get_node_label_ids(id).collect();
                    let attrs = self.get_node_attrs(id).collect();
                    self.deleted_nodes
                        .borrow_mut()
                        .insert(id, DeletedNode::new(labels, attrs));
                }
            }
            Value::Relationship(rel) => {
                let (rel_id, src, dest) = **rel;
                if !self.g.borrow().is_relationship_deleted(rel_id) {
                    self.pending
                        .borrow_mut()
                        .deleted_relationship(rel_id, src, dest);
                    let type_id = self.g.borrow().get_relationship_type_id(rel_id);
                    let attrs = self.get_relationship_attrs(rel_id).collect();
                    self.deleted_relationships
                        .borrow_mut()
                        .insert(rel_id, DeletedRelationship::new(type_id, attrs));
                }
            }
            Value::Path(values) => {
                for value in values.iter() {
                    self.delete_entity(value)?;
                }
            }
            Value::Null => {}
            _ => {
                return Err(String::from(
                    "Delete type mismatch, expecting either Node or Relationship.",
                ));
            }
        }
        Ok(())
    }
}
