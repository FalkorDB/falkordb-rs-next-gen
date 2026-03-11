//! Load CSV operator — streams rows from a CSV file or URL.
//!
//! Implements Cypher `LOAD CSV FROM ...`. Supports both local files
//! (via `file://` prefix, restricted to the configured import folder)
//! and remote HTTPS URLs. Each CSV record is bound to a variable as
//! either a `Map` (when headers are present) or a `List` (without headers).
//!
//! ```text
//!  child iter ──► env
//!                  │
//!     ┌────────────┴────────────┐
//!     │  resolve file path     │
//!     │  open CSV reader       │
//!     │  for each record:      │
//!     │    env += {var: record} │
//!     └────────────┬────────────┘
//!                  │
//!              yield Env ──► parent
//! ```

use std::path::Path;
use std::sync::Arc;

use super::OpIter;
use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{env::Env, ordermap::OrderMap, runtime::Runtime, value::Value};
use orx_tree::{Dyn, NodeIdx, NodeRef};
use ureq;

pub struct LoadCsvOp<'a> {
    runtime: &'a Runtime,
    pub iter: Box<OpIter<'a>>,
    current: Option<Box<dyn Iterator<Item = Result<Env, String>> + 'a>>,
    file_path: &'a QueryExpr<Variable>,
    headers: &'a bool,
    delimiter: &'a QueryExpr<Variable>,
    var: &'a Variable,
    idx: NodeIdx<Dyn<IR>>,
}

impl<'a> LoadCsvOp<'a> {
    pub fn new(
        runtime: &'a Runtime,
        iter: Box<OpIter<'a>>,
        file_path: &'a QueryExpr<Variable>,
        headers: &'a bool,
        delimiter: &'a QueryExpr<Variable>,
        var: &'a Variable,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            iter,
            current: None,
            file_path,
            headers,
            delimiter,
            var,
            idx,
        }
    }

    pub fn load_csv_file(
        path: &str,
        headers: bool,
        delimiter: Arc<String>,
        var: &'a Variable,
        vars: &Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(headers)
            .delimiter(delimiter.as_bytes()[0])
            .from_path(path)
            .map_err(|e| format!("Failed to read CSV file: {e}"))?;

        let vars = vars.clone();
        if headers {
            let headers = reader
                .headers()
                .map_err(|e| format!("Failed to read CSV headers: {e}"))?
                .iter()
                .map(|s| Arc::new(String::from(s)))
                .collect::<Vec<_>>();
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::Map(Arc::new(
                                record
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, field)| {
                                        if field.is_empty() {
                                            None
                                        } else {
                                            Some((
                                                headers.get(i).cloned().unwrap_or_else(|| {
                                                    Arc::new(format!("col_{i}"))
                                                }),
                                                Value::String(Arc::new(String::from(field))),
                                            ))
                                        }
                                    })
                                    .collect::<OrderMap<_, _>>(),
                            )),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        } else {
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::List(Arc::new(
                                record
                                    .iter()
                                    .map(|field| {
                                        if field.is_empty() {
                                            Value::Null
                                        } else {
                                            Value::String(Arc::new(String::from(field)))
                                        }
                                    })
                                    .collect(),
                            )),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        }
    }

    pub fn load_csv_url(
        path: &str,
        headers: bool,
        delimiter: Arc<String>,
        var: &'a Variable,
        vars: &Env,
    ) -> Result<Box<dyn Iterator<Item = Result<Env, String>> + 'a>, String> {
        let response = ureq::get(path)
            .call()
            .map_err(|e| format!("Failed to fetch CSV file: {e}"))?
            .into_body()
            .into_reader();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(headers)
            .delimiter(delimiter.as_bytes()[0])
            .from_reader(response);

        let vars = vars.clone();
        if headers {
            let headers = reader
                .headers()
                .map_err(|e| format!("Failed to read CSV headers: {e}"))?
                .iter()
                .map(|s| Arc::new(String::from(s)))
                .collect::<Vec<_>>();
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::Map(Arc::new(
                                record
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, field)| {
                                        if field.is_empty() {
                                            None
                                        } else {
                                            Some((
                                                headers.get(i).cloned().unwrap_or_else(|| {
                                                    Arc::new(format!("col_{i}"))
                                                }),
                                                Value::String(Arc::new(String::from(field))),
                                            ))
                                        }
                                    })
                                    .collect::<OrderMap<_, _>>(),
                            )),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        } else {
            Ok(Box::new(reader.into_records().map(
                move |record| match record {
                    Ok(record) => {
                        let mut env = vars.clone();
                        env.insert(
                            var,
                            Value::List(Arc::new(
                                record
                                    .iter()
                                    .map(|field| {
                                        if field.is_empty() {
                                            Value::Null
                                        } else {
                                            Value::String(Arc::new(String::from(field)))
                                        }
                                    })
                                    .collect(),
                            )),
                        );
                        Ok(env)
                    }
                    Err(e) => Err(format!("Failed to read CSV record: {e}")),
                },
            )))
        }
    }
}

impl Iterator for LoadCsvOp<'_> {
    type Item = Result<Env, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut current) = self.current {
                if let Some(item) = current.next() {
                    self.runtime.inspect_result(self.idx, &item);
                    return Some(item);
                }
                self.current = None;
            }
            let vars = match self.iter.next()? {
                Ok(vars) => vars,
                Err(e) => {
                    let result = Err(e);
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
            };
            let path = match self.runtime.run_expr(
                self.file_path,
                self.file_path.root().idx(),
                &vars,
                None,
            ) {
                Ok(v) => v,
                Err(e) => {
                    let result = Err(e);
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
            };
            let delimiter = match self.runtime.run_expr(
                self.delimiter,
                self.delimiter.root().idx(),
                &vars,
                None,
            ) {
                Ok(Value::String(s) | Value::InternedString(s)) => s,
                Ok(_) => {
                    let result = Err(String::from("Delimiter must be a string"));
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
                Err(e) => {
                    let result = Err(e);
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
            };
            if delimiter.len() != 1 {
                let result = Err(String::from("Delimiter must be a single character"));
                self.runtime.inspect_result(self.idx, &result);
                return Some(result);
            }
            let (Value::String(path) | Value::InternedString(path)) = path else {
                let result = Err(String::from("File path must be a string"));
                self.runtime.inspect_result(self.idx, &result);
                return Some(result);
            };
            let path = if let Some(path) = path.strip_prefix("file://") {
                let path = self.runtime.import_folder.clone() + path;
                let import_folder = match Path::new(&self.runtime.import_folder).canonicalize() {
                    Ok(p) => p,
                    Err(e) => {
                        let result = Err(format!(
                            "Failed to canonicalize import folder path '{}': {e}",
                            self.runtime.import_folder
                        ));
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                };
                let cpath = match Path::new(&path).canonicalize() {
                    Ok(p) => p,
                    Err(e) => {
                        let result = Err(format!("Failed to canonicalize file path '{path}': {e}"));
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                };
                if !cpath.starts_with(&import_folder) {
                    let result = Err(format!(
                        "File path '{path}' is not within the import folder '{}'",
                        self.runtime.import_folder
                    ));
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
                path
            } else if path.starts_with("https://") {
                String::from(path.as_str())
            } else {
                let result = Err(String::from("File path must start with 'file://' prefix"));
                self.runtime.inspect_result(self.idx, &result);
                return Some(result);
            };

            let csv_result = if path.starts_with("https://") {
                Self::load_csv_url(&path, *self.headers, delimiter, self.var, &vars)
            } else {
                Self::load_csv_file(&path, *self.headers, delimiter, self.var, &vars)
            };
            match csv_result {
                Ok(iter) => {
                    self.current = Some(iter);
                }
                Err(e) => {
                    let result = Err(e);
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
            }
        }
    }
}
