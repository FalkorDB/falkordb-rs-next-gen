//! Row-encoder seam: `value → sortable u64` row key(s).
//!
//! This is the **only** kind-specific thing the matrix core needs to perform
//! range / equality scans (design doc `01-mvcc-core.md` §3). Each index kind
//! supplies a [`RowEncoder`] whose encoding is **order-preserving**: the
//! unsigned `u64` ordering of the encoded keys equals the semantic ordering of
//! the values. Sorted row keys are what make a contiguous row-range scan
//! implement equality, range, `IN`, and array-contains in one primitive.
//!
//! The numeric POC ships one encoder, [`super::NumericEncoder`]. Later kinds
//! (tag / text / geo / vector) plug in their own `RowEncoder`; the core is
//! unchanged. See `04-query-api.md` §5 for how each `IndexQuery` variant lowers
//! onto row ranges using these keys.

use std::ops::Bound;

/// Which end of a range a bound sits on. Determines how an **open** bound
/// (`Unbounded`) or an **exclusive** bound (`Excluded`) collapses onto the
/// inclusive `[lo, hi]` row range a scan walks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundSide {
    /// The lower (`min`) end of the range.
    Lower,
    /// The upper (`max`) end of the range.
    Upper,
}

/// Maps an indexed value onto one or more **sortable** matrix row keys.
///
/// Implementors guarantee: for values `a`, `b` of the kind's domain,
/// `a < b` (semantically) iff `encode(a) < encode(b)` (as `u64`). This lets the
/// matrix core treat every predicate as a row-key range without knowing the
/// kind's value type.
pub trait RowEncoder {
    /// The value domain this encoder maps (numeric, tag, text, …).
    type Value;

    /// Encode a value into one or more sortable row keys, appended to `out`.
    ///
    /// A scalar pushes exactly one key; an array / multi-valued property pushes
    /// one key per element (`01-mvcc-core.md` §2: a doc legitimately occupies
    /// many rows). Non-indexable inputs push nothing — the indexability gate
    /// (`node_by_index_scan.rs::can_utilize_index`) keeps them out, and the
    /// encoder stays total rather than panicking.
    fn encode(
        &self,
        v: &Self::Value,
        out: &mut Vec<u64>,
    );

    /// Collapse a (possibly open / exclusive) range bound to the inclusive row
    /// key used by a row-range scan:
    /// - `Unbounded` lower → `u64::MIN`, upper → `u64::MAX`;
    /// - `Included(v)` → `encode(v)`;
    /// - `Excluded(v)` → the adjacent key (`+1` on the lower side, `-1` on the
    ///   upper side). Because the encoding is dense and order-preserving, the
    ///   adjacent key is exactly the first/last value strictly inside the bound.
    fn encode_bound(
        &self,
        b: Bound<&Self::Value>,
        side: BoundSide,
    ) -> u64;
}
