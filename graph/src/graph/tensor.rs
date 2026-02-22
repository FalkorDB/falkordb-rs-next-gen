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
        assert!(u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
                "Node IDs must fit in 32 bits: src={}, dest={}", src, dest);
        let row = src << 32 | dest;
        self.me.iter(row, row)
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        assert!(u32::try_from(src).is_ok() && u32::try_from(dest).is_ok(),
                "Node IDs must fit in 32 bits: src={}, dest={}", src, dest);
        self.m.set(src, dest, true);
        self.mt.set(dest, src, true);
        self.me.set(src << 32 | dest, id, true);
    }

    pub fn remove_all(
        &mut self,
        rels: Vec<(u64, u64, u64)>,
    ) {
        for (id, src, dest) in &rels {
            assert!(u32::try_from(*src).is_ok() && u32::try_from(*dest).is_ok(),
                    "Node IDs must fit in 32 bits: src={}, dest={}", src, dest);
            self.me.remove(src << 32 | dest, *id);
        }
        for (_, src, dest) in rels {
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
            assert!(u32::try_from(self.src).is_ok() && u32::try_from(self.dest).is_ok(),
                    "Node IDs must fit in 32 bits: src={}, dest={}", self.src, self.dest);
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

    #[test]
    fn test_tensor_set_get_roundtrip_small_values() {
        let mut tensor = Tensor::new(100, 100);

        // Test with small node IDs (well within 32-bit range)
        tensor.set(0, 0, 1);
        tensor.set(1, 2, 100);
        tensor.set(10, 20, 500);

        let mut edges = tensor.get(0, 0).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (0, 1));

        edges = tensor.get(1, 2).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (2, 100));

        edges = tensor.get(10, 20).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (20, 500));
    }

    #[test]
    fn test_tensor_boundary_values() {
        // Use GrB_INDEX_MAX as the tensor size - this is the GraphBLAS maximum index
        let mut tensor = Tensor::new(GrB_INDEX_MAX, GrB_INDEX_MAX);

        // Test with boundary values (max 32-bit value)
        let max_32bit = u64::from(u32::MAX);
        tensor.set(0, max_32bit, 1);
        tensor.set(max_32bit, 0, 2);
        tensor.set(max_32bit, max_32bit, 3);

        let edges = tensor.get(0, max_32bit).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].1, 1);

        let edges = tensor.get(max_32bit, 0).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].1, 2);

        let edges = tensor.get(max_32bit, max_32bit).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].1, 3);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_set_overflow_src() {
        let mut tensor = Tensor::new(u64::MAX, 100);
        // This should panic because src > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        tensor.set(overflow_value, 0, 1);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_set_overflow_dest() {
        let mut tensor = Tensor::new(100, u64::MAX);
        // This should panic because dest > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        tensor.set(0, overflow_value, 1);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_get_overflow_src() {
        let tensor = Tensor::new(u64::MAX, 100);
        // This should panic because src > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        let _ = tensor.get(overflow_value, 0).collect::<Vec<_>>();
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_get_overflow_dest() {
        let tensor = Tensor::new(100, u64::MAX);
        // This should panic because dest > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        let _ = tensor.get(0, overflow_value).collect::<Vec<_>>();
    }

    #[test]
    fn test_tensor_multiple_edges_same_pair() {
        let mut tensor = Tensor::new(100, 100);

        // Add multiple edges between same node pair
        tensor.set(5, 10, 100);
        tensor.set(5, 10, 200);
        tensor.set(5, 10, 300);

        let edges = tensor.get(5, 10).collect::<Vec<_>>();
        assert_eq!(edges.len(), 3);

        // Verify all edge IDs are present
        let edge_ids: Vec<u64> = edges.iter().map(|(_, id)| *id).collect();
        assert!(edge_ids.contains(&100));
        assert!(edge_ids.contains(&200));
        assert!(edge_ids.contains(&300));
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_remove_all_overflow() {
        let mut tensor = Tensor::new(u64::MAX, u64::MAX);
        let overflow_value = u64::from(u32::MAX) + 1;

        // This should panic because node IDs are > u32::MAX
        tensor.remove_all(vec![(1, overflow_value, 0)]);
    }
}
