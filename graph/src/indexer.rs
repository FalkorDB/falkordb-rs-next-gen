//! Index management for property-based lookups.
//!
//! This module provides indexing capabilities for graph properties using RediSearch
//! as the underlying index engine. Supports:
//!
//! ## Index Types
//!
//! - **Range**: B-tree index for numeric comparisons (=, <, >, range queries)
//! - **Fulltext**: Text search with tokenization and stemming
//! - **Vector**: Vector similarity search for embeddings
//!
//! ## Architecture
//!
//! ```text
//! Indexer
//!    │
//!    ├── label_fields: Map<Label, Map<Attr, Field>>
//!    │      (Defines which properties are indexed)
//!    │
//!    └── rs_index: Map<Label, RSIndex>
//!           (RediSearch index handles)
//! ```
//!
//! ## Index Queries
//!
//! The [`IndexQuery`] enum represents different query types:
//! - `Equal(attr, value)`: Exact match
//! - `Range(attr, min, max)`: Range query
//! - `Prefix(attr, prefix)`: Prefix search
//! - `Contains(attr, substring)`: Substring search
//! - `Fulltext(query)`: Full-text search
//! - `VectorRange`: Vector similarity search

use std::{
    collections::HashMap,
    ffi::CString,
    hash::Hash,
    os::raw::{c_char, c_int, c_void},
    ptr::null_mut,
    sync::{
        Arc, RwLock,
        atomic::{AtomicI32, Ordering},
    },
};

use roaring::RoaringTreemap;

use crate::{
    redisearch::{
        GC_POLICY_FORK, REDISEARCH_ADD_REPLACE, RSDoc, RSFLDOPT_NONE, RSFLDTYPE_FULLTEXT,
        RSFLDTYPE_GEO, RSFLDTYPE_NUMERIC, RSFLDTYPE_TAG, RSFLDTYPE_VECTOR,
        RSGeoDistance_RS_GEO_DISTANCE_M, RSIndex, RSRANGE_INF, RSRANGE_NEG_INF,
        RediSearch_CreateDocument2, RediSearch_CreateField, RediSearch_CreateGeoNode,
        RediSearch_CreateIndex, RediSearch_CreateIndexOptions, RediSearch_CreateNumericNode,
        RediSearch_CreateTagNode, RediSearch_CreateTagTokenNode, RediSearch_DeleteDocument,
        RediSearch_DocumentAddFieldGeo, RediSearch_DocumentAddFieldNumber,
        RediSearch_DocumentAddFieldString, RediSearch_DocumentAddFieldVector, RediSearch_DropIndex,
        RediSearch_FreeIndexOptions, RediSearch_GetResultsIterator, RediSearch_IndexAddDocument,
        RediSearch_IndexOptionsSetGCPolicy, RediSearch_IndexOptionsSetStopwords,
        RediSearch_QueryNodeAddChild, RediSearch_ResultsIteratorFree,
        RediSearch_ResultsIteratorNext, RediSearch_TagFieldSetCaseSensitive,
        RediSearch_TagFieldSetSeparator, RediSearch_TextFieldSetWeight,
    },
    runtime::value::Value,
};

/// Type of index for a property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexType {
    /// B-tree range index for numeric/string comparisons
    Range,
    /// Full-text search index with tokenization
    Fulltext,
    /// Vector similarity index
    Vector,
    /// Point index for geographic coordinates
    Point,
}

/// Entity type that can be indexed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityType {
    /// Index on node properties
    Node,
    /// Index on relationship properties
    Relationship,
}

/// A document to be indexed, wrapping a RediSearch document.
#[derive(Clone)]
pub struct Document {
    rs_doc: *mut RSDoc,
}

impl Document {
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            rs_doc: unsafe {
                let doc = RediSearch_CreateDocument2(
                    (&raw const id).cast::<c_void>(),
                    8,
                    null_mut(),
                    1.0,
                    null_mut(),
                );
                assert!(!doc.is_null(), "Failed to create RediSearch document");
                doc
            },
        }
    }

    pub fn set(
        &mut self,
        field: Arc<Field>,
        value: Value,
    ) {
        unsafe {
            match value {
                Value::Bool(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        f64::from(i),
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::Int(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        i as f64,
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::Float(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        i,
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::String(s) => {
                    RediSearch_DocumentAddFieldString(
                        self.rs_doc,
                        field.name.as_ptr(),
                        s.as_ptr().cast::<c_char>(),
                        s.len(),
                        if field.ty == IndexType::Fulltext {
                            RSFLDTYPE_FULLTEXT
                        } else {
                            RSFLDTYPE_TAG
                        },
                    );
                }
                Value::Datetime(ts) | Value::Date(ts) | Value::Time(ts) | Value::Duration(ts) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr().cast::<c_char>(),
                        ts as f64,
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::List(_) => todo!(),
                Value::VecF32(vec) => {
                    RediSearch_DocumentAddFieldVector(
                        self.rs_doc,
                        field.name.as_ptr().cast::<c_char>(),
                        vec.as_ptr().cast::<c_char>(),
                        vec.len() as u32,
                        vec.len() * std::mem::size_of::<f32>(),
                    );
                }
                Value::Point(p) => {
                    RediSearch_DocumentAddFieldGeo(
                        self.rs_doc,
                        field.name.as_ptr().cast::<c_char>(),
                        f64::from(p.latitude),
                        f64::from(p.longitude),
                        RSFLDTYPE_GEO,
                    );
                }
                Value::Null
                | Value::Map(_)
                | Value::Node(_)
                | Value::Relationship(_)
                | Value::Path(_)
                | Value::Arc(_) => unreachable!(),
            }
        }
    }
}

#[derive(Debug)]
pub enum IndexQuery<T> {
    Equal(Arc<String>, T),
    Range(Arc<String>, Option<T>, Option<T>),
    And(Vec<Self>),
    Or(Vec<Self>),
    Point {
        key: Arc<String>,
        point: T,
        radius: T,
    },
}

pub struct Field {
    pub name: CString,
    pub ty: IndexType,
}

impl PartialEq for Field {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.name == other.name && self.ty == other.ty
    }
}

impl Eq for Field {}

impl Hash for Field {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.name.hash(state);
    }
}

#[derive(Clone)]
pub enum IndexStatus {
    Operational,
    UnderConstruction(u64, u64),
}

struct Index {
    rs_idx: *mut RSIndex,
    fields: HashMap<Arc<String>, Vec<Arc<Field>>>,
    status: IndexStatus,
    pending_changes: AtomicI32,
}

pub struct IndexInfo {
    pub label: Arc<String>,
    pub status: IndexStatus,
    pub fields: HashMap<Arc<String>, Vec<Arc<Field>>>,
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            RediSearch_DropIndex(self.rs_idx);
        }
    }
}

#[derive(Default, Clone)]
pub struct Indexer {
    index: Arc<RwLock<HashMap<Arc<String>, Index>>>,
}

// SAFETY: Indexer is safe to Send/Sync because:
// - The internal Arc<RwLock<HashMap>> provides thread-safe shared access
// - RwLock ensures exclusive write access and concurrent read access
// - All Index fields are either thread-safe primitives (AtomicI32) or protected by the RwLock
// - RediSearch FFI calls are serialized through the RwLock, preventing data races
unsafe impl Send for Indexer {}
unsafe impl Sync for Indexer {}

impl Indexer {
    #[must_use]
    pub fn has_indices(&self) -> bool {
        !self.index.read().unwrap().is_empty()
    }

    pub fn create_index(
        &mut self,
        index_type: &IndexType,
        label: Arc<String>,
        attrs: &Vec<Arc<String>>,
        total: u64,
    ) -> Result<(), String> {
        let mut fields = HashMap::new();
        let mut index = self.index.write().unwrap();
        let a = index.entry(label.clone()).or_insert_with(|| Index {
            rs_idx: std::ptr::null_mut(),
            fields: HashMap::new(),
            status: IndexStatus::Operational,
            pending_changes: AtomicI32::new(0),
        });

        for attr in attrs {
            if let Some(f) = a.fields.get_mut(attr) {
                if f.iter().any(|f| f.ty == *index_type) {
                    return Err(format!("Attribute '{attr}' is already indexed"));
                }
                let field_name = match index_type {
                    IndexType::Range => Arc::new(format!("range:{attr}")),
                    IndexType::Fulltext => attr.clone(),
                    IndexType::Vector => Arc::new(format!("vector:{attr}")),
                    IndexType::Point => Arc::new(format!("point:{attr}")),
                };
                let field = Arc::new(Field {
                    name: CString::new(field_name.as_str()).map_err(|_| {
                        format!("Invalid field name '{}': contains null byte", field_name)
                    })?,
                    ty: index_type.clone(),
                });
                f.push(field);
            } else {
                let field_name = match index_type {
                    IndexType::Range => Arc::new(format!("range:{attr}")),
                    IndexType::Fulltext => attr.clone(),
                    IndexType::Vector => Arc::new(format!("vector:{attr}")),
                    IndexType::Point => Arc::new(format!("point:{attr}")),
                };
                let field = Arc::new(Field {
                    name: CString::new(field_name.as_str()).map_err(|_| {
                        format!("Invalid field name '{}': contains null byte", field_name)
                    })?,
                    ty: index_type.clone(),
                });
                a.fields.insert(attr.clone(), vec![field]);
            }
        }
        fields.clone_from(&a.fields);

        unsafe {
            let options = RediSearch_CreateIndexOptions();
            // RediSearch_IndexOptionsSetLanguage(options, idx->language);
            RediSearch_IndexOptionsSetGCPolicy(options, GC_POLICY_FORK as _);
            RediSearch_IndexOptionsSetStopwords(options, null_mut(), 0);

            let clabel = CString::new(label.as_str()).map_err(|_| {
                format!("Invalid label name '{}': contains null byte", label)
            })?;
            let index = RediSearch_CreateIndex(clabel.as_ptr().cast::<c_char>(), options);
            RediSearch_FreeIndexOptions(options);

            for field in fields.values().flat_map(|f| f.iter()) {
                match field.ty {
                    IndexType::Range => {
                        let types = RSFLDTYPE_NUMERIC | RSFLDTYPE_GEO | RSFLDTYPE_TAG;
                        let field_id = RediSearch_CreateField(
                            index,
                            field.name.as_ptr(),
                            types,
                            RSFLDOPT_NONE,
                        );

                        RediSearch_TagFieldSetSeparator(index, field_id, 1 as c_char);
                        RediSearch_TagFieldSetCaseSensitive(index, field_id, 1);
                    }
                    IndexType::Fulltext => {
                        let field_id = RediSearch_CreateField(
                            index,
                            field.name.as_ptr(),
                            RSFLDTYPE_FULLTEXT,
                            RSFLDOPT_NONE,
                        );

                        RediSearch_TextFieldSetWeight(index, field_id, 1.0);
                    }
                    IndexType::Vector => {
                        let _field_id = RediSearch_CreateField(
                            index,
                            field.name.as_ptr(),
                            RSFLDTYPE_VECTOR,
                            RSFLDOPT_NONE,
                        );
                        // RediSearch_VectorFieldSetDim(index, field_id, field->hnsw_options.dimension);
                        // RediSearch_VectorFieldSetHNSWParams(index, field_id, IndexField_OptionsGetM(field), IndexField_OptionsGetEfConstruction(field), IndexField_OptionsGetEfRuntime(field), IndexField_OptionsGetSimFunc(field));
                    }
                    IndexType::Point => {
                        let _field_id = RediSearch_CreateField(
                            index,
                            field.name.as_ptr(),
                            RSFLDTYPE_GEO,
                            RSFLDOPT_NONE,
                        );
                    }
                }
            }

            a.rs_idx = index;
            a.status = IndexStatus::UnderConstruction(0, total);
            a.pending_changes.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn drop_index(
        &mut self,
        label: Arc<String>,
        attrs: &Vec<Arc<String>>,
        index_type: &IndexType,
        total: u64,
    ) -> Option<Vec<Arc<String>>> {
        let mut index = self.index.write().unwrap();
        if let Some(index) = index.get_mut(&label) {
            let mut removed = false;
            for attr in attrs {
                if let Some(field) = index.fields.get(attr)
                    && field.iter().any(|f| f.ty == *index_type)
                {
                    if field.len() == 1 {
                        index.fields.remove(attr);
                        removed = true;
                    } else {
                        index
                            .fields
                            .get_mut(attr)
                            .unwrap()
                            .retain(|f| f.ty != *index_type);
                    }
                }
            }
            if index.fields.is_empty() {
                index.status = IndexStatus::UnderConstruction(0, 0);
                index.pending_changes.fetch_add(1, Ordering::SeqCst);
                return None;
            }
            if removed {
                index.status = IndexStatus::UnderConstruction(0, total);
                return Some(index.fields.keys().cloned().collect());
            }
        }
        None
    }

    pub fn remove(
        &mut self,
        label: Arc<String>,
    ) {
        self.index.write().unwrap().remove(&label);
    }

    #[must_use]
    pub fn is_label_indexed(
        &self,
        label: Arc<String>,
    ) -> bool {
        if let Some(index) = self.index.read().unwrap().get(&label)
            && matches!(index.status, IndexStatus::Operational)
        {
            return true;
        }
        false
    }

    #[must_use]
    pub fn is_attr_indexed(
        &self,
        label: Arc<String>,
        field: Arc<String>,
    ) -> bool {
        if let Some(index) = self.index.read().unwrap().get(&label)
            && matches!(index.status, IndexStatus::Operational)
        {
            return index.fields.contains_key(&field);
        }
        false
    }

    #[must_use]
    pub fn query(
        &self,
        label: Arc<String>,
        query: IndexQuery<Value>,
    ) -> Result<Vec<u64>, String> {
        if let Some(index) = self.index.read().unwrap().get(&label) {
            let query = match query {
                IndexQuery::Equal(key, Value::Int(value)) => unsafe {
                    let field = &index.fields.get(&key).unwrap()[0];
                    RediSearch_CreateNumericNode(
                        index.rs_idx,
                        field.name.as_ptr(),
                        value as f64,
                        value as f64,
                        1,
                        1,
                    )
                },
                IndexQuery::Equal(key, Value::String(value)) => unsafe {
                    let field = &index.fields.get(&key).unwrap()[0];
                    let query = RediSearch_CreateTagNode(index.rs_idx, field.name.as_ptr());
                    let msg = CString::new(value.as_str()).map_err(|_| {
                        format!("Invalid query value '{}': contains null byte", value)
                    })?;
                    let child =
                        RediSearch_CreateTagTokenNode(index.rs_idx, msg.as_ptr().cast::<c_char>());
                    RediSearch_QueryNodeAddChild(query, child);

                    query
                },
                IndexQuery::Range(key, min, max) => {
                    let (min, max) = match (min, max) {
                        (Some(Value::Float(min)), None) => (min, RSRANGE_INF),
                        (None, Some(Value::Float(max))) => (RSRANGE_NEG_INF, max),
                        (Some(Value::Float(min)), Some(Value::Float(max))) => (min, max),
                        (Some(Value::Int(min)), None) => (min as f64, RSRANGE_INF),
                        (None, Some(Value::Int(max))) => (RSRANGE_NEG_INF, max as f64),
                        (Some(Value::Int(min)), Some(Value::Int(max))) => (min as f64, max as f64),
                        _ => todo!(),
                    };
                    unsafe {
                        let field = &index.fields.get(&key).unwrap()[0];
                        RediSearch_CreateNumericNode(
                            index.rs_idx,
                            field.name.as_ptr(),
                            max,
                            min,
                            0,
                            0,
                        )
                    }
                }
                IndexQuery::Point {
                    key,
                    point: Value::Point(point),
                    radius: Value::Float(radius),
                } => {
                    unsafe {
                        let field = &index.fields.get(&key).unwrap()[0];
                        // Create a GeoNode with the given latitude, longitude, and radius, radius type is M
                        RediSearch_CreateGeoNode(
                            index.rs_idx,
                            field.name.as_ptr(),
                            point.latitude as f64,
                            point.longitude as f64,
                            radius,
                            RSGeoDistance_RS_GEO_DISTANCE_M,
                        )
                    }
                }
                IndexQuery::Point {
                    key,
                    point: Value::Point(point),
                    radius: Value::Int(radius),
                } => {
                    unsafe {
                        let field = &index.fields.get(&key).unwrap()[0];
                        // Create a GeoNode with the given latitude, longitude, and radius, radius type is M
                        RediSearch_CreateGeoNode(
                            index.rs_idx,
                            field.name.as_ptr(),
                            point.latitude as f64,
                            point.longitude as f64,
                            radius as f64,
                            RSGeoDistance_RS_GEO_DISTANCE_M,
                        )
                    }
                }

                _ => todo!(),
            };

            unsafe {
                let iter = RediSearch_GetResultsIterator(query, index.rs_idx);

                let mut res = vec![];
                loop {
                    let node_id = RediSearch_ResultsIteratorNext(iter, index.rs_idx, null_mut())
                        .cast::<u64>();
                    if node_id.is_null() {
                        break;
                    }
                    res.push(node_id.read_unaligned());
                }
                RediSearch_ResultsIteratorFree(iter);
                return Ok(res);
            }
        }

        Ok(vec![])
    }

    pub fn enable(
        &mut self,
        label: Arc<String>,
    ) -> bool {
        let mut index = self.index.write().unwrap();
        if let Some(index) = index.get_mut(&label) {
            let res = index.pending_changes.fetch_sub(1, Ordering::SeqCst);
            assert!(res > 0, "pending_changes underflow for label {}", label);
            return res == 1;
        }
        false
    }

    pub fn disable(
        &mut self,
        label: Arc<String>,
    ) {
        let mut index = self.index.write().unwrap();
        if let Some(index) = index.get_mut(&label) {
            index.pending_changes.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[must_use]
    pub fn enabled(
        &self,
        label: Arc<String>,
    ) -> bool {
        if let Some(index) = self.index.read().unwrap().get(&label) {
            return index.pending_changes.load(Ordering::SeqCst) == 0;
        }
        false
    }

    #[must_use]
    pub fn pending_changes(
        &self,
        label: Arc<String>,
    ) -> i32 {
        if let Some(index) = self.index.read().unwrap().get(&label) {
            return index.pending_changes.load(Ordering::SeqCst);
        }
        0
    }

    pub fn commit(
        &mut self,
        add_docs: &mut HashMap<Arc<String>, Vec<Document>>,
        remove_docs: &mut HashMap<Arc<String>, RoaringTreemap>,
    ) {
        let mut index = self.index.write().unwrap();
        for (label, add_docs) in add_docs {
            let Some(index) = index.get_mut(label) else {
                continue;
            };
            for doc in add_docs.drain(..) {
                unsafe {
                    let res = RediSearch_IndexAddDocument(
                        index.rs_idx,
                        doc.rs_doc,
                        REDISEARCH_ADD_REPLACE as c_int,
                        null_mut(),
                    );
                    assert_eq!(res, 0, "Failed to add document to RediSearch index");
                }
            }
            index.status = IndexStatus::Operational;
        }
        for (label, remove_docs) in remove_docs {
            let Some(index) = index.get_mut(label) else {
                continue;
            };
            for id in remove_docs.iter() {
                unsafe {
                    RediSearch_DeleteDocument(index.rs_idx, (&raw const id).cast::<c_void>(), 8);
                };
            }
        }
    }

    #[must_use]
    pub fn get_fields(
        &self,
        label: Arc<String>,
    ) -> HashMap<Arc<String>, Vec<Arc<Field>>> {
        self.index
            .read()
            .unwrap()
            .get(&label)
            .map(|index| index.fields.clone())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn index_info(&self) -> Vec<IndexInfo> {
        self.index
            .read()
            .unwrap()
            .iter()
            .map(|(label, index)| {
                let attrs = index
                    .fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                IndexInfo {
                    label: label.clone(),
                    status: index.status.clone(),
                    fields: attrs,
                }
            })
            .collect()
    }
}
