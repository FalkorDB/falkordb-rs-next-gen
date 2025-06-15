#![no_main]

use std::cell::RefCell;

use graph::{
    GraphBLAS::{GrB_Mode, GrB_init},
    ast::VarId,
    functions::init_functions,
    graph::{Graph, Plan},
    runtime::{ReturnCallback, Runtime, evaluate_param},
    value::Env,
};
use hashbrown::HashMap;
use libfuzzer_sys::{Corpus, fuzz_target};

#[derive(Default)]
struct FuzzValuesCollector;

impl ReturnCallback for FuzzValuesCollector {
    fn return_value(
        &self,
        _: &RefCell<Graph>,
        _: Env,
        _: &[VarId],
    ) {
    }
}

fuzz_target!(init: {
        unsafe {
            GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
        }
        init_functions().expect("Failed to init functions");
        let g = RefCell::new(Graph::new(1024, 1024));
    },|data: &[u8]| -> Corpus {
    std::str::from_utf8(data).map_or(Corpus::Reject, |query| {
        let Ok(Plan {
            plan, parameters, ..
        }) = g.borrow().get_plan(query)
        else {
            return Corpus::Reject;
        };
        let Ok(parameters) = parameters
            .into_iter()
            .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
            .collect::<Result<HashMap<_, _>, String>>()
        else {
            return Corpus::Reject;
        };
        let mut runtime = Runtime::new(&g, parameters, true, plan);
        match runtime.query(FuzzValuesCollector) {
            Ok(_) => Corpus::Keep,
            _ => Corpus::Reject,
        }
    })
});
