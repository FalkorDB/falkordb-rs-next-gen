//! Batch-mode remove operator — removes properties and labels from nodes/relationships.
//!
//! For each active row in each input batch, evaluates the remove items and
//! records property nullifications or label removals in the pending batch.

use crate::parser::ast::{ExprIR, QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{Batch, BatchOp},
    env::Env,
    orderset::OrderSet,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct RemoveOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    items: &'a Vec<QueryExpr<Variable>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> RemoveOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        items: &'a Vec<QueryExpr<Variable>>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        runtime.pending.borrow_mut().resize(
            runtime.g.borrow().node_cap(),
            runtime.g.borrow().labels_count(),
        );
        Self {
            runtime,
            child,
            items,
            idx,
        }
    }
}

impl<'a> Iterator for RemoveOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let batch = match self.child.next()? {
            Ok(b) => b,
            Err(e) => return Some(Err(e)),
        };

        if let Err(e) = self.runtime.remove_batch(self.items, &batch) {
            return Some(Err(e));
        }

        Some(Ok(batch))
    }
}
impl Runtime<'_> {
    pub fn remove_batch(
        &self,
        items: &Vec<QueryExpr<Variable>>,
        batch: &Batch<'_>,
    ) -> Result<(), String> {
        for row in batch.active_indices() {
            let env = batch.env_ref(row);
            self.remove(items, env)?;
        }
        Ok(())
    }

    pub fn remove(
        &self,
        items: &Vec<QueryExpr<Variable>>,
        vars: &Env<'_>,
    ) -> Result<(), String> {
        for item in items {
            let (entity, property, labels) = match item.root().data() {
                ExprIR::Property(property) => (
                    self.run_expr(item, item.root().child(0).idx(), vars, None)?,
                    Some(property),
                    None,
                ),
                ExprIR::FuncInvocation(func) if func.name == "hasLabels" => {
                    let labels = item
                        .root()
                        .child(1)
                        .children()
                        .filter_map(|c| match c.data() {
                            ExprIR::String(label) => Some(label.clone()),
                            _ => None,
                        })
                        .collect::<OrderSet<_>>();

                    (
                        self.run_expr(item, item.root().child(0).idx(), vars, None)?,
                        None,
                        Some(labels),
                    )
                }
                _ => {
                    unreachable!();
                }
            };
            match entity {
                Value::Node(node) => {
                    if (self.g.borrow().is_node_deleted(node)
                        && !self.pending.borrow().is_node_created(node))
                        || self.pending.borrow().is_node_deleted(node)
                    {
                        continue;
                    }
                    if let Some(property) = property {
                        self.pending.borrow_mut().set_node_attribute(
                            node,
                            property.clone(),
                            Value::Null,
                        )?;
                    }
                    if let Some(labels) = labels {
                        let mut current_labels = self
                            .g
                            .borrow()
                            .get_node_label_ids(node)
                            .collect::<OrderSet<_>>();
                        self.pending
                            .borrow()
                            .update_node_labels(node, &mut current_labels);
                        let labels = labels
                            .iter()
                            .filter_map(|l| self.g.borrow_mut().get_label_id(l.as_str()))
                            .filter(|l| current_labels.contains(l))
                            .collect::<Vec<_>>();
                        self.pending.borrow_mut().remove_node_labels(node, &labels);
                    }
                }
                Value::Relationship(rel) => {
                    if let Some(property) = property {
                        self.pending.borrow_mut().set_relationship_attribute(
                            rel.0,
                            property.clone(),
                            Value::Null,
                        )?;
                    }
                    if labels.is_some() {
                        return Err(String::from(
                            "Type mismatch: expected Node but was Relationship",
                        ));
                    }
                }
                Value::Null => {}
                _ => {
                    return Err(format!(
                        "Type mismatch: expected Node or Relationship but was {}",
                        entity.name()
                    ));
                }
            }
        }
        Ok(())
    }
}
