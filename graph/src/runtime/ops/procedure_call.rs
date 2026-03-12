//! Batch-mode procedure call operator — invokes built-in graph procedures.
//!
//! Implements Cypher `CALL db.procedure(args) YIELD outputs`. Evaluates
//! argument expressions, validates types, invokes the procedure function,
//! and maps the returned list of maps to output environments. This is a
//! *blocking* operator: the procedure is called once and all results are
//! materialized before yielding as batches.
//!
//! Only allowed in write queries when the procedure is marked as write;
//! returns an error for `GRAPH.RO_QUERY` on write procedures.

use std::sync::Arc;

use crate::parser::ast::{QueryExpr, Variable};
use crate::planner::IR;
use crate::runtime::{
    batch::{BATCH_SIZE, Batch},
    env::Env,
    functions::GraphFn,
    runtime::Runtime,
    value::Value,
};
use orx_tree::{Dyn, NodeIdx, NodeRef};
use thin_vec::ThinVec;

pub struct ProcedureCallOp<'a> {
    pub(crate) runtime: &'a Runtime<'a>,
    func: &'a Arc<GraphFn>,
    trees: &'a [QueryExpr<Variable>],
    name_outputs: &'a [Variable],
    batches: Option<std::vec::IntoIter<Batch<'a>>>,
    pub(crate) idx: NodeIdx<Dyn<IR>>,
}

impl<'a> ProcedureCallOp<'a> {
    pub fn new(
        runtime: &'a Runtime<'a>,
        func: &'a Arc<GraphFn>,
        trees: &'a [QueryExpr<Variable>],
        name_outputs: &'a [Variable],
        idx: NodeIdx<Dyn<IR>>,
    ) -> Result<Self, String> {
        if !runtime.write && func.write {
            return Err(String::from(
                "graph.RO_QUERY is to be executed only on read-only queries",
            ));
        }
        Ok(Self {
            runtime,
            func,
            trees,
            name_outputs,
            batches: None,
            idx,
        })
    }

    fn init_batches(&mut self) -> Result<(), String> {
        let args = self
            .trees
            .iter()
            .map(|ir| {
                self.runtime
                    .run_expr(ir, ir.root().idx(), &Env::new(self.runtime.env_pool), None)
            })
            .collect::<Result<ThinVec<_>, _>>()?;
        self.func.validate_args_type(&args)?;
        let res = (self.func.func)(self.runtime, args)?;
        match res {
            Value::List(arr) => {
                let batches: Vec<Batch<'a>> = arr
                    .chunks(BATCH_SIZE)
                    .map(|chunk| {
                        let envs: Vec<Env<'a>> = chunk
                            .iter()
                            .map(|v| {
                                let mut env = Env::new(self.runtime.env_pool);
                                if let Value::Map(map) = v {
                                    for output in self.name_outputs {
                                        let field_name = output.name.as_ref().unwrap();
                                        let value =
                                            map.get(field_name).cloned().unwrap_or(Value::Null);
                                        env.insert(output, value);
                                    }
                                }
                                env
                            })
                            .collect();
                        Batch::from_envs(envs)
                    })
                    .collect();
                self.batches = Some(batches.into_iter());
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> Iterator for ProcedureCallOp<'a> {
    type Item = Result<Batch<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Initialize batches on first call.
        if self.batches.is_none()
            && let Err(e) = self.init_batches()
        {
            return Some(Err(e));
        }

        self.batches.as_mut().unwrap().next().map(Ok)
    }
}
