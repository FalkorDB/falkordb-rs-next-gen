use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::{addr_of_mut, null_mut},
};

use crate::GraphBLAS::{
    GrB_BOOL, GrB_Info, GrB_Matrix, GrB_Matrix_apply, GrB_Matrix_extractElement_BOOL,
    GrB_Matrix_extractElement_UINT64, GrB_Matrix_free, GrB_Matrix_ncols, GrB_Matrix_new,
    GrB_Matrix_nrows, GrB_Matrix_nvals, GrB_Matrix_removeElement, GrB_Matrix_resize,
    GrB_Matrix_setElement_BOOL, GrB_Matrix_setElement_UINT64, GrB_Matrix_wait, GrB_Mode,
    GrB_UINT64, GrB_UnaryOp, GrB_UnaryOp_free, GrB_UnaryOp_new, GrB_WaitMode, GrB_finalize,
    GxB_Iterator, GxB_Iterator_free, GxB_Iterator_get_UINT64, GxB_Iterator_new,
    GxB_Matrix_Iterator_attach, GxB_Matrix_Iterator_getIndex, GxB_Matrix_Iterator_next,
    GxB_Matrix_Iterator_seek, GxB_init, GxB_unary_function,
};

pub fn init(
    user_malloc_function: Option<unsafe extern "C" fn(arg1: usize) -> *mut c_void>,
    user_calloc_function: Option<unsafe extern "C" fn(arg1: usize, arg2: usize) -> *mut c_void>,
    user_realloc_function: Option<
        unsafe extern "C" fn(arg1: *mut c_void, arg2: usize) -> *mut c_void,
    >,
    user_free_function: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
) {
    unsafe {
        GxB_init(
            GrB_Mode::GrB_NONBLOCKING as _,
            user_malloc_function,
            user_calloc_function,
            user_realloc_function,
            user_free_function,
        );
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
    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    );
    fn nvals(&self) -> u64;
}

pub trait Get<T> {
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<T>;
}

pub trait Set<T> {
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: T,
    );
}

pub trait Remove<T> {
    fn remove(
        &mut self,
        i: u64,
        j: u64,
    );
}

pub struct Matrix<T> {
    m: GrB_Matrix,
    phantom: PhantomData<T>,
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            GrB_Matrix_free(addr_of_mut!(self.m));
        }
    }
}

impl<T> Matrix<T> {
    pub fn wait(&self) {
        unsafe {
            let info = GrB_Matrix_wait(self.m, GrB_WaitMode::GrB_MATERIALIZE as _);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl<T> Size<T> for Matrix<T> {
    fn nrows(&self) -> u64 {
        unsafe {
            let mut nrows = 0u64;
            let info = GrB_Matrix_nrows(addr_of_mut!(nrows), self.m);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
            nrows
        }
    }

    fn ncols(&self) -> u64 {
        unsafe {
            let mut ncols = 0u64;
            let info = GrB_Matrix_ncols(addr_of_mut!(ncols), self.m);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
            ncols
        }
    }

    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        unsafe {
            let info = GrB_Matrix_resize(self.m, nrows, ncols);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }

    fn nvals(&self) -> u64 {
        unsafe {
            let mut nvals = 0u64;
            let info = GrB_Matrix_nvals(addr_of_mut!(nvals), self.m);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
            nvals
        }
    }
}

impl Matrix<bool> {
    pub fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        unsafe {
            let mut m: MaybeUninit<GrB_Matrix> = MaybeUninit::uninit();
            GrB_Matrix_new(m.as_mut_ptr(), GrB_BOOL, nrows, ncols);
            Self {
                m: m.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> Iter<bool> {
        Iter::new(self, min_row, max_row)
    }
}

pub struct UnaryOp<T> {
    op: GrB_UnaryOp,
    phantom: PhantomData<T>,
}

unsafe impl<T> Sync for UnaryOp<T> {}

impl<T> Drop for UnaryOp<T> {
    fn drop(&mut self) {
        unsafe {
            GrB_UnaryOp_free(addr_of_mut!(self.op));
        }
    }
}

impl UnaryOp<u64> {
    pub fn new(function: GxB_unary_function) -> Self {
        unsafe {
            let mut op: MaybeUninit<GrB_UnaryOp> = MaybeUninit::uninit();
            let info = GrB_UnaryOp_new(op.as_mut_ptr(), function, GrB_UINT64, GrB_UINT64);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
            Self {
                op: op.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    pub const fn default() -> Self {
        Self {
            op: null_mut(),
            phantom: PhantomData,
        }
    }
}

impl Matrix<u64> {
    pub fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        unsafe {
            let mut m: MaybeUninit<GrB_Matrix> = MaybeUninit::uninit();
            GrB_Matrix_new(m.as_mut_ptr(), GrB_UINT64, nrows, ncols);
            Self {
                m: m.assume_init(),
                phantom: PhantomData,
            }
        }
    }

    pub fn apply(
        &mut self,
        op: &UnaryOp<u64>,
    ) {
        unsafe {
            let info = GrB_Matrix_apply(self.m, null_mut(), null_mut(), op.op, self.m, null_mut());
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> Iter<u64> {
        Iter::new(self, min_row, max_row)
    }
}

impl<T> Remove<T> for Matrix<T> {
    fn remove(
        &mut self,
        i: u64,
        j: u64,
    ) {
        unsafe {
            let info = GrB_Matrix_removeElement(self.m, i, j);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl Get<bool> for Matrix<bool> {
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<bool> {
        unsafe {
            let mut m: MaybeUninit<bool> = MaybeUninit::uninit();
            let info = GrB_Matrix_extractElement_BOOL(m.as_mut_ptr(), self.m, i, j);
            if info == GrB_Info::GrB_SUCCESS {
                Some(m.assume_init())
            } else {
                None
            }
        }
    }
}

impl Get<u64> for Matrix<u64> {
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<u64> {
        unsafe {
            let mut m: MaybeUninit<u64> = MaybeUninit::uninit();
            let info = GrB_Matrix_extractElement_UINT64(m.as_mut_ptr(), self.m, i, j);
            if info == GrB_Info::GrB_SUCCESS {
                Some(m.assume_init())
            } else {
                None
            }
        }
    }
}

impl Set<bool> for Matrix<bool> {
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: bool,
    ) {
        unsafe {
            let info = GrB_Matrix_setElement_BOOL(self.m, value, i, j);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl Set<u64> for Matrix<u64> {
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: u64,
    ) {
        unsafe {
            let info = GrB_Matrix_setElement_UINT64(self.m, value, i, j);
            debug_assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

pub struct Iter<T> {
    iter: GxB_Iterator,
    depleted: bool,
    max_row: u64,
    phantom: PhantomData<T>,
}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        unsafe {
            GxB_Iterator_free(addr_of_mut!(self.iter));
        }
    }
}

impl<T> Iter<T> {
    #[must_use]
    pub fn new(
        m: &Matrix<T>,
        min_row: u64,
        max_row: u64,
    ) -> Self {
        unsafe {
            let mut iter = MaybeUninit::uninit();
            GxB_Iterator_new(iter.as_mut_ptr());
            let iter = iter.assume_init();
            GxB_Matrix_Iterator_attach(iter, m.m, null_mut());
            let info = GxB_Matrix_Iterator_seek(iter, min_row);
            Self {
                iter,
                depleted: info == GrB_Info::GxB_EXHAUSTED,
                max_row,
                phantom: PhantomData,
            }
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
            if row > self.max_row {
                self.depleted = true;
                return None;
            }
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info::GxB_EXHAUSTED;
            Some((row, col))
        }
    }
}

impl Iterator for Iter<u64> {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.depleted {
            return None;
        }
        unsafe {
            let mut row = 0u64;
            let mut col = 0u64;
            let value = GxB_Iterator_get_UINT64(self.iter);
            GxB_Matrix_Iterator_getIndex(self.iter, addr_of_mut!(row), addr_of_mut!(col));
            if row > self.max_row {
                self.depleted = true;
                return None;
            }
            self.depleted = GxB_Matrix_Iterator_next(self.iter) == GrB_Info::GxB_EXHAUSTED;
            Some((row, col, value))
        }
    }
}
