use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{addr_of_mut, null_mut},
    rc::Rc,
};

use crate::GraphBLAS::{
    GrB_BOOL, GrB_Info, GrB_Matrix, GrB_Matrix_extractElement_BOOL, GrB_Matrix_free,
    GrB_Matrix_ncols, GrB_Matrix_new, GrB_Matrix_nrows, GrB_Matrix_removeElement,
    GrB_Matrix_resize, GrB_Matrix_setElement_BOOL, GrB_Mode, GrB_finalize, GrB_init, GxB_Iterator,
    GxB_Iterator_free, GxB_Iterator_new, GxB_Matrix_Iterator_attach, GxB_Matrix_Iterator_getIndex,
    GxB_Matrix_Iterator_next, GxB_Matrix_Iterator_seek,
};

pub fn init() {
    unsafe {
        GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
    }
}

pub fn shutdown() {
    unsafe {
        GrB_finalize();
    }
}

pub trait Size<T> {
    fn nrows(&self) -> u64;
    fn ncols(&self) -> u64;
    fn resize(&mut self, nrows: u64, ncols: u64);
}

pub trait Get<T> {
    fn get(&self, i: u64, j: u64) -> Option<T>;
}

pub trait Set<T> {
    fn set(&mut self, i: u64, j: u64, value: T);
}

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
                depleted: info == GrB_Info::GxB_EXHAUSTED,
                data: true,
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
                depleted: info == GrB_Info::GxB_EXHAUSTED,
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
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info::GxB_EXHAUSTED;
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
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info::GxB_EXHAUSTED;
            Some((row, col))
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
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
            nrows
        }
    }

    fn ncols(&self) -> u64 {
        unsafe {
            let mut ncols = 0u64;
            let info = GrB_Matrix_ncols(addr_of_mut!(ncols), *self.m);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
            ncols
        }
    }

    fn resize(&mut self, nrows: u64, ncols: u64) {
        unsafe {
            let info = GrB_Matrix_resize(*self.m, nrows, ncols);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

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

impl<T> Delete<T> for Matrix<T> {
    fn delete(&mut self, i: u64, j: u64) {
        unsafe {
            let info = GrB_Matrix_removeElement(*self.m, i, j);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl Get<bool> for Matrix<bool> {
    fn get(&self, i: u64, j: u64) -> Option<bool> {
        unsafe {
            let mut m: MaybeUninit<bool> = MaybeUninit::uninit();
            let info = GrB_Matrix_extractElement_BOOL(m.as_mut_ptr(), *self.m, i, j);
            if info == GrB_Info::GrB_SUCCESS {
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
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}
