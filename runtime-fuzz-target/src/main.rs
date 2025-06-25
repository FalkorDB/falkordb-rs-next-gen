use std::{cell::RefCell, collections::HashMap};

use graph::{
    GraphBLAS::{GrB_Mode, GrB_init},
    functions::init_functions,
    graph::{Graph, Plan},
    runtime::{Runtime, evaluate_param},
};

#[macro_use]
extern crate afl;

fn main() {
    unsafe {
        GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
    }
    init_functions().expect("Failed to init functions");
    fuzz!(|data: &[u8]| {
        if let Ok(query) = std::str::from_utf8(data) {
            let g = RefCell::new(Graph::new(1024, 1024));
            let Ok(Plan {
                plan, parameters, ..
            }) = g.borrow().get_plan(query)
            else {
                return;
            };
            let Ok(parameters) = parameters
                .into_iter()
                .map(|(k, v)| Ok((k, evaluate_param(&v.root())?)))
                .collect::<Result<HashMap<_, _>, String>>()
            else {
                return;
            };
            let mut runtime = Runtime::new(&g, parameters, true, plan);
            let _ = runtime.query();
        }
    });
}
