//! Vectorized operations on typed columns.
//!
//! This module provides bulk comparison kernels that operate on entire columns
//! of homogeneous values at once, enabling LLVM auto-vectorization for filter
//! predicates like `n.age > 30`.
//!
//! ```text
//!  Scalar (per-row) filter          Vectorized filter
//!  =========================        ==================
//!
//!  for each row:                    1. Materialize property column
//!    eval(n.age > 30)                  ages = [25, 42, 18, 55, ...]
//!    if true -> keep row            2. compare_i64_column(ages, Gt, 30)
//!                                      mask = [F, T, F, T, ...]
//!  O(rows * expr_depth)            3. mask_to_selection(mask)
//!                                      sel  = [1, 3, ...]
//!                                   O(rows) with SIMD lanes
//! ```
//!
//! ## Components
//!
//! - [`CmpOp`] -- comparison operator enum (Eq, Neq, Lt, Le, Gt, Ge)
//! - Comparison kernels: [`compare_i64_column`], [`compare_f64_column`],
//!   [`compare_string_column`] -- tight indexed loops for auto-vectorization
//! - [`SimplePredicate`] / [`VectorizablePredicate`] -- detected filter patterns
//!   that can use the bulk path instead of per-row expression evaluation
//! - [`try_extract_vectorizable_predicate`] -- analyzes a filter expression tree
//!   to detect `entity.property <cmp> constant`, `entity.property IS [NOT] NULL`,
//!   and `entity.property CONTAINS/STARTS WITH/ENDS WITH 'pattern'` patterns
//! - [`mask_to_selection`] / [`mask_intersect_selection`] -- convert boolean
//!   masks to/from the selection vector used by [`Batch`](super::batch::Batch)
//!
//! The comparison kernels are written as tight indexed loops to enable
//! LLVM auto-vectorization on all target platforms (x86_64 SSE/AVX, ARM NEON).

use std::collections::HashMap;
use std::sync::Arc;

use crate::parser::ast::{ExprIR, Variable};
use crate::runtime::batch::NullBitmap;
use crate::runtime::value::Value;

use orx_tree::{Dyn, DynTree, NodeIdx, NodeRef};

// ---------------------------------------------------------------------------
// CmpOp — comparison operator enum
// ---------------------------------------------------------------------------

/// Comparison operator for vectorized kernels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    /// Converts from an `ExprIR` comparison node to a `CmpOp`.
    pub const fn from_expr_ir<T>(ir: &ExprIR<T>) -> Option<Self> {
        match ir {
            ExprIR::Eq => Some(Self::Eq),
            ExprIR::Neq => Some(Self::Neq),
            ExprIR::Lt => Some(Self::Lt),
            ExprIR::Le => Some(Self::Le),
            ExprIR::Gt => Some(Self::Gt),
            ExprIR::Ge => Some(Self::Ge),
            _ => None,
        }
    }

    /// Returns the flipped operator (for when operands are swapped).
    #[must_use]
    pub const fn flip(self) -> Self {
        match self {
            Self::Eq => Self::Eq,
            Self::Neq => Self::Neq,
            Self::Lt => Self::Gt,
            Self::Le => Self::Ge,
            Self::Gt => Self::Lt,
            Self::Ge => Self::Le,
        }
    }
}

// ---------------------------------------------------------------------------
// Comparison kernels — tight loops for auto-vectorization
// ---------------------------------------------------------------------------

/// Compares each element of `data` against `threshold` using `op`.
/// Null rows (per `nulls` bitmap) always produce `false`.
#[allow(clippy::needless_range_loop)]
#[inline]
#[must_use]
pub fn compare_i64_column(
    data: &[i64],
    op: CmpOp,
    threshold: i64,
    nulls: &NullBitmap,
) -> Vec<bool> {
    let len = data.len();
    let mut result = vec![false; len];
    match op {
        CmpOp::Eq => {
            for i in 0..len {
                result[i] = data[i] == threshold;
            }
        }
        CmpOp::Neq => {
            for i in 0..len {
                result[i] = data[i] != threshold;
            }
        }
        CmpOp::Lt => {
            for i in 0..len {
                result[i] = data[i] < threshold;
            }
        }
        CmpOp::Le => {
            for i in 0..len {
                result[i] = data[i] <= threshold;
            }
        }
        CmpOp::Gt => {
            for i in 0..len {
                result[i] = data[i] > threshold;
            }
        }
        CmpOp::Ge => {
            for i in 0..len {
                result[i] = data[i] >= threshold;
            }
        }
    }
    // Mask out nulls in a separate pass to avoid polluting the inner loop
    if nulls.any_null() {
        for i in 0..len {
            if nulls.is_null(i) {
                result[i] = false;
            }
        }
    }
    result
}

/// Compares each element of `data` against `threshold` using `op`.
/// NaN comparisons naturally return false, matching Cypher semantics.
/// Null rows (per `nulls` bitmap) always produce `false`.
#[allow(clippy::needless_range_loop)]
#[inline]
#[must_use]
pub fn compare_f64_column(
    data: &[f64],
    op: CmpOp,
    threshold: f64,
    nulls: &NullBitmap,
) -> Vec<bool> {
    let len = data.len();
    let mut result = vec![false; len];
    match op {
        CmpOp::Eq => {
            for i in 0..len {
                result[i] = data[i] == threshold;
            }
        }
        CmpOp::Neq => {
            for i in 0..len {
                result[i] = data[i] != threshold;
            }
        }
        CmpOp::Lt => {
            for i in 0..len {
                result[i] = data[i] < threshold;
            }
        }
        CmpOp::Le => {
            for i in 0..len {
                result[i] = data[i] <= threshold;
            }
        }
        CmpOp::Gt => {
            for i in 0..len {
                result[i] = data[i] > threshold;
            }
        }
        CmpOp::Ge => {
            for i in 0..len {
                result[i] = data[i] >= threshold;
            }
        }
    }
    if nulls.any_null() {
        for i in 0..len {
            if nulls.is_null(i) {
                result[i] = false;
            }
        }
    }
    result
}

/// Compares string values in a `Value` slice against `threshold`.
/// Non-string and Null values produce `false`.
#[must_use]
pub fn compare_string_column(
    data: &[Value],
    op: CmpOp,
    threshold: &str,
) -> Vec<bool> {
    let len = data.len();
    let mut result = vec![false; len];
    for i in 0..len {
        if let Value::String(s) = &data[i] {
            result[i] = match op {
                CmpOp::Eq => s.as_str() == threshold,
                CmpOp::Neq => s.as_str() != threshold,
                CmpOp::Lt => s.as_str() < threshold,
                CmpOp::Le => s.as_str() <= threshold,
                CmpOp::Gt => s.as_str() > threshold,
                CmpOp::Ge => s.as_str() >= threshold,
            };
        }
    }
    result
}

/// Runs a substring/prefix/suffix match on string values in a `Value` slice.
/// Non-string and Null values produce `false` (Cypher: `NULL CONTAINS x` is
/// NULL, which the filter drops).
#[must_use]
pub fn match_string_column(
    data: &[Value],
    op: StringMatchOp,
    pattern: &str,
) -> Vec<bool> {
    let len = data.len();
    let mut result = vec![false; len];
    for i in 0..len {
        if let Value::String(s) = &data[i] {
            result[i] = match op {
                StringMatchOp::Contains => s.contains(pattern),
                StringMatchOp::StartsWith => s.starts_with(pattern),
                StringMatchOp::EndsWith => s.ends_with(pattern),
            };
        }
    }
    result
}

/// Builds a boolean mask from a property column's null bitmap.
/// With `negated = false` (IS NULL) a row passes when the property is
/// missing/null; with `negated = true` (IS NOT NULL) it passes when present.
#[must_use]
pub fn null_check_mask(
    nulls: &NullBitmap,
    len: usize,
    negated: bool,
) -> Vec<bool> {
    (0..len).map(|i| nulls.is_null(i) != negated).collect()
}

/// Converts a boolean mask to a selection vector of passing row indices.
#[must_use]
pub fn mask_to_selection(mask: &[bool]) -> Vec<u16> {
    mask.iter()
        .enumerate()
        .filter_map(|(i, &pass)| if pass { Some(i as u16) } else { None })
        .collect()
}

/// Intersects a boolean mask with an existing selection vector.
/// Only rows present in both the mask AND the existing selection pass.
#[must_use]
pub fn mask_intersect_selection(
    mask: &[bool],
    existing: &[u16],
) -> Vec<u16> {
    existing
        .iter()
        .copied()
        .filter(|&i| mask[i as usize])
        .collect()
}

// ---------------------------------------------------------------------------
// Simple predicate detection
// ---------------------------------------------------------------------------

/// String matching operator for vectorized kernels
/// (`CONTAINS` / `STARTS WITH` / `ENDS WITH`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringMatchOp {
    Contains,
    StartsWith,
    EndsWith,
}

impl StringMatchOp {
    /// Maps an internal function name (as rewritten by the parser) to a
    /// string matching operator.
    fn from_fn_name(name: &str) -> Option<Self> {
        match name {
            "contains" => Some(Self::Contains),
            "starts_with" => Some(Self::StartsWith),
            "ends_with" => Some(Self::EndsWith),
            _ => None,
        }
    }
}

/// The test a [`SimplePredicate`] applies to the property column.
#[derive(Debug)]
pub enum PredicateTest {
    /// `entity.property <cmp> constant`
    Cmp {
        /// The comparison operator.
        op: CmpOp,
        /// The constant value on the other side.
        constant: Value,
    },
    /// `entity.property IS [NOT] NULL` — `negated` is true for `IS NOT NULL`.
    IsNull { negated: bool },
    /// `entity.property CONTAINS / STARTS WITH / ENDS WITH 'pattern'`
    StringMatch {
        op: StringMatchOp,
        pattern: Arc<String>,
    },
}

/// A simple predicate that can be evaluated in vectorized mode.
/// Represents a test on a single `entity_variable.property`.
#[derive(Debug)]
pub struct SimplePredicate {
    /// The variable whose property is being tested (e.g., `n` in `n.age > 30`).
    pub var: Variable,
    /// The property name (e.g., "age").
    pub attr: Arc<String>,
    /// The test applied to the property column.
    pub test: PredicateTest,
}

/// A vectorizable predicate — either a single comparison or a conjunction.
#[derive(Debug)]
pub enum VectorizablePredicate {
    Single(SimplePredicate),
    Conjunction(Vec<SimplePredicate>),
}

/// Tries to extract a vectorizable predicate from a filter expression tree.
///
/// Detects patterns like:
/// - `n.age > 30` → `Single(SimplePredicate { var: n, attr: "age", op: Gt, constant: Int(30) })`
/// - `n.age > 30 AND n.name = 'Alice'` → `Conjunction([...])`
///
/// Returns `None` for complex predicates that cannot be vectorized.
#[allow(clippy::implicit_hasher)]
pub fn try_extract_vectorizable_predicate(
    tree: &DynTree<ExprIR<Variable>>,
    params: &HashMap<String, Value>,
) -> Option<VectorizablePredicate> {
    let root = tree.root();
    let root_data = root.data();

    // Check for AND (conjunction of simple predicates)
    if matches!(root_data, ExprIR::And) {
        let mut preds = Vec::new();
        for child in root.children() {
            let child_tree = child.clone_as_tree();
            preds.push(try_extract_single_predicate(&child_tree, params)?);
        }
        if preds.is_empty() {
            return None;
        }
        return Some(VectorizablePredicate::Conjunction(preds));
    }

    // Single predicate
    try_extract_single_predicate(tree, params).map(VectorizablePredicate::Single)
}

/// Tries to extract a single `SimplePredicate` from a comparison,
/// `IS [NOT] NULL`, or string-match (`CONTAINS`/`STARTS WITH`/`ENDS WITH`)
/// expression.
fn try_extract_single_predicate(
    tree: &DynTree<ExprIR<Variable>>,
    params: &HashMap<String, Value>,
) -> Option<SimplePredicate> {
    let root = tree.root();

    if let Some(op) = CmpOp::from_expr_ir(root.data()) {
        if root.num_children() != 2 {
            return None;
        }

        let lhs_idx = root.child(0).idx();
        let rhs_idx = root.child(1).idx();

        // Try: Property(attr) -> Variable(var)  <op>  Constant
        if let Some(pred) = try_property_vs_constant(tree, lhs_idx, rhs_idx, op, params) {
            return Some(pred);
        }
        // Try: Constant  <op>  Property(attr) -> Variable(var) (flip operator)
        return try_property_vs_constant(tree, rhs_idx, lhs_idx, op.flip(), params);
    }

    // The parser rewrites `x IS [NOT] NULL` and string predicates into
    // internal function invocations (see parser/cypher.rs).
    if let ExprIR::FuncInvocation(f) = root.data() {
        if root.num_children() != 2 {
            return None;
        }

        // is_null(Constant(Bool(is_not)), Property(attr) -> Variable(var))
        if f.name == "is_null" {
            let ExprIR::Constant(Value::Bool(is_not)) = root.child(0).data() else {
                return None;
            };
            let (var, attr) = extract_property_access(tree, root.child(1).idx())?;
            return Some(SimplePredicate {
                var,
                attr,
                test: PredicateTest::IsNull { negated: *is_not },
            });
        }

        // contains/starts_with/ends_with(Property(attr) -> Variable(var), 'pattern')
        if let Some(op) = StringMatchOp::from_fn_name(&f.name) {
            let (var, attr) = extract_property_access(tree, root.child(0).idx())?;
            let Value::String(pattern) = extract_constant(tree, root.child(1).idx(), params)?
            else {
                return None;
            };
            return Some(SimplePredicate {
                var,
                attr,
                test: PredicateTest::StringMatch { op, pattern },
            });
        }
    }

    None
}

/// Checks that the node at `idx` is `Property(attr) -> Variable(var)` and
/// returns the variable and attribute name.
fn extract_property_access(
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
) -> Option<(Variable, Arc<String>)> {
    let prop_node = tree.node(idx);
    let ExprIR::Property(attr) = prop_node.data() else {
        return None;
    };
    if prop_node.num_children() != 1 {
        return None;
    }
    let ExprIR::Variable(var) = prop_node.child(0).data() else {
        return None;
    };
    Some((var.clone(), attr.clone()))
}

/// Resolves the node at `idx` to a constant value when it is a leaf literal
/// or a query parameter that resolves to a literal value (parameters are not
/// substituted into the cached plan, so `MATCH (n {id: $id})` keeps `$id` as
/// an `ExprIR::Parameter` node).
fn extract_constant(
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    params: &HashMap<String, Value>,
) -> Option<Value> {
    let const_node = tree.node(idx);
    if const_node.num_children() != 0 {
        return None;
    }
    match const_node.data() {
        ExprIR::Constant(v) => Some(v.clone()),
        ExprIR::Parameter(name) => params.get(name).cloned(),
        _ => None,
    }
}

/// Checks if `prop_side` is `Property(attr) -> Variable(var)` and
/// `const_side` is a literal constant or a resolvable query parameter.
fn try_property_vs_constant(
    tree: &DynTree<ExprIR<Variable>>,
    prop_idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    const_idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    op: CmpOp,
    params: &HashMap<String, Value>,
) -> Option<SimplePredicate> {
    let (var, attr) = extract_property_access(tree, prop_idx)?;
    let constant = extract_constant(tree, const_idx, params)?;

    Some(SimplePredicate {
        var,
        attr,
        test: PredicateTest::Cmp { op, constant },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_op_flip() {
        assert_eq!(CmpOp::Eq.flip(), CmpOp::Eq);
        assert_eq!(CmpOp::Neq.flip(), CmpOp::Neq);
        assert_eq!(CmpOp::Lt.flip(), CmpOp::Gt);
        assert_eq!(CmpOp::Le.flip(), CmpOp::Ge);
        assert_eq!(CmpOp::Gt.flip(), CmpOp::Lt);
        assert_eq!(CmpOp::Ge.flip(), CmpOp::Le);
    }

    #[test]
    fn test_compare_i64_basic() {
        let data = vec![10, 20, 30, 40, 50];
        let nulls = NullBitmap::none(5);
        assert_eq!(
            compare_i64_column(&data, CmpOp::Gt, 25, &nulls),
            vec![false, false, true, true, true]
        );
        assert_eq!(
            compare_i64_column(&data, CmpOp::Eq, 30, &nulls),
            vec![false, false, true, false, false]
        );
        assert_eq!(
            compare_i64_column(&data, CmpOp::Le, 30, &nulls),
            vec![true, true, true, false, false]
        );
    }

    #[test]
    fn test_compare_i64_with_nulls() {
        let data = vec![10, 0, 30, 0, 50]; // indices 1 and 3 are null
        let nulls = NullBitmap::from_values(&[
            Value::Int(10),
            Value::Null,
            Value::Int(30),
            Value::Null,
            Value::Int(50),
        ]);
        let result = compare_i64_column(&data, CmpOp::Gt, 5, &nulls);
        assert_eq!(result, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_compare_f64_basic() {
        let data = vec![1.5, 2.5, 3.5];
        let nulls = NullBitmap::none(3);
        assert_eq!(
            compare_f64_column(&data, CmpOp::Lt, 3.0, &nulls),
            vec![true, true, false]
        );
    }

    #[test]
    fn test_compare_f64_nan() {
        let data = vec![1.0, f64::NAN, 3.0];
        let nulls = NullBitmap::none(3);
        // NaN comparisons return false for all operators
        let result = compare_f64_column(&data, CmpOp::Gt, 0.0, &nulls);
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn test_mask_to_selection() {
        let mask = vec![true, false, true, false, true];
        assert_eq!(mask_to_selection(&mask), vec![0, 2, 4]);
    }

    #[test]
    fn test_mask_intersect_selection() {
        let mask = vec![true, false, true, true, false];
        let existing = vec![0, 2, 3, 4];
        assert_eq!(mask_intersect_selection(&mask, &existing), vec![0, 2, 3]);
    }

    #[test]
    fn test_compare_string_column() {
        let data = vec![
            Value::String(Arc::new("Alice".to_string())),
            Value::String(Arc::new("Bob".to_string())),
            Value::Null,
            Value::String(Arc::new("Alice".to_string())),
        ];
        let result = compare_string_column(&data, CmpOp::Eq, "Alice");
        assert_eq!(result, vec![true, false, false, true]);
    }

    #[test]
    fn test_compare_empty() {
        let data: Vec<i64> = vec![];
        let nulls = NullBitmap::none(0);
        assert_eq!(
            compare_i64_column(&data, CmpOp::Eq, 0, &nulls),
            Vec::<bool>::new()
        );
    }

    #[test]
    fn test_match_string_column() {
        let data = vec![
            Value::String(Arc::new("fixture_alice_1".to_string())),
            Value::String(Arc::new("fixture_bob_2".to_string())),
            Value::Null,
            Value::Int(42),
            Value::String(Arc::new("x_fixture_alice".to_string())),
        ];
        assert_eq!(
            match_string_column(&data, StringMatchOp::Contains, "fixture_alice"),
            vec![true, false, false, false, true]
        );
        assert_eq!(
            match_string_column(&data, StringMatchOp::StartsWith, "fixture_"),
            vec![true, true, false, false, false]
        );
        assert_eq!(
            match_string_column(&data, StringMatchOp::EndsWith, "alice"),
            vec![false, false, false, false, true]
        );
    }

    #[test]
    fn test_null_check_mask() {
        let nulls = NullBitmap::from_values(&[
            Value::Int(1),
            Value::Null,
            Value::String(Arc::new("x".to_string())),
            Value::Null,
        ]);
        // IS NULL
        assert_eq!(
            null_check_mask(&nulls, 4, false),
            vec![false, true, false, true]
        );
        // IS NOT NULL
        assert_eq!(
            null_check_mask(&nulls, 4, true),
            vec![true, false, true, false]
        );
    }

    mod extraction {
        use super::*;
        use crate::runtime::functions::{FnType, Type, get_functions, init_functions};
        use crate::tree;

        fn functions() -> &'static crate::runtime::functions::Functions {
            let _ = init_functions();
            get_functions()
        }

        fn var() -> Variable {
            Variable {
                name: Some(Arc::new("u".to_string())),
                id: 7,
                scope_id: 0,
                ty: Type::Any,
            }
        }

        fn prop_access(attr: &str) -> DynTree<ExprIR<Variable>> {
            tree!(
                ExprIR::Property(Arc::new(attr.to_string())),
                tree!(ExprIR::Variable(var()))
            )
        }

        #[test]
        fn test_extract_is_not_null() {
            // u.embedding IS NOT NULL
            let f = functions().get("is_null", &FnType::Internal).unwrap();
            let expr = tree!(
                ExprIR::FuncInvocation(f),
                tree!(ExprIR::Constant(Value::Bool(true))),
                prop_access("embedding")
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("IS NOT NULL should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert_eq!(p.var.id, 7);
            assert_eq!(p.attr.as_str(), "embedding");
            assert!(matches!(p.test, PredicateTest::IsNull { negated: true }));
        }

        #[test]
        fn test_extract_is_null() {
            // u.embedding IS NULL
            let f = functions().get("is_null", &FnType::Internal).unwrap();
            let expr = tree!(
                ExprIR::FuncInvocation(f),
                tree!(ExprIR::Constant(Value::Bool(false))),
                prop_access("embedding")
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("IS NULL should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert!(matches!(p.test, PredicateTest::IsNull { negated: false }));
        }

        #[test]
        fn test_extract_string_match() {
            for (name, op) in [
                ("contains", StringMatchOp::Contains),
                ("starts_with", StringMatchOp::StartsWith),
                ("ends_with", StringMatchOp::EndsWith),
            ] {
                // u.ft_text CONTAINS/STARTS WITH/ENDS WITH 'fixture_alice'
                let f = functions().get(name, &FnType::Internal).unwrap();
                let expr = tree!(
                    ExprIR::FuncInvocation(f),
                    prop_access("ft_text"),
                    tree!(ExprIR::Constant(Value::String(Arc::new(
                        "fixture_alice".to_string()
                    ))))
                );
                let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                    .unwrap_or_else(|| panic!("{name} should be vectorizable"));
                let VectorizablePredicate::Single(p) = pred else {
                    panic!("expected single predicate");
                };
                assert_eq!(p.attr.as_str(), "ft_text");
                match p.test {
                    PredicateTest::StringMatch {
                        op: extracted_op,
                        pattern,
                    } => {
                        assert_eq!(extracted_op, op);
                        assert_eq!(pattern.as_str(), "fixture_alice");
                    }
                    other => panic!("expected StringMatch, got {other:?}"),
                }
            }
        }

        #[test]
        fn test_extract_string_match_parameter() {
            // u.ft_text CONTAINS $pat
            let f = functions().get("contains", &FnType::Internal).unwrap();
            let expr = tree!(
                ExprIR::FuncInvocation(f),
                prop_access("ft_text"),
                tree!(ExprIR::Parameter("pat".to_string()))
            );
            let mut params = HashMap::new();
            params.insert(
                "pat".to_string(),
                Value::String(Arc::new("alice".to_string())),
            );
            let pred = try_extract_vectorizable_predicate(&expr, &params)
                .expect("CONTAINS $param should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert!(matches!(
                p.test,
                PredicateTest::StringMatch {
                    op: StringMatchOp::Contains,
                    ..
                }
            ));
        }

        #[test]
        fn test_extract_string_match_non_string_pattern_falls_back() {
            // u.ft_text CONTAINS 42 — not vectorizable, falls back to per-row
            let f = functions().get("contains", &FnType::Internal).unwrap();
            let expr = tree!(
                ExprIR::FuncInvocation(f),
                prop_access("ft_text"),
                tree!(ExprIR::Constant(Value::Int(42)))
            );
            assert!(try_extract_vectorizable_predicate(&expr, &HashMap::new()).is_none());
        }

        #[test]
        fn test_extract_other_function_falls_back() {
            // regex match is not vectorized
            let f = get_functions()
                .get("regex_matches", &FnType::Internal)
                .unwrap();
            let expr = tree!(
                ExprIR::FuncInvocation(f),
                prop_access("ft_text"),
                tree!(ExprIR::Constant(Value::String(Arc::new("a.*".to_string()))))
            );
            assert!(try_extract_vectorizable_predicate(&expr, &HashMap::new()).is_none());
        }

        #[test]
        fn test_extract_conjunction_with_new_predicates() {
            // u.embedding IS NOT NULL AND u.ft_text CONTAINS 'alice'
            let is_null = functions().get("is_null", &FnType::Internal).unwrap();
            let contains = functions().get("contains", &FnType::Internal).unwrap();
            let expr = tree!(
                ExprIR::And,
                tree!(
                    ExprIR::FuncInvocation(is_null),
                    tree!(ExprIR::Constant(Value::Bool(true))),
                    prop_access("embedding")
                ),
                tree!(
                    ExprIR::FuncInvocation(contains),
                    prop_access("ft_text"),
                    tree!(ExprIR::Constant(Value::String(Arc::new(
                        "alice".to_string()
                    ))))
                )
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("conjunction should be vectorizable");
            let VectorizablePredicate::Conjunction(preds) = pred else {
                panic!("expected conjunction");
            };
            assert_eq!(preds.len(), 2);
            assert!(matches!(
                preds[0].test,
                PredicateTest::IsNull { negated: true }
            ));
            assert!(matches!(preds[1].test, PredicateTest::StringMatch { .. }));
        }

        /// Parses and binds a full query, returning the MATCH clause's
        /// bound WHERE expression.
        fn bind_where_expr(query: &str) -> crate::parser::ast::QueryExpr<Variable> {
            use crate::parser::ast::QueryIR;
            use crate::parser::cypher::Parser;
            use crate::planner::binder::Binder;

            let _ = init_functions();
            let ir = Parser::new(query).parse().expect("query should parse");
            let (bound, _) = Binder::default().bind(ir).expect("query should bind");
            let QueryIR::Query { clauses, .. } = bound else {
                panic!("expected Query root");
            };
            clauses
                .iter()
                .find_map(|c| match c {
                    QueryIR::Match {
                        filter: Some(f), ..
                    } => Some(f.clone()),
                    _ => None,
                })
                .expect("query should have a WHERE filter")
        }

        #[test]
        fn test_extract_from_parsed_is_not_null_query() {
            let expr =
                bind_where_expr("MATCH (u:User) WHERE u.embedding IS NOT NULL RETURN count(u)");
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed IS NOT NULL filter should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert_eq!(p.attr.as_str(), "embedding");
            assert!(matches!(p.test, PredicateTest::IsNull { negated: true }));
        }

        #[test]
        fn test_extract_from_parsed_contains_query() {
            let expr = bind_where_expr(
                "MATCH (u:User) WHERE u.ft_text CONTAINS 'fixture_alice' RETURN count(u)",
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed CONTAINS filter should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert_eq!(p.attr.as_str(), "ft_text");
            match &p.test {
                PredicateTest::StringMatch {
                    op: StringMatchOp::Contains,
                    pattern,
                } => assert_eq!(pattern.as_str(), "fixture_alice"),
                other => panic!("expected Contains, got {other:?}"),
            }
        }

        #[test]
        fn test_extract_from_parsed_starts_and_ends_with_queries() {
            let expr =
                bind_where_expr("MATCH (u:User) WHERE u.name STARTS WITH 'fix' RETURN count(u)");
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed STARTS WITH filter should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert!(matches!(
                p.test,
                PredicateTest::StringMatch {
                    op: StringMatchOp::StartsWith,
                    ..
                }
            ));

            let expr =
                bind_where_expr("MATCH (u:User) WHERE u.name ENDS WITH 'ice' RETURN count(u)");
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed ENDS WITH filter should be vectorizable");
            let VectorizablePredicate::Single(p) = pred else {
                panic!("expected single predicate");
            };
            assert!(matches!(
                p.test,
                PredicateTest::StringMatch {
                    op: StringMatchOp::EndsWith,
                    ..
                }
            ));
        }
    }
}
