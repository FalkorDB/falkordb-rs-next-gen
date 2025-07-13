use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use ordermap::{OrderMap, OrderSet};
use roaring::RoaringTreemap;

use crate::{
    graph::{Graph, NodeId, RelationshipId},
    runtime::QueryStatistics,
    value::Value,
};

pub struct PendingRelationship {
    pub from: NodeId,
    pub to: NodeId,
    pub type_name: Rc<String>,
}

impl PendingRelationship {
    #[must_use]
    pub const fn new(
        from: NodeId,
        to: NodeId,
        type_name: Rc<String>,
    ) -> Self {
        Self {
            from,
            to,
            type_name,
        }
    }
}

#[derive(Default)]
pub struct Pending {
    created_nodes: RoaringTreemap,
    created_relationships: HashMap<RelationshipId, PendingRelationship>,
    deleted_nodes: RoaringTreemap,
    deleted_relationships: HashSet<(RelationshipId, NodeId, NodeId)>,
    set_nodes_attrs: HashMap<NodeId, OrderMap<Rc<String>, Value>>,
    set_relationships_attrs: HashMap<RelationshipId, OrderMap<Rc<String>, Value>>,
    set_node_labels: HashMap<NodeId, OrderSet<Rc<String>>>,
    remove_node_labels: HashMap<NodeId, OrderSet<Rc<String>>>,
}

impl Pending {
    pub fn created_node(
        &mut self,
        id: NodeId,
    ) {
        self.created_nodes.insert(id.into());
    }

    pub fn set_node_attributes(
        &mut self,
        id: NodeId,
        attrs: OrderMap<Rc<String>, Value>,
    ) {
        self.set_nodes_attrs.insert(id, attrs);
    }

    pub fn set_node_attribute(
        &mut self,
        id: NodeId,
        key: Rc<String>,
        value: Value,
    ) {
        self.set_nodes_attrs
            .entry(id)
            .or_default()
            .insert(key, value);
    }

    #[must_use]
    pub fn get_node_attribute(
        &self,
        id: NodeId,
        key: &Rc<String>,
    ) -> Option<&Value> {
        self.set_nodes_attrs
            .get(&id)
            .and_then(|attrs| attrs.get(key))
    }

    pub fn update_node_attrs(
        &self,
        id: NodeId,
        attrs: &mut OrderMap<Rc<String>, Value>,
    ) {
        if let Some(added) = self.set_nodes_attrs.get(&id) {
            for (key, value) in added {
                if *value == Value::Null {
                    attrs.remove(key);
                } else {
                    attrs.insert(key.clone(), value.clone());
                }
            }
        }
    }

    pub fn set_node_labels(
        &mut self,
        id: NodeId,
        labels: OrderSet<Rc<String>>,
    ) {
        self.set_node_labels.insert(id, labels);
    }

    pub fn remove_node_labels(
        &mut self,
        id: NodeId,
        labels: OrderSet<Rc<String>>,
    ) {
        self.remove_node_labels.insert(id, labels);
    }

    pub fn update_node_labels(
        &self,
        id: NodeId,
        labels: &mut OrderSet<Rc<String>>,
    ) {
        if let Some(added) = self.set_node_labels.get(&id) {
            labels.extend(added.iter().cloned());
        }
        if let Some(removed) = self.remove_node_labels.get(&id) {
            for label in removed {
                labels.remove(label);
            }
        }
    }

    pub fn deleted_node(
        &mut self,
        id: NodeId,
    ) {
        self.deleted_nodes.insert(id.into());
    }

    pub fn created_relationship(
        &mut self,
        id: RelationshipId,
        from: NodeId,
        to: NodeId,
        type_name: Rc<String>,
    ) {
        self.created_relationships
            .insert(id, PendingRelationship::new(from, to, type_name));
    }

    pub fn set_relationship_attributes(
        &mut self,
        id: RelationshipId,
        attrs: OrderMap<Rc<String>, Value>,
    ) {
        self.set_relationships_attrs.insert(id, attrs);
    }

    pub fn set_relationship_attribute(
        &mut self,
        id: RelationshipId,
        key: Rc<String>,
        value: Value,
    ) {
        self.set_relationships_attrs
            .entry(id)
            .or_default()
            .insert(key, value);
    }

    #[must_use]
    pub fn get_relationship_attribute(
        &self,
        id: RelationshipId,
        key: &Rc<String>,
    ) -> Option<&Value> {
        self.set_relationships_attrs
            .get(&id)
            .and_then(|attrs| attrs.get(key))
    }

    pub fn update_relationship_attrs(
        &self,
        id: RelationshipId,
        attrs: &mut OrderMap<Rc<String>, Value>,
    ) {
        if let Some(added) = self.set_relationships_attrs.get(&id) {
            for (key, value) in added {
                if *value == Value::Null {
                    attrs.remove(key);
                } else {
                    attrs.insert(key.clone(), value.clone());
                }
            }
        }
    }

    pub fn deleted_relationship(
        &mut self,
        id: RelationshipId,
        from: NodeId,
        to: NodeId,
    ) {
        self.deleted_relationships.insert((id, from, to));
    }

    #[must_use]
    pub fn get_relationship_type(
        &self,
        id: RelationshipId,
    ) -> Option<Rc<String>> {
        self.created_relationships
            .get(&id)
            .map(|r| r.type_name.clone())
    }

    pub fn commit(
        &mut self,
        g: &RefCell<Graph>,
        stats: &RefCell<QueryStatistics>,
    ) {
        if !self.created_nodes.is_empty() {
            stats.borrow_mut().nodes_created += self.created_nodes.len() as usize;
            g.borrow_mut().create_nodes(&self.created_nodes);
            self.created_nodes.clear();
        }
        if !self.created_relationships.is_empty() {
            stats.borrow_mut().relationships_created += self.created_relationships.len();
            g.borrow_mut()
                .create_relationships(&self.created_relationships);
            self.created_relationships.clear();
        }
        if !self.deleted_relationships.is_empty() {
            stats.borrow_mut().relationships_deleted += self.deleted_relationships.len();
            for (id, src, dest) in self.deleted_relationships.clone() {
                g.borrow_mut().delete_relationship(id, src, dest);
            }
            self.deleted_relationships.clear();
        }
        if !self.deleted_nodes.is_empty() {
            stats.borrow_mut().nodes_deleted += self.deleted_nodes.len() as usize;
            for id in self.deleted_nodes.clone() {
                g.borrow_mut().delete_node(NodeId::from(id));
            }
            self.deleted_nodes.clear();
        }
        if !self.set_nodes_attrs.is_empty() {
            stats.borrow_mut().properties_set += self
                .set_nodes_attrs
                .values()
                .flat_map(|v| v.values())
                .map(|v| match *v {
                    Value::Null => 0,
                    _ => 1,
                })
                .sum::<usize>();
            for (id, attrs) in &self.set_nodes_attrs {
                for (key, value) in attrs {
                    let attr_id = g.borrow_mut().get_or_add_node_attribute_id(key);
                    if g.borrow_mut()
                        .set_node_attribute(*id, attr_id, value.clone())
                    {
                        stats.borrow_mut().properties_removed += 1;
                    }
                }
            }
            self.set_nodes_attrs.clear();
        }
        if !self.set_node_labels.is_empty() {
            for (id, labels) in &self.set_node_labels {
                g.borrow_mut().set_node_labels(*id, labels);
            }
            self.set_node_labels.clear();
        }
        if !self.remove_node_labels.is_empty() {
            for (id, labels) in &self.remove_node_labels {
                g.borrow_mut().remove_node_labels(*id, labels);
            }
            self.remove_node_labels.clear();
        }
        if !self.set_relationships_attrs.is_empty() {
            stats.borrow_mut().properties_set += self
                .set_relationships_attrs
                .values()
                .flat_map(|v| v.values())
                .map(|v| match *v {
                    Value::Null => 0,
                    _ => 1,
                })
                .sum::<usize>();
            for (id, attrs) in &self.set_relationships_attrs {
                for (key, value) in attrs {
                    let attr_id = g.borrow_mut().get_or_add_relationship_attribute_id(key);
                    if g.borrow_mut()
                        .set_relationship_attribute(*id, attr_id, value.clone())
                    {
                        stats.borrow_mut().properties_removed += 1;
                    }
                }
            }
            self.set_relationships_attrs.clear();
        }
    }
}
