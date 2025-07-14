use std::collections::HashMap;

use crate::{
    graph::{
        matrix::{Matrix, New, Remove, Set, Size},
        tensor::GrB_INDEX_MAX,
    },
    runtime::value::Value,
};

#[derive(Clone)]
pub struct Document {
    id: u64,
    columns: HashMap<u64, Value>,
}

impl Document {
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            columns: HashMap::new(),
        }
    }

    pub fn set(
        &mut self,
        key: u64,
        value: Value,
    ) {
        self.columns.insert(key, value);
    }
}

pub enum IndexQuery {
    Equal(u64, Value),
    Range(u64, Value, Value),
    And(Vec<IndexQuery>),
    Or(Vec<IndexQuery>),
}

pub struct Indexer {
    ndocs: u64,
    int_indexer: HashMap<(u64, u64), Matrix<bool>>,
}

impl Indexer {
    #[must_use]
    pub fn new(ndocs: u64) -> Self {
        Self {
            ndocs,
            int_indexer: HashMap::new(),
        }
    }

    pub fn create_index(
        &mut self,
        label: u64,
        key: u64,
    ) {
        self.int_indexer
            .insert((label, key), Matrix::<bool>::new(GrB_INDEX_MAX, self.ndocs));
    }

    #[must_use]
    pub fn is_indexed(
        &self,
        label: u64,
        key: u64,
    ) -> bool {
        self.int_indexer.contains_key(&(label, key))
    }

    pub fn add(
        &mut self,
        label: u64,
        doc: Document,
    ) {
        for (key, value) in doc.columns {
            if let Value::Int(int_value) = value
                && let Some(index) = self.int_indexer.get_mut(&(label, key))
            {
                index.set(int_value as u64, doc.id, true);
            }
        }
    }

    pub fn remove(
        &mut self,
        label: u64,
        doc: Document,
    ) {
        for (key, value) in doc.columns {
            if let Value::Int(int_value) = value
                && let Some(index) = self.int_indexer.get_mut(&(label, key))
            {
                index.remove(int_value as u64, doc.id);
            }
        }
    }

    #[must_use]
    pub fn query(
        &self,
        label: u64,
        query: IndexQuery,
    ) -> Vec<u64> {
        match query {
            IndexQuery::Equal(key, Value::Int(value)) => self
                .int_indexer
                .get(&(label, key))
                .map_or_else(Vec::new, |index| {
                    index
                        .iter(value as u64, value as u64)
                        .map(|(_, doc_id)| doc_id)
                        .collect()
                }),
            _ => todo!(),
        }
    }

    pub fn resize(
        &mut self,
        ndocs: u64,
    ) {
        for index in self.int_indexer.values_mut() {
            index.resize(GrB_INDEX_MAX, ndocs);
        }
        self.ndocs = ndocs;
    }
}
