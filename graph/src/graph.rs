use std::{
    collections::BTreeMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use orx_tree::DynTree;
use roaring::RoaringTreemap;

use crate::{
    ast::ExprIR,
    cypher::Parser,
    matrix::{self, Matrix, Remove, Set, Size},
    planner::{IR, Planner},
    tensor::{self, Tensor},
    value::Value,
};

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
    zero_tensor: Tensor,
    adjacancy_matrix: Tensor,
    node_labels_matrix: Matrix<bool>,
    relationship_type_matrix: Matrix<bool>,
    all_nodes_matrix: Matrix<bool>,
    labels_matices: BTreeMap<usize, Matrix<bool>>,
    relationship_matrices: BTreeMap<usize, Tensor>,
    node_properties_map: BTreeMap<u64, BTreeMap<u64, Value>>,
    relationship_properties_map: BTreeMap<u64, BTreeMap<u64, Value>>,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
    node_properties: Vec<String>,
    relationship_properties: Vec<String>,
    cache: Mutex<BTreeMap<String, DynTree<IR>>>,
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
            zero_tensor: Tensor::new(0, 0),
            adjacancy_matrix: Tensor::new(n, n),
            node_labels_matrix: Matrix::<bool>::new(0, 0),
            relationship_type_matrix: Matrix::<bool>::new(0, 0),
            all_nodes_matrix: Matrix::<bool>::new(n, n),
            labels_matices: BTreeMap::new(),
            relationship_matrices: BTreeMap::new(),
            node_properties_map: BTreeMap::new(),
            relationship_properties_map: BTreeMap::new(),
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
            node_properties: Vec::new(),
            relationship_properties: Vec::new(),
            cache: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn get_labels_count(&self) -> usize {
        self.node_labels.len()
    }

    pub fn get_labels(&self) -> impl Iterator<Item = &String> {
        self.node_labels.iter()
    }

    pub fn get_label_by_id(
        &self,
        id: usize,
    ) -> &String {
        &self.node_labels[id]
    }

    pub fn get_types(&self) -> impl Iterator<Item = &String> {
        self.relationship_types.iter()
    }

    pub fn get_properties(&self) -> impl Iterator<Item = &String> {
        self.node_properties
            .iter()
            .chain(self.relationship_properties.iter())
    }

    pub fn get_label_id(
        &self,
        label: &str,
    ) -> Option<u64> {
        self.node_labels
            .iter()
            .position(|l| l == label)
            .map(|p| p as u64)
    }

    pub fn get_type_id(
        &self,
        relationship_type: &str,
    ) -> Option<u64> {
        self.relationship_types
            .iter()
            .position(|t| t == relationship_type)
            .map(|p| p as u64)
    }

    pub fn get_plan(
        &self,
        query: &str,
    ) -> Result<
        (
            DynTree<IR>,
            BTreeMap<String, DynTree<ExprIR>>,
            Duration,
            Duration,
        ),
        String,
    > {
        let mut parse_duration = Duration::ZERO;
        let mut plan_duration = Duration::ZERO;

        let mut parser = Parser::new(query);
        let (parameters, query) = parser.parse_parameters()?;

        match self.cache.lock() {
            Ok(mut cache) => {
                if let Some(f) = cache.get(query) {
                    Ok((f.clone(), parameters, parse_duration, plan_duration))
                } else {
                    let start = Instant::now();
                    let ir = parser.parse()?;
                    parse_duration = start.elapsed();

                    let mut planner = Planner::new();
                    let start = Instant::now();
                    let value = planner.plan(ir);
                    plan_duration = start.elapsed();

                    cache.insert(query.to_string(), value.clone());
                    Ok((value, parameters, parse_duration, plan_duration))
                }
            }
            Err(_) => Err("Failed to acquire read lock on cache".to_string()),
        }
    }

    fn get_label_matrix(
        &self,
        label: &String,
    ) -> Option<&Matrix<bool>> {
        self.node_labels
            .iter()
            .position(|l| l == label)
            .map(|i| &self.labels_matices[&i])
    }

    fn get_label_matrix_mut(
        &mut self,
        label: &String,
    ) -> &mut Matrix<bool> {
        if !self.node_labels.contains(label) {
            self.node_labels.push(label.to_string());

            self.labels_matices.insert(
                self.node_labels.len() - 1,
                Matrix::<bool>::new(self.node_cap, self.node_cap),
            );
        }

        self.labels_matices
            .get_mut(&self.node_labels.iter().position(|l| l == label).unwrap())
            .unwrap()
    }

    fn get_relationship_matrix_mut(
        &mut self,
        relationship_type: &String,
    ) -> &mut Tensor {
        if !self.relationship_types.contains(relationship_type) {
            self.relationship_types.push(relationship_type.to_string());

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
                    .position(|l| l == relationship_type)
                    .unwrap(),
            )
            .unwrap()
    }

    fn get_relationship_matrix(
        &self,
        relationship_type: &String,
    ) -> Option<&Tensor> {
        if !self.relationship_types.contains(relationship_type) {
            return None;
        }

        self.relationship_matrices.get(
            &self
                .relationship_types
                .iter()
                .position(|l| l == relationship_type)
                .unwrap(),
        )
    }

    pub fn get_node_property_id(
        &self,
        key: &String,
    ) -> Option<u64> {
        self.node_properties
            .iter()
            .position(|p| p == key)
            .map(|property_id| property_id as u64)
    }

    pub fn get_or_add_node_property_id(
        &mut self,
        key: &String,
    ) -> u64 {
        let property_id = self
            .node_properties
            .iter()
            .position(|p| p == key)
            .unwrap_or_else(|| {
                let len = self.node_properties.len();
                self.node_properties.push(key.to_string());
                len
            });
        property_id as u64
    }

    pub fn get_or_add_relationship_property_id(
        &mut self,
        key: &String,
    ) -> u64 {
        let property_id = self
            .relationship_properties
            .iter()
            .position(|p| p == key)
            .unwrap_or_else(|| {
                let len = self.relationship_properties.len();
                self.relationship_properties.push(key.to_string());
                len
            });
        property_id as u64
    }

    pub fn get_relationship_property_id(
        &self,
        key: &String,
    ) -> Option<u64> {
        self.relationship_properties
            .iter()
            .position(|p| p == key)
            .map(|property_id| property_id as u64)
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
        nodes: &BTreeMap<u64, (Vec<String>, BTreeMap<String, Value>)>,
    ) {
        self.node_count += nodes.len() as u64;
        self.reserved_node_count -= nodes.len() as u64;

        for (id, _) in nodes {
            if self.deleted_nodes.is_empty() {
                break;
            }
            self.deleted_nodes.remove(*id);
        }

        self.resize();

        for (id, (labels, attrs)) in nodes {
            self.all_nodes_matrix.set(*id, *id, true);

            for label in labels {
                let label_matrix = self.get_label_matrix_mut(label);
                label_matrix.set(*id, *id, true);
                let label_id = self.get_label_id(label).unwrap();
                self.resize();
                self.node_labels_matrix.set(*id, label_id, true);
            }

            let mut map = BTreeMap::new();
            for (key, value) in attrs {
                if *value == Value::Null {
                    continue;
                }
                let property_id = self.get_or_add_node_property_id(key);
                map.insert(property_id, value.clone());
            }
            self.node_properties_map.insert(*id, map);
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

        self.node_properties_map.remove(&id);
    }

    pub fn get_node_relationships(
        &self,
        id: u64,
    ) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        self.relationship_matrices
            .values()
            .flat_map(move |m| m.iter(id, id))
    }

    pub fn get_nodes(
        &self,
        labels: &[String],
    ) -> matrix::Iter<bool> {
        if labels.is_empty() {
            return self.all_nodes_matrix.iter(0, u64::MAX);
        }
        self.get_label_matrix(&labels[0]).map_or_else(
            || self.zero_matrix.iter(0, u64::MAX),
            |m| m.iter(0, u64::MAX),
        )
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

    pub fn get_node_property(
        &self,
        node_id: u64,
        property_id: u64,
    ) -> Option<Value> {
        self.node_properties_map
            .get(&node_id)
            .unwrap()
            .get(&property_id)
            .cloned()
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
        relationships: &BTreeMap<u64, (String, u64, u64, BTreeMap<String, Value>)>,
    ) {
        self.relationship_count += relationships.len() as u64;
        self.reserved_relationship_count -= relationships.len() as u64;

        for (id, _) in relationships {
            if self.deleted_relationships.is_empty() {
                break;
            }
            self.deleted_relationships.remove(*id);
        }

        for (id, (relationship_type, from, to, _)) in relationships {
            let relationship_type_matrix = self.get_relationship_matrix_mut(relationship_type);
            relationship_type_matrix.set(*from, *to, *id);
        }

        self.resize();

        for (id, (relationship_type, src, dest, attrs)) in relationships {
            self.adjacancy_matrix.set(*src, *dest, *id);
            self.relationship_type_matrix.set(
                *id,
                self.relationship_types
                    .iter()
                    .position(|p| p == relationship_type)
                    .unwrap() as u64,
                true,
            );

            let mut map = BTreeMap::new();
            for (key, value) in attrs {
                if *value == Value::Null {
                    continue;
                }
                let property_id = self.get_or_add_relationship_property_id(key);
                map.insert(property_id, value.clone());
            }
            self.relationship_properties_map.insert(*id, map);
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

    pub fn get_relationships(
        &self,
        types: &[String],
    ) -> tensor::Iter {
        if types.is_empty() {
            return self.adjacancy_matrix.iter(0, u64::MAX);
        }
        self.get_relationship_matrix(&types[0]).map_or_else(
            || self.zero_tensor.iter(0, u64::MAX),
            |m| {
                m.wait();
                m.iter(0, u64::MAX)
            },
        )
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

    pub fn get_relationship_property(
        &self,
        relationship_id: u64,
        property_id: u64,
    ) -> Option<Value> {
        self.relationship_properties_map
            .get(&relationship_id)
            .unwrap()
            .get(&property_id)
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

    pub fn get_node_properties(
        &self,
        id: u64,
    ) -> &BTreeMap<u64, Value> {
        self.node_properties_map
            .get(&id)
            .unwrap_or_else(|| panic!("Node with id {id} not found"))
    }

    pub fn get_relationship_properties(
        &self,
        id: u64,
    ) -> &BTreeMap<u64, Value> {
        self.relationship_properties_map
            .get(&id)
            .unwrap_or_else(|| panic!("Relationship with id {id} not found"))
    }
}
