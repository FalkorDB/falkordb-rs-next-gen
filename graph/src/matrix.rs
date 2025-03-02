use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
    ptr::{addr_of, addr_of_mut, null_mut},
    rc::Rc,
};

use crate::{
    value::Value,
    GraphBLAS::{
        GrB_BOOL, GrB_Info_GrB_SUCCESS, GrB_Info_GxB_EXHAUSTED, GrB_Matrix,
        GrB_Matrix_extractElement_BOOL, GrB_Matrix_extractElement_UDT, GrB_Matrix_free,
        GrB_Matrix_ncols, GrB_Matrix_new, GrB_Matrix_nrows, GrB_Matrix_removeElement,
        GrB_Matrix_resize, GrB_Matrix_setElement_BOOL, GrB_Matrix_setElement_UDT,
        GrB_Mode_GrB_NONBLOCKING, GrB_Type, GrB_Type_free, GrB_Type_new, GrB_finalize, GrB_init,
        GxB_Iterator, GxB_Iterator_free, GxB_Iterator_get_UDT, GxB_Iterator_new,
        GxB_Matrix_Iterator_attach, GxB_Matrix_Iterator_getIndex, GxB_Matrix_Iterator_next,
        GxB_Matrix_Iterator_seek,
    },
};

// This module provides a Matrix struct that wraps a GraphBLAS matrix.
pub fn init() {
    unsafe {
        GrB_init(GrB_Mode_GrB_NONBLOCKING as _);
        let mut t: MaybeUninit<GrB_Type> = MaybeUninit::uninit();
        GrB_Type_new(t.as_mut_ptr(), size_of::<Value>());
        VALUE_TYPE = t.assume_init();
    }
}

// This function is called when the Redis module is unloaded.
// It is responsible for freeing the memory allocated for the Matrix object.
pub fn shutdown() {
    unsafe {
        GrB_Type_free(addr_of_mut!(VALUE_TYPE));
        GrB_finalize();
    }
}

// This trait provides methods for getting the number of rows and columns of a matrix,
// resizing a matrix, and getting an iterator over the rows of a matrix.
pub trait Size<T> {
    fn nrows(&self) -> u64;
    fn ncols(&self) -> u64;
    fn resize(&mut self, nrows: u64, ncols: u64);
}

// This trait provides a method for getting the value at a given row and column of a matrix.
pub trait Get<T> {
    fn get(&self, i: u64, j: u64) -> Option<T>;
}

// This trait provides a method for setting the value at a given row and column of a matrix.
pub trait Set<T> {
    fn set(&mut self, i: u64, j: u64, value: T);
}

// This trait provides a method for deleting the value at a given row and column of a matrix.
pub trait Delete<T> {
    fn delete(&mut self, i: u64, j: u64);
}

pub struct Matrix<T> {
    m: Rc<GrB_Matrix>,
    phantom: PhantomData<T>,
}

pub struct Row<T> {
    row: u64,
    phantom: PhantomData<T>,
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

impl Iter<bool> {
    #[must_use]
    pub fn new(m: &Matrix<bool>) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            GxB_Iterator_new(iter.as_mut_ptr());
            let iter = iter.assume_init();
            GxB_Matrix_Iterator_attach(iter, *m.m, null_mut());
            let info = GxB_Matrix_Iterator_seek(iter, 0);
            Self {
                iter,
                depleted: info == GrB_Info_GxB_EXHAUSTED,
                data: true,
            }
        }
    }
}

impl Iter<Value> {
    #[must_use]
    pub fn new(m: &Matrix<Value>) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            GxB_Iterator_new(iter.as_mut_ptr());
            let iter = iter.assume_init();
            GxB_Matrix_Iterator_attach(iter, *m.m, null_mut());
            let info = GxB_Matrix_Iterator_seek(iter, 0);
            Self {
                iter,
                depleted: info == GrB_Info_GxB_EXHAUSTED,
                data: Value::Bool(true),
            }
        }
    }
}

impl<T> Iter<Row<T>> {
    #[must_use]
    pub fn new(m: &Matrix<T>, row: u64) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            GxB_Iterator_new(iter.as_mut_ptr());
            let iter = iter.assume_init();
            GxB_Matrix_Iterator_attach(iter, *m.m, null_mut());
            let info = GxB_Matrix_Iterator_seek(iter, row);
            Self {
                iter,
                depleted: info == GrB_Info_GxB_EXHAUSTED,
                data: Row {
                    row,
                    phantom: PhantomData,
                },
            }
        }
    }
}

impl Iterator for Iter<Row<bool>> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.depleted {
            return None;
        }
        unsafe {
            let mut row = 0u64;
            let mut col = 0u64;
            GxB_Matrix_Iterator_getIndex(self.iter, addr_of_mut!(row), addr_of_mut!(col));
            if row > self.data.row {
                self.depleted = true;
                return None;
            }
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info_GxB_EXHAUSTED;
            Some((row, col))
        }
    }
}

impl Iterator for Iter<bool> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.depleted {
            return None;
        }
        unsafe {
            let mut row = 0u64;
            let mut col = 0u64;
            GxB_Matrix_Iterator_getIndex(self.iter, addr_of_mut!(row), addr_of_mut!(col));
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info_GxB_EXHAUSTED;
            Some((row, col))
        }
    }
}

impl Iterator for Iter<Value> {
    type Item = (u64, Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.depleted {
            return None;
        }
        unsafe {
            let mut row = 0u64;
            let mut col = 0u64;
            GxB_Matrix_Iterator_getIndex(self.iter, addr_of_mut!(row), addr_of_mut!(col));
            let node_id = row;
            let mut keys = Vec::new();
            let mut values = Vec::new();
            while !self.depleted && row == node_id {
                let mut value: MaybeUninit<Value> = MaybeUninit::uninit();
                GxB_Iterator_get_UDT(self.iter, value.as_mut_ptr().cast::<c_void>());
                keys.push(Value::Int(col as _));
                values.push(value.assume_init());
                self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info_GxB_EXHAUSTED;
                if !self.depleted {
                    GxB_Matrix_Iterator_getIndex(self.iter, addr_of_mut!(row), addr_of_mut!(col));
                }
            }

            Some((node_id, Value::Array(keys), Value::Array(values)))
        }
    }
}

impl<T> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            m: self.m.clone(),
            phantom: self.phantom,
        }
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            Rc::get_mut(&mut self.m).map(|m| GrB_Matrix_free(addr_of_mut!(*m)));
        }
    }
}

impl<T> Matrix<T> {
    #[must_use]
    pub fn iter_row(&self, row: u64) -> Iter<Row<T>> {
        Iter::<Row<T>>::new(self, row)
    }
}

impl<T> Size<T> for Matrix<T> {
    fn nrows(&self) -> u64 {
        unsafe {
            let mut nrows = 0u64;
            let info = GrB_Matrix_nrows(addr_of_mut!(nrows), *self.m);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
            nrows
        }
    }

    fn ncols(&self) -> u64 {
        unsafe {
            let mut ncols = 0u64;
            let info = GrB_Matrix_ncols(addr_of_mut!(ncols), *self.m);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
            ncols
        }
    }

    fn resize(&mut self, nrows: u64, ncols: u64) {
        unsafe {
            let info = GrB_Matrix_resize(*self.m, nrows, ncols);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
        }
    }
}

static mut VALUE_TYPE: GrB_Type = null_mut();

impl Matrix<bool> {
    pub fn new(nrows: u64, ncols: u64) -> Self {
        unsafe {
            let mut m: MaybeUninit<GrB_Matrix> = MaybeUninit::uninit();
            GrB_Matrix_new(m.as_mut_ptr(), GrB_BOOL, nrows, ncols);
            Self {
                m: Rc::new(m.assume_init()),
                phantom: PhantomData,
            }
        }
    }

    #[must_use]
    pub fn iter(&self) -> Iter<bool> {
        Iter::<bool>::new(self)
    }
}

impl Matrix<Value> {

    // Creates a new matrix with the given number of rows and columns.
    pub fn new(nrows: u64, ncols: u64) -> Self {
        unsafe {
            let mut m: MaybeUninit<GrB_Matrix> = MaybeUninit::uninit();
            GrB_Matrix_new(m.as_mut_ptr(), VALUE_TYPE, nrows, ncols);
            Self {
                m: Rc::new(m.assume_init()),
                phantom: PhantomData,
            }
        }
    }

    // Returns an iterator over the rows of the matrix.
    #[must_use]
    pub fn iter(&self) -> Iter<Value> {
        Iter::<Value>::new(self)
    }
}

impl<T> Delete<T> for Matrix<T> {

    // Deletes the value at a given row and column of a matrix.
    fn delete(&mut self, i: u64, j: u64) {
        unsafe {
            let info = GrB_Matrix_removeElement(*self.m, i, j);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
        }
    }
}

impl Get<bool> for Matrix<bool> {

    // Gets the value at a given row and column of a matrix.
    fn get(&self, i: u64, j: u64) -> Option<bool> {
        unsafe {
            let mut m: MaybeUninit<bool> = MaybeUninit::uninit();
            let info = GrB_Matrix_extractElement_BOOL(m.as_mut_ptr(), *self.m, i, j);
            if info == GrB_Info_GrB_SUCCESS {
                Some(m.assume_init())
            } else {
                None
            }
        }
    }
}

impl Set<bool> for Matrix<bool> {
    fn set(&mut self, i: u64, j: u64, value: bool) {
        unsafe {
            let info = GrB_Matrix_setElement_BOOL(*self.m, value, i, j);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
        }
    }
}

impl Get<Value> for Matrix<Value> {
    fn get(&self, i: u64, j: u64) -> Option<Value> {
        unsafe {
            let mut m: MaybeUninit<Value> = MaybeUninit::uninit();
            let info =
                GrB_Matrix_extractElement_UDT(m.as_mut_ptr().cast::<c_void>(), *self.m, i, j);
            if info == GrB_Info_GrB_SUCCESS {
                Some(m.assume_init())
            } else {
                None
            }
        }
    }
}

// This implementation of the Set trait sets the value at a given row and column of a matrix.
impl Set<Value> for Matrix<Value> {

    // This function sets the value at a given row and column of a matrix.
    fn set(&mut self, i: u64, j: u64, value: Value) {
        unsafe {
            let info = GrB_Matrix_setElement_UDT(*self.m, addr_of!(value) as *mut c_void, i, j);
            assert_eq!(info, GrB_Info_GrB_SUCCESS);
        }
    }
}
