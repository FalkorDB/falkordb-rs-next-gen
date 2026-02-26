//! Sparse vector operations using GraphBLAS.
//!
//! This module wraps GraphBLAS sparse vectors, used for:
//! - Storing node membership in labels (label → vector of node IDs)
//! - Intermediate results in graph algorithms
//! - Filtering operations
//!
//! ## GraphBLAS Vectors
//!
//! Like matrices, vectors use sparse storage and count toward Redis memory.
//! Boolean vectors are used to represent sets of node IDs efficiently.

use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{addr_of_mut, null_mut},
};

use crate::graph::GraphBLAS::{
    GrB_BOOL, GrB_Info, GrB_Vector, GrB_Vector_free, GrB_Vector_new, GrB_Vector_removeElement,
    GrB_Vector_resize, GrB_Vector_setElement_BOOL, GrB_Vector_size, GrB_Vector_wait, GrB_WaitMode,
    GxB_Iterator, GxB_Iterator_free, GxB_Iterator_new, GxB_Vector_Iterator_attach,
    GxB_Vector_Iterator_getIndex, GxB_Vector_Iterator_next, GxB_Vector_Iterator_seek,
};

/// A sparse vector backed by GraphBLAS.
///
/// Generic over element type T, though currently only bool is implemented.
/// The vector automatically frees its GraphBLAS resources on drop.
pub struct Vector<T> {
    v: GrB_Vector,
    phantom: PhantomData<T>,
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            let info = GrB_Vector_free(addr_of_mut!(self.v));
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_Vector_free failed");
        }
    }
}

impl From<GrB_Vector> for Vector<bool> {
    fn from(v: GrB_Vector) -> Self {
        Self {
            v,
            phantom: PhantomData,
        }
    }
}

impl Vector<bool> {
    pub fn new(nrows: u64) -> Self {
        unsafe {
            let mut v: MaybeUninit<GrB_Vector> = MaybeUninit::uninit();
            let info = GrB_Vector_new(v.as_mut_ptr(), GrB_BOOL, nrows);
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_Vector_new failed");
            Self {
                v: v.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    pub fn set(
        &mut self,
        i: u64,
        value: bool,
    ) {
        unsafe {
            let info = GrB_Vector_setElement_BOOL(self.v, value, i);
            assert_eq!(
                info,
                GrB_Info::GrB_SUCCESS,
                "GrB_Vector_setElement_BOOL failed"
            );
        }
    }

    pub fn wait(&mut self) {
        unsafe {
            let info = GrB_Vector_wait(self.v, GrB_WaitMode::GrB_MATERIALIZE as _);
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_Vector_wait failed");
        }
    }

    #[must_use]
    pub const fn ptr(&self) -> GrB_Vector {
        self.v
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<bool> {
        Iter::new(self)
    }
}

pub trait Size<T> {
    fn size(&self) -> u64;
    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    );
}

pub trait Set<T> {
    fn set(
        &mut self,
        i: u64,
        value: T,
    );
}

pub trait Remove<T> {
    fn remove(
        &mut self,
        i: u64,
    );
}

impl Size<bool> for Vector<bool> {
    fn size(&self) -> u64 {
        unsafe {
            let mut size: u64 = 0;
            let info = GrB_Vector_size(&raw mut size, self.v);
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_Vector_size failed");
            size
        }
    }

    fn resize(
        &mut self,
        nrows: u64,
        _ncols: u64,
    ) {
        unsafe {
            let info = GrB_Vector_resize(self.v, nrows);
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GrB_Vector_resize failed");
        }
    }
}

impl Set<bool> for Vector<bool> {
    fn set(
        &mut self,
        i: u64,
        value: bool,
    ) {
        unsafe {
            let info = GrB_Vector_setElement_BOOL(self.v, value, i);
            assert_eq!(
                info,
                GrB_Info::GrB_SUCCESS,
                "GrB_Vector_setElement_BOOL failed"
            );
        }
    }
}

impl Remove<bool> for Vector<bool> {
    fn remove(
        &mut self,
        i: u64,
    ) {
        unsafe {
            let info = GrB_Vector_removeElement(self.v, i);
            assert_eq!(
                info,
                GrB_Info::GrB_SUCCESS,
                "GrB_Vector_removeElement failed"
            );
        }
    }
}

pub struct Iter<T> {
    inner: GxB_Iterator,
    depleted: bool,
    phantom: PhantomData<T>,
}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        unsafe {
            let info = GxB_Iterator_free(addr_of_mut!(self.inner));
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GxB_Iterator_free failed");
        }
    }
}

impl<T> Iter<T> {
    #[must_use]
    pub fn new(v: &Vector<T>) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            let info = GxB_Iterator_new(iter.as_mut_ptr());
            assert_eq!(info, GrB_Info::GrB_SUCCESS, "GxB_Iterator_new failed");
            let iter = iter.assume_init();
            let info = GxB_Vector_Iterator_attach(iter, v.v, null_mut());
            assert_eq!(
                info,
                GrB_Info::GrB_SUCCESS,
                "GxB_Vector_Iterator_attach failed"
            );
            let info = GxB_Vector_Iterator_seek(iter, 0);
            Self {
                inner: iter,
                depleted: info == GrB_Info::GxB_EXHAUSTED,
                phantom: PhantomData,
            }
        }
    }
}

impl Iterator for Iter<bool> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.depleted {
            return None;
        }
        unsafe {
            let row = GxB_Vector_Iterator_getIndex(self.inner);
            self.depleted = GxB_Vector_Iterator_next(self.inner) == GrB_Info::GxB_EXHAUSTED;
            Some(row)
        }
    }
}
