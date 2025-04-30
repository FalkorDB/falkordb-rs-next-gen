use graph::{cypher::Parser, graph::Graph, planner::Planner, value::Value};
use redis_module::{
    Context, NextArg, REDISMODULE_TYPE_METHOD_VERSION, RedisError, RedisModuleTypeMethods,
    RedisResult, RedisString, RedisValue, Status, native_types::RedisType, redis_module,
    redisvalue::RedisValueKey,
};
use std::os::raw::c_void;

const EMPTY_KEY_ERR: RedisResult = Err(RedisError::Str("ERR Invalid graph operation on empty key"));

static GRAPH_TYPE: RedisType = RedisType::new(
    "graphdata",
    0,
    RedisModuleTypeMethods {
        version: REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
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
unsafe extern "C" fn my_free(value: *mut c_void) {
    unsafe {
        drop(Box::from_raw(value.cast::<Graph>()));
    }
}

fn raw_value_to_redis_value(
    g: &Graph,
    r: &Value,
) -> RedisValue {
    match r {
        Value::List(values) => RedisValue::Array(
            values
                .iter()
                .map(|v| inner_raw_value_to_redis_value(g, v))
                .collect(),
        ),
        _ => todo!(),
    }
}

fn inner_raw_value_to_redis_value(
    g: &Graph,
    r: &Value,
) -> RedisValue {
    match r {
        Value::Null => RedisValue::Array(vec![RedisValue::Integer(1), RedisValue::Null]),
        Value::Bool(x) => RedisValue::Array(vec![
            RedisValue::Integer(4),
            RedisValue::SimpleString((if *x { "true" } else { "false" }).to_string()),
        ]),
        Value::Int(x) => RedisValue::Array(vec![RedisValue::Integer(3), RedisValue::Integer(*x)]),
        Value::Float(x) => RedisValue::Array(vec![
            RedisValue::Integer(5),
            RedisValue::SimpleString(format!("{:.14e}", *x)),
        ]),
        Value::String(x) => RedisValue::Array(vec![
            RedisValue::Integer(2),
            RedisValue::SimpleString(x.to_string()),
        ]),
        Value::List(values) => RedisValue::Array(vec![
            RedisValue::Integer(6),
            RedisValue::Array(
                values
                    .iter()
                    .map(|v| inner_raw_value_to_redis_value(g, v))
                    .collect(),
            ),
        ]),
        Value::Map(map) => RedisValue::Array(vec![
            RedisValue::Integer(10),
            RedisValue::OrderedMap(
                map.iter()
                    .map(|(key, value)| {
                        (
                            RedisValueKey::String(key.to_string()),
                            inner_raw_value_to_redis_value(g, value),
                        )
                    })
                    .collect(),
            ),
        ]),
        Value::Node(id) => {
            let mut props = Vec::new();
            for (key, value) in g.get_node_properties(*id) {
                let mut prop = Vec::new();
                prop.push(RedisValue::Integer(*key as _));
                if let RedisValue::Array(mut v) = inner_raw_value_to_redis_value(g, value) {
                    prop.append(&mut v);
                }
                props.push(RedisValue::Array(prop));
            }
            RedisValue::Array(vec![
                RedisValue::Integer(8),
                RedisValue::Array(vec![
                    RedisValue::Integer(*id as _),
                    RedisValue::Array(
                        g.get_node_label_ids(*id)
                            .map(|l| RedisValue::Integer(l as _))
                            .collect(),
                    ),
                    RedisValue::Array(props),
                ]),
            ])
        }
        Value::Relationship(id, from, to) => {
            let mut props = Vec::new();
            for (key, value) in g.get_relationship_properties(*id) {
                let mut prop = Vec::new();
                prop.push(RedisValue::Integer(*key as _));
                if let RedisValue::Array(mut v) = inner_raw_value_to_redis_value(g, value) {
                    prop.append(&mut v);
                }
                props.push(RedisValue::Array(prop));
            }
            RedisValue::Array(vec![
                RedisValue::Integer(7),
                RedisValue::Array(vec![
                    RedisValue::Integer(*id as _),
                    RedisValue::Integer(g.get_relationship_type_id(*id) as _),
                    RedisValue::Integer(*from as _),
                    RedisValue::Integer(*to as _),
                    RedisValue::Array(props),
                ]),
            ])
        }
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
    if key.get_value::<Graph>(&GRAPH_TYPE)?.is_some() {
        key.delete()
    } else {
        EMPTY_KEY_ERR
    }
}

#[inline]
fn query_mut(
    graph: &mut Graph,
    debug: u64,
    query: &str,
) -> Result<RedisValue, RedisError> {
    let mut res = Vec::new();
    graph
        .query(
            query,
            &mut |g, r| {
                res.push(raw_value_to_redis_value(g, &r));
            },
            debug > 0,
        )
        .map(|summary| {
            vec![
                vec![
                    vec![
                        RedisValue::Integer(1),
                        RedisValue::SimpleString("a".to_string()),
                    ]
                    .into(),
                ],
                res,
                vec![
                    RedisValue::SimpleString(format!("Labels added: {}", summary.labels_added)),
                    RedisValue::SimpleString(format!("Nodes created: {}", summary.nodes_created)),
                    RedisValue::SimpleString(format!("Nodes deleted: {}", summary.nodes_deleted)),
                    RedisValue::SimpleString(format!("Properties set: {}", summary.properties_set)),
                    RedisValue::SimpleString(format!(
                        "Relationships created: {}",
                        summary.relationships_created
                    )),
                ],
            ]
            .into()
        })
        .map_err(RedisError::String)
}

fn graph_query(
    ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_str()?;
    let debug = args.next_u64().unwrap_or(0);

    let key = ctx.open_key_writable(&key);

    if let Some(graph) = key.get_value::<Graph>(&GRAPH_TYPE)? {
        query_mut(graph, debug, query)
    } else {
        let mut value = Graph::new(16384, 16384);
        let res = query_mut(&mut value, debug, query);
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
    let debug = args.next_u64().unwrap_or(0);

    let key = ctx.open_key(&key);

    // We check if the key exists and is of type Graph if wrong type `get_value` return an error
    (key.get_value::<Graph>(&GRAPH_TYPE)?).map_or(
        // If the key does not exist, we return an error
        EMPTY_KEY_ERR,
        |graph| {
            let mut res = Vec::new();
            match graph.ro_query(
                query,
                &mut |g, r| {
                    res.push(raw_value_to_redis_value(g, &r));
                },
                debug > 0,
            ) {
                Ok(_) => Ok(vec![
                    vec![
                        vec![
                            RedisValue::Integer(1),
                            RedisValue::SimpleString("a".to_string()),
                        ]
                        .into(),
                    ],
                    res,
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
            RedisValue::Array(arr) => {
                if let RedisValue::Array(arr) = &arr[1] {
                    res.extend(arr.iter().filter_map(|v| {
                        if let RedisValue::SimpleString(key) = v {
                            Some(RedisValue::SimpleString(key.to_string()))
                        } else {
                            None
                        }
                    }));
                }
                if let RedisValue::SimpleString(i) = &arr[0] {
                    if i == "0" {
                        return Ok(RedisValue::Array(res));
                    }
                    a[0] = ctx.create_string(i.to_string());
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
        Ok(ir) => Ok(RedisValue::BulkString(format!("{ir:?}"))),
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
            let mut planner = Planner::new();
            let ir = planner.plan(ir, false);
            Ok(RedisValue::BulkString(format!("{ir}")))
        }
        Err(err) => Err(RedisError::String(err)),
    }
}

fn graph_init(
    _: &Context,
    _: &Vec<RedisString>,
) -> Status {
    Graph::init();
    Status::Ok
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
