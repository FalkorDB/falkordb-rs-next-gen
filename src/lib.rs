use graph::{graph::Graph, value::Value};
use redis_module::{
    native_types::RedisType, redis_module, redisvalue::RedisValueKey, Context, NextArg,
    RedisModuleTypeMethods, RedisResult, RedisString, RedisValue, Status,
    REDISMODULE_TYPE_METHOD_VERSION,
};
use std::{collections::HashMap, os::raw::c_void};

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

#[no_mangle]
unsafe extern "C" fn my_free(value: *mut c_void) {
    drop(Box::from_raw(value.cast::<Graph>()));
}

fn raw_value_to_redis_value(g: &mut Graph, r: &Value) -> RedisValue {
    match r {
        Value::Null => RedisValue::Null,
        Value::Bool(x) => RedisValue::Bool(*x),
        Value::Int(x) => RedisValue::Integer(*x),
        Value::Float(x) => RedisValue::Float(*x),
        Value::String(x) => RedisValue::SimpleString(x.to_string()),
        Value::Array(values) => RedisValue::Array(
            values
                .iter()
                .map(|v| raw_value_to_redis_value(g, v))
                .collect(),
        ),
        Value::Map(map) => RedisValue::Map(
            map.iter()
                .map(|(key, value)| {
                    (
                        RedisValueKey::String(key.to_string()),
                        raw_value_to_redis_value(g, value),
                    )
                })
                .collect(),
        ),
        Value::Node(id) => {
            let mut vec = HashMap::new();
            vec.insert(
                RedisValueKey::String("id".to_string()),
                RedisValue::Integer(*id as _),
            );
            let mut labels = Vec::new();
            for label in g.get_node_labels(*id) {
                labels.push(RedisValue::SimpleString(label.to_string()));
            }
            vec.insert(
                RedisValueKey::String("labels".to_string()),
                RedisValue::Array(labels),
            );
            RedisValue::Map(vec)
        }
        Value::Link(id) => {
            let mut vec = HashMap::new();
            vec.insert(
                RedisValueKey::String("id".to_string()),
                RedisValue::Integer(*id as _),
            );
            RedisValue::Map(vec)
        }
    }
}

fn graph_query(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let query = args.next_string()?;
    let debug = args.next_u64().unwrap_or(0);

    let key = ctx.open_key_writable(&key);

    let graph = if let Some(graph) = key.get_value::<Graph>(&GRAPH_TYPE)? {
        graph
    } else {
        let value = Graph::new(1024, 1024);

        key.set_value(&GRAPH_TYPE, value)?;
        key.get_value::<Graph>(&GRAPH_TYPE)?.unwrap()
    };
    let mut res = Vec::new();
    match graph.query(
        query.as_str(),
        &mut |g, r| {
            res.push(raw_value_to_redis_value(g, &r));
        },
        debug > 0,
    ) {
        Ok(_) => Ok(res.into()),
        Err(err) => {
            ctx.reply_error_string(err.as_str());
            Ok(RedisValue::NoReply)
        }
    }
}

fn graph_init(_: &Context, _: &Vec<RedisString>) -> Status {
    Graph::init();
    Status::Ok
}

//////////////////////////////////////////////////////

redis_module! {
    name: "matrixdb",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [GRAPH_TYPE],
    init: graph_init,
    commands: [
        ["graph.query", graph_query, "write", 1, 1, 1, ""],
    ],
}
