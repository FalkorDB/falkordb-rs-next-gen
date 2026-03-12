use std::collections::HashMap;

use graph::{
    graph::{
        graph::Plan,
        graphblas::{GrB_Mode, GrB_init},
        mvcc_graph::MvccGraph,
    },
    runtime::{
        functions::init_functions,
        pool::Pool,
        runtime::{Runtime, evaluate_param},
    },
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
            let g = MvccGraph::new(1024, 1024, 25, "fuzz_test");
            let Ok(Plan {
                plan, parameters, ..
            }) = g.read().borrow().get_plan(query)
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
            let pool = Pool::new();
            let mut runtime = Runtime::new(
                g.read(),
                parameters,
                true,
                plan,
                false,
                String::new(),
                &pool,
            );
            let _ = runtime.query();
        }
    });
}
