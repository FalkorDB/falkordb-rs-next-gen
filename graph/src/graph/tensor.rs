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
