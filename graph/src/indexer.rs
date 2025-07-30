use std::{
    collections::HashMap,
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
    ptr::null_mut,
    rc::Rc,
};

use crate::{
    redisearch::{
        GC_POLICY_FORK, REDISEARCH_ADD_REPLACE, RSFLDOPT_NONE, RSFLDTYPE_GEO, RSFLDTYPE_NUMERIC,
        RSFLDTYPE_TAG, RSIndex, RediSearch_CreateDocument2, RediSearch_CreateField,
        RediSearch_CreateIndex, RediSearch_CreateIndexOptions, RediSearch_CreateNumericNode,
        RediSearch_CreateTagNode, RediSearch_CreateTagTokenNode, RediSearch_DeleteDocument,
        RediSearch_DocumentAddFieldNumber, RediSearch_DocumentAddFieldString, RediSearch_DropIndex,
        RediSearch_FreeIndexOptions, RediSearch_GetResultsIterator, RediSearch_IndexAddDocument,
        RediSearch_IndexOptionsSetGCPolicy, RediSearch_IndexOptionsSetStopwords,
        RediSearch_QueryNodeAddChild, RediSearch_ResultsIteratorFree,
        RediSearch_ResultsIteratorNext, RediSearch_TagFieldSetCaseSensitive,
        RediSearch_TagFieldSetSeparator,
    },
    runtime::value::Value,
};

#[derive(Clone)]
pub struct Document {
    id: u64,
    columns: HashMap<Rc<String>, Value>,
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
        key: Rc<String>,
        value: Value,
    ) {
        self.columns.insert(key, value);
    }
}

#[derive(Debug)]
pub enum IndexQuery<T> {
    Equal(Rc<String>, T),
    Range(Rc<String>, Option<T>, Option<T>),
    And(Vec<IndexQuery<T>>),
    Or(Vec<IndexQuery<T>>),
}

struct Index {
    index: *mut RSIndex,
    fields: HashMap<Rc<String>, Rc<CString>>,
    add_docs: Vec<Document>,
    remove_docs: Vec<u64>,
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            RediSearch_DropIndex(self.index);
        }
    }
}

#[derive(Default)]
pub struct Indexer {
    index: HashMap<u64, Index>,
}

impl Indexer {
    pub fn create_index(
        &mut self,
        label: u64,
        attrs: &Vec<Rc<String>>,
    ) {
        unsafe {
            let options = RediSearch_CreateIndexOptions();
            // RediSearch_IndexOptionsSetLanguage(options, idx->language);
            RediSearch_IndexOptionsSetGCPolicy(options, GC_POLICY_FORK as _);
            RediSearch_IndexOptionsSetStopwords(options, null_mut(), 0);

            let index = RediSearch_CreateIndex(c"index".as_ptr().cast::<i8>(), options);
            RediSearch_FreeIndexOptions(options);

            let mut fields = HashMap::new();

            for attr in attrs {
                let types = RSFLDTYPE_NUMERIC | RSFLDTYPE_GEO | RSFLDTYPE_TAG;
                let msg = CString::new(attr.as_str()).unwrap();
                let field_id = RediSearch_CreateField(index, msg.as_ptr(), types, RSFLDOPT_NONE);
                fields.insert(attr.clone(), Rc::new(msg));

                RediSearch_TagFieldSetSeparator(index, field_id, 1 as c_char);
                RediSearch_TagFieldSetCaseSensitive(index, field_id, 1);
            }

            self.index.insert(
                label,
                Index {
                    index,
                    fields,
                    add_docs: Vec::new(),
                    remove_docs: Vec::new(),
                },
            );
        }
    }

    pub fn drop_index(
        &mut self,
        label: u64,
        attrs: &Vec<Rc<String>>,
    ) -> Option<Vec<Rc<String>>> {
        if let Some(index) = self.index.get_mut(&label) {
            let mut removed = false;
            for attr in attrs {
                if index.fields.remove(attr).is_some() {
                    removed = true;
                }
            }
            if index.fields.is_empty() {
                self.index.remove(&label).unwrap();
                return None;
            }
            if removed {
                return Some(index.fields.keys().cloned().collect());
            }
        }
        None
    }

    #[must_use]
    pub fn is_label_indexed(
        &self,
        label: u64,
    ) -> bool {
        self.index.contains_key(&label)
    }

    #[must_use]
    pub fn is_attr_indexed(
        &self,
        label: u64,
        key: Rc<String>,
    ) -> bool {
        if let Some(index) = self.index.get(&label) {
            return index.fields.contains_key(&key);
        }
        false
    }

    #[must_use]
    pub fn query(
        &self,
        label: u64,
        query: IndexQuery<Value>,
    ) -> Vec<u64> {
        if let Some(index) = self.index.get(&label) {
            match query {
                IndexQuery::Equal(key, Value::Int(value)) => unsafe {
                    let msg = index.fields.get(&key).unwrap();
                    let query = RediSearch_CreateNumericNode(
                        index.index,
                        msg.as_ptr(),
                        value as f64,
                        value as f64,
                        1,
                        1,
                    );
                    let iter = RediSearch_GetResultsIterator(query, index.index);

                    let mut res = vec![];
                    loop {
                        let node_id = RediSearch_ResultsIteratorNext(iter, index.index, null_mut())
                            .cast::<u64>();
                        if node_id.is_null() {
                            break;
                        }
                        res.push(node_id.read_unaligned());
                    }
                    RediSearch_ResultsIteratorFree(iter);
                    return res;
                },
                IndexQuery::Equal(key, Value::String(value)) => unsafe {
                    let msg = index.fields.get(&key).unwrap();
                    let query = RediSearch_CreateTagNode(index.index, msg.as_ptr());
                    let msg = CString::new(value.as_str()).unwrap();
                    let child =
                        RediSearch_CreateTagTokenNode(index.index, msg.as_ptr().cast::<c_char>());
                    RediSearch_QueryNodeAddChild(query, child);
                    let iter = RediSearch_GetResultsIterator(query, index.index);

                    let mut res = vec![];
                    loop {
                        let node_id = RediSearch_ResultsIteratorNext(iter, index.index, null_mut())
                            .cast::<u64>();
                        if node_id.is_null() {
                            break;
                        }
                        res.push(node_id.read_unaligned());
                    }
                    RediSearch_ResultsIteratorFree(iter);
                    return res;
                },
                _ => todo!(),
            }
        }

        vec![]
    }

    pub fn add(
        &mut self,
        label: u64,
        doc: Document,
    ) {
        if let Some(index) = self.index.get_mut(&label) {
            index.add_docs.push(doc);
        }
    }

    pub fn remove(
        &mut self,
        label: u64,
        id: u64,
    ) {
        if let Some(index) = self.index.get_mut(&label) {
            index.remove_docs.push(id);
        }
    }

    pub fn commit(&mut self) {
        for index in self.index.values_mut() {
            for doc in index.add_docs.drain(..) {
                unsafe {
                    let rs_doc = RediSearch_CreateDocument2(
                        (&raw const doc.id).cast::<c_void>(),
                        8,
                        null_mut(),
                        1.0,
                        null_mut(),
                    );

                    for (key, value) in doc.columns {
                        let msg = index.fields.get(&key).unwrap();
                        match value {
                            Value::Bool(i) => {
                                RediSearch_DocumentAddFieldNumber(
                                    rs_doc,
                                    msg.as_ptr(),
                                    f64::from(i),
                                    RSFLDTYPE_NUMERIC,
                                );
                            }
                            Value::Int(i) => {
                                RediSearch_DocumentAddFieldNumber(
                                    rs_doc,
                                    msg.as_ptr(),
                                    i as f64,
                                    RSFLDTYPE_NUMERIC,
                                );
                            }
                            Value::Float(i) => {
                                RediSearch_DocumentAddFieldNumber(
                                    rs_doc,
                                    msg.as_ptr(),
                                    i,
                                    RSFLDTYPE_NUMERIC,
                                );
                            }
                            Value::String(s) => {
                                RediSearch_DocumentAddFieldString(
                                    rs_doc,
                                    msg.as_ptr(),
                                    s.as_ptr().cast::<i8>(),
                                    s.len(),
                                    RSFLDTYPE_TAG,
                                );
                            }
                            Value::List(values) => todo!(),
                            Value::VecF32(items) => todo!(),
                            Value::Null
                            | Value::Map(_)
                            | Value::Node(_)
                            | Value::Relationship(_, _, _)
                            | Value::Path(_)
                            | Value::Rc(_) => unreachable!(),
                        }
                    }

                    let res = RediSearch_IndexAddDocument(
                        index.index,
                        rs_doc,
                        REDISEARCH_ADD_REPLACE as c_int,
                        null_mut(),
                    );
                    debug_assert_eq!(res, 0);
                }
            }
            for id in index.remove_docs.drain(..) {
                unsafe {
                    RediSearch_DeleteDocument(index.index, (&raw const id).cast::<c_void>(), 8)
                };
            }
        }
    }

    #[must_use]
    pub fn get_fields(
        &self,
        label: u64,
    ) -> Vec<Rc<String>> {
        self.index
            .get(&label)
            .map(|index| index.fields.keys().cloned().collect())
            .unwrap_or_default()
    }
}
