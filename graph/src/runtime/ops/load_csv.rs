//! Batch-mode load CSV operator — streams rows from a CSV file or URL.
//!
//! Implements Cypher `LOAD CSV FROM ...`. For each active row in each input
//! batch, resolves the file path and delimiter, opens a CSV reader, and
//! expands each CSV record into output rows. Output rows are accumulated
//! into batches of up to `BATCH_SIZE`.

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch, BatchOp},
    env::Env,
    ordermap::OrderMap,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct LoadCsvOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    pub(crate) child: Box<BatchOp<'a>>,
    pending: VecDeque<Env<'a>>,
    file_path: &'a QueryExpr<Variable>,
    headers: &'a bool,
    delimiter: &'a QueryExpr<Variable>,
    var: &'a Variable,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> LoadCsvOp<'a> {
    pub const fn new(
        runtime: &'a Runtime<'a>,
        child: Box<BatchOp<'a>>,
        file_path: &'a QueryExpr<Variable>,
        headers: &'a bool,
        delimiter: &'a QueryExpr<Variable>,
        var: &'a Variable,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            child,
            pending: VecDeque::new(),
            file_path,
            headers,
            delimiter,
            var,
            idx,
        }
    }

    fn load_csv_records(
        &self,
        path: &str,
        delimiter: &Arc<String>,
        vars: &Env<'a>,
    ) -> Result<Vec<Env<'a>>, String> {
        let mut results = Vec::new();

        if path.starts_with("https://") {
            let response = ureq::get(path)
                .call()
                .map_err(|e| format!("Failed to fetch CSV file: {e}"))?
                .into_body()
                .into_reader();
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(*self.headers)
                .delimiter(delimiter.as_bytes()[0])
                .from_reader(response);
            self.collect_records(&mut reader, vars, &mut results)?;
        } else {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(*self.headers)
                .delimiter(delimiter.as_bytes()[0])
                .from_path(path)
                .map_err(|e| format!("Failed to read CSV file: {e}"))?;
            self.collect_records(&mut reader, vars, &mut results)?;
        }

        Ok(results)
    }

    fn collect_records<R: std::io::Read>(
        &self,
        reader: &mut csv::Reader<R>,
        vars: &Env<'a>,
        results: &mut Vec<Env<'a>>,
    ) -> Result<(), String> {
        if *self.headers {
            let headers = reader
                .headers()
                .map_err(|e| format!("Failed to read CSV headers: {e}"))?
                .iter()
                .map(|s| Arc::new(String::from(s)))
                .collect::<Vec<_>>();
            for record in reader.records() {
                let record = record.map_err(|e| format!("Failed to read CSV record: {e}"))?;
                let mut env = vars.clone_pooled(self.runtime.env_pool);
                env.insert(
                    self.var,
                    Value::Map(Arc::new(
                        record
                            .iter()
                            .enumerate()
                            .filter_map(|(i, field)| {
                                if field.is_empty() {
                                    None
                                } else {
                                    Some((
                                        headers
                                            .get(i)
                                            .cloned()
                                            .unwrap_or_else(|| Arc::new(format!("col_{i}"))),
                                        Value::String(Arc::new(String::from(field))),
                                    ))
                                }
                            })
                            .collect::<OrderMap<_, _>>(),
                    )),
                );
                results.push(env);
            }
        } else {
            for record in reader.records() {
                let record = record.map_err(|e| format!("Failed to read CSV record: {e}"))?;
                let mut env = vars.clone_pooled(self.runtime.env_pool);
                env.insert(
                    self.var,
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
                results.push(env);
            }
        }
        Ok(())
    }

    /// Drains rows from `self.pending` into `envs` until `BATCH_SIZE` is reached
    /// or all pending rows are exhausted.
    fn drain_pending(
        &mut self,
        envs: &mut Vec<Env<'a>>,
    ) {
        while envs.len() < BATCH_SIZE {
            if let Some(row) = self.pending.pop_front() {
                envs.push(row);
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for LoadCsvOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut envs = Vec::with_capacity(BATCH_SIZE);

        // Drain leftover rows from previous call.
        self.drain_pending(&mut envs);

        while envs.len() < BATCH_SIZE {
            let batch = match self.child.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            };

            for vars in batch.active_env_iter() {
                let path = match self.runtime.run_expr(
                    self.file_path,
                    self.file_path.root().idx(),
                    vars,
                    None,
                ) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let delimiter = match self.runtime.run_expr(
                    self.delimiter,
                    self.delimiter.root().idx(),
                    vars,
                    None,
                ) {
                    Ok(Value::String(s) | Value::InternedString(s)) => s,
                    Ok(_) => return Some(Err(String::from("Delimiter must be a string"))),
                    Err(e) => return Some(Err(e)),
                };
                if delimiter.len() != 1 {
                    return Some(Err(String::from("Delimiter must be a single character")));
                }
                let (Value::String(path) | Value::InternedString(path)) = path else {
                    return Some(Err(String::from("File path must be a string")));
                };
                let path = if let Some(path) = path.strip_prefix("file://") {
                    let path = self.runtime.import_folder.clone() + path;
                    let import_folder = match Path::new(&self.runtime.import_folder).canonicalize()
                    {
                        Ok(p) => p,
                        Err(e) => {
                            return Some(Err(format!(
                                "Failed to canonicalize import folder path '{}': {e}",
                                self.runtime.import_folder
                            )));
                        }
                    };
                    let cpath = match Path::new(&path).canonicalize() {
                        Ok(p) => p,
                        Err(e) => {
                            return Some(Err(format!(
                                "Failed to canonicalize file path '{path}': {e}"
                            )));
                        }
                    };
                    if !cpath.starts_with(&import_folder) {
                        return Some(Err(format!(
                            "File path '{path}' is not within the import folder '{}'",
                            self.runtime.import_folder
                        )));
                    }
                    path
                } else if path.starts_with("https://") {
                    String::from(path.as_str())
                } else {
                    return Some(Err(String::from(
                        "File path must start with 'file://' prefix",
                    )));
                };

                // Read CSV and expand rows
                match self.load_csv_records(&path, &delimiter, vars) {
                    Ok(rows) => {
                        self.pending.extend(rows);
                    }
                    Err(e) => return Some(Err(e)),
                }

                self.drain_pending(&mut envs);

                if envs.len() >= BATCH_SIZE {
                    break;
                }
            }
        }

        if envs.is_empty() {
            None
        } else {
            Some(Ok(Batch::from_envs(envs)))
        }
    }
}
