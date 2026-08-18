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
//! - [`Tri`] / [`TriMask`] -- three-valued (Kleene) row mask; each row is
//!   `True`, `False`, or `Null`, so `NOT`/`AND`/`OR` combine with correct
//!   Cypher NULL semantics. `Tri` is ordered `False < Null < True` so that
//!   Kleene `AND`/`OR`/`NOT` reduce to `min`/`max`/`2 - x` over the
//!   discriminant byte -- one byte per row instead of two, and the same
//!   branch-free shape that lets the comparison kernels auto-vectorize
//! - [`SimplePredicate`] / [`VectorizablePredicate`] -- detected filter patterns
//!   that can use the bulk path instead of per-row expression evaluation
//! - [`try_extract_vectorizable_predicate`] -- recursively analyzes a filter
//!   expression tree: any `AND`/`OR`/`NOT` combination of
//!   `entity.property <cmp> constant`, `entity.property IS [NOT] NULL`,
//!   and `entity.property CONTAINS/STARTS WITH/ENDS WITH 'pattern'` leaves
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

/// Compares string values in a `Value` slice against `threshold`,
/// producing a three-valued mask that matches the scalar comparison
/// semantics ([`compare_value`](crate::runtime::value::CompareValue)):
///
/// - `String` rows compare normally (byte-wise `str` ordering).
/// - `Null` rows are NULL for every operator.
/// - Rows of any other type are disjoint from a string: `=` is false,
///   `<>` is true, and ordering comparisons (`<`, `<=`, `>`, `>=`) are NULL.
#[must_use]
pub fn compare_string_column(
    data: &[Value],
    op: CmpOp,
    threshold: &str,
) -> TriMask {
    TriMask::from_tri(
        data.iter()
            .map(|v| match v {
                Value::String(s) => Tri::from_bool(match op {
                    CmpOp::Eq => s.as_str() == threshold,
                    CmpOp::Neq => s.as_str() != threshold,
                    CmpOp::Lt => s.as_str() < threshold,
                    CmpOp::Le => s.as_str() <= threshold,
                    CmpOp::Gt => s.as_str() > threshold,
                    CmpOp::Ge => s.as_str() >= threshold,
                }),
                Value::Null => Tri::Null,
                // Disjoint types: `=` is false, `<>` is true, ordering is NULL.
                _ => match op {
                    CmpOp::Eq => Tri::False,
                    CmpOp::Neq => Tri::True,
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => Tri::Null,
                },
            })
            .collect(),
    )
}

/// Runs a substring/prefix/suffix match on string values in a `Value` slice.
/// Non-string and Null rows produce NULL, mirroring the scalar
/// `internal_contains`/`internal_starts_with`/`internal_ends_with` functions.
///
/// Matching is intentionally byte-wise (`str::contains`/`starts_with`/
/// `ends_with` — UTF-8 byte sequences, not Unicode graphemes), exactly like
/// the scalar internal functions this kernel mirrors.
#[must_use]
pub fn match_string_column(
    data: &[Value],
    op: StringMatchOp,
    pattern: &str,
) -> TriMask {
    TriMask::from_tri(
        data.iter()
            .map(|v| match v {
                Value::String(s) => Tri::from_bool(match op {
                    StringMatchOp::Contains => s.contains(pattern),
                    StringMatchOp::StartsWith => s.starts_with(pattern),
                    StringMatchOp::EndsWith => s.ends_with(pattern),
                }),
                _ => Tri::Null,
            })
            .collect(),
    )
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
// TriMask — three-valued (Kleene) row mask
// ---------------------------------------------------------------------------

/// A single row's three-valued (Kleene logic) predicate result.
///
/// Deliberately ordered `False(0) < Null(1) < True(2)` (as the `u8` repr) so
/// that Kleene `AND`/`OR`/`NOT` reduce to `min`/`max`/`2 - x` on the
/// discriminant — plain branch-free arithmetic over a byte, rather than a
/// per-row match. One byte per row (same as `bool`), instead of the two
/// separate `truthy`/`nulls` `bool` columns an earlier version of this type
/// used.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tri {
    False = 0,
    Null = 1,
    True = 2,
}

impl Tri {
    #[must_use]
    const fn from_bool(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }

    #[must_use]
    const fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::False,
            1 => Self::Null,
            _ => Self::True,
        }
    }
}

/// A three-valued (Kleene logic) row mask: each row's predicate result is
/// `Tri::True`, `Tri::False`, or `Tri::Null`.
///
/// A plain boolean "row passed" mask is not enough once `NOT` and `OR` enter
/// the picture: in Cypher `NOT NULL` is NULL (the row is still dropped), so
/// complementing a boolean mask would incorrectly resurrect NULL rows.
/// Tracking each row's `Tri` state lets `AND`/`OR`/`NOT` combine with exactly
/// the same semantics as the per-row evaluator.
#[derive(Debug)]
pub struct TriMask(Vec<Tri>);

impl TriMask {
    /// Creates a mask from separate `true` and NULL row vectors.
    /// A row must not be flagged in both (NULL wins if it is).
    #[must_use]
    pub fn new(
        truthy: Vec<bool>,
        nulls: Vec<bool>,
    ) -> Self {
        debug_assert_eq!(truthy.len(), nulls.len());
        Self(
            truthy
                .into_iter()
                .zip(nulls)
                .map(|(t, n)| if n { Tri::Null } else { Tri::from_bool(t) })
                .collect(),
        )
    }

    /// Creates a mask directly from per-row three-valued results.
    #[must_use]
    pub fn from_tri(values: Vec<Tri>) -> Self {
        Self(values)
    }

    /// Creates a mask from a plain boolean result (no NULL rows).
    #[must_use]
    pub fn from_bools(truthy: Vec<bool>) -> Self {
        Self(truthy.into_iter().map(Tri::from_bool).collect())
    }

    /// Creates an all-NULL mask (e.g. a predicate that structurally cannot
    /// apply to this column's type, which Cypher treats as NULL per row).
    #[must_use]
    pub fn all_null(len: usize) -> Self {
        Self(vec![Tri::Null; len])
    }

    /// The rows where the predicate evaluated to exactly `true` — the rows a
    /// filter keeps (both `false` and NULL are dropped).
    #[must_use]
    pub fn truthy(&self) -> Vec<bool> {
        self.0.iter().map(|&t| t == Tri::True).collect()
    }

    /// Selection vector of rows where the predicate evaluated to exactly
    /// `true`, read directly off the packed `Tri` mask (no intermediate
    /// boolean vector).
    #[must_use]
    pub fn selection(&self) -> Vec<u16> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, &t)| (t == Tri::True).then_some(i as u16))
            .collect()
    }

    /// Logical `AND` (Kleene): `false AND x = false`, `true AND true = true`,
    /// everything else is NULL. Encoded as `min` over `False(0) < Null(1) <
    /// True(2)`.
    #[must_use]
    pub fn and(
        mut self,
        other: &Self,
    ) -> Self {
        debug_assert_eq!(self.0.len(), other.0.len());
        for (a, b) in self.0.iter_mut().zip(&other.0) {
            *a = Tri::from_u8((*a as u8).min(*b as u8));
        }
        self
    }

    /// Logical `OR` (Kleene): `true OR x = true`, `false OR false = false`,
    /// everything else is NULL. Encoded as `max`.
    #[must_use]
    pub fn or(
        mut self,
        other: &Self,
    ) -> Self {
        debug_assert_eq!(self.0.len(), other.0.len());
        for (a, b) in self.0.iter_mut().zip(&other.0) {
            *a = Tri::from_u8((*a as u8).max(*b as u8));
        }
        self
    }
}

/// Logical `NOT` (Kleene): `NOT true = false`, `NOT false = true`,
/// `NOT NULL = NULL`. Encoded as `2 - x`, which swaps `False`/`True` (0/2)
/// and fixes `Null` (1).
impl std::ops::Not for TriMask {
    type Output = Self;

    fn not(mut self) -> Self {
        for t in &mut self.0 {
            *t = Tri::from_u8(2 - (*t as u8));
        }
        self
    }
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

/// A vectorizable predicate — a boolean expression tree whose leaves are
/// [`SimplePredicate`]s, combined with `AND`/`OR`/`NOT`. Each node evaluates
/// to a [`TriMask`], so NULL propagates exactly like the per-row evaluator.
#[derive(Debug)]
pub enum VectorizablePredicate {
    Leaf(SimplePredicate),
    And(Vec<VectorizablePredicate>),
    Or(Vec<VectorizablePredicate>),
    Not(Box<VectorizablePredicate>),
}

/// Tries to extract a vectorizable predicate from a filter expression tree.
///
/// Recursively walks `AND`/`OR`/`NOT` nodes; leaves must be one of:
/// - `n.age > 30` (comparison against a literal or `$parameter`)
/// - `n.embedding IS [NOT] NULL`
/// - `n.name CONTAINS / STARTS WITH / ENDS WITH 'pattern'`
///
/// e.g. `NOT (n.age > 30 OR n.name CONTAINS 'x')` →
/// `Not(Or([Leaf(..), Leaf(..)]))`.
///
/// Returns `None` when any part of the expression cannot be vectorized
/// (the filter then falls back to per-row evaluation).
#[allow(clippy::implicit_hasher)]
pub fn try_extract_vectorizable_predicate(
    tree: &DynTree<ExprIR<Variable>>,
    params: &HashMap<String, Value>,
) -> Option<VectorizablePredicate> {
    extract_predicate_expr(tree, tree.root().idx(), params)
}

/// Recursive worker for [`try_extract_vectorizable_predicate`]: converts the
/// boolean expression rooted at `idx` into a [`VectorizablePredicate`] tree.
fn extract_predicate_expr(
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    params: &HashMap<String, Value>,
) -> Option<VectorizablePredicate> {
    let node = tree.node(idx);
    match node.data() {
        ExprIR::And | ExprIR::Or => {
            let mut children = Vec::with_capacity(node.num_children());
            for child in node.children() {
                children.push(extract_predicate_expr(tree, child.idx(), params)?);
            }
            if children.is_empty() {
                return None;
            }
            Some(if matches!(node.data(), ExprIR::And) {
                VectorizablePredicate::And(children)
            } else {
                VectorizablePredicate::Or(children)
            })
        }
        ExprIR::Not => {
            if node.num_children() != 1 {
                return None;
            }
            let inner = extract_predicate_expr(tree, node.child(0).idx(), params)?;
            Some(VectorizablePredicate::Not(Box::new(inner)))
        }
        // Transparent wrapper — recurse into the single child.
        ExprIR::Paren => {
            if node.num_children() != 1 {
                return None;
            }
            extract_predicate_expr(tree, node.child(0).idx(), params)
        }
        _ => try_extract_single_predicate(tree, idx, params).map(VectorizablePredicate::Leaf),
    }
}

/// Tries to extract a single `SimplePredicate` from a comparison,
/// `IS [NOT] NULL`, or string-match (`CONTAINS`/`STARTS WITH`/`ENDS WITH`)
/// expression rooted at `idx`.
fn try_extract_single_predicate(
    tree: &DynTree<ExprIR<Variable>>,
    idx: NodeIdx<Dyn<ExprIR<Variable>>>,
    params: &HashMap<String, Value>,
) -> Option<SimplePredicate> {
    let root = tree.node(idx);

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
            Value::Int(42),
        ];
        let mask = compare_string_column(&data, CmpOp::Eq, "Alice");
        assert_eq!(
            mask.0,
            vec![Tri::True, Tri::False, Tri::Null, Tri::True, Tri::False]
        );

        // Disjoint types: `<>` is true, ordering is NULL.
        let mask = compare_string_column(&data, CmpOp::Neq, "Alice");
        assert_eq!(
            mask.0,
            vec![Tri::False, Tri::True, Tri::Null, Tri::False, Tri::True]
        );
        let mask = compare_string_column(&data, CmpOp::Lt, "Bob");
        assert_eq!(
            mask.0,
            vec![Tri::True, Tri::False, Tri::Null, Tri::True, Tri::Null]
        );
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
        let mask = match_string_column(&data, StringMatchOp::Contains, "fixture_alice");
        // Non-string rows are NULL, matching the scalar internal functions.
        assert_eq!(
            mask.0,
            vec![Tri::True, Tri::False, Tri::Null, Tri::Null, Tri::True]
        );
        assert_eq!(
            match_string_column(&data, StringMatchOp::StartsWith, "fixture_").0,
            vec![Tri::True, Tri::True, Tri::Null, Tri::Null, Tri::False]
        );
        assert_eq!(
            match_string_column(&data, StringMatchOp::EndsWith, "alice").0,
            vec![Tri::False, Tri::False, Tri::Null, Tri::Null, Tri::True]
        );
    }

    #[test]
    fn test_tri_mask_kleene_ops() {
        // Rows: [True, False, Null]
        let m = || TriMask::new(vec![true, false, false], vec![false, false, true]);

        // NOT: [False, True, Null]
        let not = !m();
        assert_eq!(not.0, vec![Tri::False, Tri::True, Tri::Null]);

        // AND with [True, True, True]: [True, False, Null]
        let all_true = TriMask::from_bools(vec![true, true, true]);
        let and = m().and(&all_true);
        assert_eq!(and.0, vec![Tri::True, Tri::False, Tri::Null]);
        // null AND false = false; null AND null = null
        let nulls = TriMask::new(vec![false; 3], vec![true; 3]);
        let and = nulls.and(&m());
        assert_eq!(and.0, vec![Tri::Null, Tri::False, Tri::Null]);

        // null OR true = true; null OR false = null; null OR null = null
        let nulls = TriMask::new(vec![false; 3], vec![true; 3]);
        let or = nulls.or(&m());
        assert_eq!(or.0, vec![Tri::True, Tri::Null, Tri::Null]);
        // false OR false = false
        let all_false = TriMask::from_bools(vec![false, false, false]);
        let or = m().or(&all_false);
        assert_eq!(or.0, vec![Tri::True, Tri::False, Tri::Null]);
    }

    /// Exhaustively checks `and`/`or`/`not` against the Kleene truth table for
    /// every one of the 9 `(Tri, Tri)` input pairs, independent of the
    /// `min`/`max`/`2 - x` encoding used to implement them.
    #[test]
    fn test_tri_exhaustive_truth_table() {
        fn expected_and(
            a: Tri,
            b: Tri,
        ) -> Tri {
            match (a, b) {
                (Tri::False, _) | (_, Tri::False) => Tri::False,
                (Tri::True, Tri::True) => Tri::True,
                _ => Tri::Null,
            }
        }
        fn expected_or(
            a: Tri,
            b: Tri,
        ) -> Tri {
            match (a, b) {
                (Tri::True, _) | (_, Tri::True) => Tri::True,
                (Tri::False, Tri::False) => Tri::False,
                _ => Tri::Null,
            }
        }
        fn expected_not(a: Tri) -> Tri {
            match a {
                Tri::True => Tri::False,
                Tri::False => Tri::True,
                Tri::Null => Tri::Null,
            }
        }

        let states = [Tri::True, Tri::False, Tri::Null];
        for &a in &states {
            for &b in &states {
                let and = TriMask::from_tri(vec![a]).and(&TriMask::from_tri(vec![b]));
                assert_eq!(and.0[0], expected_and(a, b), "{a:?} AND {b:?}");
                let or = TriMask::from_tri(vec![a]).or(&TriMask::from_tri(vec![b]));
                assert_eq!(or.0[0], expected_or(a, b), "{a:?} OR {b:?}");
            }
            let not = !TriMask::from_tri(vec![a]);
            assert_eq!(not.0[0], expected_not(a), "NOT {a:?}");
        }
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
                let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::And(preds) = pred else {
                panic!("expected conjunction");
            };
            assert_eq!(preds.len(), 2);
            let VectorizablePredicate::Leaf(first) = &preds[0] else {
                panic!("expected leaf");
            };
            let VectorizablePredicate::Leaf(second) = &preds[1] else {
                panic!("expected leaf");
            };
            assert!(matches!(
                first.test,
                PredicateTest::IsNull { negated: true }
            ));
            assert!(matches!(second.test, PredicateTest::StringMatch { .. }));
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::Leaf(p) = pred else {
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
            let VectorizablePredicate::Leaf(p) = pred else {
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

        #[test]
        fn test_extract_from_parsed_not_contains_query() {
            let expr = bind_where_expr(
                "MATCH (u:User) WHERE NOT u.ft_text CONTAINS 'fixture_alice' RETURN count(u)",
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed NOT ... CONTAINS filter should be vectorizable");
            let VectorizablePredicate::Not(inner) = pred else {
                panic!("expected NOT predicate");
            };
            let VectorizablePredicate::Leaf(p) = *inner else {
                panic!("expected leaf under NOT");
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
        fn test_extract_from_parsed_or_query() {
            let expr = bind_where_expr(
                "MATCH (u:User) WHERE u.embedding IS NULL OR u.age > 30 RETURN count(u)",
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("parsed OR filter should be vectorizable");
            let VectorizablePredicate::Or(children) = pred else {
                panic!("expected OR predicate");
            };
            assert_eq!(children.len(), 2);
            let VectorizablePredicate::Leaf(first) = &children[0] else {
                panic!("expected leaf");
            };
            let VectorizablePredicate::Leaf(second) = &children[1] else {
                panic!("expected leaf");
            };
            assert!(matches!(
                first.test,
                PredicateTest::IsNull { negated: false }
            ));
            assert!(matches!(
                second.test,
                PredicateTest::Cmp { op: CmpOp::Gt, .. }
            ));
        }

        #[test]
        fn test_extract_from_parsed_nested_boolean_query() {
            let expr = bind_where_expr(
                "MATCH (u:User) WHERE NOT (u.age > 30 OR u.name CONTAINS 'x') RETURN count(u)",
            );
            let pred = try_extract_vectorizable_predicate(&expr, &HashMap::new())
                .expect("nested NOT/OR filter should be vectorizable");
            let VectorizablePredicate::Not(inner) = pred else {
                panic!("expected NOT predicate");
            };
            assert!(matches!(*inner, VectorizablePredicate::Or(_)));
        }

        #[test]
        fn test_extract_or_with_unsupported_side_falls_back() {
            // One OR branch is not vectorizable — the whole predicate must
            // fall back to per-row evaluation.
            let expr = bind_where_expr(
                "MATCH (u:User) WHERE u.age > 30 OR size(u.name) > 3 RETURN count(u)",
            );
            assert!(try_extract_vectorizable_predicate(&expr, &HashMap::new()).is_none());
        }
    }
}
