//! MVCC-aware sparse matrix with delta tracking.
//!
//! This module provides [`VersionedMatrix`], which wraps a base matrix with
//! delta matrices to track pending additions and deletions. This enables
//! snapshot isolation for concurrent readers.
//!
//! ## Structure
//!
//! ```text
//! VersionedMatrix
//!    ├── m: Base matrix (committed state)
//!    ├── dp: Delta-plus (pending additions)
//!    └── dm: Delta-minus (pending deletions)
//!
//! Effective state = (m ∪ dp) - dm
//! ```
//!
//! ## MVCC Semantics
//!
//! - Readers see the committed base matrix `m`
//! - Writers accumulate changes in `dp` and `dm`
//! - On commit, deltas merge into base; on rollback, deltas are discarded

use crate::graph::{
    GraphBLAS::GxB_Print_Level,
    cow::Cow,
    matrix::{self, Dup, Get, MaskedElementWiseAdd, Matrix, New, Remove, Set, Size, Transpose},
};

/// A matrix with MVCC delta tracking for snapshot isolation.
///
/// Wraps a base matrix with separate matrices for tracking additions
/// and deletions, enabling concurrent reads during writes.
pub struct VersionedMatrix {
    /// Base committed matrix
    m: Cow<Matrix>,
    /// Delta-plus: edges added in current transaction
    dp: Cow<Matrix>,
    /// Delta-minus: edges removed in current transaction  
    dm: Cow<Matrix>,
}

// SAFETY: VersionedMatrix contains three Cow<Matrix> fields (base, delta-plus, delta-minus).
// Each Cow<Matrix> wraps a Matrix which is Send+Sync (see matrix.rs). The MVCC protocol
// ensures that only one writer modifies a VersionedMatrix at a time, while readers
// operate on separate snapshot copies created via dup().
unsafe impl Send for VersionedMatrix {}
unsafe impl Sync for VersionedMatrix {}

impl Size for VersionedMatrix {
    fn nrows(&self) -> u64 {
        self.m.nrows()
    }

    fn ncols(&self) -> u64 {
        self.m.ncols()
    }

    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.wait();
        self.m.resize(nrows, ncols);
        self.dp.resize(nrows, ncols);
        self.dm.resize(nrows, ncols);
    }

    fn nvals(&self) -> u64 {
        self.wait();
        self.m.nvals() + self.dp.nvals() - self.dm.nvals()
    }
}

impl New for VersionedMatrix {
    fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: Cow::new(Matrix::new(nrows, ncols)),
            dp: Cow::new(Matrix::new(nrows, ncols)),
            dm: Cow::new(Matrix::new(nrows, ncols)),
        }
    }
}

impl Dup<Self> for VersionedMatrix {
    fn dup(&self) -> Self {
        Self {
            m: self.m.new_version(),
            dp: self.dp.new_version(),
            dm: self.dm.new_version(),
        }
    }
}

impl VersionedMatrix {
    pub fn flush(&mut self) {
        self.wait();
        if self.dp.nvals() >= 10000 {
            self.m.element_wise_add(None, None, Some(&self.dp), None);
            self.dp.clear();
        }
        if self.dm.nvals() >= 10000 {
            self.m.remove_all(&self.dm);
            self.dm.clear();
        }
    }

    pub fn wait(&self) {
        debug_assert!(!self.m.pending());
        self.dp.wait();
        self.dm.wait();
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.m.memory_usage() + self.dp.memory_usage() + self.dm.memory_usage()
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> Iter {
        self.wait();
        Iter::new(self, min_row, max_row)
    }

    #[must_use]
    pub fn to_matrix(&self) -> Matrix {
        // TODO: remove
        self.wait();
        let mut m = self.m.dup();
        m.remove_all(&self.dm);
        m.element_wise_add(None, None, Some(&self.dp), None);
        m
    }

    pub fn print(
        &self,
        level: GxB_Print_Level,
    ) {
        self.m.print(level);
        self.dp.print(level);
        self.dm.print(level);
    }
}

impl Remove for VersionedMatrix {
    fn remove(
        &mut self,
        i: u64,
        j: u64,
    ) {
        if self.m.get(i, j).is_some() {
            debug_assert!(self.dp.get(i, j).is_none());
            self.dm.set(i, j, true);
        } else {
            self.dp.remove(i, j);
        }
    }
}

// impl MxM<bool> for VersionedMatrix<bool> {
//     fn lmxm(
//         &mut self,
//         b: &Self,
//     ) {

//     }

//     fn rmxm(
//         &mut self,
//         b: &Self,
//     ) {

//     }
// }

impl Get for VersionedMatrix {
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<bool> {
        self.wait();
        self.m.get(i, j).map_or_else(
            || self.dp.get(i, j),
            |value| {
                if self.dm.get(i, j).is_some() {
                    None
                } else {
                    Some(value)
                }
            },
        )
    }
}

impl Set for VersionedMatrix {
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: bool,
    ) {
        debug_assert!(!self.m.pending());
        if self.m.get(i, j).is_some() {
            debug_assert!(self.dp.get(i, j).is_none());
            self.dm.remove(i, j);
        } else {
            debug_assert!(self.dm.get(i, j).is_none());
            self.dp.set(i, j, value);
        }
    }
}

impl Transpose for VersionedMatrix
where
    Self: New,
{
    /// Transposes the matrix.
    ///
    /// # Returns
    /// A new matrix that is the transpose of the original.
    fn transpose(&self) -> Self {
        Self {
            m: Cow::new(self.m.transpose()),
            dp: Cow::new(self.dp.transpose()),
            dm: Cow::new(self.dm.transpose()),
        }
    }
}

pub struct Iter {
    mit: matrix::Iter,
    dpit: matrix::Iter,
    dm: Cow<Matrix>,
}

// SAFETY: Iter holds matrix::Iter instances (which are Send+Sync, see matrix.rs) and a
// Cow<Matrix> for the delta-minus mask. All are safe to move between threads.
// The iterator is consumed single-threaded via Iterator trait (&mut self).
unsafe impl Send for Iter {}
unsafe impl Sync for Iter {}

impl Iter {
    /// Creates a new iterator for traversing all elements in a matrix.
    ///
    /// # Parameters
    /// - `m`: The matrix to iterate over.
    /// - `min_row`: The minimum row index to start iterating from.
    /// - `max_row`: The maximum row index to stop iterating at.
    #[must_use]
    pub fn new(
        m: &VersionedMatrix,
        min_row: u64,
        max_row: u64,
    ) -> Self {
        Self {
            mit: m.m.iter(min_row, max_row),
            dpit: m.dp.iter(min_row, max_row),
            dm: m.dm.clone(),
        }
    }
}

impl Iterator for Iter {
    type Item = (u64, u64);

    /// Advances the iterator and returns the next element in the matrix.
    ///
    /// # Returns
    /// - `Some((u64, u64))`: The next element in the matrix.
    /// - `None`: The iterator is depleted.
    fn next(&mut self) -> Option<Self::Item> {
        for (i, j) in &mut self.mit {
            if self.dm.get(i, j).is_none() {
                return Some((i, j));
            }
        }
        self.dpit.next()
    }
}
