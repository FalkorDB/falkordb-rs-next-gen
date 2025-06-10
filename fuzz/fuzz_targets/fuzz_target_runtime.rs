#![no_main]

use std::cell::RefCell;

use graph::{
    GraphBLAS::{GrB_Mode, GrB_init},
    ast::VarId,
    functions::init_functions,
    graph::Graph,
    matrix,
    runtime::{ReturnCallback, Runtime},
    value::Env,
};
use libfuzzer_sys::{Corpus, fuzz_target};

#[derive(Default)]
struct FuzzValuesCollector;

impl ReturnCallback for FuzzValuesCollector {
    fn return_value(
        &self,
        graph: &RefCell<Graph>,
        env: Env,
        return_names: &Vec<VarId>,
    ) {
    }
}

fuzz_target!(|data: &[u8]| -> Corpus {
    unsafe {
        GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
    }
    init_functions();
    let g = RefCell::new(Graph::new(1024, 1024));
    let res = std::str::from_utf8(data).map_or(Corpus::Reject, |query| {
        let Ok((plan, parameters, _, _)) = g.borrow().get_plan(query) else {
            return Corpus::Reject;
        };
        let mut runtime = Runtime::new(&g, parameters, true, plan);
        match runtime.query(FuzzValuesCollector) {
            Ok(_) => Corpus::Keep,
            _ => Corpus::Reject,
        }
    });
    // matrix::shutdown();
    res
});
