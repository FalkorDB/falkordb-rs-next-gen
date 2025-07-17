#![no_main]

use std::{cell::RefCell, collections::HashMap};

use graph::{
    graph::{
        GraphBLAS::{GrB_Mode, GrB_init},
        graph::{Graph, Plan},
    },
    runtime::{
        functions::init_functions,
        runtime::{Runtime, evaluate_param},
    },
};
use libfuzzer_sys::{Corpus, fuzz_target};

fuzz_target!(init: {
        unsafe {
            GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
        }
        init_functions().expect("Failed to init functions");
    },|data: &[u8]| -> Corpus {
        let g = RefCell::new(Graph::new(1024, 1024, 25));
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
        let mut runtime = Runtime::new(&g, parameters, true, plan, false, String::new());
        match runtime.query() {
            Ok(_) => Corpus::Keep,
            _ => Corpus::Reject,
        }
    })
});
