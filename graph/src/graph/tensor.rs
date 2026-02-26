//! Tensor storage for multi-edges between node pairs.
//!
//! This module provides [`Tensor`], which extends the matrix model to support
//! multiple edges between the same pair of nodes. While the adjacency matrix
//! only records edge existence, the tensor stores individual edge IDs.
//!
//! ## Structure
//!
//! ```text
//! Tensor
//!    ├── m: Forward adjacency (src → dst exists?)
//!    ├── mt: Backward adjacency (dst → src exists?)
//!    └── me: Edge matrix ((src,dst) → edge_id)
//! ```
//!
//! The `me` matrix uses a compound key `(src << 32 | dst)` as the row index,
//! allowing multiple edge IDs to be stored for the same node pair.
//!
//! ## Use Case
//!
//! In property graphs, multiple edges of the same type can connect two nodes.
//! For example: two "KNOWS" relationships between the same people with
//! different "since" dates.

use crate::graph::{
    matrix::{Dup, New, Remove, Set, Size},
    versioned_matrix::{self, VersionedMatrix},
};

/// Maximum GraphBLAS index value (2^60 - 1).
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

/// Multi-edge storage supporting multiple edges between node pairs.
///
/// Maintains three matrices for efficient traversal in both directions
/// and edge ID lookup.
pub struct Tensor {
    /// Forward adjacency matrix (src → dst)
    m: VersionedMatrix,
    /// Transpose/backward adjacency (dst → src)
    mt: VersionedMatrix,
    /// Edge ID storage keyed by (src, dst) pair
    me: VersionedMatrix,
}

impl New for Tensor {
    fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: VersionedMatrix::new(nrows, ncols),
            mt: VersionedMatrix::new(ncols, nrows),
            me: VersionedMatrix::new(GrB_INDEX_MAX, GrB_INDEX_MAX),
        }
    }
}

impl Tensor {
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> versioned_matrix::Iter {
        assert!(
            u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
            "tensor key overflow: src={src}, dest={dest} must fit in u32"
        );
        let row = src << 32 | dest;
        self.me.iter(row, row)
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        assert!(
            u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
            "tensor key overflow: src={src}, dest={dest} must fit in u32"
        );
        self.m.set(src, dest, true);
        self.mt.set(dest, src, true);
        self.me.set(src << 32 | dest, id, true);
    }

    pub fn remove_all(
        &mut self,
        rels: Vec<(u64, u64, u64)>,
    ) {
        for (id, src, dest) in &rels {
            assert!(
                u32::try_from(*src).is_ok() && u32::try_from(*dest).is_ok(),
                "tensor key overflow: src={src}, dest={dest} must fit in u32"
            );
            self.me.remove(src << 32 | dest, *id);
        }
        for (_, src, dest) in rels {
            assert!(
                u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
                "tensor key overflow: src={src}, dest={dest} must fit in u32"
            );
            if self
                .me
                .iter(src << 32 | dest, src << 32 | dest)
                .next()
                .is_none()
            {
                self.m.remove(src, dest);
                self.mt.remove(dest, src);
            }
        }
    }

    pub fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.m.resize(nrows, ncols);
        self.mt.resize(ncols, nrows);
    }

    #[must_use]
    pub fn dup(&self) -> Self {
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            me: self.me.dup(),
        }
    }

    #[must_use]
    pub const fn matrix(&self) -> &VersionedMatrix {
        &self.m
    }

    #[must_use]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Iter<'_> {
        Iter::new(self, min_row, max_row, transpose)
    }

    pub fn wait(&mut self) {
        self.m.wait();
        self.mt.wait();
        self.me.wait();
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.m.memory_usage() + self.mt.memory_usage() + self.me.memory_usage()
    }
}

/// Encode a (src, dest) pair into a compound key for the edge matrix.
///
/// The key packs two u32-range values into a single u64: `src << 32 | dest`.
/// Both `src` and `dest` must fit in u32; this function panics otherwise.
#[inline]
fn encode_key(
    src: u64,
    dest: u64,
) -> u64 {
    assert!(
        u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
        "tensor key overflow: src={src}, dest={dest} must fit in u32"
    );
    src << 32 | dest
}

/// Decode a compound key back into (src, dest).
#[inline]
fn decode_key(key: u64) -> (u64, u64) {
    let src = key >> 32;
    let dest = key & 0xFFFF_FFFF;
    (src, dest)
}

pub struct Iter<'a> {
    t: &'a Tensor,
    mit: versioned_matrix::Iter,
    vit: Option<versioned_matrix::Iter>,
    transpose: bool,
    src: u64,
    dest: u64,
}

impl<'a> Iter<'a> {
    fn new(
        t: &'a Tensor,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Self {
        Self {
            t,
            mit: if transpose {
                t.mt.iter(min_row, max_row)
            } else {
                t.m.iter(min_row, max_row)
            },
            vit: None,
            transpose,
            src: 0,
            dest: 0,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vit) = &mut self.vit {
            if let Some((_, id)) = vit.next() {
                return Some((self.src, self.dest, id));
            }
            self.vit = None;
        }

        if let Some((src, dest)) = self.mit.next() {
            if self.transpose {
                self.src = dest;
                self.dest = src;
            } else {
                self.src = src;
                self.dest = dest;
            }
            assert!(
                u32::try_from(self.src).is_ok() && u32::try_from(self.dest).is_ok(),
                "tensor key overflow: src={}, dest={} must fit in u32",
                self.src,
                self.dest
            );
            let row = self.src << 32 | self.dest;
            self.vit = Some(self.t.me.iter(row, row));
            return self.next();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Key Encoding Roundtrip Tests
    // ========================================================================

    #[test]
    fn test_key_roundtrip_zero_zero() {
        let key = encode_key(0, 0);
        assert_eq!(decode_key(key), (0, 0));
    }

    #[test]
    fn test_key_roundtrip_zero_one() {
        let key = encode_key(0, 1);
        assert_eq!(decode_key(key), (0, 1));
    }

    #[test]
    fn test_key_roundtrip_one_zero() {
        let key = encode_key(1, 0);
        assert_eq!(decode_key(key), (1, 0));
    }

    #[test]
    fn test_key_roundtrip_u32_max() {
        let max = u32::MAX as u64;
        let key = encode_key(max, max);
        assert_eq!(decode_key(key), (max, max));
    }

    #[test]
    fn test_key_roundtrip_asymmetric() {
        // Verify encoding is not symmetric: (1, 2) != (2, 1)
        let key_a = encode_key(1, 2);
        let key_b = encode_key(2, 1);
        assert_ne!(key_a, key_b);
        assert_eq!(decode_key(key_a), (1, 2));
        assert_eq!(decode_key(key_b), (2, 1));
    }

    #[test]
    fn test_key_roundtrip_mixed_boundary() {
        let max = u32::MAX as u64;
        let key = encode_key(0, max);
        assert_eq!(decode_key(key), (0, max));

        let key = encode_key(max, 0);
        assert_eq!(decode_key(key), (max, 0));
    }

    // ========================================================================
    // Overflow / Panic Tests
    // ========================================================================

    #[test]
    #[should_panic(expected = "tensor key overflow")]
    fn test_key_encode_panics_src_overflow() {
        let overflow = u32::MAX as u64 + 1;
        encode_key(overflow, 0);
    }

    #[test]
    #[should_panic(expected = "tensor key overflow")]
    fn test_key_encode_panics_dest_overflow() {
        let overflow = u32::MAX as u64 + 1;
        encode_key(0, overflow);
    }

    #[test]
    #[should_panic(expected = "tensor key overflow")]
    fn test_key_encode_panics_both_overflow() {
        let overflow = u32::MAX as u64 + 1;
        encode_key(overflow, overflow);
    }
}
