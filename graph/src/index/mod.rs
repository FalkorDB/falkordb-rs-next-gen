pub mod indexer;
pub mod redisearch;
pub mod text_index_options;
pub use text_index_options::TextIndexOptions;

use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    hash::Hash,
    os::raw::{c_char, c_int, c_void},
    ptr::null_mut,
    sync::{
        Arc,
        atomic::{AtomicI32, Ordering},
    },
};

use crate::runtime::value::Value;
use redisearch::{
    GC_POLICY_FORK, REDISEARCH_ADD_REPLACE, RSDoc, RSFLDOPT_NONE, RSFLDOPT_TXTNOSTEM,
    RSFLDOPT_TXTPHONETIC, RSFLDTYPE_FULLTEXT, RSFLDTYPE_GEO, RSFLDTYPE_NUMERIC, RSFLDTYPE_TAG,
    RSFLDTYPE_VECTOR, RSGeoDistance_RS_GEO_DISTANCE_M, RSIndex, RSRANGE_INF, RSRANGE_NEG_INF,
    RSResultsIterator, RediSearch_CreateDocument2, RediSearch_CreateField,
    RediSearch_CreateGeoNode, RediSearch_CreateIndex, RediSearch_CreateIndexOptions,
    RediSearch_CreateNumericNode, RediSearch_CreateTagNode, RediSearch_CreateTagTokenNode,
    RediSearch_DeleteDocument, RediSearch_DocumentAddFieldGeo, RediSearch_DocumentAddFieldNumber,
    RediSearch_DocumentAddFieldString, RediSearch_DocumentAddFieldVector, RediSearch_DropIndex,
    RediSearch_FreeIndexOptions, RediSearch_GetResultsIterator, RediSearch_IndexAddDocument,
    RediSearch_IndexOptionsSetGCPolicy, RediSearch_IndexOptionsSetLanguage,
    RediSearch_IndexOptionsSetStopwords, RediSearch_IterateQuery, RediSearch_QueryNodeAddChild,
    RediSearch_ResultsIteratorFree, RediSearch_ResultsIteratorGetScore,
    RediSearch_ResultsIteratorNext, RediSearch_TagFieldSetCaseSensitive,
    RediSearch_TagFieldSetSeparator, RediSearch_TextFieldSetWeight,
};

/// Type of index for a property.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum IndexType {
    /// B-tree range index for numeric/string/geo comparisons
    Range,
    /// Full-text search index with tokenization
    #[default]
    Fulltext,
    /// Vector similarity index
    Vector,
}

#[derive(Debug, Default)]
pub struct Field {
    pub name: CString,
    pub ty: IndexType,
    options: Option<TextIndexOptions>,
}

impl Field {
    #[must_use]
    pub const fn new(
        name: CString,
        ty: IndexType,
        options: Option<TextIndexOptions>,
    ) -> Self {
        Self { name, ty, options }
    }

    #[must_use]
    pub const fn options(&self) -> Option<&TextIndexOptions> {
        self.options.as_ref()
    }
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

pub struct IndexInfo {
    pub label: Arc<String>,
    pub pending: i32,
    pub progress: u64,
    pub total: u64,
    pub fields: HashMap<Arc<String>, Vec<Arc<Field>>>,
    pub language: Option<Arc<String>>,
    pub stopwords: Option<Vec<Arc<String>>>,
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

/// Lazy iterator over RediSearch query results.
///
/// Wraps the C `RSResultsIterator` and calls `RediSearch_ResultsIteratorNext`
/// on each `.next()`. Frees the C iterator on `Drop`.
///
/// The mapper function `F` extracts the desired item type from each raw
/// iterator step (e.g. just the ID, or ID + score).
pub struct IndexResultsIter<T, F: FnMut(*mut RSResultsIterator, u64) -> T> {
    iter: *mut RSResultsIterator,
    rs_idx: *mut RSIndex,
    map: F,
}

impl<T, F: FnMut(*mut RSResultsIterator, u64) -> T> IndexResultsIter<T, F> {
    const fn new(
        iter: *mut RSResultsIterator,
        rs_idx: *mut RSIndex,
        map: F,
    ) -> Self {
        Self { iter, rs_idx, map }
    }
}

impl IndexResultsIter<u64, fn(*mut RSResultsIterator, u64) -> u64> {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            iter: null_mut(),
            rs_idx: null_mut(),
            map: |_, id| id,
        }
    }
}

impl IndexResultsIter<(u64, f64), fn(*mut RSResultsIterator, u64) -> (u64, f64)> {
    #[must_use]
    pub fn empty_scored() -> Self {
        Self {
            iter: null_mut(),
            rs_idx: null_mut(),
            map: |_, id| (id, 0.0),
        }
    }
}

impl<T, F: FnMut(*mut RSResultsIterator, u64) -> T> Iterator for IndexResultsIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_null() {
            return None;
        }
        unsafe {
            let id =
                RediSearch_ResultsIteratorNext(self.iter, self.rs_idx, null_mut()).cast::<u64>();
            if id.is_null() {
                return None;
            }
            Some((self.map)(self.iter, id.read_unaligned()))
        }
    }
}

impl<T, F: FnMut(*mut RSResultsIterator, u64) -> T> Drop for IndexResultsIter<T, F> {
    fn drop(&mut self) {
        if !self.iter.is_null() {
            unsafe {
                RediSearch_ResultsIteratorFree(self.iter);
            }
        }
    }
}

/// Iterator yielding entity IDs from range/tag/geo index queries.
pub type IdIter = IndexResultsIter<u64, fn(*mut RSResultsIterator, u64) -> u64>;

/// Iterator yielding (entity ID, score) pairs from fulltext index queries.
pub type ScoredIdIter = IndexResultsIter<(u64, f64), fn(*mut RSResultsIterator, u64) -> (u64, f64)>;

/// A document to be indexed, wrapping a RediSearch document.
#[derive(Clone)]
pub struct Document {
    rs_doc: *mut RSDoc,
    id: u64,
}

impl Document {
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            rs_doc: unsafe {
                let doc = RediSearch_CreateDocument2(
                    (&raw const id).cast::<c_void>(),
                    8,
                    null_mut(),
                    1.0,
                    null_mut(),
                );
                debug_assert!(!doc.is_null(), "Failed to create RediSearch document");
                doc
            },
        }
    }

    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn set(
        &mut self,
        field: &Field,
        value: &Value,
    ) {
        unsafe {
            match value {
                Value::Bool(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        f64::from(*i),
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::Int(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        *i as f64,
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::Float(i) => {
                    RediSearch_DocumentAddFieldNumber(
                        self.rs_doc,
                        field.name.as_ptr(),
                        *i,
                        RSFLDTYPE_NUMERIC,
                    );
                }
                Value::String(s) | Value::InternedString(s) => {
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
                        *ts as f64,
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
                | Value::Path(_) => unreachable!(),
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Index {
    rs_idx: *mut RSIndex,
    fields: HashMap<Arc<String>, Vec<Arc<Field>>>,
    pending_changes: AtomicI32,
    progress: u64,
    total: u64,
    language: Option<Arc<String>>,
    stopwords: Option<Vec<Arc<String>>>,
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            if !self.rs_idx.is_null() {
                RediSearch_DropIndex(self.rs_idx);
            }
        }
    }
}

impl Index {
    // --- RediSearch index lifecycle ---

    /// Returns true if a RediSearch index has been created.
    #[must_use]
    pub const fn has_rs_index(&self) -> bool {
        !self.rs_idx.is_null()
    }

    /// Create the underlying RediSearch index with the given options.
    /// Should only be called when `!self.has_rs_index()`.
    pub fn create_rs_index(
        &mut self,
        label: &Arc<String>,
        stopwords: Option<&Vec<Arc<String>>>,
        language: Option<&Arc<String>>,
    ) -> Result<(), String> {
        unsafe {
            let options = RediSearch_CreateIndexOptions();
            RediSearch_IndexOptionsSetGCPolicy(options, GC_POLICY_FORK as _);

            if let Some(stop_words) = stopwords {
                let c_stopwords: Vec<CString> = stop_words
                    .iter()
                    .map(|s| CString::new(s.as_str()).map_err(|e| e.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;
                let mut ptrs: Vec<*const c_char> =
                    c_stopwords.iter().map(|cs| cs.as_ptr()).collect();
                RediSearch_IndexOptionsSetStopwords(
                    options,
                    ptrs.as_mut_ptr(),
                    ptrs.len() as c_int,
                );
            } else {
                RediSearch_IndexOptionsSetStopwords(options, null_mut(), 0);
            }

            if let Some(lang) = language {
                let c_lang = CString::new(lang.as_str()).map_err(|e| e.to_string())?;
                if RediSearch_IndexOptionsSetLanguage(options, c_lang.as_ptr()) != 0 {
                    return Err(format!("Language is not supported: {lang}"));
                }
            } else {
                RediSearch_IndexOptionsSetLanguage(options, null_mut());
            }

            let clabel = CString::new(label.as_str()).map_err(|e| e.to_string())?;
            self.rs_idx = RediSearch_CreateIndex(clabel.as_ptr().cast::<c_char>(), options);
            RediSearch_FreeIndexOptions(options);
        }
        Ok(())
    }

    /// Register fields in the RediSearch index. Must be called after `create_rs_index`.
    pub fn register_fields(
        &self,
        fields: &HashMap<Arc<String>, Vec<Arc<Field>>>,
        field_options: Option<&TextIndexOptions>,
    ) {
        unsafe {
            for field in fields.values().flat_map(|f| f.iter()) {
                match field.ty {
                    IndexType::Range => {
                        let types = RSFLDTYPE_NUMERIC | RSFLDTYPE_GEO | RSFLDTYPE_TAG;
                        let field_id = RediSearch_CreateField(
                            self.rs_idx,
                            field.name.as_ptr(),
                            types,
                            RSFLDOPT_NONE,
                        );

                        RediSearch_TagFieldSetSeparator(self.rs_idx, field_id, 1 as c_char);
                        RediSearch_TagFieldSetCaseSensitive(self.rs_idx, field_id, 1);
                    }
                    IndexType::Fulltext => {
                        let mut field_options_flag = RSFLDOPT_NONE;
                        let mut weight = 1.0;
                        let effective_options = field_options.or_else(|| field.options());
                        if let Some(options) = effective_options {
                            weight = options.weight.unwrap_or(1.0);
                            if options.nostem.unwrap_or(false) {
                                field_options_flag |= RSFLDOPT_TXTNOSTEM;
                            }
                            if options.phonetic.unwrap_or(false) {
                                field_options_flag |= RSFLDOPT_TXTPHONETIC;
                            }
                        }

                        let field_id = RediSearch_CreateField(
                            self.rs_idx,
                            field.name.as_ptr(),
                            RSFLDTYPE_FULLTEXT,
                            field_options_flag,
                        );

                        RediSearch_TextFieldSetWeight(self.rs_idx, field_id, weight);
                    }
                    IndexType::Vector => {
                        let _field_id = RediSearch_CreateField(
                            self.rs_idx,
                            field.name.as_ptr(),
                            RSFLDTYPE_VECTOR,
                            RSFLDOPT_NONE,
                        );
                    }
                }
            }
        }
    }

    /// Execute an index query and return matching entity IDs.
    pub fn query(
        &self,
        query: IndexQuery<Value>,
    ) -> IdIter {
        let query = match query {
            IndexQuery::Equal(key, Value::Int(value)) => unsafe {
                let field = &self.fields.get(&key).unwrap()[0];
                RediSearch_CreateNumericNode(
                    self.rs_idx,
                    field.name.as_ptr(),
                    value as f64,
                    value as f64,
                    1,
                    1,
                )
            },
            IndexQuery::Equal(key, Value::String(value) | Value::InternedString(value)) => unsafe {
                let field = &self.fields.get(&key).unwrap()[0];
                let query = RediSearch_CreateTagNode(self.rs_idx, field.name.as_ptr());
                let msg = CString::new(value.as_str()).unwrap();
                let child =
                    RediSearch_CreateTagTokenNode(self.rs_idx, msg.as_ptr().cast::<c_char>());
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
                    let field = &self.fields.get(&key).unwrap()[0];
                    RediSearch_CreateNumericNode(self.rs_idx, field.name.as_ptr(), max, min, 0, 0)
                }
            }
            IndexQuery::Point {
                key,
                point: Value::Point(point),
                radius: Value::Float(radius),
            } => unsafe {
                let field = &self.fields.get(&key).unwrap()[0];
                RediSearch_CreateGeoNode(
                    self.rs_idx,
                    field.name.as_ptr(),
                    point.latitude as f64,
                    point.longitude as f64,
                    radius,
                    RSGeoDistance_RS_GEO_DISTANCE_M,
                )
            },
            IndexQuery::Point {
                key,
                point: Value::Point(point),
                radius: Value::Int(radius),
            } => unsafe {
                let field = &self.fields.get(&key).unwrap()[0];
                RediSearch_CreateGeoNode(
                    self.rs_idx,
                    field.name.as_ptr(),
                    point.latitude as f64,
                    point.longitude as f64,
                    radius as f64,
                    RSGeoDistance_RS_GEO_DISTANCE_M,
                )
            },

            _ => todo!(),
        };

        unsafe {
            let iter = RediSearch_GetResultsIterator(query, self.rs_idx);
            IndexResultsIter::new(iter, self.rs_idx, |_, id| id)
        }
    }

    /// Execute a fulltext query and return matching entity IDs with scores.
    pub fn fulltext_query(
        &self,
        query: &str,
    ) -> Result<ScoredIdIter, String> {
        let cstr = CString::new(query).map_err(|e| e.to_string())?;
        let mut err: *mut c_char = null_mut();
        unsafe {
            let iter = RediSearch_IterateQuery(self.rs_idx, cstr.as_ptr(), query.len(), &mut err);
            if !err.is_null() {
                let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
                drop(CString::from_raw(err));
                return Err(msg);
            }
            Ok(IndexResultsIter::new(iter, self.rs_idx, |iter, id| {
                let score = RediSearch_ResultsIteratorGetScore(iter);
                (id, score)
            }))
        }
    }

    /// Add a document to the index.
    pub fn add_document(
        &self,
        doc: &Document,
    ) {
        unsafe {
            let res = RediSearch_IndexAddDocument(
                self.rs_idx,
                doc.rs_doc,
                REDISEARCH_ADD_REPLACE as c_int,
                null_mut(),
            );
            debug_assert_eq!(res, 0);
        }
    }

    /// Delete a document from the index by entity ID.
    pub fn delete_document(
        &self,
        id: u64,
    ) {
        unsafe {
            RediSearch_DeleteDocument(self.rs_idx, (&raw const id).cast::<c_void>(), 8);
        }
    }

    // --- fields ---

    /// Check if any field has the Fulltext index type.
    #[must_use]
    pub fn has_fulltext_field(&self) -> bool {
        self.fields
            .values()
            .any(|fields| fields.iter().any(|f| f.ty == IndexType::Fulltext))
    }

    /// Check if a specific attribute is indexed.
    #[must_use]
    pub fn contains_field(
        &self,
        attr: &Arc<String>,
    ) -> bool {
        self.fields.contains_key(attr)
    }

    /// Check if a specific attribute has a field with the given index type.
    #[must_use]
    pub fn has_field_with_type(
        &self,
        attr: &Arc<String>,
        index_type: &IndexType,
    ) -> bool {
        self.fields
            .get(attr)
            .is_some_and(|fields| fields.iter().any(|f| f.ty == *index_type))
    }

    /// Get all fields for a given attribute.
    #[must_use]
    pub fn get_fields(
        &self,
        attr: &Arc<String>,
    ) -> Option<&Vec<Arc<Field>>> {
        self.fields.get(attr)
    }

    /// Push a field to an existing attribute's field list.
    pub fn add_field_to_existing(
        &mut self,
        attr: &Arc<String>,
        field: Arc<Field>,
    ) {
        if let Some(fields) = self.fields.get_mut(attr) {
            fields.push(field);
        }
    }

    /// Insert a new attribute with its initial field.
    pub fn insert_field(
        &mut self,
        attr: Arc<String>,
        field: Arc<Field>,
    ) {
        self.fields.insert(attr, vec![field]);
    }

    /// Remove all fields for an attribute. Returns true if the attr existed.
    pub fn remove_field(
        &mut self,
        attr: &Arc<String>,
    ) -> bool {
        self.fields.remove(attr).is_some()
    }

    /// Retain only fields that don't match the given index type for a specific attribute.
    pub fn retain_fields(
        &mut self,
        attr: &Arc<String>,
        index_type: &IndexType,
    ) {
        if let Some(fields) = self.fields.get_mut(attr) {
            fields.retain(|f| f.ty != *index_type);
        }
    }

    /// Check if the index has no fields at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Get all attribute names.
    #[must_use]
    pub fn field_keys(&self) -> Vec<Arc<String>> {
        self.fields.keys().cloned().collect()
    }

    /// Get a reference to all fields.
    #[must_use]
    pub const fn fields(&self) -> &HashMap<Arc<String>, Vec<Arc<Field>>> {
        &self.fields
    }

    /// Iterate over all Field objects (flattened across all attributes).
    pub fn all_fields(&self) -> impl Iterator<Item = &Arc<Field>> {
        self.fields.values().flat_map(|f| f.iter())
    }

    // --- status ---

    /// An index is operational when there are no pending changes.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.pending_changes.load(Ordering::SeqCst) == 0
    }

    /// Set the index population progress.
    pub const fn set_progress(
        &mut self,
        progress: u64,
        total: u64,
    ) {
        self.progress = progress;
        self.total = total;
    }

    /// Get the current progress values.
    #[must_use]
    pub const fn progress(&self) -> (u64, u64) {
        (self.progress, self.total)
    }

    // --- pending_changes ---

    /// Increment the pending changes counter. Returns the previous value.
    pub fn increment_pending(&self) -> i32 {
        self.pending_changes.fetch_add(1, Ordering::SeqCst)
    }

    /// Decrement the pending changes counter. Returns the previous value.
    pub fn decrement_pending(&self) -> i32 {
        self.pending_changes.fetch_sub(1, Ordering::SeqCst)
    }

    /// Get the current pending changes count.
    #[must_use]
    pub fn pending_count(&self) -> i32 {
        self.pending_changes.load(Ordering::SeqCst)
    }

    // --- language ---

    /// Get a reference to the language setting, if any.
    #[must_use]
    pub const fn language(&self) -> Option<&Arc<String>> {
        self.language.as_ref()
    }

    /// Set the language for this index.
    pub fn set_language(
        &mut self,
        language: Option<Arc<String>>,
    ) {
        self.language = language;
    }

    // --- stopwords ---

    /// Get a reference to the stopwords list, if any.
    #[must_use]
    pub const fn stopwords(&self) -> Option<&Vec<Arc<String>>> {
        self.stopwords.as_ref()
    }

    /// Set the stopwords for this index.
    pub fn set_stopwords(
        &mut self,
        stopwords: Option<Vec<Arc<String>>>,
    ) {
        self.stopwords = stopwords;
    }

    // --- index count ---

    /// Get the number of indexed documents.
    #[must_use]
    pub fn index_count(&self) -> usize {
        self.fields.values().map(Vec::len).sum()
    }

    pub fn recreate_index(
        &mut self,
        label: &Arc<String>,
    ) -> Result<(), String> {
        unsafe {
            if !self.rs_idx.is_null() {
                RediSearch_DropIndex(self.rs_idx);
                self.rs_idx = null_mut();
            }
        }
        let stopwords = self.stopwords.clone();
        let language = self.language.clone();
        self.create_rs_index(label, stopwords.as_ref(), language.as_ref())?;
        self.register_fields(self.fields(), None);
        Ok(())
    }
}
