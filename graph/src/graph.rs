use std::{
    rc::Rc,
    sync::Mutex,
    time::{Duration, Instant},
};

use hashbrown::HashMap;
use ordermap::{OrderMap, OrderSet};
use orx_tree::DynTree;
use roaring::RoaringTreemap;

use crate::{
    ast::ExprIR,
    cypher::Parser,
    matrix::{self, Dup, ElementWiseAdd, ElementWiseMultiply, Matrix, MxM, New, Remove, Set, Size},
    planner::{IR, Planner},
    runtime::PendingRelationship,
    tensor::Tensor,
    value::{RcValue, Value},
};

pub struct Plan {
    pub plan: Rc<DynTree<IR>>,
    pub parameters: HashMap<String, DynTree<ExprIR>>,
    pub parse_duration: Duration,
    pub plan_duration: Duration,
}

impl Plan {
    #[must_use]
    pub const fn new(
        plan: Rc<DynTree<IR>>,
        parameters: HashMap<String, DynTree<ExprIR>>,
        parse_duration: Duration,
        plan_duration: Duration,
    ) -> Self {
        Self {
            plan,
            parameters,
            parse_duration,
            plan_duration,
        }
    }
}

pub struct Graph {
    node_cap: u64,
    relationship_cap: u64,
    reserved_node_count: u64,
    reserved_relationship_count: u64,
    node_count: u64,
    relationship_count: u64,
    deleted_nodes: RoaringTreemap,
    deleted_relationships: RoaringTreemap,
    zero_matrix: Matrix<bool>,
    adjacancy_matrix: Matrix<bool>,
    node_labels_matrix: Matrix<bool>,
    relationship_type_matrix: Matrix<bool>,
    all_nodes_matrix: Matrix<bool>,
    labels_matices: HashMap<usize, Matrix<bool>>,
    relationship_matrices: HashMap<usize, Tensor>,
    empty_map: OrderMap<u64, RcValue>,
    node_attrs: HashMap<u64, OrderMap<u64, RcValue>>,
    relationship_attrs: HashMap<u64, OrderMap<u64, RcValue>>,
    node_labels: Vec<Rc<String>>,
    relationship_types: Vec<Rc<String>>,
    node_attrs_name: Vec<Rc<String>>,
    relationship_attrs_name: Vec<Rc<String>>,
    cache: Mutex<HashMap<String, Rc<DynTree<IR>>>>,
}

impl Graph {
    #[must_use]
    pub fn new(
        n: u64,
        e: u64,
    ) -> Self {
        Self {
            node_cap: n,
            relationship_cap: e,
            reserved_node_count: 0,
            reserved_relationship_count: 0,
            node_count: 0,
            relationship_count: 0,
            deleted_nodes: RoaringTreemap::new(),
            deleted_relationships: RoaringTreemap::new(),
            zero_matrix: Matrix::<bool>::new(0, 0),
            adjacancy_matrix: Matrix::<bool>::new(n, n),
            node_labels_matrix: Matrix::<bool>::new(0, 0),
            relationship_type_matrix: Matrix::<bool>::new(0, 0),
            all_nodes_matrix: Matrix::<bool>::new(n, n),
            labels_matices: HashMap::new(),
            relationship_matrices: HashMap::new(),
            empty_map: OrderMap::new(),
            node_attrs: HashMap::new(),
            relationship_attrs: HashMap::new(),
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
            node_attrs_name: Vec::new(),
            relationship_attrs_name: Vec::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub const fn get_labels_count(&self) -> usize {
        self.node_labels.len()
    }

    pub fn get_labels(&self) -> impl Iterator<Item = &Rc<String>> {
        self.node_labels.iter()
    }

    pub fn get_label_by_id(
        &self,
        id: usize,
    ) -> Rc<String> {
        self.node_labels[id].clone()
    }

    pub fn get_types(&self) -> impl Iterator<Item = &Rc<String>> {
        self.relationship_types.iter()
    }

    pub fn get_attrs(&self) -> impl Iterator<Item = &Rc<String>> {
        self.node_attrs_name
            .iter()
            .chain(self.relationship_attrs_name.iter())
    }

    pub fn get_label_id(
        &self,
        label: &str,
    ) -> Option<u64> {
        self.node_labels
            .iter()
            .position(|l| l.as_str() == label)
            .map(|p| p as u64)
    }

    pub fn get_type_id(
        &self,
        relationship_type: &str,
    ) -> Option<u64> {
        self.relationship_types
            .iter()
            .position(|t| t.as_str() == relationship_type)
            .map(|p| p as u64)
    }

    pub fn get_plan(
        &self,
        query: &str,
    ) -> Result<Plan, String> {
        let mut parse_duration = Duration::ZERO;
        let mut plan_duration = Duration::ZERO;

        let mut parser = Parser::new(query);
        let (parameters, query) = parser.parse_parameters()?;

        match self.cache.lock() {
            Ok(mut cache) => {
                if let Some(f) = cache.get(query) {
                    Ok(Plan::new(
                        f.clone(),
                        parameters,
                        parse_duration,
                        plan_duration,
                    ))
                } else {
                    let start = Instant::now();
                    let ir = parser.parse()?;
                    parse_duration = start.elapsed();

                    let planner = Planner::new();
                    let start = Instant::now();
                    let value = Rc::new(planner.plan(ir));
                    plan_duration = start.elapsed();

                    cache.insert(query.to_string(), value.clone());
                    Ok(Plan::new(value, parameters, parse_duration, plan_duration))
                }
            }
            Err(_) => Err("Failed to acquire read lock on cache".to_string()),
        }
    }

    fn get_label_matrix(
        &self,
        label: &str,
    ) -> Option<&Matrix<bool>> {
        self.node_labels
            .iter()
            .position(|l| l.as_str() == label)
            .map(|i| &self.labels_matices[&i])
    }

    fn get_label_matrix_mut(
        &mut self,
        label: &Rc<String>,
    ) -> &mut Matrix<bool> {
        if !self.node_labels.contains(label) {
            self.node_labels.push(label.clone());

            self.labels_matices.insert(
                self.node_labels.len() - 1,
                Matrix::<bool>::new(self.node_cap, self.node_cap),
            );
        }

        self.labels_matices
            .get_mut(
                &self
                    .node_labels
                    .iter()
                    .position(|l| l.as_str() == label.as_str())
                    .unwrap(),
            )
            .unwrap()
    }

    fn get_relationship_matrix_mut(
        &mut self,
        relationship_type: &Rc<String>,
    ) -> &mut Tensor {
        if !self.relationship_types.contains(relationship_type) {
            self.relationship_types.push(relationship_type.clone());

            self.relationship_matrices.insert(
                self.relationship_types.len() - 1,
                Tensor::new(self.node_cap, self.node_cap),
            );
        }

        self.relationship_matrices
            .get_mut(
                &self
                    .relationship_types
                    .iter()
                    .position(|l| l.as_str() == relationship_type.as_str())
                    .unwrap(),
            )
            .unwrap()
    }

    fn get_relationship_matrix(
        &self,
        relationship_type: &Rc<String>,
    ) -> Option<&Tensor> {
        if !self.relationship_types.contains(relationship_type) {
            return None;
        }

        self.relationship_matrices.get(
            &self
                .relationship_types
                .iter()
                .position(|l| l.as_str() == relationship_type.as_str())
                .unwrap(),
        )
    }

    pub fn get_node_attribute_id(
        &self,
        key: &str,
    ) -> Option<u64> {
        self.node_attrs_name
            .iter()
            .position(|p| p.as_str() == key)
            .map(|attr_id| attr_id as u64)
    }

    pub fn get_node_attribute_string(
        &self,
        id: u64,
    ) -> Option<Rc<String>> {
        self.node_attrs_name.get(id as usize).cloned()
    }

    pub fn get_or_add_node_attribute_id(
        &mut self,
        key: &Rc<String>,
    ) -> u64 {
        let attr_id = self
            .node_attrs_name
            .iter()
            .position(|p| p.as_str() == key.as_str())
            .unwrap_or_else(|| {
                let len = self.node_attrs_name.len();
                self.node_attrs_name.push(key.clone());
                len
            });
        attr_id as u64
    }

    pub fn get_or_add_relationship_attribute_id(
        &mut self,
        key: &String,
    ) -> u64 {
        let attr_id = self
            .relationship_attrs_name
            .iter()
            .position(|p| p.as_str() == key)
            .unwrap_or_else(|| {
                let len = self.relationship_attrs_name.len();
                self.relationship_attrs_name.push(Rc::new(key.clone()));
                len
            });
        attr_id as u64
    }

    pub fn get_relationship_attribute_id(
        &self,
        key: &String,
    ) -> Option<u64> {
        self.relationship_attrs_name
            .iter()
            .position(|p| p.as_str() == key)
            .map(|attr_id| attr_id as u64)
    }

    pub fn get_relationship_attribute_string(
        &self,
        id: u64,
    ) -> Option<Rc<String>> {
        self.relationship_attrs_name.get(id as usize).cloned()
    }

    pub fn reserve_node(&mut self) -> u64 {
        let mut iter = self.deleted_nodes.iter();
        iter.advance_to(self.reserved_node_count);
        if let Some(id) = iter.next() {
            self.reserved_node_count += 1;
            return id;
        }
        self.reserved_node_count += 1;
        self.node_count + self.reserved_node_count - 1
    }

    pub fn create_nodes(
        &mut self,
        nodes: &Vec<u64>,
    ) {
        self.node_count += nodes.len() as u64;
        self.reserved_node_count -= nodes.len() as u64;

        for id in nodes {
            if self.deleted_nodes.is_empty() {
                break;
            }
            self.deleted_nodes.remove(*id);
        }

        self.resize();

        for id in nodes {
            self.all_nodes_matrix.set(*id, *id, true);
        }
    }

    pub fn set_node_attribute(
        &mut self,
        id: u64,
        attr_id: u64,
        value: RcValue,
    ) -> bool {
        let attrs = self.node_attrs.entry(id).or_default();
        if *value == Value::Null {
            attrs.remove(&attr_id).is_some()
        } else {
            attrs.insert(attr_id, value).is_some()
        }
    }

    pub fn set_node_labels(
        &mut self,
        id: u64,
        labels: &OrderSet<Rc<String>>,
    ) {
        for label in labels {
            let label_matrix = self.get_label_matrix_mut(label);
            label_matrix.set(id, id, true);
            let label_id = self.get_label_id(label).unwrap();
            self.resize();
            self.node_labels_matrix.set(id, label_id, true);
        }
    }

    pub fn remove_node_labels(
        &mut self,
        id: u64,
        labels: &OrderSet<Rc<String>>,
    ) {
        for label in labels {
            if !self.node_labels.contains(label) {
                continue;
            }
            let label_matrix = self.get_label_matrix_mut(label);
            label_matrix.set(id, id, true);
            let label_id = self.get_label_id(label).unwrap();
            self.node_labels_matrix.remove(id, label_id);
        }
    }

    pub fn delete_node(
        &mut self,
        id: u64,
    ) {
        self.deleted_nodes.insert(id);
        self.node_count -= 1;
        self.all_nodes_matrix.remove(id, id);

        for (label_id, label_matrix) in &mut self.labels_matices {
            label_matrix.remove(id, id);
            self.node_labels_matrix.remove(id, *label_id as _);
        }

        self.node_attrs.remove(&id);
    }

    pub fn get_node_relationships(
        &self,
        id: u64,
    ) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.relationship_matrices
            .values()
            .flat_map(move |m| m.iter(id, id).chain(m.transpose().iter(id, id)))
    }

    pub fn get_nodes(
        &self,
        labels: &OrderSet<Rc<String>>,
    ) -> matrix::Iter<bool> {
        if labels.is_empty() {
            return self.all_nodes_matrix.iter(0, u64::MAX);
        }
        let mut iter = labels.iter();
        let mut m = if let Some(label_matrix) = self.get_label_matrix(iter.next().unwrap()) {
            label_matrix.dup()
        } else {
            return self.zero_matrix.iter(0, u64::MAX);
        };
        for label in iter {
            if let Some(label_matrix) = self.get_label_matrix(label) {
                m.element_wise_multiply(label_matrix);
            } else {
                return self.zero_matrix.iter(0, u64::MAX);
            }
        }
        m.iter(0, u64::MAX)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn get_node_label_ids(
        &self,
        id: u64,
    ) -> impl Iterator<Item = usize> {
        self.node_labels_matrix
            .iter(id, id)
            .map(|(_, l)| l as usize)
    }

    pub fn get_node_labels(
        &self,
        id: u64,
    ) -> impl Iterator<Item = Rc<String>> {
        self.get_node_label_ids(id)
            .map(move |label_id| self.node_labels[label_id].clone())
    }

    pub fn get_node_attribute(
        &self,
        node_id: u64,
        attr_id: u64,
    ) -> Option<RcValue> {
        self.node_attrs
            .get(&node_id)
            .map_or_else(|| None, |attrs| attrs.get(&attr_id).cloned())
    }

    pub fn reserve_relationship(&mut self) -> u64 {
        let mut iter = self.deleted_relationships.iter();
        iter.advance_to(self.reserved_relationship_count);
        if let Some(id) = iter.next() {
            self.reserved_relationship_count += 1;
            return id;
        }
        self.reserved_relationship_count += 1;
        self.relationship_count + self.reserved_relationship_count - 1
    }

    pub fn create_relationships(
        &mut self,
        relationships: &HashMap<u64, PendingRelationship>,
    ) {
        self.relationship_count += relationships.len() as u64;
        self.reserved_relationship_count -= relationships.len() as u64;

        for (id, _) in relationships {
            if self.deleted_relationships.is_empty() {
                break;
            }
            self.deleted_relationships.remove(*id);
        }

        for (
            id,
            PendingRelationship {
                type_name,
                from: start,
                to: end,
                ..
            },
        ) in relationships
        {
            let relationship_type_matrix = self.get_relationship_matrix_mut(type_name);
            relationship_type_matrix.set(*start, *end, *id);
        }

        self.resize();

        for (
            id,
            PendingRelationship {
                type_name,
                from: start,
                to: end,
            },
        ) in relationships
        {
            self.adjacancy_matrix.set(*start, *end, true);
            self.relationship_type_matrix.set(
                *id,
                self.relationship_types
                    .iter()
                    .position(|p| p.as_str() == type_name.as_str())
                    .unwrap() as u64,
                true,
            );
        }
    }

    pub fn set_relationship_attribute(
        &mut self,
        id: u64,
        attr_id: u64,
        value: RcValue,
    ) -> bool {
        let attrs = self.relationship_attrs.entry(id).or_default();
        if *value == Value::Null {
            attrs.remove(&attr_id).is_some()
        } else {
            attrs.insert(attr_id, value).is_some()
        }
    }

    pub fn delete_relationship(
        &mut self,
        id: u64,
        src: u64,
        dest: u64,
    ) {
        self.deleted_relationships.insert(id);
        self.relationship_count -= 1;
        self.relationship_matrices
            .values_mut()
            .for_each(|m| m.remove(src, dest, id));
    }

    pub fn get_src_dest_relationships(
        &self,
        src: u64,
        dest: u64,
        types: &[Rc<String>],
    ) -> Vec<u64> {
        let mut vec = vec![];
        for relationship_type in if types.is_empty() {
            &self.relationship_types
        } else {
            types
        } {
            if let Some(relationship_matrix) = self.get_relationship_matrix(relationship_type) {
                if let Some(id) = relationship_matrix.get(src, dest) {
                    vec.push(id);
                }
            }
        }
        vec
    }

    pub fn get_relationships(
        &self,
        types: &[Rc<String>],
        src_lables: &OrderSet<Rc<String>>,
        dest_labels: &OrderSet<Rc<String>>,
    ) -> matrix::Iter<bool> {
        let mut iter = types.iter();
        let mut m = if let Some(relationship_type) = iter.next() {
            if let Some(relationship_matrix) = self.get_relationship_matrix(relationship_type) {
                relationship_matrix.dup_bool()
            } else {
                return self.zero_matrix.iter(0, u64::MAX);
            }
        } else {
            self.adjacancy_matrix.dup()
        };
        for relationship_type in iter {
            if let Some(relationship_matrix) = self.get_relationship_matrix(relationship_type) {
                m.element_wise_add(&relationship_matrix.dup_bool());
            } else {
                return self.zero_matrix.iter(0, u64::MAX);
            }
        }
        if !src_lables.is_empty() {
            let mut iter = src_lables.iter();
            let mut src_matrix =
                if let Some(label_matrix) = self.get_label_matrix(iter.next().unwrap()) {
                    label_matrix.dup()
                } else {
                    return self.zero_matrix.iter(0, u64::MAX);
                };
            for label in iter {
                if let Some(label_matrix) = self.get_label_matrix(label) {
                    src_matrix.element_wise_multiply(label_matrix);
                } else {
                    return self.zero_matrix.iter(0, u64::MAX);
                }
            }
            m.rmxm(&src_matrix);
        }
        if !dest_labels.is_empty() {
            let mut iter = dest_labels.iter();
            let mut dest_matrix =
                if let Some(label_matrix) = self.get_label_matrix(iter.next().unwrap()) {
                    label_matrix.dup()
                } else {
                    return self.zero_matrix.iter(0, u64::MAX);
                };
            for label in iter {
                if let Some(label_matrix) = self.get_label_matrix(label) {
                    dest_matrix.element_wise_multiply(label_matrix);
                } else {
                    return self.zero_matrix.iter(0, u64::MAX);
                }
            }
            m.lmxm(&dest_matrix);
        }
        m.iter(0, u64::MAX)
    }

    pub fn get_relationship_type_id(
        &self,
        id: u64,
    ) -> u64 {
        self.relationship_type_matrix
            .iter(id, id)
            .map(|(_, l)| l)
            .next()
            .unwrap()
    }

    pub fn get_relationship_attribute(
        &self,
        relationship_id: u64,
        attr_id: u64,
    ) -> Option<RcValue> {
        self.relationship_attrs
            .get(&relationship_id)
            .unwrap()
            .get(&attr_id)
            .cloned()
    }

    fn resize(&mut self) {
        if self.node_count > self.node_cap {
            while self.node_count > self.node_cap {
                self.node_cap *= 2;
            }
            self.adjacancy_matrix.resize(self.node_cap, self.node_cap);
            self.node_labels_matrix
                .resize(self.node_cap, self.labels_matices.len() as u64);
            self.all_nodes_matrix.resize(self.node_cap, self.node_cap);
            for label_matrix in self.labels_matices.iter_mut().map(|(_, m)| m) {
                label_matrix.resize(self.node_cap, self.node_cap);
            }
            for relationship_matrix in self.relationship_matrices.iter_mut().map(|(_, m)| m) {
                relationship_matrix.resize(self.node_cap, self.node_cap);
            }
        }

        if self.labels_matices.len() as u64 > self.node_labels_matrix.ncols() {
            self.node_labels_matrix
                .resize(self.node_cap, self.labels_matices.len() as u64);
        }

        if self.relationship_count > self.relationship_cap {
            while self.relationship_count > self.relationship_cap {
                self.relationship_cap *= 2;
            }
            self.relationship_type_matrix
                .resize(self.relationship_cap, self.relationship_types.len() as u64);
        }

        if self.relationship_types.len() as u64 > self.relationship_type_matrix.ncols() {
            self.relationship_type_matrix
                .resize(self.relationship_cap, self.relationship_types.len() as u64);
        }
    }

    pub fn get_node_attrs(
        &self,
        id: u64,
    ) -> &OrderMap<u64, RcValue> {
        self.node_attrs.get(&id).unwrap_or(&self.empty_map)
    }

    pub fn get_relationship_attrs(
        &self,
        id: u64,
    ) -> &OrderMap<u64, RcValue> {
        self.relationship_attrs.get(&id).unwrap_or(&self.empty_map)
    }
}
