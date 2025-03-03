use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use roaring::RoaringTreemap;

use crate::{
    matrix::{Delete, Get, Iter, Matrix, Row, Set, Size},
    parser::Parser,
    runtime::{run, Runtime, IR},
    value::Value,
};

pub struct Graph {
    node_cap: u64,
    link_cap: u64,
    node_count: u64,
    link_count: u64,
    deleted_nodes: RoaringTreemap,
    deleted_links: RoaringTreemap,
    adj: Matrix<bool>,
    node_labels: Matrix<bool>,
    all_nodes: Matrix<bool>,
    labels: Vec<(String, Matrix<bool>)>,
    links: Vec<(String, Matrix<bool>)>,
    node_properties: Matrix<Value>,
    link_properties: Matrix<Value>,
    node_property_ids: Vec<String>,
    link_property_ids: Vec<String>,
    pub runtime: Runtime,
    cache: Mutex<HashMap<String, IR>>,
}

impl Graph {
    #[must_use]
    pub fn new(n: u64, e: u64) -> Self {
        Self {
            node_cap: n,
            link_cap: e,
            node_count: 0,
            link_count: 0,
            deleted_nodes: RoaringTreemap::new(),
            deleted_links: RoaringTreemap::new(),
            adj: Matrix::<bool>::new(0, 0),
            node_labels: Matrix::<bool>::new(0, 0),
            all_nodes: Matrix::<bool>::new(n, n),
            labels: Vec::new(),
            links: Vec::new(),
            node_properties: Matrix::<Value>::new(0, 0),
            link_properties: Matrix::<Value>::new(0, 0),
            node_property_ids: Vec::new(),
            link_property_ids: Vec::new(),
            runtime: Runtime::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn init() {
        crate::matrix::init();
    }

    pub fn shutdown() {
        crate::matrix::shutdown();
    }

    pub fn get_label_id(&self, label: &str) -> Option<u64> {
        self.labels
            .iter()
            .position(|p| p.0 == label)
            .map(|p| p as u64)
    }

    pub fn get_link_id(&self, link_type: &str) -> Option<u64> {
        self.links
            .iter()
            .position(|p| p.0 == link_type)
            .map(|p| p as u64)
    }

    pub fn query(
        &mut self,
        query: &str,
        result_fn: &mut dyn FnMut(&mut Self, Value),
        debug: bool,
    ) -> Result<ResultSummary, String> {
        let mut parse_duration = Duration::ZERO;
        let mut jit_duration = Duration::ZERO;

        let evaluate = {
            match self.cache.lock() {
                Ok(mut cache) => {
                    if let Some(f) = cache.get(query) {
                        f.to_owned()
                    } else {
                        let mut parser = Parser::new(query);
                        let start = Instant::now();
                        let ir = parser.parse()?;
                        parse_duration = start.elapsed();

                        let start = Instant::now();
                        let value = self.runtime.plan(&ir, debug);
                        jit_duration = start.elapsed();

                        cache.insert(query.to_string(), value.clone());
                        value
                    }
                }
                Err(_) => {
                    return Err("Failed to acquire read lock on cache".to_string());
                }
            }
        };

        let start = Instant::now();
        run(&mut HashMap::new(), self, result_fn, &evaluate);
        let run_duration = start.elapsed();

        Ok(ResultSummary {
            parse_duration,
            jit_duration,
            run_duration,
        })
    }

    fn get_label_matrix(&self, label: &String) -> Option<&Matrix<bool>> {
        self.labels.iter().find(|(l, _)| l == label).map(|l| &l.1)
    }

    fn get_label_matrix_mut(&mut self, label: &String) -> &mut Matrix<bool> {
        if let Some(i) = self.labels.iter().position(|(l, _)| l == label) {
            return &mut self.labels[i].1;
        }

        self.labels.push((
            label.to_string(),
            Matrix::<bool>::new(self.node_cap, self.node_cap),
        ));
        self.labels.last_mut().map(|(_, o)| o).unwrap()
    }

    fn get_link_type_matrix_mut(&mut self, link_type: &String) -> &mut Matrix<bool> {
        if let Some(i) = self.links.iter().position(|(l, _)| l == link_type) {
            return &mut self.links[i].1;
        }

        self.links.push((
            link_type.to_string(),
            Matrix::<bool>::new(self.node_cap, self.node_cap),
        ));
        self.links.last_mut().map(|(_, o)| o).unwrap()
    }

    pub fn get_node_property_id(&mut self, key: &String) -> u64 {
        let property_id = self
            .node_property_ids
            .iter()
            .position(|p| p == key)
            .unwrap_or_else(|| {
                let len = self.node_property_ids.len();
                self.node_property_ids.push(key.to_string());
                len
            });
        property_id as u64
    }

    fn get_link_property_id(&mut self, key: &String) -> u64 {
        let property_id = self
            .link_property_ids
            .iter()
            .position(|p| p == key)
            .unwrap_or_else(|| {
                let len = self.link_property_ids.len();
                self.link_property_ids.push(key.to_string());
                len
            });
        property_id as u64
    }

    pub fn create_node(
        &mut self,
        labels: &Vec<String>,
        attr_keys: Value,
        attr_values: Value,
    ) -> Value {
        let id = self.deleted_nodes.min().unwrap_or(self.node_count);
        self.deleted_nodes.remove(id);
        self.node_count += 1;

        self.resize();

        self.all_nodes.set(id, id, true);

        for label in labels {
            let label_matrix = self.get_label_matrix_mut(label);
            label_matrix.set(id, id, true);
            let label_id = self.get_label_id(label).unwrap();
            self.resize();
            self.node_labels.set(id, label_id, true);
        }

        match (attr_keys, attr_values) {
            (Value::Array(keys), Value::Array(values)) => {
                for (key, value) in keys.into_iter().zip(values.into_iter()) {
                    match key {
                        Value::String(key) => {
                            let property_id = self.get_node_property_id(&key);
                            self.resize();
                            self.node_properties.set(id, property_id, value);
                        }
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }

        Value::Node(id)
    }

    pub fn delete_node(&mut self, id: u64) {
        self.deleted_nodes.insert(id);
        self.node_count -= 1;
        self.all_nodes.delete(id, id);

        for (label_id, label_matrix) in self.labels.iter_mut().map(|(_, m)| m).enumerate() {
            label_matrix.delete(id, id);
            self.node_labels.delete(id, label_id as _);
        }

        for property_id in 0..self.node_property_ids.len() {
            self.node_properties.delete(id, property_id as _);
        }
    }

    pub fn get_nodes(&self, labels: &[String]) -> Option<Iter<bool>> {
        if labels.is_empty() {
            return Some(self.all_nodes.iter());
        }
        self.get_label_matrix(&labels[0]).map(|m| m.iter())
    }

    pub fn get_node_labels(&self, id: u64) -> impl Iterator<Item = &String> {
        self.node_labels.iter_row(id).map(|(_, l)| {
            self.labels
                .iter()
                .enumerate()
                .filter(|(i, _)| *i as u64 == l)
                .map(|(_, (l, _))| l)
                .next()
                .unwrap()
        })
    }

    pub fn get_node_properties(&self, id: u64) -> Iter<Row<Value>> {
        self.node_properties.iter_row(id)
    }

    pub fn get_node_property(&self, node_id: u64, property_id: u64) -> Option<Value> {
        self.node_properties.get(node_id, property_id)
    }

    pub fn create_link(
        &mut self,
        link_type: &String,
        from: u64,
        to: u64,
        attr_keys: Value,
        attr_values: Value,
    ) -> Value {
        let id = self.deleted_links.min().unwrap_or(self.link_count);
        self.deleted_links.remove(id);
        self.link_count += 1;

        self.resize();

        let link_type_matrix = self.get_link_type_matrix_mut(link_type);
        link_type_matrix.set(from, to, true);

        match (attr_keys, attr_values) {
            (Value::Array(keys), Value::Array(values)) => {
                for (key, value) in keys.into_iter().zip(values.into_iter()) {
                    match key {
                        Value::String(key) => {
                            let property_id = self.get_link_property_id(&key);
                            self.resize();
                            self.link_properties.set(id, property_id, value);
                        }
                        _ => todo!(),
                    }
                }
            }
            _ => todo!(),
        }

        Value::Link(id)
    }

    fn resize(&mut self) {
        if self.node_count > self.node_cap {
            self.node_cap *= 2;
            self.node_properties
                .resize(self.node_cap, self.node_property_ids.len() as u64);
            self.adj.resize(self.node_cap, self.node_cap);
            self.node_labels
                .resize(self.node_cap, self.labels.len() as u64);
            self.all_nodes.resize(self.node_cap, self.node_cap);
            for label_matrix in self.labels.iter_mut().map(|(_, m)| m) {
                label_matrix.resize(self.node_cap, self.node_cap);
            }
            for link_matrix in self.links.iter_mut().map(|(_, m)| m) {
                link_matrix.resize(self.node_cap, self.node_cap);
            }
        }

        if self.labels.len() as u64 > self.node_labels.ncols() {
            self.node_labels
                .resize(self.node_cap, self.labels.len() as u64);
        }

        if self.node_property_ids.len() as u64 > self.node_properties.ncols() {
            self.node_properties
                .resize(self.node_cap, self.node_property_ids.len() as u64);
        }

        if self.link_count > self.link_cap {
            self.link_cap *= 2;
            self.link_properties
                .resize(self.link_cap, self.link_property_ids.len() as u64);
        }

        if self.link_property_ids.len() as u64 > self.link_properties.ncols() {
            self.link_properties
                .resize(self.link_cap, self.link_property_ids.len() as u64);
        }
    }
}

pub struct ResultSummary {
    pub parse_duration: Duration,
    pub jit_duration: Duration,
    pub run_duration: Duration,
}
