use graph::functions::init_functions;
use graph::runtime::{ReturnCallback, Runtime};
use graph::value::Env;
use graph::{cypher::Parser, graph::Graph, matrix::init, planner::Planner, value::Value};
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use opentelemetry_zipkin::ZipkinExporter;
use redis_module::RedisModuleIO;
use redis_module::{
    Context, NextArg, REDISMODULE_TYPE_METHOD_VERSION, RedisError, RedisModule_Alloc,
    RedisModule_Calloc, RedisModule_Free, RedisModule_Realloc, RedisModuleTypeMethods, RedisResult,
    RedisString, RedisValue, Status, native_types::RedisType, redis_module,
};
use std::cell::RefCell;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::rc::Rc;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const EMPTY_KEY_ERR: RedisResult = Err(RedisError::Str("ERR Invalid graph operation on empty key"));

static GRAPH_TYPE: RedisType = RedisType::new(
    "graphdata",
    0,
    RedisModuleTypeMethods {
        version: REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(graph_rdb_load),
        rdb_save: Some(graph_rdb_save),
        aof_rewrite: None,
        free: Some(my_free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save2: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,

        copy2: None,
        free_effort2: None,
        mem_usage2: None,
        unlink2: None,
    },
);

#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_load(
    _: *mut RedisModuleIO,
    _: i32,
) -> *mut c_void {
    null_mut()
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn graph_rdb_save(
    _: *mut RedisModuleIO,
    _: *mut c_void,
) {
}

#[unsafe(no_mangle)]
unsafe extern "C" fn my_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<RefCell<Graph>>()));
    }
}

fn raw_value_to_redis_value(
    g: &RefCell<Graph>,
    mut env: Env,
    return_names: &Vec<Rc<String>>,
) -> RedisValue {
    return_names
        .iter()
        .map(|v| inner_raw_value_to_redis_value(g, env.take(v).unwrap()))
        .collect::<Vec<RedisValue>>()
        .into()
}

fn inner_raw_value_to_redis_value(
    g: &RefCell<Graph>,
    r: Value,
) -> RedisValue {
    match r {
        Value::Null => RedisValue::Array(vec![RedisValue::Integer(1), RedisValue::Null]),
        Value::Bool(x) => RedisValue::Array(vec![
            RedisValue::Integer(4),
            RedisValue::SimpleStringStatic(if x { "true" } else { "false" }),
        ]),
        Value::Int(x) => RedisValue::Array(vec![RedisValue::Integer(3), RedisValue::Integer(x)]),
        Value::Float(x) => RedisValue::Array(vec![
            RedisValue::Integer(5),
            RedisValue::SimpleString(format!("{:.14e}", x)),
        ]),
        Value::String(x) => RedisValue::Array(vec![
            RedisValue::Integer(2),
            RedisValue::BulkString(x.to_string()),
        ]),
        Value::List(values) => RedisValue::Array(vec![
            RedisValue::Integer(6),
            RedisValue::Array(
                values
                    .into_iter()
                    .map(|v| inner_raw_value_to_redis_value(g, v))
                    .collect(),
            ),
        ]),
        Value::Map(map) => {
            let mut vec = vec![];
            for (key, value) in map {
                vec.push(RedisValue::BulkString(key.to_string()));
                vec.push(inner_raw_value_to_redis_value(g, value));
            }
            RedisValue::Array(vec![RedisValue::Integer(10), RedisValue::Array(vec)])
        }
        Value::Node(id) => {
            let mut props = Vec::new();
            for (key, value) in g.borrow().get_node_properties(id) {
                let mut prop = Vec::new();
                prop.push(RedisValue::Integer(*key as _));
                if let RedisValue::Array(mut v) = inner_raw_value_to_redis_value(g, value.clone()) {
                    prop.append(&mut v);
                }
                props.push(RedisValue::Array(prop));
            }
            RedisValue::Array(vec![
                RedisValue::Integer(8),
                RedisValue::Array(vec![
                    RedisValue::Integer(id as _),
                    RedisValue::Array(
                        g.borrow()
                            .get_node_label_ids(id)
                            .map(|l| RedisValue::Integer(l as _))
                            .collect(),
                    ),
                    RedisValue::Array(props),
                ]),
            ])
        }
        Value::Relationship(id, from, to) => {
            let mut props = Vec::new();
            for (key, value) in g.borrow().get_relationship_properties(id) {
                let mut prop = Vec::new();
                prop.push(RedisValue::Integer(*key as _));
                if let RedisValue::Array(mut v) = inner_raw_value_to_redis_value(g, value.clone()) {
                    prop.append(&mut v);
                }
                props.push(RedisValue::Array(prop));
            }
            RedisValue::Array(vec![
                RedisValue::Integer(7),
                RedisValue::Array(vec![
                    RedisValue::Integer(id as _),
                    RedisValue::Integer(g.borrow().get_relationship_type_id(id) as _),
                    RedisValue::Integer(from as _),
                    RedisValue::Integer(to as _),
                    RedisValue::Array(props),
                ]),
            ])
        }
        Value::Path(path) => {
            let mut nodes = Vec::new();
            let mut rels = Vec::new();
            for node in path {
                match node {
                    Value::Node(id) => nodes.push(RedisValue::Integer(id as _)),
                    Value::Relationship(id, from, to) => {
                        rels.push(RedisValue::Array(vec![
                            RedisValue::Integer(id as _),
                            RedisValue::Integer(from as _),
                            RedisValue::Integer(to as _),
                        ]));
                    }
                    _ => unreachable!("Path should only contain nodes and relationships"),
                }
            }
            RedisValue::Array(vec![
                RedisValue::Integer(9),
                RedisValue::Array(vec![RedisValue::Array(nodes), RedisValue::Array(rels)]),
            ])
        }
    }
}

struct RedisValuesCollector {
    res: RefCell<Vec<RedisValue>>,
}

impl RedisValuesCollector {
    const fn new() -> Self {
        Self {
            res: RefCell::new(Vec::new()),
        }
    }

    fn take(&self) -> Vec<RedisValue> {
        std::mem::take(&mut *self.res.borrow_mut())
    }
}

impl ReturnCallback for RedisValuesCollector {
    fn return_value(
        &self,
        graph: &RefCell<Graph>,
        env: Env,
        return_names: &Vec<Rc<String>>,
    ) {
        self.res
            .borrow_mut()
            .push(raw_value_to_redis_value(graph, env, return_names));
    }
}

/// This function is used to delete a graph
///
/// See: https://docs.falkordb.com/commands/graph.delete.html
///
/// # Example
///
/// ```sh
/// 127.0.0.1:6379> GRAPH.DELETE graph
/// OK
/// ```
fn graph_delete(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let key = ctx.open_key_writable(&key);
    if key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)?.is_some() {
        key.delete()
    } else {
        EMPTY_KEY_ERR
    }
}

#[inline]
fn query_mut(
    graph: &RefCell<Graph>,
    query: &str,
) -> Result<RedisValue, RedisError> {
    // Create a child span for parsing and execution
    tracing::debug_span!("query_execution", query = %query).in_scope(|| {
        let collector = RedisValuesCollector::new();
        let (plan, parameters, _, _) =
            graph.borrow().get_plan(query).map_err(RedisError::String)?;
        let mut runtime = Runtime::new(graph, parameters, true, plan);
        runtime
            .query(&collector)
            .map(|summary| {
                let mut stats = vec![];
                if summary.labels_added > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Labels added: {}",
                        summary.labels_added
                    )));
                }
                if summary.nodes_created > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Nodes created: {}",
                        summary.nodes_created
                    )));
                }
                if summary.nodes_deleted > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Nodes deleted: {}",
                        summary.nodes_deleted
                    )));
                }
                if summary.properties_set > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Properties set: {}",
                        summary.properties_set
                    )));
                }
                if summary.relationships_created > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Relationships created: {}",
                        summary.relationships_created
                    )));
                }
                if summary.relationships_deleted > 0 {
                    stats.push(RedisValue::SimpleString(format!(
                        "Relationships deleted: {}",
                        summary.relationships_deleted
                    )));
                }
                let columns = summary
                    .return_names
                    .into_iter()
                    .map(|n| vec![RedisValue::Integer(1), RedisValue::BulkString(n)].into())
                    .collect();
                vec![columns, collector.take(), stats].into()
            })
            .map_err(RedisError::String)
    })
}

fn graph_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)? {
        query_mut(graph, query)
    } else {
        let value = RefCell::new(Graph::new(16384, 16384));
        let res = query_mut(&value, query);
        key.set_value(&GRAPH_TYPE, value)?;
        res
    }
}

/// This function is used to execute a read only query on a graph
///
/// See: https://docs.falkordb.com/commands/graph.ro_query.html
///
/// # Example
///
/// ```sh
/// GRAPH.RO_QUERY graph "MATCH (n) RETURN n"
/// ```
fn graph_ro_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;

    let key = ctx.open_key(&key);

    // We check if the key exists and is of type Graph if wrong type `get_value` return an error
    (key.get_value::<RefCell<Graph>>(&GRAPH_TYPE)?).map_or(
        // If the key does not exist, we return an error
        EMPTY_KEY_ERR,
        |graph| {
            let collector = RedisValuesCollector::new();
            let (plan, parameters, _, _) =
                graph.borrow().get_plan(query).map_err(RedisError::String)?;
            let mut runtime = Runtime::new(graph, parameters, false, plan);
            match runtime.query(&collector) {
                Ok(summary) => Ok(vec![
                    summary
                        .return_names
                        .into_iter()
                        .map(|n| vec![RedisValue::Integer(1), RedisValue::BulkString(n)].into())
                        .collect(),
                    collector.take(),
                    vec![],
                ]
                .into()),
                Err(err) => Err(RedisError::String(err)),
            }
        },
    )
}

/// This function is used to list all the graphs
/// in the database. It returns a list of graphs IDs
/// that are currently stored in the database.
///
/// See: https://docs.falkordb.com/commands/graph.list.html
///
/// # Example
///
/// ```sh
/// 127.0.0.1:6379> GRAPH.LIST
/// 2) G
/// 3) resources
/// 4) players
/// ```
fn graph_list(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    if args.len() != 1 {
        return Err(RedisError::WrongArity);
    }

    let mut a = [
        ctx.create_string("0"),
        ctx.create_string("TYPE"),
        ctx.create_string("graphdata"),
    ];
    let mut res = Vec::new();
    loop {
        let call_res = ctx.call("SCAN", a.iter().collect::<Vec<_>>().as_slice())?;
        match call_res {
            RedisValue::Array(mut arr) => {
                if let RedisValue::Array(arr) = arr.remove(1) {
                    res.extend(arr);
                }
                if let RedisValue::SimpleString(i) = arr.remove(0) {
                    if i == "0" {
                        return Ok(RedisValue::Array(res));
                    }
                    a[0] = ctx.create_string(i);
                }
            }
            _ => return Err(RedisError::Str("ERR Failed to list graphs")),
        }
    }
}

fn graph_parse(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let query = args.next_str()?;

    let mut parser = Parser::new(query);
    match parser.parse() {
        Ok(ir) => Ok(RedisValue::BulkString(format!("{ir}"))),
        Err(err) => Err(RedisError::String(err)),
    }
}

fn graph_plan(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let query = args.next_str()?;

    let mut parser = Parser::new(query);
    match parser.parse() {
        Ok(ir) => {
            let planner = Planner::new();
            let ir = planner.plan(ir);
            Ok(RedisValue::BulkString(format!("{ir}")))
        }
        Err(err) => Err(RedisError::String(err)),
    }
}

#[cfg(zipkin)]
fn init_zipkin() {
    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());

    let exporter = ZipkinExporter::builder().build().unwrap();

    let batch = BatchSpanProcessor::builder(exporter)
        .with_batch_config(
            BatchConfigBuilder::default()
                .with_max_queue_size(4096)
                .build(),
        )
        .build();

    let provider = SdkTracerProvider::builder()
        .with_span_processor(batch)
        .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
        .with_resource(
            Resource::builder_empty()
                .with_service_name("falkordb-graph-engine")
                .build(),
        )
        .build();
    let tracer = provider.tracer("falkordb-graph-engine");
    let layer = OpenTelemetryLayer::new(tracer);
    tracing_subscriber::registry().with(layer).init();

    global::set_tracer_provider(provider);
}

fn graph_init(
    _: &Context,
    _: &Vec<RedisString>,
) -> Status {
    #[cfg(zipkin)]
    init_zipkin();

    unsafe {
        init(
            RedisModule_Alloc,
            RedisModule_Calloc,
            RedisModule_Realloc,
            RedisModule_Free,
        );
    }
    match init_functions() {
        Ok(_) => Status::Ok,
        Err(_) => Status::Err,
    }
}

//////////////////////////////////////////////////////

redis_module! {
    name: "falkordb",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [GRAPH_TYPE],
    init: graph_init,
    commands: [
        ["graph.DELETE", graph_delete, "write", 1, 1, 1, ""],
        ["graph.QUERY", graph_query, "write deny-oom", 1, 1, 1, ""],
        ["graph.RO_QUERY", graph_ro_query, "readonly", 1, 1, 1, ""],
        ["graph.LIST", graph_list, "readonly", 0, 0, 0, ""],
        ["graph.PARSE", graph_parse, "readonly", 0, 0, 0, ""],
        ["graph.PLAN", graph_plan, "readonly", 0, 0, 0, ""],
    ],
}
