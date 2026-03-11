//! Fulltext scan operator — retrieves nodes via a fulltext index query.
//!
//! Implements `CALL db.idx.fulltext.queryNodes(label, query)`. Evaluates
//! the label and query expressions, then delegates to the graph's fulltext
//! index. Each matching node is yielded with an optional relevance score.

use super::OpIter;
use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{env::Env, runtime::Runtime, value::Value};
use orx_tree::{Dyn, NodeIdx, NodeRef};

pub struct NodeByFulltextScanOp<'a> {
    runtime: &'a Runtime,
    pub iter: Box<OpIter<'a>>,
    current: Option<Box<dyn Iterator<Item = Result<Env, String>> + 'a>>,
    node: &'a Variable,
    label: &'a QueryExpr<Variable>,
    query: &'a QueryExpr<Variable>,
    score: &'a Option<Variable>,
    idx: NodeIdx<Dyn<IR>>,
}

impl<'a> NodeByFulltextScanOp<'a> {
    pub fn new(
        runtime: &'a Runtime,
        iter: Box<OpIter<'a>>,
        node: &'a Variable,
        label: &'a QueryExpr<Variable>,
        query: &'a QueryExpr<Variable>,
        score: &'a Option<Variable>,
        idx: NodeIdx<Dyn<IR>>,
    ) -> Self {
        Self {
            runtime,
            iter,
            current: None,
            node,
            label,
            query,
            score,
            idx,
        }
    }
}

impl Iterator for NodeByFulltextScanOp<'_> {
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
            let label_str =
                match self
                    .runtime
                    .run_expr(self.label, self.label.root().idx(), &vars, None)
                {
                    Ok(Value::String(s) | Value::InternedString(s)) => s,
                    Ok(_) => {
                        let result = Err("fulltext query expects a string label".into());
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                    Err(e) => {
                        let result = Err(e);
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                };
            let query_str =
                match self
                    .runtime
                    .run_expr(self.query, self.query.root().idx(), &vars, None)
                {
                    Ok(Value::String(s) | Value::InternedString(s)) => s,
                    Ok(_) => {
                        let result = Err("fulltext query expects a string query".into());
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                    Err(e) => {
                        let result = Err(e);
                        self.runtime.inspect_result(self.idx, &result);
                        return Some(result);
                    }
                };
            let value = self
                .runtime
                .g
                .borrow()
                .fulltext_query_nodes(&label_str, &query_str);
            let fulltext_results = match value {
                Ok(iter) => iter,
                Err(e) => {
                    let result = Err(e);
                    self.runtime.inspect_result(self.idx, &result);
                    return Some(result);
                }
            };
            let node = self.node;
            let score = self.score;
            self.current = Some(Box::new(fulltext_results.map(move |(node_id, s)| {
                let mut vars = vars.clone();
                vars.insert(node, Value::Node(node_id));
                if let Some(score) = score {
                    vars.insert(score, Value::Float(s));
                }
                Ok(vars)
            })));
        }
    }
}
