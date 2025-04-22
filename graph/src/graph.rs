use std::{
    collections::BTreeMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use roaring::RoaringTreemap;

use crate::{
    cypher::Parser,
    matrix::{self, Matrix, Remove, Set, Size},
    planner::{Planner, IR},
    runtime::{evaluate_param, ro_run, run, Runtime, Value},
    tensor::{self, Tensor},
};

pub struct Graph {
    node_cap: u64,
    relationship_cap: u64,
    node_count: u64,
    relationship_count: u64,
    deleted_nodes: RoaringTreemap,
    deleted_relationships: RoaringTreemap,
    adjacancy_matrix: Matrix<bool>,
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
    cache: Mutex<BTreeMap<String, IR>>,
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
            node_count: 0,
            relationship_count: 0,
            deleted_nodes: RoaringTreemap::new(),
            deleted_relationships: RoaringTreemap::new(),
            adjacancy_matrix: Matrix::<bool>::new(0, 0),
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

    pub fn get_labels(&self) -> impl Iterator<Item = &String> {
        self.node_labels.iter()
    }

    pub fn get_label_by_id(
        &self,
        id: u64,
    ) -> &String {
        &self.node_labels[id as usize]
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

    pub fn query(
        &mut self,
        query: &str,
        result_fn: &mut dyn FnMut(&Self, Value),
        debug: bool,
    ) -> Result<ResultSummary, String> {
        let mut parse_duration = Duration::ZERO;
        let mut plan_duration = Duration::ZERO;

        let mut parser = Parser::new(query);
        let (parameters, query) = parser.parse_parameters()?;

        let evaluate = {
            match self.cache.lock() {
                Ok(mut cache) => {
                    if let Some(f) = cache.get(query) {
                        f.to_owned()
                    } else {
                        let start = Instant::now();
                        let ir = parser.parse()?;
                        parse_duration = start.elapsed();

                        let mut planner = Planner::new();
                        let start = Instant::now();
                        let value = planner.plan(ir, debug);
                        plan_duration = start.elapsed();

                        cache.insert(query.to_string(), value.clone());
                        value
                    }
                }
                Err(_) => {
                    return Err("Failed to acquire read lock on cache".to_string());
                }
            }
        };

        let labels_count = self.node_labels.len();
        let mut runtime = Runtime::new(
            parameters
                .into_iter()
                .map(|(k, v)| (k, evaluate_param(v)))
                .collect(),
        );
        let start = Instant::now();
        run(
            &mut BTreeMap::new(),
            self,
            &mut runtime,
            result_fn,
            &evaluate,
        )?;
        let run_duration = start.elapsed();

        Ok(ResultSummary {
            parse_duration,
            plan_duration,
            run_duration,
            labels_added: self.node_labels.len() as i32 - labels_count as i32,
            labels_removed: 0,
            nodes_created: runtime.nodes_created,
            relationships_created: runtime.relationships_created,
            nodes_deleted: runtime.nodes_deleted,
            relationships_deleted: runtime.relationships_deleted,
            properties_set: runtime.properties_set,
            properties_removed: runtime.properties_removed,
        })
    }

    pub fn ro_query(
        &self,
        query: &str,
        result_fn: &mut dyn FnMut(&Self, Value),
        debug: bool,
    ) -> Result<ResultSummary, String> {
        let mut parse_duration = Duration::ZERO;
        let mut plan_duration = Duration::ZERO;

        let mut parser = Parser::new(query);
        let (parameters, query) = parser.parse_parameters()?;

        let evaluate = {
            match self.cache.lock() {
                Ok(mut cache) => {
                    if let Some(f) = cache.get(query) {
                        f.to_owned()
                    } else {
                        let start = Instant::now();
                        let ir = parser.parse()?;
                        parse_duration = start.elapsed();

                        let mut planner = Planner::new();
                        let start = Instant::now();
                        let value = planner.plan(ir, debug);
                        plan_duration = start.elapsed();

                        cache.insert(query.to_string(), value.clone());
                        value
                    }
                }
                Err(_) => {
                    return Err("Failed to acquire read lock on cache".to_string());
                }
            }
        };

        let mut runtime = Runtime::new(
            parameters
                .into_iter()
                .map(|(k, v)| (k, evaluate_param(v)))
                .collect(),
        );
        let start = Instant::now();
        ro_run(
            &mut BTreeMap::new(),
            self,
            &mut runtime,
            result_fn,
            &evaluate,
        )?;
        let run_duration = start.elapsed();

        Ok(ResultSummary {
            parse_duration,
            plan_duration,
            run_duration,
            labels_added: 0,
            labels_removed: 0,
            nodes_created: 0,
            relationships_created: 0,
            nodes_deleted: 0,
            relationships_deleted: 0,
            properties_set: 0,
            properties_removed: 0,
        })
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

    fn get_relationship_property_id(
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

    pub fn create_node(
        &mut self,
        labels: &Vec<String>,
        attrs: BTreeMap<String, Value>,
    ) -> Value {
        let id = self.deleted_nodes.min().unwrap_or(self.node_count);
        self.deleted_nodes.remove(id);
        self.node_count += 1;

        self.resize();

        self.all_nodes_matrix.set(id, id, true);

        for label in labels {
            let label_matrix = self.get_label_matrix_mut(label);
            label_matrix.set(id, id, true);
            let label_id = self.get_label_id(label).unwrap();
            self.resize();
            self.node_labels_matrix.set(id, label_id, true);
        }

        let mut map = BTreeMap::new();
        for (key, value) in attrs {
            if value == Value::Null {
                continue;
            }
            let property_id = self.get_or_add_node_property_id(&key);
            map.insert(property_id, value);
        }
        self.node_properties_map.insert(id, map);

        Value::Node(id)
    }

    pub fn delete_node(
        &mut self,
        id: u64,
    ) {
        self.deleted_nodes.insert(id);
        self.node_count -= 1;
        self.all_nodes_matrix.remove(id, id);

        for (label_id, label_matrix) in self.labels_matices.iter_mut() {
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
    ) -> Option<matrix::Iter<bool>> {
        if labels.is_empty() {
            return Some(self.all_nodes_matrix.iter(0, u64::MAX));
        }
        self.get_label_matrix(&labels[0])
            .map(|m| m.iter(0, u64::MAX))
    }

    pub fn get_node_label_ids(
        &self,
        id: u64,
    ) -> impl Iterator<Item = u64> {
        self.node_labels_matrix.iter(id, id).map(|(_, l)| l)
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

    pub fn create_relationship(
        &mut self,
        relationship_type: &String,
        from: u64,
        to: u64,
        attrs: BTreeMap<String, Value>,
    ) -> Value {
        let id = self
            .deleted_relationships
            .min()
            .unwrap_or(self.relationship_count);
        self.deleted_relationships.remove(id);
        self.relationship_count += 1;

        self.resize();

        let relationship_type_matrix = self.get_relationship_matrix_mut(relationship_type);
        relationship_type_matrix.set(from, to, id);

        self.resize();

        self.relationship_type_matrix.set(
            id,
            self.relationship_types
                .iter()
                .position(|p| p == relationship_type)
                .unwrap() as u64,
            true,
        );

        let mut map = BTreeMap::new();
        for (key, value) in attrs {
            if value == Value::Null {
                continue;
            }
            let property_id = self.get_relationship_property_id(&key);
            map.insert(property_id, value);
        }
        self.relationship_properties_map.insert(id, map);

        Value::Relationship(id, from, to)
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
    ) -> Option<tensor::Iter> {
        if types.is_empty() {
            return None;
        }
        self.get_relationship_matrix(&types[0]).map(|m| {
            m.wait();
            m.iter(0, u64::MAX)
        })
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

    fn resize(&mut self) {
        if self.node_count > self.node_cap {
            self.node_cap *= 2;
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
            self.relationship_cap *= 2;
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

pub struct ResultSummary {
    pub parse_duration: Duration,
    pub plan_duration: Duration,
    pub run_duration: Duration,
    pub labels_added: i32,
    pub labels_removed: i32,
    pub nodes_created: i32,
    pub relationships_created: i32,
    pub nodes_deleted: i32,
    pub relationships_deleted: i32,
    pub properties_set: i32,
    pub properties_removed: i32,
}
