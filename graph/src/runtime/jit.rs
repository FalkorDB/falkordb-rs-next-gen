//! Cranelift JIT prototype for `ExprIR` evaluation.
//!
//! Compiles a curated subset of `ExprIR` variants to a single function that
//! takes `(eval, env, out)` and writes a `Value` into `*out`. Falls back to
//! the interpreter for unsupported variants by returning `None` from
//! [`try_compile`].
//!
//! Design (see `enumerated-strolling-allen.md`):
//! - Every helper writes a `Value` to its out-slot, even on error (Null on
//!   error). This gives uniform drop semantics — every slot is always
//!   initialized after its helper runs and can be unconditionally dropped
//!   at function exit.
//! - Errors are stashed in a thread-local rather than threaded through
//!   Cranelift as a `Result`.
//! - `Arc<String>` constants (property names) are owned by `CompiledExpr`
//!   and their addresses are baked into emitted code as i64 constants.

#![allow(unsafe_op_in_unsafe_fn)]

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::mem::{MaybeUninit, size_of};
use std::ptr;
use std::sync::Arc;

use cranelift_codegen::ir::{
    AbiParam, Function, InstBuilder, Signature, UserFuncName, condcodes::IntCC, types,
};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::{Context, ir};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use orx_tree::{Dyn, DynNode, DynTree, NodeIdx, NodeRef};
use thin_vec::ThinVec;

use crate::parser::ast::{ExprIR, QueryExprInner, Variable};
use crate::runtime::env::Env;
use crate::runtime::eval::{
    ExprEval, all_equals, all_not_equals, get_elements, list_contains, logical_xor,
};
use crate::runtime::functions::math::apply_pow;
use crate::runtime::functions::{FnType, GraphFn};
use crate::runtime::ordermap::OrderMap;
use crate::runtime::runtime::Runtime;
use crate::runtime::value::{CompareValue, DisjointOrNull, Value};

const VALUE_SIZE: u32 = size_of::<Value>() as u32;

thread_local! {
    static LAST_ERR: RefCell<String> = RefCell::new(String::new());
}

fn set_err<S: Into<String>>(msg: S) {
    LAST_ERR.with(|e| *e.borrow_mut() = msg.into());
}

fn err_set() -> bool {
    LAST_ERR.with(|e| !e.borrow().is_empty())
}

fn swap_err(new: String) -> String {
    LAST_ERR.with(|e| std::mem::replace(&mut *e.borrow_mut(), new))
}

// ---------------------------------------------------------------------------
// Compiled function wrapper
// ---------------------------------------------------------------------------

pub type JitFn = unsafe extern "C" fn(
    *const c_void, // eval
    *const c_void, // env
    *const c_void, // tree (DynTree<ExprIR<Variable>>)
    *const c_void, // idx (NodeIdx<Dyn<ExprIR<Variable>>>) — the subtree root
    // being evaluated. The compiled function may have been
    // cached against a different tree with the same content
    // fingerprint, so the runtime caller passes the actual
    // idx; specialized bridges navigate relative paths from
    // here, never from `tree.root()`.
    *mut Value, // out
) -> u8;

pub struct CompiledExpr {
    func: JitFn,
    /// Owned constants whose pointers are baked into the compiled code.
    /// Boxed so each `Arc` has a stable heap address regardless of Vec
    /// reallocation during compilation.
    _strings: Vec<Box<Arc<String>>>,
    _fn_arcs: Vec<Box<Arc<GraphFn>>>,
    _params: Vec<Box<String>>,
    /// Paths from root to specialized-bridge nodes. Each path is a sequence
    /// of child indices, baked into the compiled function. Tree-structure-
    /// relative so it's safe to reuse across distinct trees with the same
    /// fingerprint (unlike `NodeIdx`, which is tied to a tree's memory).
    _paths: Vec<Box<Vec<usize>>>,
    /// Owned pointer arrays for the `helpers` bridge param. Each entry is a
    /// boxed `Vec<*const CompiledExpr>` whose pointer is baked as a constant
    /// into the compiled function.
    _helper_ptr_arrays: Vec<Box<Vec<*const CompiledExpr>>>,
    /// Owned `Arc<CompiledExpr>` instances kept alive for the lifetime of
    /// this CompiledExpr; their addresses populate `_helper_ptr_arrays`.
    _helper_arcs: Vec<Arc<CompiledExpr>>,
    /// Owned constant `Value`s pre-evaluated at codegen (e.g. all-literal
    /// list/map literals). Their addresses are baked as constants and the
    /// runtime clones from them.
    _const_values: Vec<Box<Value>>,
    // Option so Drop can take ownership and call free_memory(self).
    // cranelift's JITModule has no Drop impl — dropping it without
    // free_memory leaks the mmap'd executable pages.
    _module: Option<JITModule>,
}

impl Drop for CompiledExpr {
    fn drop(&mut self) {
        if let Some(m) = self._module.take() {
            // SAFETY: self.func pointed into m's code, and self is being
            // dropped — no live references into the JIT memory remain.
            unsafe { m.free_memory() };
        }
    }
}

impl CompiledExpr {
    /// Invoke the compiled function. Safety: caller guarantees the eval and
    /// env references outlive the call.
    pub fn call(
        &self,
        eval: &ExprEval<'_>,
        env: Option<&Env<'_>>,
        tree: &QueryExprInner<Variable>,
        idx: &NodeIdx<Dyn<ExprIR<Variable>>>,
        agg_group_key: Option<u64>,
    ) -> Result<Value, String> {
        let mut out = MaybeUninit::<Value>::uninit();
        let env_ptr = env.map_or(ptr::null(), |e| e as *const Env<'_> as *const c_void);
        let eval_ptr = eval as *const ExprEval<'_> as *const c_void;
        let tree_ptr = tree as *const QueryExprInner<Variable> as *const c_void;
        let idx_ptr = idx as *const NodeIdx<Dyn<ExprIR<Variable>>> as *const c_void;
        let prev_key = AGG_KEY.with(|c| c.replace(agg_group_key));
        let prev_err = swap_err(String::new());
        let rc = unsafe { (self.func)(eval_ptr, env_ptr, tree_ptr, idx_ptr, out.as_mut_ptr()) };
        AGG_KEY.with(|c| c.set(prev_key));
        let value = unsafe { out.assume_init() };
        if rc == 0 && !err_set() {
            // Restore the caller's pending error (if any).
            swap_err(prev_err);
            Ok(value)
        } else {
            drop(value);
            let our_err = swap_err(prev_err);
            Err(our_err)
        }
    }
}

// SAFETY: JITModule contains raw allocations but no thread-local state once
// the function is compiled. The thread-local LAST_ERR is per-thread and
// independent of the compiled function.
unsafe impl Send for CompiledExpr {}
unsafe impl Sync for CompiledExpr {}

// ---------------------------------------------------------------------------
// JIT compilation cache lives in `QueryExprInner` (one OnceLock per root
// expression, one Mutex<HashMap<NodeIdx-hash, Option<Arc<CompiledExpr>>>> for
// sub-expressions). See `ExprEval::eval` in runtime/eval.rs.
// ---------------------------------------------------------------------------

thread_local! {
    static JIT_STATS: RefCell<JitStats> = RefCell::new(JitStats::default());
    // Current aggregation group key, threaded into the JIT via TLS rather
    // than the calling convention so helper signatures stay uniform. None
    // outside aggregation; Some(group_id) inside Aggregate's accumulate /
    // finalize phases. Saved/restored across each `CompiledExpr::call`.
    static AGG_KEY: Cell<Option<u64>> = const { Cell::new(None) };
}

pub(crate) fn current_agg_key() -> Option<u64> {
    AGG_KEY.with(|c| c.get())
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JitStats {
    pub compiles: u64,
    pub compile_failures: u64,
    pub hits: u64,
}

pub fn stats_snapshot() -> JitStats {
    JIT_STATS.with(|s| *s.borrow())
}

pub(crate) fn record_compile(success: bool) {
    JIT_STATS.with(|s| {
        let mut s = s.borrow_mut();
        if success {
            s.compiles += 1;
        } else {
            s.compile_failures += 1;
        }
    });
}

pub(crate) fn record_hit() {
    JIT_STATS.with(|s| s.borrow_mut().hits += 1);
}

// ---------------------------------------------------------------------------
// Helper functions called from JIT-emitted code
// ---------------------------------------------------------------------------
//
// Calling convention contract:
// - Out parameter is *mut Value pointing to uninitialized memory; helper
//   must always write a Value (Value::Null on error) so the caller can
//   unconditionally drop it.
// - Returns 0 on success, 1 on error. On error, the helper sets LAST_ERR.

unsafe extern "C" fn jit_int(
    out: *mut Value,
    lo: i64,
) -> u8 {
    ptr::write(out, Value::Int(lo));
    0
}

unsafe extern "C" fn jit_const_clone(
    out: *mut Value,
    val_ptr: *const Value,
) -> u8 {
    ptr::write(out, (*val_ptr).clone());
    0
}

unsafe extern "C" fn jit_float(
    out: *mut Value,
    bits: u64,
) -> u8 {
    ptr::write(out, Value::Float(f64::from_bits(bits)));
    0
}

unsafe extern "C" fn jit_bool(
    out: *mut Value,
    b: u8,
) -> u8 {
    ptr::write(out, Value::Bool(b != 0));
    0
}

unsafe extern "C" fn jit_null(out: *mut Value) -> u8 {
    ptr::write(out, Value::Null);
    0
}

unsafe extern "C" fn jit_var(
    env: *const c_void,
    var_id: u32,
    out: *mut Value,
) -> u8 {
    if env.is_null() {
        ptr::write(out, Value::Null);
        set_err(format!("Variable {var_id} not found"));
        return 1;
    }
    let env = &*(env as *const Env<'_>);
    match env.get_by_id(var_id) {
        Some(v) => {
            ptr::write(out, v.clone());
            0
        }
        None => {
            ptr::write(out, Value::Null);
            set_err(format!("Variable {var_id} not found"));
            1
        }
    }
}

unsafe extern "C" fn jit_property(
    eval: *const c_void,
    src: *const Value,
    name: *const Arc<String>,
    out: *mut Value,
) -> u8 {
    let eval = &*(eval as *const ExprEval<'_>);
    let src = &*src;
    let name = &*name;
    let rt = match eval.runtime_opt() {
        Some(rt) => rt,
        None => {
            ptr::write(out, Value::Null);
            set_err("not a constant expression".to_string());
            return 1;
        }
    };
    let result = property_lookup(rt, src, name);
    match result {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

fn property_lookup(
    rt: &Runtime<'_>,
    src: &Value,
    name: &Arc<String>,
) -> Result<Value, String> {
    Ok(match src {
        Value::Node(id) => rt.get_node_attribute(*id, name).unwrap_or(Value::Null),
        Value::Relationship(rel) => rt
            .get_relationship_attribute(rel.0, name)
            .unwrap_or(Value::Null),
        Value::Null => Value::Null,
        other => other.clone().get_attr(name)?,
    })
}

macro_rules! arith_helper {
    ($name:ident, $op:tt, $what:literal) => {
        unsafe extern "C" fn $name(
            lhs: *const Value,
            rhs: *const Value,
            out: *mut Value,
        ) -> u8 {
            let l = (&*lhs).clone();
            let r = (&*rhs).clone();
            match l $op r {
                Ok(v) => { ptr::write(out, v); 0 }
                Err(e) => { ptr::write(out, Value::Null); set_err(e); 1 }
            }
        }
    };
}

arith_helper!(jit_add, +, "add");
arith_helper!(jit_sub, -, "sub");
arith_helper!(jit_mul, *, "mul");
arith_helper!(jit_div, /, "div");
arith_helper!(jit_modulo, %, "modulo");

unsafe extern "C" fn jit_negate(
    src: *const Value,
    out: *mut Value,
) -> u8 {
    match &*src {
        Value::Int(i) => match i.checked_neg() {
            Some(n) => {
                ptr::write(out, Value::Int(n));
                0
            }
            None => {
                ptr::write(out, Value::Null);
                set_err(String::from("integer overflow"));
                1
            }
        },
        Value::Float(f) => {
            ptr::write(out, Value::Float(-*f));
            0
        }
        Value::Null => {
            ptr::write(out, Value::Null);
            0
        }
        v => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                v.name()
            ));
            1
        }
    }
}

unsafe extern "C" fn jit_not(
    src: *const Value,
    out: *mut Value,
) -> u8 {
    match &*src {
        Value::Bool(b) => {
            ptr::write(out, Value::Bool(!*b));
            0
        }
        Value::Null => {
            ptr::write(out, Value::Null);
            0
        }
        v => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Boolean or Null but was {}",
                v.name()
            ));
            1
        }
    }
}

unsafe extern "C" fn jit_and(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    let result = match (l, r) {
        (Value::Bool(false), _) | (_, Value::Bool(false)) => Value::Bool(false),
        (Value::Bool(true), Value::Bool(true)) => Value::Bool(true),
        (Value::Null, _) | (_, Value::Null) => Value::Null,
        (a, b) => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Bool but was {a:?} / {b:?}"
            ));
            return 1;
        }
    };
    ptr::write(out, result);
    0
}

unsafe extern "C" fn jit_or(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    let result = match (l, r) {
        (Value::Bool(true), _) | (_, Value::Bool(true)) => Value::Bool(true),
        (Value::Bool(false), Value::Bool(false)) => Value::Bool(false),
        (Value::Null, _) | (_, Value::Null) => Value::Null,
        (a, b) => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Bool but was {a:?} / {b:?}"
            ));
            return 1;
        }
    };
    ptr::write(out, result);
    0
}

macro_rules! cmp_helper {
    ($name:ident, $true_arm:pat) => {
        unsafe extern "C" fn $name(
            lhs: *const Value,
            rhs: *const Value,
            out: *mut Value,
        ) -> u8 {
            let l = &*lhs;
            let r = &*rhs;
            match l.compare_value(r) {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => {
                    ptr::write(out, Value::Null);
                }
                (_, DisjointOrNull::NaN) => {
                    ptr::write(out, Value::Bool(false));
                }
                ($true_arm, _) => {
                    ptr::write(out, Value::Bool(true));
                }
                _ => {
                    ptr::write(out, Value::Bool(false));
                }
            }
            0
        }
    };
}

cmp_helper!(jit_lt, std::cmp::Ordering::Less);
cmp_helper!(jit_gt, std::cmp::Ordering::Greater);
cmp_helper!(jit_le, std::cmp::Ordering::Less | std::cmp::Ordering::Equal);
cmp_helper!(
    jit_ge,
    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
);

// State-returning comparison helpers — used by short-circuit chains to skip
// the Value materialization + jit_bool_state call. Returns u8 state code:
// 0=false, 1=true, 2=null, 3=err. Comparisons can't error structurally
// (compare_value returns Disjoint/Null instead), so 3 is unused here.
macro_rules! cmp_state_helper {
    ($name:ident, $true_arm:pat) => {
        unsafe extern "C" fn $name(
            lhs: *const Value,
            rhs: *const Value,
        ) -> u8 {
            let l = &*lhs;
            let r = &*rhs;
            match l.compare_value(r) {
                (_, DisjointOrNull::ComparedNull | DisjointOrNull::Disjoint) => 2,
                (_, DisjointOrNull::NaN) => 0,
                ($true_arm, _) => 1,
                _ => 0,
            }
        }
    };
}

cmp_state_helper!(jit_lt_state, std::cmp::Ordering::Less);
cmp_state_helper!(jit_gt_state, std::cmp::Ordering::Greater);
cmp_state_helper!(
    jit_le_state,
    std::cmp::Ordering::Less | std::cmp::Ordering::Equal
);
cmp_state_helper!(
    jit_ge_state,
    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
);

unsafe extern "C" fn jit_eq_state(
    lhs: *const Value,
    rhs: *const Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    match l.compare_value(r) {
        (_, DisjointOrNull::ComparedNull) => 2,
        (_, DisjointOrNull::Disjoint | DisjointOrNull::NaN) => 0,
        (std::cmp::Ordering::Equal, _) => 1,
        _ => 0,
    }
}

unsafe extern "C" fn jit_neq_state(
    lhs: *const Value,
    rhs: *const Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    match l.compare_value(r) {
        (_, DisjointOrNull::ComparedNull) => 2,
        (_, DisjointOrNull::Disjoint | DisjointOrNull::NaN) => 1,
        (std::cmp::Ordering::Equal, _) => 0,
        _ => 1,
    }
}

unsafe extern "C" fn jit_eq(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    match l.compare_value(r) {
        (_, DisjointOrNull::ComparedNull) => ptr::write(out, Value::Null),
        (_, DisjointOrNull::Disjoint | DisjointOrNull::NaN) => {
            ptr::write(out, Value::Bool(false));
        }
        (std::cmp::Ordering::Equal, _) => ptr::write(out, Value::Bool(true)),
        _ => ptr::write(out, Value::Bool(false)),
    }
    0
}

unsafe extern "C" fn jit_neq(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    match l.compare_value(r) {
        (_, DisjointOrNull::ComparedNull) => ptr::write(out, Value::Null),
        (_, DisjointOrNull::Disjoint | DisjointOrNull::NaN) => {
            ptr::write(out, Value::Bool(true));
        }
        (std::cmp::Ordering::Equal, _) => ptr::write(out, Value::Bool(false)),
        _ => ptr::write(out, Value::Bool(true)),
    }
    0
}

unsafe extern "C" fn jit_value_drop(slot: *mut Value) {
    ptr::drop_in_place(slot);
}

unsafe extern "C" fn jit_string(
    out: *mut Value,
    name: *const Arc<String>,
) -> u8 {
    ptr::write(out, Value::String((*name).clone()));
    0
}

unsafe extern "C" fn jit_param(
    eval: *const c_void,
    name: *const String,
    out: *mut Value,
) -> u8 {
    let eval = &*(eval as *const ExprEval<'_>);
    let name = &*name;
    let rt = match eval.runtime_opt() {
        Some(rt) => rt,
        None => {
            ptr::write(out, Value::Null);
            set_err("not a constant expression".to_string());
            return 1;
        }
    };
    match rt.parameters.get(name) {
        Some(v) => {
            ptr::write(out, v.clone());
            0
        }
        None => {
            ptr::write(out, Value::Null);
            set_err(format!("Parameter {name} not found"));
            1
        }
    }
}

unsafe extern "C" fn jit_length(
    src: *const Value,
    out: *mut Value,
) -> u8 {
    match &*src {
        Value::List(arr) => {
            ptr::write(out, Value::Int(arr.len() as i64));
            0
        }
        _ => {
            ptr::write(out, Value::Null);
            set_err("Length operator requires a list".to_string());
            1
        }
    }
}

unsafe extern "C" fn jit_is_node(
    src: *const Value,
    out: *mut Value,
) -> u8 {
    let v = matches!(&*src, Value::Node(_));
    ptr::write(out, Value::Bool(v));
    0
}

unsafe extern "C" fn jit_is_rel(
    src: *const Value,
    out: *mut Value,
) -> u8 {
    let v = matches!(&*src, Value::Relationship(_));
    ptr::write(out, Value::Bool(v));
    0
}

unsafe extern "C" fn jit_in(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let value = (*lhs).clone();
    let list = &*rhs;
    match list_contains(list, value) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

unsafe extern "C" fn jit_pow(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let v = apply_pow((*lhs).clone(), (*rhs).clone());
    ptr::write(out, v);
    0
}

unsafe extern "C" fn jit_xor(
    lhs: *const Value,
    rhs: *const Value,
    out: *mut Value,
) -> u8 {
    let l = &*lhs;
    let r = &*rhs;
    let result = match (l, r) {
        (Value::Null, _) | (_, Value::Null) => Value::Null,
        (Value::Bool(a), Value::Bool(b)) => Value::Bool(logical_xor(*a, *b)),
        (a, b) => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Bool but was {a:?} / {b:?}"
            ));
            return 1;
        }
    };
    ptr::write(out, result);
    0
}

unsafe extern "C" fn jit_and_n(
    argc: u32,
    argv: *const *const Value,
    out: *mut Value,
) -> u8 {
    let mut is_null = false;
    for i in 0..argc as usize {
        let v = &*(*argv.add(i));
        match v {
            Value::Bool(false) => {
                ptr::write(out, Value::Bool(false));
                return 0;
            }
            Value::Bool(true) => {}
            Value::Null => is_null = true,
            other => {
                ptr::write(out, Value::Null);
                set_err(format!("Type mismatch: expected Bool but was {other:?}"));
                return 1;
            }
        }
    }
    ptr::write(
        out,
        if is_null {
            Value::Null
        } else {
            Value::Bool(true)
        },
    );
    0
}

unsafe extern "C" fn jit_or_n(
    argc: u32,
    argv: *const *const Value,
    out: *mut Value,
) -> u8 {
    let mut is_null = false;
    for i in 0..argc as usize {
        let v = &*(*argv.add(i));
        match v {
            Value::Bool(true) => {
                ptr::write(out, Value::Bool(true));
                return 0;
            }
            Value::Bool(false) => {}
            Value::Null => is_null = true,
            other => {
                ptr::write(out, Value::Null);
                set_err(format!("Type mismatch: expected Bool but was {other:?}"));
                return 1;
            }
        }
    }
    ptr::write(
        out,
        if is_null {
            Value::Null
        } else {
            Value::Bool(false)
        },
    );
    0
}

// State codes returned by jit_bool_state and consumed by jit_write_logical.
// Kept in sync with emit_short_circuit.
//   0 = Bool(false), 1 = Bool(true), 2 = Null, 3 = type error (LAST_ERR set).
unsafe extern "C" fn jit_bool_state(v: *const Value) -> u8 {
    match &*v {
        Value::Bool(false) => 0,
        Value::Bool(true) => 1,
        Value::Null => 2,
        other => {
            set_err(format!("Type mismatch: expected Bool but was {other:?}"));
            3
        }
    }
}

unsafe extern "C" fn jit_write_logical(
    out: *mut Value,
    state: u8,
) {
    let v = match state {
        0 => Value::Bool(false),
        1 => Value::Bool(true),
        _ => Value::Null,
    };
    ptr::write(out, v);
}

unsafe extern "C" fn jit_eq_n(
    argc: u32,
    argv: *const *const Value,
    out: *mut Value,
) -> u8 {
    let iter = (0..argc as usize).map(|i| Ok((*(*argv.add(i))).clone()));
    match all_equals(iter) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

unsafe extern "C" fn jit_neq_n(
    argc: u32,
    argv: *const *const Value,
    out: *mut Value,
) -> u8 {
    let iter = (0..argc as usize).map(|i| Ok((*(*argv.add(i))).clone()));
    match all_not_equals(iter) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

unsafe extern "C" fn jit_list(
    argc: u32,
    argv: *const *const Value,
    out: *mut Value,
) -> u8 {
    let mut list: ThinVec<Value> = ThinVec::with_capacity(argc as usize);
    for i in 0..argc as usize {
        list.push((*(*argv.add(i))).clone());
    }
    ptr::write(out, Value::List(Arc::new(list)));
    0
}

// Build a Map from `n` (key, value) pairs.
// `keys` is an array of `*const Arc<String>`; `values` is an array of
// `*const Value`. Both pointers point into the JIT'd function's stack.
unsafe extern "C" fn jit_map(
    n: u32,
    keys: *const *const Arc<String>,
    values: *const *const Value,
    out: *mut Value,
) -> u8 {
    let mut map = OrderMap::default();
    for i in 0..n as usize {
        let k = (*(*keys.add(i))).clone();
        let v = (*(*values.add(i))).clone();
        map.insert(k, v);
    }
    ptr::write(out, Value::Map(Arc::new(map)));
    0
}

unsafe extern "C" fn jit_get_element(
    eval: *const c_void,
    arr: *const Value,
    i: *const Value,
    out: *mut Value,
) -> u8 {
    let arr = (*arr).clone();
    let i = (*i).clone();
    let v = match (arr, i) {
        (Value::List(values), Value::Int(idx)) => {
            let len = values.len() as i64;
            let normalized = if idx < 0 { len + idx } else { idx };
            if normalized >= 0 && normalized < len {
                values[normalized as usize].clone()
            } else {
                Value::Null
            }
        }
        (Value::List(_), Value::Null) => Value::Null,
        (Value::List(_), v) => {
            ptr::write(out, Value::Null);
            set_err(format!(
                "Type mismatch: expected Integer but was {}",
                v.name()
            ));
            return 1;
        }
        (Value::Map(map), Value::String(key)) => map.get(&key).cloned().unwrap_or(Value::Null),
        (Value::Node(id), Value::String(key)) => {
            let eval = &*(eval as *const ExprEval<'_>);
            let rt = match eval.runtime_opt() {
                Some(rt) => rt,
                None => {
                    ptr::write(out, Value::Null);
                    set_err("not a constant expression".to_string());
                    return 1;
                }
            };
            rt.get_node_attribute(id, &key).unwrap_or(Value::Null)
        }
        (Value::Relationship(rel), Value::String(key)) => {
            let eval = &*(eval as *const ExprEval<'_>);
            let rt = match eval.runtime_opt() {
                Some(rt) => rt,
                None => {
                    ptr::write(out, Value::Null);
                    set_err("not a constant expression".to_string());
                    return 1;
                }
            };
            rt.get_relationship_attribute(rel.0, &key)
                .unwrap_or(Value::Null)
        }
        (Value::Map(_), Value::Null) | (Value::Null, _) => Value::Null,
        (a, b) => {
            ptr::write(out, Value::Null);
            set_err(format!("Type mismatch: unexpected types ({a:?}, {b:?})"));
            return 1;
        }
    };
    ptr::write(out, v);
    0
}

unsafe extern "C" fn jit_get_elements(
    arr: *const Value,
    start: *const Value,
    end: *const Value,
    out: *mut Value,
) -> u8 {
    match get_elements(&*arr, &*start, &*end) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

// Macro for specialized "bridge" helpers. Each one shares the same ABI and
// dispatches to a specific `ExprEval` method based on the IR variant
// at the path. This lets JIT skip the generic `eval()` match dispatch.
//
// The bridge receives a path (sequence of child indices from the root) rather
// than a NodeIdx, because NodeIdx is tied to a specific tree's memory: a
// CompiledExpr cached across distinct trees with the same fingerprint must
// not bake in indices from the original tree.
unsafe fn resolve_node<'t>(
    tree: &'t DynTree<ExprIR<Variable>>,
    root_idx: &NodeIdx<Dyn<ExprIR<Variable>>>,
    path: &[usize],
) -> DynNode<'t, ExprIR<Variable>> {
    let mut node = tree.node(root_idx.clone());
    for &i in path {
        node = node.child(i);
    }
    node
}

macro_rules! specialized_bridge {
    ($name:ident, |$eval:ident, $env:ident, $tree:ident, $node:ident, $args:ident, $helpers:ident| $body:expr) => {
        unsafe extern "C" fn $name(
            eval: *const c_void,
            env: *const c_void,
            tree: *const c_void,
            idx: *const c_void,
            path_ptr: *const usize,
            path_len: usize,
            args_ptr: *const *const Value,
            n_args: usize,
            helpers_ptr: *const *const CompiledExpr,
            n_helpers: usize,
            out: *mut Value,
        ) -> u8 {
            let $eval = &*(eval as *const ExprEval<'_>);
            let $tree = &*(tree as *const QueryExprInner<Variable>);
            let $env = if env.is_null() {
                None
            } else {
                Some(&*(env as *const Env<'_>))
            };
            let root_idx = &*(idx as *const NodeIdx<Dyn<ExprIR<Variable>>>);
            let path = std::slice::from_raw_parts(path_ptr, path_len);
            let $node = resolve_node(&$tree.tree, root_idx, path);
            let $args: &[*const Value] = if n_args == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(args_ptr, n_args)
            };
            let $helpers: &[*const CompiledExpr] = if n_helpers == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(helpers_ptr, n_helpers)
            };
            let result: Result<Value, String> = $body;
            match result {
                Ok(v) => {
                    ptr::write(out, v);
                    0
                }
                Err(e) => {
                    ptr::write(out, Value::Null);
                    set_err(e);
                    1
                }
            }
        }
    };
}

specialized_bridge!(jit_quantifier, |eval, env, tree, node, args, helpers| {
    if let ExprIR::Quantifier {
        quantifier_type,
        var,
    } = node.data()
    {
        debug_assert_eq!(args.len(), 1);
        debug_assert_eq!(helpers.len(), 1);
        let list: &Value = unsafe { &*args[0] };
        match unsafe { helpers[0].as_ref() } {
            Some(body_ce) => eval.eval_quantifier_expr(
                tree,
                node.idx(),
                env,
                current_agg_key(),
                quantifier_type,
                var,
                list,
                body_ce,
            ),
            None => Err(String::from("quantifier body not JIT-compilable")),
        }
    } else {
        Err("jit_quantifier: not a Quantifier node".into())
    }
});

specialized_bridge!(jit_list_comp, |eval, env, tree, node, _args, helpers| {
    if let ExprIR::ListComprehension(var) = node.data() {
        debug_assert!(helpers.len() >= 2);
        match (unsafe { helpers[0].as_ref() }, unsafe {
            helpers[1].as_ref()
        }) {
            (Some(pred_ce), Some(proj_ce)) => eval.eval_list_comprehension_expr(
                tree,
                node.idx(),
                env,
                current_agg_key(),
                var,
                pred_ce,
                proj_ce,
                &helpers[2..],
            ),
            _ => Err(String::from("list-comp pred/proj not JIT-compilable")),
        }
    } else {
        Err("jit_list_comp: not a ListComprehension node".into())
    }
});

specialized_bridge!(jit_reduce, |eval, env, tree, node, args, helpers| {
    if let ExprIR::Reduce {
        accumulator,
        iterator,
    } = node.data()
    {
        debug_assert_eq!(args.len(), 2);
        debug_assert_eq!(helpers.len(), 1);
        let init: &Value = unsafe { &*args[0] };
        let list: &Value = unsafe { &*args[1] };
        match unsafe { helpers[0].as_ref() } {
            Some(body_ce) => eval.eval_reduce_expr(
                tree,
                node.idx(),
                env,
                current_agg_key(),
                accumulator,
                iterator,
                init,
                list,
                body_ce,
            ),
            None => Err(String::from("reduce body not JIT-compilable")),
        }
    } else {
        Err("jit_reduce: not a Reduce node".into())
    }
});

specialized_bridge!(
    jit_map_projection,
    |eval, env, tree, node, args, helpers| {
        debug_assert_eq!(args.len(), 1);
        let base: &Value = unsafe { &*args[0] };
        eval.eval_map_projection(tree, node.idx(), env, current_agg_key(), base, helpers)
    }
);

specialized_bridge!(jit_distinct, |eval, env, tree, node, args, _helpers| {
    let borrowed: ThinVec<&Value> = args.iter().map(|p| unsafe { &**p }).collect();
    eval.eval_distinct(tree, node.idx(), env, current_agg_key(), &borrowed)
});

specialized_bridge!(
    jit_list_runtime,
    |_eval, _env, _tree, _node, args, _helpers| {
        let list: ThinVec<Value> = args.iter().map(|p| unsafe { (&**p).clone() }).collect();
        Ok(Value::List(Arc::new(list)))
    }
);

specialized_bridge!(
    jit_shortest_path,
    |eval, env, tree, node, args, _helpers| {
        if let ExprIR::ShortestPath {
            rel_types,
            min_hops,
            max_hops,
            directed,
            all_paths,
        } = node.data()
        {
            debug_assert_eq!(args.len(), 2);
            let src: &Value = unsafe { &*args[0] };
            let dst: &Value = unsafe { &*args[1] };
            eval.eval_shortest_path(
                tree,
                node.idx(),
                env,
                None,
                rel_types,
                *min_hops,
                *max_hops,
                *directed,
                *all_paths,
                src,
                dst,
            )
        } else {
            Err("jit_shortest_path: not a ShortestPath node".into())
        }
    }
);

// Aggregator FuncInvocation bridge. Mirrors the original interpreter:
// during finalization (agg_group_key is None and last child is a Variable
// holding the accumulator) we MUST NOT evaluate the other args — only read
// the accumulator from env and apply the finalizer. The standard JIT path
// would eagerly evaluate every arg, which fails when the post-aggregation
// env rebinds variables that were used inside arg expressions (e.g. the
// group-key var rebound to the key value).
unsafe extern "C" fn jit_agg_func_call(
    eval: *const c_void,
    env: *const c_void,
    tree: *const c_void,
    idx: *const c_void,
    path_ptr: *const usize,
    path_len: usize,
    _args_ptr: *const *const Value,
    _n_args: usize,
    helpers_ptr: *const *const CompiledExpr,
    n_helpers: usize,
    func_ptr: *const Arc<GraphFn>,
    out: *mut Value,
) -> u8 {
    let eval_ref = &*(eval as *const ExprEval<'_>);
    let tree_ref = &*(tree as *const QueryExprInner<Variable>);
    let env_opt = if env.is_null() {
        None
    } else {
        Some(&*(env as *const Env<'_>))
    };
    let root_idx = &*(idx as *const NodeIdx<Dyn<ExprIR<Variable>>>);
    let path = std::slice::from_raw_parts(path_ptr, path_len);
    let node = resolve_node(&tree_ref.tree, root_idx, path);
    let func = &**func_ptr;
    let agg_key = current_agg_key();
    let helpers: &[*const CompiledExpr] = if n_helpers == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(helpers_ptr, n_helpers)
    };

    if agg_key.is_none()
        && let FnType::Aggregation { finalizer, .. } = &func.fn_type
        && node.num_children() >= 1
        && let ExprIR::Variable(key) = node.child(node.num_children() - 1).data()
    {
        let Some(e) = env_opt else {
            ptr::write(out, Value::Null);
            set_err(String::from("Variable not found"));
            return 1;
        };
        let acc = e.get_by_id(key.id).cloned().unwrap_or(Value::Null);
        let v = match finalizer {
            Some(f) => f(acc),
            None => acc,
        };
        ptr::write(out, v);
        return 0;
    }

    // Accumulate / non-finalize path: evaluate all children via pre-compiled
    // helpers (one per child), then call the function.
    let rt = match eval_ref.runtime_opt() {
        Some(rt) => rt,
        None => {
            ptr::write(out, Value::Null);
            set_err(String::from("not a constant expression"));
            return 1;
        }
    };
    let n = node.num_children();
    debug_assert_eq!(helpers.len(), n);
    let mut args: ThinVec<Value> = ThinVec::with_capacity(n);
    for i in 0..n {
        let child = node.child(i);
        let child_idx = child.idx();
        let child_ce_ptr = helpers[i];
        let res = if child_ce_ptr.is_null() {
            Err(format!(
                "agg arg #{i} not JIT-compilable: {:?}",
                child.data()
            ))
        } else {
            let ce = &*child_ce_ptr;
            ce.call(eval_ref, env_opt, tree_ref, &child_idx, agg_key)
        };
        match res {
            Ok(v) => args.push(v),
            Err(e) => {
                ptr::write(out, Value::Null);
                set_err(e);
                return 1;
            }
        }
    }
    // count(DISTINCT x): first arg is a Distinct list-wrapper; splice
    // its elements into the args list.
    if n >= 1 && matches!(node.child(0).data(), ExprIR::Distinct) {
        if let Value::List(values) = args.remove(0) {
            let mut values = Arc::unwrap_or_clone(values);
            values.append(&mut args);
            args = values;
        }
    }
    if let Err(e) = func.validate_args_type(&args) {
        ptr::write(out, Value::Null);
        set_err(e);
        return 1;
    }
    if !rt.write && func.write {
        ptr::write(out, Value::Null);
        set_err(String::from(
            "graph.RO_QUERY is to be executed only on read-only queries",
        ));
        return 1;
    }
    match (func.func)(rt, args) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

unsafe extern "C" fn jit_func_call(
    eval: *const c_void,
    func: *const Arc<GraphFn>,
    argc: u32,
    args_ptr: *const *const Value,
    out: *mut Value,
) -> u8 {
    jit_func_call_impl(eval, func, argc, args_ptr, out, false)
}

unsafe extern "C" fn jit_func_call_distinct(
    eval: *const c_void,
    func: *const Arc<GraphFn>,
    argc: u32,
    args_ptr: *const *const Value,
    out: *mut Value,
) -> u8 {
    jit_func_call_impl(eval, func, argc, args_ptr, out, true)
}

unsafe fn jit_func_call_impl(
    eval: *const c_void,
    func: *const Arc<GraphFn>,
    argc: u32,
    args_ptr: *const *const Value,
    out: *mut Value,
    unwrap_distinct: bool,
) -> u8 {
    let eval = &*(eval as *const ExprEval<'_>);
    let func = &**func;
    let rt = match eval.runtime_opt() {
        Some(rt) => rt,
        None => {
            ptr::write(out, Value::Null);
            set_err("not a constant expression".to_string());
            return 1;
        }
    };
    let mut args: ThinVec<Value> = ThinVec::with_capacity(argc as usize);
    for i in 0..argc as usize {
        let p = *args_ptr.add(i);
        args.push((*p).clone());
    }
    // Aggregator finalization: when called outside an accumulate phase,
    // the last arg is the accumulator; apply the finalizer if any.
    if current_agg_key().is_none()
        && let FnType::Aggregation { finalizer, .. } = &func.fn_type
    {
        let acc = args.pop().unwrap_or(Value::Null);
        let v = match finalizer {
            Some(f) => f(acc),
            None => acc,
        };
        ptr::write(out, v);
        return 0;
    }
    // count(DISTINCT x): the first arg is a Distinct list-wrapper; splice
    // its elements into the args list before invoking the aggregator.
    if unwrap_distinct
        && args.len() >= 1
        && let Value::List(values) = args.remove(0)
    {
        let mut values = Arc::unwrap_or_clone(values);
        values.append(&mut args);
        args = values;
    }
    if let Err(e) = func.validate_args_type(&args) {
        ptr::write(out, Value::Null);
        set_err(e);
        return 1;
    }
    if !rt.write && func.write {
        ptr::write(out, Value::Null);
        set_err("graph.RO_QUERY is to be executed only on read-only queries".to_string());
        return 1;
    }
    match (func.func)(rt, args) {
        Ok(v) => {
            ptr::write(out, v);
            0
        }
        Err(e) => {
            ptr::write(out, Value::Null);
            set_err(e);
            1
        }
    }
}

// ---------------------------------------------------------------------------
// Compiler
// ---------------------------------------------------------------------------

pub fn try_compile(
    tree: &DynTree<ExprIR<Variable>>,
    root: NodeIdx<Dyn<ExprIR<Variable>>>,
) -> Option<CompiledExpr> {
    // PatternComprehension/Pattern aren't JIT-compilable (see emit()).
    // When they appear at the root we'd waste a Cranelift module setup
    // on a guaranteed Err. Bail early.
    match tree.node(root).data() {
        ExprIR::PatternComprehension(_) | ExprIR::Pattern(_) => return None,
        _ => {}
    }
    let mut flags_builder = settings::builder();
    flags_builder.set("opt_level", "speed").ok();
    flags_builder.set("regalloc_algorithm", "single_pass").ok();
    let isa_builder = cranelift_native::builder().ok()?;
    let isa = isa_builder
        .finish(settings::Flags::new(flags_builder))
        .ok()?;

    let mut jb = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    register_helpers(&mut jb);
    let mut module = JITModule::new(jb);

    let pointer_type = module.target_config().pointer_type();
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(pointer_type)); // eval
    sig.params.push(AbiParam::new(pointer_type)); // env
    sig.params.push(AbiParam::new(pointer_type)); // tree
    sig.params.push(AbiParam::new(pointer_type)); // idx (subtree root NodeIdx)
    sig.params.push(AbiParam::new(pointer_type)); // out
    sig.returns.push(AbiParam::new(types::I8));

    let func_id = module.declare_function("expr", Linkage::Local, &sig).ok()?;

    let mut ctx = Context::new();
    ctx.func = Function::with_name_signature(UserFuncName::user(0, 0), sig);

    let mut builder_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

    let entry = builder.create_block();
    builder.append_block_params_for_function_params(entry);
    builder.switch_to_block(entry);
    builder.seal_block(entry);

    let eval_param = builder.block_params(entry)[0];
    let env_param = builder.block_params(entry)[1];
    let tree_param = builder.block_params(entry)[2];
    let idx_param = builder.block_params(entry)[3];
    let out_param = builder.block_params(entry)[4];

    // init_block initializes every stack slot to Value::Null. This makes the
    // exit block's value_drop calls safe even when conditional control flow
    // (short-circuit AND/OR) bypasses the helper that would normally write
    // a slot. We emit the init calls AFTER emit() finishes (when all slot
    // ids are known), then jump from init_block to main_block.
    let init_block = builder.create_block();
    let main_block = builder.create_block();
    builder.ins().jump(init_block, &[]);
    builder.switch_to_block(main_block);

    let mut state = CodegenState {
        module: &mut module,
        pointer_type,
        eval_param,
        env_param,
        tree_param,
        idx_param,
        root_idx: root.clone(),
        slots: Vec::new(),
        strings: Vec::new(),
        fn_arcs: Vec::new(),
        params: Vec::new(),
        paths: Vec::new(),
        helper_ptr_arrays: Vec::new(),
        helper_arcs: Vec::new(),
        const_values: Vec::new(),
        helpers: Helpers::default(),
    };

    let root_slot = match emit(&mut state, &mut builder, tree, root) {
        Ok(slot) => slot,
        Err(()) => {
            // Compilation gave up; nothing was finalized yet.
            return None;
        }
    };

    // Now that all stack slots are known, emit the per-slot Null inits in
    // init_block, then jump to main_block. This guarantees every slot is
    // initialized before any helper or drop runs, regardless of which CLIF
    // path the runtime takes.
    //
    // Cranelift requires the current block to be terminated before
    // `switch_to_block`, so jump from wherever emit() left us into a fresh
    // resume_block; the post-init code is appended there.
    let resume_block = builder.create_block();
    builder.ins().jump(resume_block, &[]);
    builder.switch_to_block(init_block);
    let null_helper = get_null(&mut state.helpers, state.module, &mut builder.func);
    let slots_snapshot: Vec<ir::StackSlot> = state.slots.clone();
    for slot in &slots_snapshot {
        let addr = builder.ins().stack_addr(state.pointer_type, *slot, 0);
        builder.ins().call(null_helper, &[addr]);
    }
    builder.ins().jump(main_block, &[]);
    builder.seal_block(init_block);
    builder.seal_block(main_block);
    builder.switch_to_block(resume_block);
    builder.seal_block(resume_block);

    let exit_block = builder.create_block();
    builder.append_block_param(exit_block, types::I8);

    // Success: copy root slot into out, then drop all slots.
    let root_addr = builder.ins().stack_addr(state.pointer_type, root_slot, 0);
    let value_size_bytes = builder
        .ins()
        .iconst(state.pointer_type, i64::from(VALUE_SIZE));
    builder.call_memcpy(
        state.module.target_config(),
        out_param,
        root_addr,
        value_size_bytes,
    );

    // Drop every non-root slot (root was moved into out).
    for &slot in &state.slots {
        if slot == root_slot {
            continue;
        }
        let drop_helper = state.helpers.value_drop(state.module, &mut builder.func);
        let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
        builder.ins().call(drop_helper, &[addr]);
    }
    let zero = builder.ins().iconst(types::I8, 0);
    builder.ins().jump(exit_block, &[zero.into()]);

    builder.switch_to_block(exit_block);
    builder.seal_block(exit_block);
    let rc = builder.block_params(exit_block)[0];
    builder.ins().return_(&[rc]);

    builder.finalize();

    let strings = std::mem::take(&mut state.strings);
    let fn_arcs = std::mem::take(&mut state.fn_arcs);
    let params = std::mem::take(&mut state.params);
    let paths = std::mem::take(&mut state.paths);
    let helper_ptr_arrays = std::mem::take(&mut state.helper_ptr_arrays);
    let helper_arcs = std::mem::take(&mut state.helper_arcs);
    let const_values = std::mem::take(&mut state.const_values);
    drop(state);

    module
        .define_function(func_id, &mut ctx)
        .map_err(|e| eprintln!("jit define error: {e}"))
        .ok()?;
    module.clear_context(&mut ctx);
    module
        .finalize_definitions()
        .map_err(|e| eprintln!("jit finalize error: {e}"))
        .ok()?;

    let raw = module.get_finalized_function(func_id);
    let func: JitFn = unsafe { std::mem::transmute(raw) };

    Some(CompiledExpr {
        func,
        _strings: strings,
        _fn_arcs: fn_arcs,
        _params: params,
        _paths: paths,
        _helper_ptr_arrays: helper_ptr_arrays,
        _helper_arcs: helper_arcs,
        _const_values: const_values,
        _module: Some(module),
    })
}

struct CodegenState<'m> {
    module: &'m mut JITModule,
    pointer_type: ir::Type,
    eval_param: ir::Value,
    env_param: ir::Value,
    tree_param: ir::Value,
    idx_param: ir::Value,
    /// The compile-time subtree root. Specialized bridges store paths
    /// relative to this root so they remain valid when the CompiledExpr
    /// is reused for a different tree (different NodeIdx) with the same
    /// fingerprint.
    root_idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    slots: Vec<ir::StackSlot>,
    /// Owned strings whose addresses are baked as constants. Box gives each
    /// `Arc` a stable heap address regardless of `Vec` reallocation.
    strings: Vec<Box<Arc<String>>>,
    /// Owned function Arcs whose addresses are baked as constants.
    fn_arcs: Vec<Box<Arc<GraphFn>>>,
    /// Owned parameter name strings whose addresses are baked as constants.
    params: Vec<Box<String>>,
    /// Owned paths (sequences of child indices from root) whose addresses
    /// are baked as constants for bridge helpers that re-enter
    /// `ExprEval::eval` on a sub-expression. Paths are tree-structure-
    /// relative so a CompiledExpr remains valid when shared across distinct
    /// trees with the same fingerprint.
    paths: Vec<Box<Vec<usize>>>,
    helper_ptr_arrays: Vec<Box<Vec<*const CompiledExpr>>>,
    helper_arcs: Vec<Arc<CompiledExpr>>,
    const_values: Vec<Box<Value>>,
    helpers: Helpers,
}

#[derive(Default)]
struct Helpers {
    int: Option<ir::FuncRef>,
    const_clone: Option<ir::FuncRef>,
    float: Option<ir::FuncRef>,
    bool_: Option<ir::FuncRef>,
    null: Option<ir::FuncRef>,
    var: Option<ir::FuncRef>,
    property: Option<ir::FuncRef>,
    add: Option<ir::FuncRef>,
    sub: Option<ir::FuncRef>,
    mul: Option<ir::FuncRef>,
    div: Option<ir::FuncRef>,
    modulo: Option<ir::FuncRef>,
    negate: Option<ir::FuncRef>,
    not_: Option<ir::FuncRef>,
    lt: Option<ir::FuncRef>,
    gt: Option<ir::FuncRef>,
    le: Option<ir::FuncRef>,
    ge: Option<ir::FuncRef>,
    drop_: Option<ir::FuncRef>,
    func_call: Option<ir::FuncRef>,
    func_call_distinct: Option<ir::FuncRef>,
    string: Option<ir::FuncRef>,
    param: Option<ir::FuncRef>,
    length: Option<ir::FuncRef>,
    is_node: Option<ir::FuncRef>,
    is_rel: Option<ir::FuncRef>,
    in_: Option<ir::FuncRef>,
    pow: Option<ir::FuncRef>,
    xor: Option<ir::FuncRef>,
    and_n: Option<ir::FuncRef>,
    or_n: Option<ir::FuncRef>,
    bool_state: Option<ir::FuncRef>,
    write_logical: Option<ir::FuncRef>,
    lt_state: Option<ir::FuncRef>,
    gt_state: Option<ir::FuncRef>,
    le_state: Option<ir::FuncRef>,
    ge_state: Option<ir::FuncRef>,
    eq_state: Option<ir::FuncRef>,
    neq_state: Option<ir::FuncRef>,
    eq_n: Option<ir::FuncRef>,
    neq_n: Option<ir::FuncRef>,
    list: Option<ir::FuncRef>,
    map: Option<ir::FuncRef>,
    get_element: Option<ir::FuncRef>,
    get_elements: Option<ir::FuncRef>,
    quantifier: Option<ir::FuncRef>,
    list_comp: Option<ir::FuncRef>,
    reduce: Option<ir::FuncRef>,
    map_projection: Option<ir::FuncRef>,
    shortest_path: Option<ir::FuncRef>,
    distinct: Option<ir::FuncRef>,
    list_runtime: Option<ir::FuncRef>,
    agg_func_call: Option<ir::FuncRef>,
}

fn register_helpers(jb: &mut JITBuilder) {
    macro_rules! reg {
        ($name:literal, $func:ident) => {
            jb.symbol($name, $func as *const u8);
        };
    }
    reg!("jit_int", jit_int);
    reg!("jit_const_clone", jit_const_clone);
    reg!("jit_float", jit_float);
    reg!("jit_bool", jit_bool);
    reg!("jit_null", jit_null);
    reg!("jit_var", jit_var);
    reg!("jit_property", jit_property);
    reg!("jit_add", jit_add);
    reg!("jit_sub", jit_sub);
    reg!("jit_mul", jit_mul);
    reg!("jit_div", jit_div);
    reg!("jit_modulo", jit_modulo);
    reg!("jit_negate", jit_negate);
    reg!("jit_not", jit_not);
    reg!("jit_and", jit_and);
    reg!("jit_or", jit_or);
    reg!("jit_lt", jit_lt);
    reg!("jit_gt", jit_gt);
    reg!("jit_le", jit_le);
    reg!("jit_ge", jit_ge);
    reg!("jit_eq", jit_eq);
    reg!("jit_neq", jit_neq);
    reg!("jit_value_drop", jit_value_drop);
    reg!("jit_func_call", jit_func_call);
    reg!("jit_func_call_distinct", jit_func_call_distinct);
    reg!("jit_string", jit_string);
    reg!("jit_param", jit_param);
    reg!("jit_length", jit_length);
    reg!("jit_is_node", jit_is_node);
    reg!("jit_is_rel", jit_is_rel);
    reg!("jit_in", jit_in);
    reg!("jit_pow", jit_pow);
    reg!("jit_xor", jit_xor);
    reg!("jit_and_n", jit_and_n);
    reg!("jit_or_n", jit_or_n);
    reg!("jit_bool_state", jit_bool_state);
    reg!("jit_write_logical", jit_write_logical);
    reg!("jit_lt_state", jit_lt_state);
    reg!("jit_gt_state", jit_gt_state);
    reg!("jit_le_state", jit_le_state);
    reg!("jit_ge_state", jit_ge_state);
    reg!("jit_eq_state", jit_eq_state);
    reg!("jit_neq_state", jit_neq_state);
    reg!("jit_eq_n", jit_eq_n);
    reg!("jit_neq_n", jit_neq_n);
    reg!("jit_list", jit_list);
    reg!("jit_map", jit_map);
    reg!("jit_get_element", jit_get_element);
    reg!("jit_get_elements", jit_get_elements);
    reg!("jit_quantifier", jit_quantifier);
    reg!("jit_list_comp", jit_list_comp);
    reg!("jit_reduce", jit_reduce);
    reg!("jit_map_projection", jit_map_projection);
    reg!("jit_shortest_path", jit_shortest_path);
    reg!("jit_distinct", jit_distinct);
    reg!("jit_list_runtime", jit_list_runtime);
    reg!("jit_agg_func_call", jit_agg_func_call);
}

impl Helpers {
    fn declare(
        module: &mut JITModule,
        name: &str,
        sig: Signature,
        func: &mut Function,
    ) -> ir::FuncRef {
        let id = module
            .declare_function(name, Linkage::Import, &sig)
            .expect("declare helper");
        module.declare_func_in_func(id, func)
    }

    fn binary_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // lhs
        sig.params.push(AbiParam::new(ptr)); // rhs
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn unary_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn bool_state_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // *const Value
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn write_logical_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // *mut Value
        sig.params.push(AbiParam::new(types::I8)); // state
        sig
    }

    fn cmp_state_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // *const Value (lhs)
        sig.params.push(AbiParam::new(ptr)); // *const Value (rhs)
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn int_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // out
        sig.params.push(AbiParam::new(types::I64));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn null_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn var_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // env
        sig.params.push(AbiParam::new(types::I32)); // var_id
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn property_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // eval
        sig.params.push(AbiParam::new(ptr)); // src
        sig.params.push(AbiParam::new(ptr)); // name
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn drop_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr));
        sig
    }

    fn func_call_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // eval
        sig.params.push(AbiParam::new(ptr)); // func arc ptr
        sig.params.push(AbiParam::new(types::I32)); // argc
        sig.params.push(AbiParam::new(ptr)); // args array ptr
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (out, *const Arc<String>) -> u8
    fn string_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // out
        sig.params.push(AbiParam::new(ptr)); // *const Arc<String>
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (eval, *const String, out) -> u8
    fn param_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (argc: i32, *const *const Value, out) -> u8
    fn nary_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(types::I32));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (n, keys_ptr, values_ptr, out) -> u8
    fn map_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(types::I32));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (arr, start, end, out) -> u8
    fn ternary_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.params.push(AbiParam::new(ptr));
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (eval, env, tree, *const NodeIdx idx, *const usize path, usize path_len,
    ///  *const *const Value args, usize n_args,
    ///  *const *const CompiledExpr helpers, usize n_helpers, out) -> u8
    fn bridge_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // eval
        sig.params.push(AbiParam::new(ptr)); // env
        sig.params.push(AbiParam::new(ptr)); // tree
        sig.params.push(AbiParam::new(ptr)); // idx (NodeIdx)
        sig.params.push(AbiParam::new(ptr)); // path_ptr
        sig.params.push(AbiParam::new(ptr)); // path_len
        sig.params.push(AbiParam::new(ptr)); // args_ptr (*const *const Value)
        sig.params.push(AbiParam::new(ptr)); // n_args
        sig.params.push(AbiParam::new(ptr)); // helpers_ptr (*const *const CompiledExpr)
        sig.params.push(AbiParam::new(ptr)); // n_helpers
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    /// (eval, env, tree, idx, path_ptr, path_len, args_ptr, n_args,
    ///  helpers_ptr, n_helpers, func_ptr, out) -> u8
    fn agg_bridge_sig(ptr: ir::Type) -> Signature {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(ptr)); // eval
        sig.params.push(AbiParam::new(ptr)); // env
        sig.params.push(AbiParam::new(ptr)); // tree
        sig.params.push(AbiParam::new(ptr)); // idx
        sig.params.push(AbiParam::new(ptr)); // path_ptr
        sig.params.push(AbiParam::new(ptr)); // path_len
        sig.params.push(AbiParam::new(ptr)); // args_ptr
        sig.params.push(AbiParam::new(ptr)); // n_args
        sig.params.push(AbiParam::new(ptr)); // helpers_ptr
        sig.params.push(AbiParam::new(ptr)); // n_helpers
        sig.params.push(AbiParam::new(ptr)); // func_ptr
        sig.params.push(AbiParam::new(ptr)); // out
        sig.returns.push(AbiParam::new(types::I8));
        sig
    }

    fn value_drop(
        &mut self,
        module: &mut JITModule,
        func: &mut Function,
    ) -> ir::FuncRef {
        if let Some(r) = self.drop_ {
            return r;
        }
        let ptr = module.target_config().pointer_type();
        let r = Self::declare(module, "jit_value_drop", Self::drop_sig(ptr), func);
        self.drop_ = Some(r);
        r
    }
}

macro_rules! helper_getter {
    ($func:ident, $field:ident, $name:literal, $sig:expr) => {
        fn $func(
            helpers: &mut Helpers,
            module: &mut JITModule,
            func: &mut Function,
        ) -> ir::FuncRef {
            if let Some(r) = helpers.$field {
                return r;
            }
            let ptr = module.target_config().pointer_type();
            let r = Helpers::declare(module, $name, $sig(ptr), func);
            helpers.$field = Some(r);
            r
        }
    };
}

helper_getter!(get_int, int, "jit_int", Helpers::int_sig);
helper_getter!(
    get_const_clone,
    const_clone,
    "jit_const_clone",
    Helpers::string_sig
);
helper_getter!(get_float, float, "jit_float", Helpers::int_sig);
helper_getter!(get_bool, bool_, "jit_bool", Helpers::int_sig);
helper_getter!(get_null, null, "jit_null", Helpers::null_sig);
helper_getter!(get_var, var, "jit_var", Helpers::var_sig);
helper_getter!(
    get_property,
    property,
    "jit_property",
    Helpers::property_sig
);
helper_getter!(get_add, add, "jit_add", Helpers::binary_sig);
helper_getter!(get_sub, sub, "jit_sub", Helpers::binary_sig);
helper_getter!(get_mul, mul, "jit_mul", Helpers::binary_sig);
helper_getter!(get_div, div, "jit_div", Helpers::binary_sig);
helper_getter!(get_modulo, modulo, "jit_modulo", Helpers::binary_sig);
helper_getter!(get_negate, negate, "jit_negate", Helpers::unary_sig);
helper_getter!(get_not, not_, "jit_not", Helpers::unary_sig);
helper_getter!(get_lt, lt, "jit_lt", Helpers::binary_sig);
helper_getter!(get_gt, gt, "jit_gt", Helpers::binary_sig);
helper_getter!(get_le, le, "jit_le", Helpers::binary_sig);
helper_getter!(get_ge, ge, "jit_ge", Helpers::binary_sig);
helper_getter!(
    get_func_call,
    func_call,
    "jit_func_call",
    Helpers::func_call_sig
);
helper_getter!(
    get_func_call_distinct,
    func_call_distinct,
    "jit_func_call_distinct",
    Helpers::func_call_sig
);
helper_getter!(get_string, string, "jit_string", Helpers::string_sig);
helper_getter!(get_param, param, "jit_param", Helpers::param_sig);
helper_getter!(get_length, length, "jit_length", Helpers::unary_sig);
helper_getter!(get_is_node, is_node, "jit_is_node", Helpers::unary_sig);
helper_getter!(get_is_rel, is_rel, "jit_is_rel", Helpers::unary_sig);
helper_getter!(get_in, in_, "jit_in", Helpers::binary_sig);
helper_getter!(get_pow, pow, "jit_pow", Helpers::binary_sig);
helper_getter!(get_xor, xor, "jit_xor", Helpers::binary_sig);
helper_getter!(get_and_n, and_n, "jit_and_n", Helpers::nary_sig);
helper_getter!(get_or_n, or_n, "jit_or_n", Helpers::nary_sig);
helper_getter!(
    get_bool_state,
    bool_state,
    "jit_bool_state",
    Helpers::bool_state_sig
);
helper_getter!(
    get_write_logical,
    write_logical,
    "jit_write_logical",
    Helpers::write_logical_sig
);
helper_getter!(
    get_lt_state,
    lt_state,
    "jit_lt_state",
    Helpers::cmp_state_sig
);
helper_getter!(
    get_gt_state,
    gt_state,
    "jit_gt_state",
    Helpers::cmp_state_sig
);
helper_getter!(
    get_le_state,
    le_state,
    "jit_le_state",
    Helpers::cmp_state_sig
);
helper_getter!(
    get_ge_state,
    ge_state,
    "jit_ge_state",
    Helpers::cmp_state_sig
);
helper_getter!(
    get_eq_state,
    eq_state,
    "jit_eq_state",
    Helpers::cmp_state_sig
);
helper_getter!(
    get_neq_state,
    neq_state,
    "jit_neq_state",
    Helpers::cmp_state_sig
);
helper_getter!(get_eq_n, eq_n, "jit_eq_n", Helpers::nary_sig);
helper_getter!(get_neq_n, neq_n, "jit_neq_n", Helpers::nary_sig);
helper_getter!(get_list, list, "jit_list", Helpers::nary_sig);
helper_getter!(get_map, map, "jit_map", Helpers::map_sig);
helper_getter!(
    get_get_element,
    get_element,
    "jit_get_element",
    Helpers::property_sig
);
helper_getter!(
    get_get_elements,
    get_elements,
    "jit_get_elements",
    Helpers::ternary_sig
);
helper_getter!(
    get_quantifier,
    quantifier,
    "jit_quantifier",
    Helpers::bridge_sig
);
helper_getter!(
    get_list_comp,
    list_comp,
    "jit_list_comp",
    Helpers::bridge_sig
);
helper_getter!(get_reduce, reduce, "jit_reduce", Helpers::bridge_sig);
helper_getter!(
    get_map_projection,
    map_projection,
    "jit_map_projection",
    Helpers::bridge_sig
);
helper_getter!(
    get_shortest_path,
    shortest_path,
    "jit_shortest_path",
    Helpers::bridge_sig
);
helper_getter!(get_distinct, distinct, "jit_distinct", Helpers::bridge_sig);
helper_getter!(
    get_list_runtime,
    list_runtime,
    "jit_list_runtime",
    Helpers::bridge_sig
);
helper_getter!(
    get_agg_func_call,
    agg_func_call,
    "jit_agg_func_call",
    Helpers::agg_bridge_sig
);

fn alloc_slot(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
) -> ir::StackSlot {
    let slot = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        VALUE_SIZE,
        3, // align 8
    ));
    state.slots.push(slot);
    slot
}

fn emit(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
) -> Result<ir::StackSlot, ()> {
    let node = tree.node(idx);
    match node.data() {
        ExprIR::Null => emit_null(state, builder),
        ExprIR::Bool(b) => emit_bool(state, builder, *b),
        ExprIR::Integer(i) => emit_int(state, builder, *i),
        ExprIR::Float(f) => emit_float(state, builder, *f),
        ExprIR::String(s) => emit_string(state, builder, s.clone()),
        ExprIR::Parameter(name) => emit_param(state, builder, name.clone()),
        ExprIR::Variable(v) => emit_var(state, builder, v.id),
        ExprIR::Property(name) => {
            let src = emit(state, builder, tree, node.child(0).idx())?;
            emit_property(state, builder, src, name.clone())
        }
        ExprIR::Add => emit_chain(state, builder, tree, &node, get_add),
        ExprIR::Sub => emit_chain(state, builder, tree, &node, get_sub),
        ExprIR::Mul => emit_chain(state, builder, tree, &node, get_mul),
        ExprIR::Div => emit_chain(state, builder, tree, &node, get_div),
        ExprIR::Modulo => emit_chain(state, builder, tree, &node, get_modulo),
        ExprIR::Pow => emit_chain(state, builder, tree, &node, get_pow),
        ExprIR::Negate => emit_unary(state, builder, tree, &node, get_negate),
        ExprIR::Not => emit_unary(state, builder, tree, &node, get_not),
        ExprIR::Length => emit_unary(state, builder, tree, &node, get_length),
        ExprIR::IsNode => emit_unary(state, builder, tree, &node, get_is_node),
        ExprIR::IsRelationship => emit_unary(state, builder, tree, &node, get_is_rel),
        ExprIR::In => emit_binary(state, builder, tree, &node, get_in),
        ExprIR::Xor if node.num_children() == 2 => {
            emit_binary(state, builder, tree, &node, get_xor)
        }
        ExprIR::Xor => emit_chain(state, builder, tree, &node, get_xor),
        ExprIR::And => emit_short_circuit(state, builder, tree, &node, true),
        ExprIR::Or => emit_short_circuit(state, builder, tree, &node, false),
        ExprIR::Eq => emit_nary(state, builder, tree, &node, get_eq_n),
        ExprIR::Neq => emit_nary(state, builder, tree, &node, get_neq_n),
        ExprIR::Lt => emit_binary(state, builder, tree, &node, get_lt),
        ExprIR::Gt => emit_binary(state, builder, tree, &node, get_gt),
        ExprIR::Le => emit_binary(state, builder, tree, &node, get_le),
        ExprIR::Ge => emit_binary(state, builder, tree, &node, get_ge),
        ExprIR::Paren => emit(state, builder, tree, node.child(0).idx()),
        ExprIR::FuncInvocation(func) => emit_func_call(state, builder, tree, &node, func.clone()),
        ExprIR::List if node.num_children() > 65536 => {
            // Very large list literals would require a multi-MB stack slot for
            // the args array (8 bytes per child). Try to constant-fold; if all
            // children are simple literals we can pre-evaluate the whole list
            // at codegen and bake it as a const. Otherwise bail out of JIT.
            match try_fold_literal_list(&node) {
                Some(value) => emit_const_value(state, builder, value),
                None => Err(()),
            }
        }
        ExprIR::List if node.num_children() > 64 => {
            let all: Vec<usize> = (0..node.num_children()).collect();
            emit_specialized_bridge_with_args(state, builder, tree, idx, get_list_runtime, &all)
        }
        ExprIR::List => emit_nary(state, builder, tree, &node, get_list),
        ExprIR::GetElement => emit_get_element(state, builder, tree, &node),
        ExprIR::GetElements => emit_ternary(state, builder, tree, &node, get_get_elements),
        ExprIR::Map => emit_map(state, builder, tree, &node),
        // Specialized bridges that dispatch directly to extracted ExprEval methods,
        // avoiding the generic eval-dispatch match.
        ExprIR::MapProjection => {
            let mut pre_compile_paths: Vec<Vec<usize>> = Vec::new();
            for i in 1..node.num_children() {
                if matches!(node.child(i).data(), ExprIR::String(_)) {
                    pre_compile_paths.push(vec![i, 0]);
                }
            }
            emit_specialized_bridge_full(
                state,
                builder,
                tree,
                idx,
                get_map_projection,
                &[0],
                &pre_compile_paths,
            )
        }
        ExprIR::Quantifier { .. } => emit_specialized_bridge_full(
            state,
            builder,
            tree,
            idx,
            get_quantifier,
            &[0],
            &[vec![1]],
        ),
        ExprIR::ListComprehension(_) => {
            // helpers: [pred, proj, then iter helpers — either [range.start,
            // range.end, optional range.step] or [whole_list_expr]].
            let mut pre_compile_paths: Vec<Vec<usize>> = vec![vec![1], vec![2]];
            let iter_node = node.child(0);
            if let ExprIR::FuncInvocation(func) = iter_node.data()
                && func.name == "range"
            {
                let n = iter_node.num_children();
                for i in 0..n {
                    pre_compile_paths.push(vec![0, i]);
                }
            } else {
                pre_compile_paths.push(vec![0]);
            }
            emit_specialized_bridge_full(
                state,
                builder,
                tree,
                idx,
                get_list_comp,
                &[],
                &pre_compile_paths,
            )
        }
        ExprIR::Reduce { .. } => {
            emit_specialized_bridge_full(state, builder, tree, idx, get_reduce, &[0, 1], &[vec![2]])
        }
        ExprIR::ShortestPath { .. } => {
            emit_specialized_bridge_with_args(state, builder, tree, idx, get_shortest_path, &[0, 1])
        }
        ExprIR::Distinct => {
            let all: Vec<usize> = (0..node.num_children()).collect();
            emit_specialized_bridge_with_args(state, builder, tree, idx, get_distinct, &all)
        }
        // Pattern/PatternComprehension are unreachable at eval time (the
        // planner consumes them). JIT skips them; expressions containing
        // them fall back to the interpreter.
        ExprIR::PatternComprehension(_) | ExprIR::Pattern(_) => Err(()),
    }
}

fn emit_null(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_null(&mut state.helpers, state.module, &mut builder.func);
    let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let call = builder.ins().call(helper, &[addr]);
    let _ = builder.inst_results(call);
    Ok(slot)
}

fn emit_bool(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    b: bool,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_bool(&mut state.helpers, state.module, &mut builder.func);
    let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let val = builder.ins().iconst(types::I64, i64::from(b));
    builder.ins().call(helper, &[addr, val]);
    Ok(slot)
}

fn emit_int(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    i: i64,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_int(&mut state.helpers, state.module, &mut builder.func);
    let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let val = builder.ins().iconst(types::I64, i);
    builder.ins().call(helper, &[addr, val]);
    Ok(slot)
}

fn emit_const_value(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    value: Value,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_const_clone(&mut state.helpers, state.module, &mut builder.func);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let boxed: Box<Value> = Box::new(value);
    let val_ptr = boxed.as_ref() as *const Value as usize;
    state.const_values.push(boxed);
    let val_const = builder.ins().iconst(state.pointer_type, val_ptr as i64);
    builder.ins().call(helper, &[out_addr, val_const]);
    Ok(slot)
}

fn try_fold_literal(node: &DynNode<'_, ExprIR<Variable>>) -> Option<Value> {
    match node.data() {
        ExprIR::Null => Some(Value::Null),
        ExprIR::Bool(b) => Some(Value::Bool(*b)),
        ExprIR::Integer(i) => Some(Value::Int(*i)),
        ExprIR::Float(f) => Some(Value::Float(*f)),
        ExprIR::String(s) => Some(Value::String(s.clone())),
        _ => None,
    }
}

fn try_fold_literal_list(node: &DynNode<'_, ExprIR<Variable>>) -> Option<Value> {
    let n = node.num_children();
    let mut items: ThinVec<Value> = ThinVec::with_capacity(n);
    for i in 0..n {
        items.push(try_fold_literal(&node.child(i))?);
    }
    Some(Value::List(Arc::new(items)))
}

fn emit_float(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    f: f64,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_float(&mut state.helpers, state.module, &mut builder.func);
    let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let val = builder.ins().iconst(types::I64, f.to_bits() as i64);
    builder.ins().call(helper, &[addr, val]);
    Ok(slot)
}

fn emit_var(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    var_id: u32,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_var(&mut state.helpers, state.module, &mut builder.func);
    let addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let id = builder.ins().iconst(types::I32, i64::from(var_id));
    builder.ins().call(helper, &[state.env_param, id, addr]);
    Ok(slot)
}

fn emit_property(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    src: ir::StackSlot,
    name: Arc<String>,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_property(&mut state.helpers, state.module, &mut builder.func);
    let src_addr = builder.ins().stack_addr(state.pointer_type, src, 0);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let boxed = Box::new(name);
    let name_ptr = boxed.as_ref() as *const Arc<String> as usize;
    state.strings.push(boxed);
    let name_const = builder.ins().iconst(state.pointer_type, name_ptr as i64);
    builder
        .ins()
        .call(helper, &[state.eval_param, src_addr, name_const, out_addr]);
    Ok(slot)
}

type HelperGetter = fn(&mut Helpers, &mut JITModule, &mut Function) -> ir::FuncRef;

fn emit_unary(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    let src = emit(state, builder, tree, node.child(0).idx())?;
    let slot = alloc_slot(state, builder);
    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    let src_addr = builder.ins().stack_addr(state.pointer_type, src, 0);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    builder.ins().call(helper, &[src_addr, out_addr]);
    Ok(slot)
}

fn emit_get_element(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
) -> Result<ir::StackSlot, ()> {
    let lhs = emit(state, builder, tree, node.child(0).idx())?;
    let rhs = emit(state, builder, tree, node.child(1).idx())?;
    let slot = alloc_slot(state, builder);
    let helper = get_get_element(&mut state.helpers, state.module, &mut builder.func);
    let lhs_addr = builder.ins().stack_addr(state.pointer_type, lhs, 0);
    let rhs_addr = builder.ins().stack_addr(state.pointer_type, rhs, 0);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    builder
        .ins()
        .call(helper, &[state.eval_param, lhs_addr, rhs_addr, out_addr]);
    Ok(slot)
}

/// Short-circuiting AND/OR codegen.
///
/// `is_and` controls semantics:
/// - AND: short-circuit on `false`(0); default result `true`(1).
/// - OR:  short-circuit on `true`(1);  default result `false`(0).
/// State 2 (Null) propagates per Cypher three-valued logic. State 3 is a
/// type error; LAST_ERR is already set, we abort to done so later operands
/// (which may also error) don't run.
/// Evaluate `idx` directly to a bool state code (i8: 0=false, 1=true,
/// 2=null, 3=err) without materializing it as a `Value`. Falls back to
/// emit + jit_bool_state for non-bool-producing variants.
fn emit_state(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
) -> Result<ir::Value, ()> {
    let node = tree.node(idx.clone());
    match node.data() {
        ExprIR::Bool(b) => Ok(builder.ins().iconst(types::I8, if *b { 1 } else { 0 })),
        ExprIR::Null => Ok(builder.ins().iconst(types::I8, 2)),
        ExprIR::Lt => emit_cmp_state(state, builder, tree, &node, get_lt_state),
        ExprIR::Gt => emit_cmp_state(state, builder, tree, &node, get_gt_state),
        ExprIR::Le => emit_cmp_state(state, builder, tree, &node, get_le_state),
        ExprIR::Ge => emit_cmp_state(state, builder, tree, &node, get_ge_state),
        ExprIR::Eq if node.num_children() == 2 => {
            emit_cmp_state(state, builder, tree, &node, get_eq_state)
        }
        ExprIR::Neq if node.num_children() == 2 => {
            emit_cmp_state(state, builder, tree, &node, get_neq_state)
        }
        ExprIR::Not => {
            let inner = emit_state(state, builder, tree, node.child(0).idx())?;
            // Map 0→1, 1→0, 2→2, 3→3.
            let two = builder.ins().iconst(types::I8, 2);
            let one = builder.ins().iconst(types::I8, 1);
            let is_bool = builder.ins().icmp(IntCC::UnsignedLessThan, inner, two);
            let inverted = builder.ins().isub(one, inner);
            Ok(builder.ins().select(is_bool, inverted, inner))
        }
        _ => {
            let v_slot = emit(state, builder, tree, idx)?;
            let v_addr = builder.ins().stack_addr(state.pointer_type, v_slot, 0);
            let helper = get_bool_state(&mut state.helpers, state.module, &mut builder.func);
            let call = builder.ins().call(helper, &[v_addr]);
            Ok(builder.inst_results(call)[0])
        }
    }
}

fn emit_cmp_state(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::Value, ()> {
    let lhs = emit(state, builder, tree, node.child(0).idx())?;
    let rhs = emit(state, builder, tree, node.child(1).idx())?;
    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    let lhs_addr = builder.ins().stack_addr(state.pointer_type, lhs, 0);
    let rhs_addr = builder.ins().stack_addr(state.pointer_type, rhs, 0);
    let call = builder.ins().call(helper, &[lhs_addr, rhs_addr]);
    Ok(builder.inst_results(call)[0])
}

fn emit_short_circuit(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    is_and: bool,
) -> Result<ir::StackSlot, ()> {
    let result_slot = alloc_slot(state, builder);
    let null_seen_slot = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        1,
        0,
    ));
    let zero_i8 = builder.ins().iconst(types::I8, 0);
    builder.ins().stack_store(zero_i8, null_seen_slot, 0);

    let short_val: i64 = if is_and { 0 } else { 1 };
    let default_val: i64 = if is_and { 1 } else { 0 };
    let null_val: i64 = 2;
    let err_val: i64 = 3;

    let done_block = builder.create_block();
    builder.append_block_param(done_block, types::I8);

    let bool_state_helper = get_bool_state(&mut state.helpers, state.module, &mut builder.func);
    let _ = bool_state_helper; // referenced indirectly via emit_state

    let n = node.num_children();
    for i in 0..n {
        let st = emit_state(state, builder, tree, node.child(i).idx())?;

        let short_const = builder.ins().iconst(types::I8, short_val);
        let is_short = builder.ins().icmp(IntCC::Equal, st, short_const);
        let not_short = builder.create_block();
        builder
            .ins()
            .brif(is_short, done_block, &[st.into()], not_short, &[]);
        builder.switch_to_block(not_short);
        builder.seal_block(not_short);

        let err_const = builder.ins().iconst(types::I8, err_val);
        let is_err = builder.ins().icmp(IntCC::Equal, st, err_const);
        let not_err = builder.create_block();
        builder
            .ins()
            .brif(is_err, done_block, &[st.into()], not_err, &[]);
        builder.switch_to_block(not_err);
        builder.seal_block(not_err);

        let null_const = builder.ins().iconst(types::I8, null_val);
        let is_null = builder.ins().icmp(IntCC::Equal, st, null_const);
        let null_block = builder.create_block();
        let next_block = builder.create_block();
        builder
            .ins()
            .brif(is_null, null_block, &[], next_block, &[]);
        builder.switch_to_block(null_block);
        builder.seal_block(null_block);
        let one_i8 = builder.ins().iconst(types::I8, 1);
        builder.ins().stack_store(one_i8, null_seen_slot, 0);
        builder.ins().jump(next_block, &[]);
        builder.switch_to_block(next_block);
        builder.seal_block(next_block);
    }

    let ns = builder.ins().stack_load(types::I8, null_seen_slot, 0);
    let default_const = builder.ins().iconst(types::I8, default_val);
    let null_const = builder.ins().iconst(types::I8, null_val);
    let ns_nonzero = builder.ins().icmp_imm(IntCC::NotEqual, ns, 0);
    let final_state = builder.ins().select(ns_nonzero, null_const, default_const);
    builder.ins().jump(done_block, &[final_state.into()]);

    builder.switch_to_block(done_block);
    builder.seal_block(done_block);
    let final_state_param = builder.block_params(done_block)[0];
    let write_helper = get_write_logical(&mut state.helpers, state.module, &mut builder.func);
    let result_addr = builder.ins().stack_addr(state.pointer_type, result_slot, 0);
    builder
        .ins()
        .call(write_helper, &[result_addr, final_state_param]);

    Ok(result_slot)
}

fn emit_binary(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    let lhs = emit(state, builder, tree, node.child(0).idx())?;
    let rhs = emit(state, builder, tree, node.child(1).idx())?;
    let slot = alloc_slot(state, builder);
    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    let lhs_addr = builder.ins().stack_addr(state.pointer_type, lhs, 0);
    let rhs_addr = builder.ins().stack_addr(state.pointer_type, rhs, 0);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    builder.ins().call(helper, &[lhs_addr, rhs_addr, out_addr]);
    Ok(slot)
}

fn emit_ternary(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    let a = emit(state, builder, tree, node.child(0).idx())?;
    let b = emit(state, builder, tree, node.child(1).idx())?;
    let c = emit(state, builder, tree, node.child(2).idx())?;
    let slot = alloc_slot(state, builder);
    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    let a_addr = builder.ins().stack_addr(state.pointer_type, a, 0);
    let b_addr = builder.ins().stack_addr(state.pointer_type, b, 0);
    let c_addr = builder.ins().stack_addr(state.pointer_type, c, 0);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    builder
        .ins()
        .call(helper, &[a_addr, b_addr, c_addr, out_addr]);
    Ok(slot)
}

fn emit_specialized_bridge(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    emit_specialized_bridge_full(state, builder, tree, idx, get, &[], &[])
}

/// Pre-evaluates the children listed in `pre_eval_children` (passed as
/// `args`) and pre-compiles the sub-trees rooted at the children listed in
/// `pre_compile_children` (passed as `helpers`). The bridge sees them as
/// `args: &[*const Value]` and `helpers: &[*const CompiledExpr]`.
fn emit_specialized_bridge_with_args(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    get: HelperGetter,
    pre_eval_children: &[usize],
) -> Result<ir::StackSlot, ()> {
    emit_specialized_bridge_full(state, builder, tree, idx, get, pre_eval_children, &[])
}

fn emit_specialized_bridge_full(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    get: HelperGetter,
    pre_eval_children: &[usize],
    pre_compile_paths: &[Vec<usize>],
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);

    // Pre-evaluate the requested children.
    let mut arg_slots: Vec<ir::StackSlot> = Vec::with_capacity(pre_eval_children.len());
    for &ci in pre_eval_children {
        let child_slot = emit(state, builder, tree, tree.node(idx.clone()).child(ci).idx())?;
        arg_slots.push(child_slot);
    }

    let ptr_size = u32::from(state.pointer_type.bytes());
    let n_args = arg_slots.len();
    let (args_addr, n_args_const) = if n_args == 0 {
        (
            builder.ins().iconst(state.pointer_type, 0),
            builder.ins().iconst(state.pointer_type, 0),
        )
    } else {
        let args_slot = builder.create_sized_stack_slot(ir::StackSlotData::new(
            ir::StackSlotKind::ExplicitSlot,
            ptr_size * n_args as u32,
            3,
        ));
        for (i, child_slot) in arg_slots.iter().enumerate() {
            let child_addr = builder.ins().stack_addr(state.pointer_type, *child_slot, 0);
            builder.ins().stack_store(
                child_addr,
                args_slot,
                i32::try_from(i).unwrap() * i32::try_from(ptr_size).unwrap(),
            );
        }
        (
            builder.ins().stack_addr(state.pointer_type, args_slot, 0),
            builder.ins().iconst(state.pointer_type, n_args as i64),
        )
    };

    // Pre-compile sub-trees and bake their `*const CompiledExpr` ptrs into
    // a heap-allocated array. Null entries indicate sub-trees that were not
    // JIT-compilable; bridges must treat them as a hard error.
    let n_helpers = pre_compile_paths.len();
    let (helpers_addr, n_helpers_const) = if n_helpers == 0 {
        (
            builder.ins().iconst(state.pointer_type, 0),
            builder.ins().iconst(state.pointer_type, 0),
        )
    } else {
        let mut ptrs: Vec<*const CompiledExpr> = Vec::with_capacity(n_helpers);
        for path in pre_compile_paths {
            let mut child_node = tree.node(idx.clone());
            for &p in path {
                child_node = child_node.child(p);
            }
            let child_idx = child_node.idx();
            match try_compile(tree, child_idx) {
                Some(ce) => {
                    let arc = Arc::new(ce);
                    let raw = Arc::as_ptr(&arc);
                    state.helper_arcs.push(arc);
                    ptrs.push(raw);
                }
                None => ptrs.push(std::ptr::null()),
            }
        }
        let boxed: Box<Vec<*const CompiledExpr>> = Box::new(ptrs);
        let arr_ptr = boxed.as_ptr() as usize;
        state.helper_ptr_arrays.push(boxed);
        (
            builder.ins().iconst(state.pointer_type, arr_ptr as i64),
            builder.ins().iconst(state.pointer_type, n_helpers as i64),
        )
    };

    // Path from compile-time root to this node (for `resolve_node` inside
    // the bridge).
    let mut path: Vec<usize> = Vec::new();
    let root_node_ptr = tree.node(state.root_idx.clone()).idx();
    let mut node = tree.node(idx);
    while node.idx() != root_node_ptr {
        path.push(node.sibling_idx());
        node = match node.parent() {
            Some(p) => p,
            None => break,
        };
    }
    path.reverse();
    let boxed: Box<Vec<usize>> = Box::new(path);
    let path_ptr = boxed.as_ptr() as usize;
    let path_len = boxed.len();
    state.paths.push(boxed);

    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    let path_const = builder.ins().iconst(state.pointer_type, path_ptr as i64);
    let path_len_const = builder.ins().iconst(state.pointer_type, path_len as i64);
    builder.ins().call(
        helper,
        &[
            state.eval_param,
            state.env_param,
            state.tree_param,
            state.idx_param,
            path_const,
            path_len_const,
            args_addr,
            n_args_const,
            helpers_addr,
            n_helpers_const,
            out_addr,
        ],
    );
    Ok(slot)
}

fn emit_chain(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    let n = node.num_children();
    if n == 0 {
        return Err(());
    }
    let mut acc = emit(state, builder, tree, node.child(0).idx())?;
    for i in 1..n {
        let rhs = emit(state, builder, tree, node.child(i).idx())?;
        let slot = alloc_slot(state, builder);
        let helper = get(&mut state.helpers, state.module, &mut builder.func);
        let lhs_addr = builder.ins().stack_addr(state.pointer_type, acc, 0);
        let rhs_addr = builder.ins().stack_addr(state.pointer_type, rhs, 0);
        let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
        builder.ins().call(helper, &[lhs_addr, rhs_addr, out_addr]);
        acc = slot;
    }
    Ok(acc)
}

fn emit_string(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    name: Arc<String>,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_string(&mut state.helpers, state.module, &mut builder.func);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let boxed = Box::new(name);
    let name_ptr = boxed.as_ref() as *const Arc<String> as usize;
    state.strings.push(boxed);
    let name_const = builder.ins().iconst(state.pointer_type, name_ptr as i64);
    builder.ins().call(helper, &[out_addr, name_const]);
    Ok(slot)
}

fn emit_param(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    name: String,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_param(&mut state.helpers, state.module, &mut builder.func);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);
    let boxed: Box<String> = Box::new(name);
    let name_ptr = boxed.as_ref() as *const String as usize;
    state.params.push(boxed);
    let name_const = builder.ins().iconst(state.pointer_type, name_ptr as i64);
    builder
        .ins()
        .call(helper, &[state.eval_param, name_const, out_addr]);
    Ok(slot)
}

fn emit_nary(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    get: HelperGetter,
) -> Result<ir::StackSlot, ()> {
    let argc = node.num_children();
    let mut arg_slots = Vec::with_capacity(argc);
    for i in 0..argc {
        arg_slots.push(emit(state, builder, tree, node.child(i).idx())?);
    }
    let ptr_size = u32::from(state.pointer_type.bytes());
    let args_array = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        ptr_size * argc.max(1) as u32,
        3,
    ));
    for (i, slot) in arg_slots.iter().enumerate() {
        let slot_addr = builder.ins().stack_addr(state.pointer_type, *slot, 0);
        builder.ins().stack_store(
            slot_addr,
            args_array,
            i32::try_from(i).unwrap() * i32::try_from(ptr_size).unwrap(),
        );
    }
    let args_array_addr = builder.ins().stack_addr(state.pointer_type, args_array, 0);
    let out_slot = alloc_slot(state, builder);
    let out_addr = builder.ins().stack_addr(state.pointer_type, out_slot, 0);
    let argc_const = builder.ins().iconst(types::I32, i64::from(argc as u32));
    let helper = get(&mut state.helpers, state.module, &mut builder.func);
    builder
        .ins()
        .call(helper, &[argc_const, args_array_addr, out_addr]);
    Ok(out_slot)
}

fn emit_map(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
) -> Result<ir::StackSlot, ()> {
    let n = node.num_children();
    // Each child must be a String key whose own first child is the value expr.
    let mut value_slots = Vec::with_capacity(n);
    let mut key_ptrs = Vec::with_capacity(n);
    for i in 0..n {
        let child = node.child(i);
        let key = match child.data() {
            ExprIR::String(k) => k.clone(),
            _ => return Err(()),
        };
        if child.num_children() != 1 {
            return Err(());
        }
        value_slots.push(emit(state, builder, tree, child.child(0).idx())?);
        let boxed = Box::new(key);
        let ptr = boxed.as_ref() as *const Arc<String> as usize;
        state.strings.push(boxed);
        key_ptrs.push(ptr);
    }
    let ptr_size = u32::from(state.pointer_type.bytes());
    let n_max = n.max(1) as u32;
    let keys_array = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        ptr_size * n_max,
        3,
    ));
    let values_array = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        ptr_size * n_max,
        3,
    ));
    for (i, (key_ptr, value_slot)) in key_ptrs.iter().zip(value_slots.iter()).enumerate() {
        let off = i32::try_from(i).unwrap() * i32::try_from(ptr_size).unwrap();
        let key_const = builder.ins().iconst(state.pointer_type, *key_ptr as i64);
        builder.ins().stack_store(key_const, keys_array, off);
        let val_addr = builder.ins().stack_addr(state.pointer_type, *value_slot, 0);
        builder.ins().stack_store(val_addr, values_array, off);
    }
    let keys_addr = builder.ins().stack_addr(state.pointer_type, keys_array, 0);
    let values_addr = builder
        .ins()
        .stack_addr(state.pointer_type, values_array, 0);
    let out_slot = alloc_slot(state, builder);
    let out_addr = builder.ins().stack_addr(state.pointer_type, out_slot, 0);
    let n_const = builder.ins().iconst(types::I32, i64::from(n as u32));
    let helper = get_map(&mut state.helpers, state.module, &mut builder.func);
    builder
        .ins()
        .call(helper, &[n_const, keys_addr, values_addr, out_addr]);
    Ok(out_slot)
}

fn emit_func_call(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    node: &DynNode<'_, ExprIR<Variable>>,
    func: Arc<GraphFn>,
) -> Result<ir::StackSlot, ()> {
    // Aggregator FuncInvocation requires special finalize-vs-accumulate
    // semantics that can't be expressed by eagerly emitting all child
    // evaluations: in finalize phase we must read only the accumulator
    // (last-child Variable) and skip evaluating the data args, because
    // those args reference variables (e.g. `x` in `sum(x[1])`) that get
    // rebound after grouping. Route through a bridge that decides at
    // runtime based on AGG_KEY.
    if matches!(func.fn_type, FnType::Aggregation { .. }) {
        return emit_agg_func_call(state, builder, tree, node.idx(), func);
    }
    let argc = node.num_children();
    let has_distinct_first = argc >= 1 && matches!(node.child(0).data(), ExprIR::Distinct);

    // Evaluate args first.
    let mut arg_slots = Vec::with_capacity(argc);
    for i in 0..argc {
        arg_slots.push(emit(state, builder, tree, node.child(i).idx())?);
    }

    let ptr_size = u32::from(state.pointer_type.bytes());
    let args_array = builder.create_sized_stack_slot(ir::StackSlotData::new(
        ir::StackSlotKind::ExplicitSlot,
        ptr_size * argc.max(1) as u32,
        3,
    ));
    for (i, slot) in arg_slots.iter().enumerate() {
        let slot_addr = builder.ins().stack_addr(state.pointer_type, *slot, 0);
        builder.ins().stack_store(
            slot_addr,
            args_array,
            i32::try_from(i).unwrap() * i32::try_from(ptr_size).unwrap(),
        );
    }
    let args_array_addr = builder.ins().stack_addr(state.pointer_type, args_array, 0);

    let out_slot = alloc_slot(state, builder);
    let out_addr = builder.ins().stack_addr(state.pointer_type, out_slot, 0);

    let boxed = Box::new(func);
    let func_ptr = boxed.as_ref() as *const Arc<GraphFn> as usize;
    state.fn_arcs.push(boxed);
    let func_const = builder.ins().iconst(state.pointer_type, func_ptr as i64);
    let argc_const = builder.ins().iconst(types::I32, i64::from(argc as u32));

    let helper = if has_distinct_first {
        get_func_call_distinct(&mut state.helpers, state.module, &mut builder.func)
    } else {
        get_func_call(&mut state.helpers, state.module, &mut builder.func)
    };
    builder.ins().call(
        helper,
        &[
            state.eval_param,
            func_const,
            argc_const,
            args_array_addr,
            out_addr,
        ],
    );
    Ok(out_slot)
}

fn emit_agg_func_call(
    state: &mut CodegenState<'_>,
    builder: &mut FunctionBuilder<'_>,
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    func: Arc<GraphFn>,
) -> Result<ir::StackSlot, ()> {
    let slot = alloc_slot(state, builder);
    let helper = get_agg_func_call(&mut state.helpers, state.module, &mut builder.func);
    let out_addr = builder.ins().stack_addr(state.pointer_type, slot, 0);

    // Path relative to the compile-time subtree root.
    let mut path: Vec<usize> = Vec::new();
    let root_node_ptr = tree.node(state.root_idx.clone()).idx();
    let mut node = tree.node(idx.clone());
    while node.idx() != root_node_ptr {
        path.push(node.sibling_idx());
        node = match node.parent() {
            Some(p) => p,
            None => break,
        };
    }
    path.reverse();
    let boxed_path: Box<Vec<usize>> = Box::new(path);
    let path_ptr = boxed_path.as_ptr() as usize;
    let path_len = boxed_path.len();
    state.paths.push(boxed_path);

    let boxed_fn = Box::new(func);
    let func_ptr = boxed_fn.as_ref() as *const Arc<GraphFn> as usize;
    state.fn_arcs.push(boxed_fn);

    // Pre-compile every child arg into a helper. Null entries are tolerated
    // by the bridge in the finalize path (where children aren't evaluated).
    let n = tree.node(idx.clone()).num_children();
    let mut ptrs: Vec<*const CompiledExpr> = Vec::with_capacity(n);
    for i in 0..n {
        let child_idx = tree.node(idx.clone()).child(i).idx();
        match try_compile(tree, child_idx) {
            Some(ce) => {
                let arc = Arc::new(ce);
                let raw = Arc::as_ptr(&arc);
                state.helper_arcs.push(arc);
                ptrs.push(raw);
            }
            None => ptrs.push(std::ptr::null()),
        }
    }
    let helpers_boxed: Box<Vec<*const CompiledExpr>> = Box::new(ptrs);
    let helpers_arr_ptr = helpers_boxed.as_ptr() as usize;
    state.helper_ptr_arrays.push(helpers_boxed);

    let path_const = builder.ins().iconst(state.pointer_type, path_ptr as i64);
    let path_len_const = builder.ins().iconst(state.pointer_type, path_len as i64);
    let func_const = builder.ins().iconst(state.pointer_type, func_ptr as i64);
    let null_args = builder.ins().iconst(state.pointer_type, 0);
    let zero_n = builder.ins().iconst(state.pointer_type, 0);
    let helpers_const = builder
        .ins()
        .iconst(state.pointer_type, helpers_arr_ptr as i64);
    let n_helpers_const = builder.ins().iconst(state.pointer_type, n as i64);

    builder.ins().call(
        helper,
        &[
            state.eval_param,
            state.env_param,
            state.tree_param,
            state.idx_param,
            path_const,
            path_len_const,
            null_args,
            zero_n,
            helpers_const,
            n_helpers_const,
            func_const,
            out_addr,
        ],
    );
    Ok(slot)
}
