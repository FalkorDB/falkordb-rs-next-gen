# FalkorDB Rust Security Review

**Date:** 2026-06-30  
**Scope:** Full Rust codebase (`src/`, `graph/src/`) — ~90 K lines, 160 files  
**Working tree:** clean (no uncommitted changes; full codebase review)  
**Method:** 5 parallel focused security-review agents (deserialization, command handlers,
parser/binder, UDF/CSV sandbox, FFI/unsafe memory), followed by independent verification
of highest-stakes findings and cross-agent conflict resolution.

---

## Cross-cutting Amplifier

> **Every reachable Rust panic crashes the entire redis-server process.**

A global panic hook installed in `graph_init` (`src/module_init.rs:136–160`) calls
`std::process::exit(1)` **before** unwinding. The `catch_unwind` in
`graph/src/threadpool.rs:72` is therefore bypassed — the hook fires first. Net effect:
any reliably-triggered panic (on any thread — worker, main, or MULTI) terminates the
whole multi-tenant server, taking down all graphs and connected clients.

This elevates every "remote panic" finding below to a full-server DoS.

---

## Summary Table

| # | Severity | File | Lines | Vulnerability | Confidence |
|---|----------|------|-------|---------------|------------|
| 1 | 🔴 HIGH | `src/serializers/decoder/mod.rs` (+`serializers/mod.rs`, `redis_type.rs`) | 40, 412 | `GRAPH.RESTORE`: unbounded `Vec::with_capacity(attacker_count)` → capacity-overflow panic → full server crash | 9/10 |
| 2 | 🔴 HIGH | `graph/src/parser/cypher.rs` | 2663–2693 (+1728, 2306–2417, 1243–1408) | Unbounded parser recursion (maps / func-args / comprehensions) on `GRAPH.QUERY` → stack overflow → server crash | 9/10 |
| 3 | 🔴 HIGH | `graph/src/runtime/pending.rs` (via `commands/effect.rs:157`) | 1510, 1513 | `GRAPH.EFFECT` `read_string` length-check integer overflow → OOB slice → server crash | 9/10 |
| 4 | 🔴 HIGH | `graph/src/graph/graphblas/matrix.rs` (+`vector.rs:344`) | 638 | `GRAPH.RESTORE`: attacker-named type + unvalidated `n_entries` overflows GraphBLAS size guard → **heap OOB read** (crash / heap disclosure) | 8/10 |
| 5 | 🔴 HIGH | `graph/src/graph/graphblas/serialization.rs` | 201–216 | `count * 8` integer overflow → OOB slice-index panic in `RoaringTreemap::decode_with_count` | 8/10 |
| 6 | 🔴 HIGH | `graph/src/graph/graphblas/vector.rs` | 184 | Hard `assert_eq!` on `GxB_Vector_deserialize` failure → remote abort via malformed `RESTORE` blob | 8/10 |
| 7 | 🔴 HIGH | `graph/src/planner/binder.rs` | 1632–1844 | Unbounded `bind_expr_node` recursion on nested list literals (parses OK, overflows during binding) → stack overflow → crash | 8/10 |
| 8 | 🔴 HIGH | `src/commands/effect.rs` (→ `graph.rs:1817–1823`) | 161 | Unvalidated `rel_id` → `edge_endpoints` OOB index + `max_id+1` overflow → crash | 8/10 |
| 9 | 🔴 HIGH | `src/commands/bulk_insert.rs` (→ `graph.rs:1220`) | 452–470 | `GRAPH.BULK` `node_count`/`edge_count` → `Vec::with_capacity(usize::MAX)` → crash | 8/10 |
| 10 | 🟠 MEDIUM | `graph/src/udf/type_convert.rs` | 185–374 | Unbounded recursion converting hostile UDF JS return values → native stack overflow → process abort | 8/10 |

---

## Detailed Findings

---

### #1 · HIGH (9/10) — Unbounded `Vec::with_capacity(attacker_count)` on RESTORE/RDB path

**Files:** `src/serializers/decoder/mod.rs:40,412`, `src/serializers/mod.rs:160,423,431,455,497,538,552`, `src/redis_type.rs:237`  
**Entry point:** `GRAPH.RESTORE` → `vec_load_graph` (no CRC/integrity check), also RDB load and replica load.

**Vulnerability:**  
`Header::decode`, `Schema::decode`, and related decoders read counts (relationship count,
label count, attribute count, etc.) straight from the attacker-controlled byte stream and
pass them directly to `Vec::with_capacity(count as usize)` **before** reading any
elements. Supplying a count such as `2^60` causes Rust's `RawVec` to raise a
`capacity overflow` panic (the requested byte size exceeds `isize::MAX`). The panic hook
fires → `process::exit(1)` → server dead.

**Remediation:**
- Never pre-size from an untrusted count.
- Use `Vec::new()` + `push`, or clamp the reserve to `min(count, remaining_input_len)`.
- Apply the same fix to every `Vec::with_capacity(decoded_count)` call in the decode path.

---

### #2 · HIGH (9/10) — Unbounded parser recursion → stack overflow on `GRAPH.QUERY`

**File:** `graph/src/parser/cypher.rs:2663–2693` (and `:1728`, `:2306–2417`, `:1243–1408`)

**Vulnerability:**  
The recursive-descent parser has no depth limit. While binary operators and parenthesised
expressions use an explicit heap stack, several constructs still recurse natively:

- **Map literals** — `parse_primary_expr` → `parse_map` (line 2663) → `parse_expr` (2679) → `parse_primary_expr` → `parse_map` …
- **Function arguments** — `parse_expression_list` (2313) → `parse_expr` → …
- **List comprehensions, CASE, quantifiers, reduce** — each calls `parse_expr` per nesting level.

Each level consumes multiple native stack frames. Worker threads use ~2 MB stack; the main
thread ~8 MB. A guard-page `SIGSEGV` from a stack overflow is not catchable by
`catch_unwind` — and the panic hook fires first anyway — so the entire server is aborted.

**Example malicious query:**
```
GRAPH.QUERY g "RETURN {a:{a:{a:...}}}  (×100 000 nestings)"
GRAPH.QUERY g "RETURN abs(abs(abs(...abs(1)...)))  (×100 000)"
```

**Remediation:**
- Add a recursion-depth counter (increment/decrement around every recursive `parse_*` call in `parse_expr`, `parse_map`, `parse_expression_list`).
- Return a parse error (not a panic) when depth exceeds a sane bound (e.g. 512).

---

### #3 · HIGH (9/10) — `GRAPH.EFFECT` `read_string` integer overflow → OOB slice → crash

**Files:** `graph/src/runtime/pending.rs:1510,1513` (called from `src/commands/effect.rs:157`)

**Vulnerability:**  
`read_string` reads an attacker-supplied `u64` length, then checks:
```rust
if *offset + len > buf.len() { return Err(...) }
```
When `len ≈ usize::MAX`, `*offset + len` **overflows** and wraps to a small value,
passing the check. The subsequent slice `&buf[*offset..*offset + len]` then panics with
an OOB range. `GRAPH.EFFECT` executes synchronously on the main Redis thread → panic
hook → `process::exit(1)`.

**Remediation:**
```rust
// Replace the unchecked addition with:
let end = offset.checked_add(len)
    .filter(|&e| e <= buf.len())
    .ok_or_else(|| "effects buffer truncated".to_string())?;
```
Apply the same fix to all `read_*` length/count helpers in `pending.rs`.

---

### #4 · HIGH (8/10) — Heap OOB read via attacker-controlled GraphBLAS container metadata (GRAPH.RESTORE)

**Files:** `graph/src/graph/graphblas/matrix.rs:638`, `graph/src/graph/graphblas/vector.rs:264–369`, `src/serializers/decoder/mod.rs:176–466`, `src/commands/restore.rs:40`

**Vulnerability:**  
`Matrix::decode` copies 608 attacker-controlled bytes verbatim into a
`GxB_Container_struct` (`copy_nonoverlapping`), placing attacker-chosen values into
`format`, `nrows`, `ncols`, `nvals`, and the component-vector fields. `Vector::<bool>::decode`
validates `n_bytes == arr_data.len()` but **not** `n_entries`. The GraphBLAS
`GxB_Vector_load` guard `X_size < n * type->size` can be bypassed by integer overflow:

1. Attacker names type `GrB_UINT64` (`type_name` is attacker-controlled, resolved via
   `GxB_Type_from_name` at vector.rs:300).
2. With `n_entries = 2^61 + 1` and `type_size = 8`, `n * 8` wraps to `8` — smaller than
   `X_size` so the guard is bypassed.
3. GraphBLAS sets `vlen = 2^61+1` with an 8-byte backing array.
4. `Matrix::decode` discards the load return code (`debug_assert_eq!` compiled out in
   release).
5. Subsequent query execution iterates the matrix → **heap out-of-bounds read** → crash
   or heap content disclosure into query results.

**Remediation:**
- In `Vector::decode`: reject if `n_bytes != n_entries.checked_mul(type_size).ok_or_err()?`.
- Do not `copy_nonoverlapping` the raw 608-byte struct. Parse and validate each container
  field (format, dimensions, nvals) against the decoded component arrays.
- Check the `GxB_load_Matrix_from_Container` return code in release builds (not just `debug_assert`).

---

### #5 · HIGH (8/10) — `count * 8` overflow → OOB slice-index panic in `RoaringTreemap::decode_with_count`

**File:** `graph/src/graph/graphblas/serialization.rs:201–216`  
**Entry point:** `GRAPH.RESTORE` → `vec_load_graph` → `deleted_nodes.decode_with_count(r, count)`

**Vulnerability:**
```rust
let expected_len = count as usize * 8;  // overflows when count = 2^61+1
```
With `count = 2^61+1`, `expected_len` wraps to `8`. An 8-byte buffer passes the length
check. The loop `for i in 0..count` then indexes `bytes[i*8..(i+1)*8]` — the second
iteration (bytes 8..16) is out of bounds → OOB panic → server crash.

**Remediation:**
```rust
let expected_len = count.checked_mul(8)
    .filter(|&n| n == bytes.len())
    .ok_or_else(|| "decode_with_count: length mismatch or overflow".to_string())?;
// Drive the loop from bytes.len()/8, not from count
```

---

### #6 · HIGH (8/10) — Hard `assert_eq!` on `GxB_Vector_deserialize` failure → remote abort

**File:** `graph/src/graph/graphblas/vector.rs:184`

**Vulnerability:**  
`Vector::<u64>::decode` uses a hard `assert_eq!(info, GrB_SUCCESS)` (not
`debug_assert`). A malformed serialized-vector blob inside any
`RelationMatrices`/`Tensor` payload (within a `GRAPH.RESTORE` payload) causes
`GxB_Vector_deserialize` to return an error code → `assert_eq!` panics → server crash.
The hardened `Vector::<bool>::decode` already returns `Err` on the equivalent failure;
the `u64` variant was not updated to match.

**Remediation:**
```rust
// Replace:
assert_eq!(info, GrB_Info::GrB_SUCCESS, "GxB_Vector_deserialize failed: {info:?}");
// With:
if info != GrB_Info::GrB_SUCCESS {
    return Err(format!("GxB_Vector_deserialize failed: {info:?}"));
}
```

---

### #7 · HIGH (8/10) — Unbounded `bind_expr_node` recursion on nested list literals → stack overflow

**File:** `graph/src/planner/binder.rs:1632–1844`

**Vulnerability:**  
The parser handles `[[[]]]`-style nested list literals with an explicit heap stack (safe,
returns `Ok`), but the resulting AST is a `List(List(List(...)))` tree nested to attacker
depth. `bind_expr_node` then **recursively** walks the AST (default arm at lines
1841–1844) with no depth limit → stack overflow → `SIGSEGV` → server crash. This is a
distinct sink from finding #2: the parser survives but the binder does not.

**Example malicious query:**
```
GRAPH.QUERY g "RETURN " + "[" × 100 000 + "]" × 100 000
```

**Remediation:**
- Add a depth parameter to `bind_expr_node` (and any other recursive AST walkers —
  planner, optimizer, runtime eval).
- Return an error past a bounded depth (e.g. 512) instead of recursing.
- Alternatively, convert `bind_expr_node` to an iterative traversal with an explicit
  work-stack.

---

### #8 · HIGH (8/10) — Unvalidated `rel_id` → `edge_endpoints` OOB index + `max_id+1` overflow

**Files:** `src/commands/effect.rs:161`, `graph/src/graph/graph.rs:1817–1823`

**Vulnerability:**  
`EFFECT_CREATE_EDGE` reads a raw `rel_id: u64` from the attacker-controlled effects
buffer (effect.rs:154) and passes it directly to `create_relationships_bulk`. Inside:
```rust
let needed = max_id + 1;   // overflows when max_id = u64::MAX
```
In release (no overflow checks), `needed` wraps to `0`, skipping the `edge_endpoints`
resize. `self.edge_endpoints[id as usize]` then indexes at `usize::MAX` → OOB panic.
The same `max_id + 1` overflow exists in `create_nodes` (graph.rs:1253) reachable via
`EFFECT_CREATE_NODE`, and from `GRAPH.BULK`.

**Remediation:**
- Validate `rel_id`/`src`/`dst` against the graph's current reserved-entity range before
  use; reject IDs that exceed it.
- Use `checked_add(1)` for `max_id + 1` and all capacity-doubling arithmetic.
- Treat all entity IDs arriving via effects/bulk buffers as untrusted.

---

### #9 · HIGH (8/10) — `GRAPH.BULK` counts → `Vec::with_capacity(usize::MAX)` → crash

**Files:** `src/commands/bulk_insert.rs:452–470`, `graph/src/graph/graph.rs:1220`

**Vulnerability:**  
`GRAPH.BULK` parses `node_count` and `edge_count` as `usize` directly from client
argument strings. The only validation (line 470) checks the *token* count, not these
counts. `reserve_nodes(node_count)` / `reserve_relationships(edge_count)` immediately
call `Vec::with_capacity(count)`. With `node_count = usize::MAX` this triggers a
`capacity overflow` panic → `process::exit(1)`.

**Example:**
```
GRAPH.BULK mygraph BEGIN 18446744073709551615 0 0 0
```

**Remediation:**
- Add an upper bound on `node_count`/`edge_count` (e.g. a configured `GRAPH_BULK_MAX`
  or cap to the number of supplied tokens).
- Have `reserve_nodes`/`reserve_relationships` grow capacity incrementally rather than
  allocating all at once.

---

### #10 · MEDIUM (8/10) — Unbounded UDF JS-value recursion → native stack overflow → process abort

**File:** `graph/src/udf/type_convert.rs:185–374`

**Vulnerability:**  
`js_to_value` and `value_to_js` walk JS values (List → item, Map → value, Path →
element) by **native Rust recursion** with no depth limit. A UDF can construct an
arbitrarily deep nested structure within its JS heap / stack / time budgets, then return
it. The Rust-side conversion recurses one native frame per nesting level → native stack
overflow → `SIGSEGV`/`SIGABRT` → process abort. This bypasses the 256 MB JS heap,
1 MB JS stack, and 5–30 s CPU time sandbox limits, all of which bound *JS* execution but
not the subsequent *Rust* conversion.

**Example malicious UDF:**
```js
falkor.register('boom', function() {
    let a = [];
    for (let i = 0; i < 300000; i++) a = [a];
    return a;
});
```
```
RETURN evil.boom()
```

**Remediation:**
- Add an explicit depth cap to `js_to_value` / `value_to_js` (e.g. reject nesting beyond
  128–256 levels with a query error).
- Alternatively, rewrite both functions iteratively using an explicit Rust-side work-stack.

---

## Areas Confirmed as Well-Hardened (No Issues Found)

| Area | Status |
|------|--------|
| `LOAD CSV` path traversal (`file://`) | ✅ `canonicalize()` + import-folder prefix check; symlink-safe; canonical path used for I/O |
| `LOAD CSV` SSRF (`https://`) | ✅ DNS resolved once; all candidate IPs blocked if private/loopback/link-local/CGNAT/multicast; addresses pinned into custom `ureq` resolver to close DNS-rebind window |
| JS UDF sandbox (fs / net / process escape) | ✅ `Context::full` exposes only `falkor.*`/`graph.*` Rust callbacks; no fs, network, `require`, or `eval` of host capabilities |
| GraphBLAS FFI wrappers | ✅ Allocator pairing consistent (`RedisModule_Alloc/Free` throughout); `AttrArray` refcount correct; MVCC/COW snapshots hold `Arc` to prevent UAF |
| Custom allocator (`src/allocator.rs`) | ✅ Delegates to `RedisAlloc`; no size/align overflow; no re-entrancy hazard |
| `string_escape.rs` (unicode escapes) | ✅ `char::from_u32` rejects surrogates/invalid codepoints; all escapes return `Err` on truncation |
| Lexer numeric parsing | ✅ Uses `from_str_radix`/`parse::<f64>` returning `Err` on overflow; `i64::MIN` edge handled |

---

## Root-Cause Themes & Recommended Remediation

1. **Checked arithmetic everywhere on untrusted input** — replace `a + b`, `a * b`,
   `Vec::with_capacity(n)` with `checked_add`/`checked_mul`/`min(n, remaining)` on every
   count, length, and ID decoded from client bytes or RDB streams.

2. **Recursion-depth guards in all recursive descent** — the parser (`cypher.rs`) and all
   AST consumers (`binder.rs`, optimizer, runtime eval) must carry a depth counter and
   return a clean error beyond a bounded limit.

3. **Validate before FFI** — untrusted fields fed to `GxB_*` must be cross-validated in
   Rust (checked multiply for `n_entries * type_size`; all container dimensions consistent
   with array lengths) before the FFI call; always propagate non-`GrB_SUCCESS` codes as
   `Err`, never `assert_eq!` or `debug_assert_eq!`.

4. **RESTORE payload integrity** — consider an HMAC or version-tagged CRC prefix for
   `GRAPH.RESTORE` payloads so replicas/operators can reject tampered blobs before
   decoding; alternatively, restrict `GRAPH.RESTORE` to privileged clients only.
