//! The numeric / range [`RowEncoder`] — the POC kind's value→row-key map.

use std::ops::Bound;

use crate::index::native::api::{BoundSide, RowEncoder};
use crate::runtime::value::Value;

/// Numeric / range encoder — the POC kind.
///
/// All numerics are encoded through an order-preserving `f64 → u64` transform,
/// matching RediSearch (which indexes every numeric as a `double`) so an `Int`
/// and a `Float` of equal value share a row and interleave correctly. Integers
/// that cannot round-trip through `f64` are kept out by the existing
/// indexability gate (`int_loses_f64_precision`); the big-int row-key partition
/// that would *lift* that limitation is future work (`01-mvcc-core.md` §3).
#[derive(Debug, Clone, Copy, Default)]
pub struct NumericEncoder;

impl NumericEncoder {
    /// Order-preserving `f64 → u64`: the unsigned ordering of the result equals
    /// the numeric ordering of the input (the standard IEEE-754 "radix" total
    /// order). `-0.0` is canonicalized to `+0.0` so the two share a row.
    ///
    /// Transform: non-negative values get their sign bit set (so they sort
    /// above all negatives); negative values are bitwise-inverted (so larger
    /// magnitude sorts lower).
    #[must_use]
    #[inline]
    pub fn encode_f64(v: f64) -> u64 {
        // Canonicalize -0.0 to +0.0: numerically equal, must share a row.
        let v = if v == 0.0 { 0.0 } else { v };
        let bits = v.to_bits();
        // Sign bit set => negative => invert all bits; else flip just the sign
        // bit. The compiler lowers this `if` to a branchless select.
        let mask = if bits >> 63 == 1 { u64::MAX } else { 1 << 63 };
        bits ^ mask
    }

    /// Encode a single runtime value as a numeric row key, if it is numeric.
    ///
    /// Returns `None` for non-numeric values (an array element of the wrong
    /// type, say) so the caller can skip them without panicking — unlike
    /// `Value::get_numeric`, which is `unreachable!` off the numeric path.
    #[must_use]
    #[inline]
    pub fn encode_value(v: &Value) -> Option<u64> {
        match v {
            Value::Int(i) => Some(Self::encode_f64(*i as f64)),
            Value::Float(f) => Some(Self::encode_f64(*f)),
            _ => None,
        }
    }
}

impl RowEncoder for NumericEncoder {
    type Value = Value;

    fn encode(
        &self,
        v: &Value,
        out: &mut Vec<u64>,
    ) {
        match v {
            // Array / multi-valued property: one row per numeric element
            // (`01-mvcc-core.md` §2). Replaces RediSearch's `numeric:arr`.
            Value::List(items) => {
                for item in items.iter() {
                    if let Some(k) = Self::encode_value(item) {
                        out.push(k);
                    }
                }
            }
            // Scalar numeric: exactly one row.
            other => {
                if let Some(k) = Self::encode_value(other) {
                    out.push(k);
                }
            }
        }
    }

    fn encode_bound(
        &self,
        b: Bound<&Value>,
        side: BoundSide,
    ) -> u64 {
        match (b, side) {
            (Bound::Unbounded, BoundSide::Lower) => u64::MIN,
            (Bound::Unbounded, BoundSide::Upper) => u64::MAX,
            (Bound::Included(v), _) => Self::encode_value(v).unwrap_or(0),
            (Bound::Excluded(v), BoundSide::Lower) => {
                Self::encode_value(v).unwrap_or(0).saturating_add(1)
            }
            (Bound::Excluded(v), BoundSide::Upper) => {
                Self::encode_value(v).unwrap_or(u64::MAX).saturating_sub(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use thin_vec::ThinVec;

    fn enc(v: f64) -> u64 {
        NumericEncoder::encode_f64(v)
    }

    #[test]
    fn order_preserving_across_sign_boundary() {
        // A strictly-increasing sequence of representative f64 values; their
        // encodings must be strictly increasing too.
        let ordered = [
            f64::NEG_INFINITY,
            -1.0e300,
            -1.5,
            -1.0,
            -f64::MIN_POSITIVE,
            0.0,
            f64::MIN_POSITIVE,
            1.0,
            1.5,
            2.5,
            3.0,
            1.0e300,
            f64::INFINITY,
        ];
        for w in ordered.windows(2) {
            assert!(
                enc(w[0]) < enc(w[1]),
                "encoding must preserve order: {} (=>{}) should be < {} (=>{})",
                w[0],
                enc(w[0]),
                w[1],
                enc(w[1]),
            );
        }
    }

    #[test]
    fn negative_zero_canonicalizes_to_positive_zero() {
        assert_eq!(enc(-0.0), enc(0.0));
    }

    #[test]
    fn int_and_float_interleave_and_match() {
        // Equal value, different Value variant => identical row key.
        assert_eq!(
            NumericEncoder::encode_value(&Value::Int(5)),
            NumericEncoder::encode_value(&Value::Float(5.0)),
        );
        // Int 3 sorts above Float 2.5 and below Float 3.5.
        let k_25 = NumericEncoder::encode_value(&Value::Float(2.5)).unwrap();
        let k_3 = NumericEncoder::encode_value(&Value::Int(3)).unwrap();
        let k_35 = NumericEncoder::encode_value(&Value::Float(3.5)).unwrap();
        assert!(k_25 < k_3 && k_3 < k_35);
    }

    #[test]
    fn non_numeric_value_is_skipped() {
        assert_eq!(
            NumericEncoder::encode_value(&Value::String(Arc::new("x".to_string()))),
            None
        );
        let mut out = Vec::new();
        NumericEncoder.encode(&Value::Null, &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn scalar_pushes_one_key_array_pushes_many() {
        let mut out = Vec::new();
        NumericEncoder.encode(&Value::Int(7), &mut out);
        assert_eq!(out, vec![enc(7.0)]);

        out.clear();
        let list = Value::List(Arc::new(ThinVec::from(vec![
            Value::Int(1),
            Value::Float(2.0),
            Value::Int(3),
        ])));
        NumericEncoder.encode(&list, &mut out);
        assert_eq!(out, vec![enc(1.0), enc(2.0), enc(3.0)]);
    }

    #[test]
    fn array_skips_non_numeric_elements() {
        let mut out = Vec::new();
        let list = Value::List(Arc::new(ThinVec::from(vec![
            Value::Int(1),
            Value::String(Arc::new("skip".to_string())),
            Value::Float(2.5),
        ])));
        NumericEncoder.encode(&list, &mut out);
        assert_eq!(out, vec![enc(1.0), enc(2.5)]);
    }

    #[test]
    fn bound_open_ends_map_to_full_range() {
        let e = NumericEncoder;
        assert_eq!(e.encode_bound(Bound::Unbounded, BoundSide::Lower), u64::MIN);
        assert_eq!(e.encode_bound(Bound::Unbounded, BoundSide::Upper), u64::MAX);
    }

    #[test]
    fn bound_inclusive_equals_encode() {
        let e = NumericEncoder;
        let v = Value::Float(2.5);
        assert_eq!(
            e.encode_bound(Bound::Included(&v), BoundSide::Lower),
            enc(2.5)
        );
        assert_eq!(
            e.encode_bound(Bound::Included(&v), BoundSide::Upper),
            enc(2.5)
        );
    }

    #[test]
    fn bound_exclusive_steps_one_key_inward() {
        let e = NumericEncoder;
        let v = Value::Float(2.5);
        assert_eq!(
            e.encode_bound(Bound::Excluded(&v), BoundSide::Lower),
            enc(2.5) + 1
        );
        assert_eq!(
            e.encode_bound(Bound::Excluded(&v), BoundSide::Upper),
            enc(2.5) - 1
        );
    }

    #[test]
    fn range_scan_selects_expected_subset() {
        // Simulate `iter(lo, hi)` selection over a sorted key set: a closed
        // range [2.0, 4.0] must select exactly {2.0, 3.0, 4.0}.
        let values = [-1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let e = NumericEncoder;
        let lo = e.encode_bound(Bound::Included(&Value::Float(2.0)), BoundSide::Lower);
        let hi = e.encode_bound(Bound::Included(&Value::Float(4.0)), BoundSide::Upper);
        let selected: Vec<f64> = values
            .iter()
            .copied()
            .filter(|&v| {
                let k = enc(v);
                k >= lo && k <= hi
            })
            .collect();
        assert_eq!(selected, vec![2.0, 3.0, 4.0]);

        // Half-open (2.0, 4.0] must drop the 2.0 endpoint.
        let lo_ex = e.encode_bound(Bound::Excluded(&Value::Float(2.0)), BoundSide::Lower);
        let selected_ex: Vec<f64> = values
            .iter()
            .copied()
            .filter(|&v| {
                let k = enc(v);
                k >= lo_ex && k <= hi
            })
            .collect();
        assert_eq!(selected_ex, vec![3.0, 4.0]);
    }
}
