//! A/B benchmark for `AttributeStore` implementations.
//!
//! Exercises only the public API so it runs unchanged against any internal
//! representation (columnar chunks, row B+tree, ...). Deterministic (fixed
//! xorshift seed) so runs are comparable across builds.
//!
//! Run with:
//! ```sh
//! cargo test --release -p graph --test attr_store_bench -- --ignored --nocapture
//! ```

use std::{hint::black_box, sync::Arc, time::Instant};

use graph::{
    graph::attribute_store::AttributeStore,
    runtime::{ordermap::OrderMap, value::Value},
};
use roaring::RoaringTreemap;
use rustc_hash::FxHashMap;

const N: u64 = 200_000; // entities
const BATCH: u64 = 1_000; // entities per insert_attrs call (commit batch)
const POINT_READS: u64 = 2_000_000;
const COLUMN_PASSES: u64 = 20;
const SCAN_PASSES: u64 = 10;
const VERSIONS: u64 = 200; // MVCC churn iterations
const UPDATES_PER_VERSION: u64 = 1_000;

struct XorShift(u64);

impl XorShift {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn report(
    name: &str,
    elapsed: std::time::Duration,
    ops: u64,
) {
    println!(
        "{name:<28} {:>10.1} ms {:>10.1} ns/op ({ops} ops)",
        elapsed.as_secs_f64() * 1e3,
        elapsed.as_nanos() as f64 / ops as f64,
    );
}

/// Build the per-entity attribute map for entity `id`.
fn entity_attrs(
    names: &[Arc<String>; 5],
    id: u64,
) -> OrderMap<Arc<String>, Value> {
    let mut m = OrderMap::default();
    m.insert(names[0].clone(), Value::Int(id as i64));
    m.insert(
        names[1].clone(),
        Value::String(Arc::new(format!("entity-{id}"))),
    );
    m.insert(names[2].clone(), Value::Float(id as f64 * 0.5));
    m.insert(names[3].clone(), Value::Bool(id % 2 == 0));
    if id % 5 == 0 {
        // Variable row width: every 5th entity has an extra string attr.
        m.insert(
            names[4].clone(),
            Value::String(Arc::new(format!("extra-payload-{id}"))),
        );
    }
    m
}

#[test]
#[ignore = "benchmark; run explicitly with --ignored --nocapture in release"]
fn attr_store_bench() {
    let names: [Arc<String>; 5] = [
        Arc::new("id".to_owned()),
        Arc::new("name".to_owned()),
        Arc::new("score".to_owned()),
        Arc::new("active".to_owned()),
        Arc::new("extra".to_owned()),
    ];

    let mut store = AttributeStore::new(0);
    for n in &names {
        store.get_or_create_attr_id(n);
    }
    let idx_id = store.get_attr_id(&names[0]).unwrap() as u16;
    let idx_name = store.get_attr_id(&names[1]).unwrap() as u16;

    println!("== AttributeStore bench: N={N} entities ==");

    // ---- 1. insert_attrs (CREATE path) ---------------------------------
    let t = Instant::now();
    let mut total_attrs = 0u64;
    for base in (0..N).step_by(BATCH as usize) {
        let mut batch: FxHashMap<u64, OrderMap<Arc<String>, Value>> = FxHashMap::default();
        for id in base..(base + BATCH).min(N) {
            let attrs = entity_attrs(&names, id);
            total_attrs += attrs.len() as u64;
            batch.insert(id, attrs);
        }
        store.insert_attrs(&batch).unwrap();
    }
    report("insert_attrs (load)", t.elapsed(), total_attrs);

    // ---- 2. point reads: sequential then random -------------------------
    let t = Instant::now();
    let mut acc = 0i64;
    for i in 0..POINT_READS {
        let id = i % N;
        if let Some(Value::Int(v)) = store.get_attr_by_idx(id, idx_id) {
            acc = acc.wrapping_add(v);
        }
    }
    black_box(acc);
    report("point get seq (int)", t.elapsed(), POINT_READS);

    let mut rng = XorShift(0x9E37_79B9_7F4A_7C15);
    let t = Instant::now();
    let mut acc = 0i64;
    for _ in 0..POINT_READS {
        let id = rng.next() % N;
        if let Some(Value::Int(v)) = store.get_attr_by_idx(id, idx_id) {
            acc = acc.wrapping_add(v);
        }
    }
    black_box(acc);
    report("point get rand (int)", t.elapsed(), POINT_READS);

    let mut rng = XorShift(0xDEAD_BEEF_CAFE_F00D);
    let t = Instant::now();
    let mut len_acc = 0usize;
    for _ in 0..POINT_READS / 2 {
        let id = rng.next() % N;
        if let Some(Value::String(s)) = store.get_attr_by_idx(id, idx_name) {
            len_acc += s.len();
        }
    }
    black_box(len_acc);
    report("point get rand (string)", t.elapsed(), POINT_READS / 2);

    // ---- 3. bulk column read (FILTER/AGG path) --------------------------
    let keys: Vec<u64> = (0..N).collect();
    let default = Value::Null;
    let t = Instant::now();
    let mut out: Vec<Value> = Vec::new();
    for _ in 0..COLUMN_PASSES {
        for chunk in keys.chunks(1024) {
            out.clear();
            store.get_attrs_by_idx_batch_into(chunk, idx_id, &default, &mut out);
            black_box(out.len());
        }
    }
    report("batch column read", t.elapsed(), N * COLUMN_PASSES);

    // ---- 4. full-entity scan via iter_named (reply path) ----------------
    let t = Instant::now();
    let mut prop_count = 0u64;
    for _ in 0..SCAN_PASSES {
        for id in 0..N {
            for (name, v) in store.get_attrs(id).iter_named() {
                black_box(name);
                black_box(&*v);
                prop_count += 1;
            }
        }
    }
    report("full scan iter_named", t.elapsed(), prop_count);
    assert_eq!(prop_count / SCAN_PASSES, total_attrs, "scan saw every attr");

    // ---- 4b. clustered scan (heap-only native full-table scan) ----------
    #[cfg(feature = "attr-store-heap")]
    {
        let t = Instant::now();
        let mut prop_count2 = 0u64;
        for _ in 0..SCAN_PASSES {
            for (_id, view) in store.iter_rows() {
                for (name, v) in view.iter_named() {
                    black_box(name);
                    black_box(&*v);
                    prop_count2 += 1;
                }
            }
        }
        report("full scan iter_rows", t.elapsed(), prop_count2);
        assert_eq!(prop_count2, prop_count, "clustered scan sees every attr");
    }

    // ---- 4c. per-id scan in RANDOM order (worst-case row returns) --------
    // Same work as phase 4 — every entity visited once per pass via
    // get_attrs(id).iter_named() — but through a shuffled permutation, so
    // neither page reuse nor prefetching helps.
    let mut shuffled: Vec<u64> = (0..N).collect();
    let mut rng = XorShift(0x5851_F42D_4C95_7F2D);
    for i in (1..shuffled.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        shuffled.swap(i, j);
    }
    let t = Instant::now();
    let mut prop_count3 = 0u64;
    for _ in 0..SCAN_PASSES {
        for &id in &shuffled {
            for (name, v) in store.get_attrs(id).iter_named() {
                black_box(name);
                black_box(&*v);
                prop_count3 += 1;
            }
        }
    }
    report("rand scan iter_named", t.elapsed(), prop_count3);
    assert_eq!(prop_count3, prop_count, "random-order scan sees every attr");

    // ---- 5. MVCC churn: new_version + sparse update ----------------------
    // Keep the base immutable; verify snapshot isolation at the end.
    let base_val = store.get_attr_by_idx(7, idx_id);
    let mut rng = XorShift(0x1234_5678_9ABC_DEF1);
    let t = Instant::now();
    let mut latest = store.new_version(1);
    for v in 0..VERSIONS {
        let mut next = latest.new_version(v + 2);
        let mut batch: FxHashMap<u64, OrderMap<Arc<String>, Value>> = FxHashMap::default();
        for _ in 0..UPDATES_PER_VERSION {
            let id = rng.next() % N;
            let mut m = OrderMap::default();
            m.insert(names[0].clone(), Value::Int(-(id as i64)));
            batch.insert(id, m);
        }
        next.insert_attrs(&batch).unwrap();
        latest = next;
    }
    report(
        "mvcc version+update",
        t.elapsed(),
        VERSIONS * UPDATES_PER_VERSION,
    );
    assert_eq!(
        store.get_attr_by_idx(7, idx_id),
        base_val,
        "old snapshot must not see new-version writes"
    );

    // ---- 6. snapshot read-back (read latest version after churn) --------
    let t = Instant::now();
    let mut acc = 0i64;
    for id in 0..N {
        if let Some(Value::Int(v)) = latest.get_attr_by_idx(id, idx_id) {
            acc = acc.wrapping_add(v);
        }
    }
    black_box(acc);
    report("post-churn seq read", t.elapsed(), N);

    // ---- 7. memory ------------------------------------------------------
    println!(
        "memory_usage             {:>10.1} MiB ({:.1} B/entity)",
        store.memory_usage() as f64 / (1024.0 * 1024.0),
        store.memory_usage() as f64 / N as f64,
    );
    println!(
        "structural_memory_usage  {:>10.1} MiB ({:.1} B/entity)",
        store.structural_memory_usage() as f64 / (1024.0 * 1024.0),
        store.structural_memory_usage() as f64 / N as f64,
    );

    // ---- 8. remove_all (DELETE path) -------------------------------------
    let mut victims = RoaringTreemap::new();
    let mut rng = XorShift(0xABCD_EF01_2345_6789);
    while victims.len() < N / 4 {
        victims.insert(rng.next() % N);
    }
    let t = Instant::now();
    store.remove_all(&victims);
    report("remove_all N/4", t.elapsed(), N / 4);
    let sample = victims.iter().next().unwrap();
    assert!(!store.has_attributes(sample), "victim rows removed");
}
