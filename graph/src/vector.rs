use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{addr_of_mut, null_mut},
};

use crate::GraphBLAS::{
    GrB_BOOL, GrB_Info, GrB_Vector, GrB_Vector_free, GrB_Vector_new, GrB_Vector_removeElement,
    GrB_Vector_resize, GrB_Vector_setElement_BOOL, GrB_Vector_size, GrB_Vector_wait, GrB_WaitMode,
    GxB_Iterator, GxB_Iterator_free, GxB_Iterator_new, GxB_Vector_Iterator_attach,
    GxB_Vector_Iterator_getIndex, GxB_Vector_Iterator_next, GxB_Vector_Iterator_seek,
};

pub struct Vector<T> {
    v: GrB_Vector,
    phantom: PhantomData<T>,
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            GrB_Vector_free(addr_of_mut!(self.v));
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
            GrB_Vector_new(v.as_mut_ptr(), GrB_BOOL, nrows);
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
            GrB_Vector_setElement_BOOL(self.v, value, i);
        }
    }

    pub fn wait(&mut self) {
        unsafe {
            GrB_Vector_wait(self.v, GrB_WaitMode::GrB_MATERIALIZE as _);
        }
    }

    #[must_use]
    pub const fn ptr(&self) -> GrB_Vector {
        self.v
    }

    pub fn iter(&self) -> Iter<bool> {
        Iter::new(self, true)
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

pub trait Delete<T> {
    fn delete(
        &mut self,
        i: u64,
    );
}

impl Size<bool> for Vector<bool> {
    fn size(&self) -> u64 {
        unsafe {
            let mut size: u64 = 0;
            GrB_Vector_size(&mut size, self.v);
            size
        }
    }

    fn resize(
        &mut self,
        nrows: u64,
        _ncols: u64,
    ) {
        unsafe {
            GrB_Vector_resize(self.v, nrows);
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
            GrB_Vector_setElement_BOOL(self.v, value, i);
        }
    }
}

impl Delete<bool> for Vector<bool> {
    fn delete(
        &mut self,
        i: u64,
    ) {
        unsafe {
            GrB_Vector_removeElement(self.v, i);
        }
    }
}

pub struct Iter<T> {
    iter: GxB_Iterator,
    depleted: bool,
    data: T,
}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        unsafe {
            GxB_Iterator_free(addr_of_mut!(self.iter));
        }
    }
}

impl<TItem> Iter<TItem> {
    #[must_use]
    pub fn new<T>(
        v: &Vector<T>,
        data: TItem,
    ) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            GxB_Iterator_new(iter.as_mut_ptr());
            let iter = iter.assume_init();
            GxB_Vector_Iterator_attach(iter, v.v, null_mut());
            let info = GxB_Vector_Iterator_seek(iter, 0);
            Self {
                iter,
                depleted: info == GrB_Info::GxB_EXHAUSTED,
                data,
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
            let row = GxB_Vector_Iterator_getIndex(self.iter);
            self.depleted = GxB_Vector_Iterator_next(self.iter) == GrB_Info::GxB_EXHAUSTED;
            Some(row)
        }
    }
}
