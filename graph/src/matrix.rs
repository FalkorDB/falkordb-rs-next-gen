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

/// Initializes the GraphBLAS library in non-blocking mode.
pub fn init() {
    unsafe {
        GrB_init(GrB_Mode::GrB_NONBLOCKING as _);
    }
}

/// Finalizes the GraphBLAS library, releasing all resources.
pub fn shutdown() {
    unsafe {
        GrB_finalize();
    }
}

/// A trait for querying and modifying the size of a matrix.
pub trait Size<T> {
    /// Returns the number of rows in the matrix.
    fn nrows(&self) -> u64;

    /// Returns the number of columns in the matrix.
    fn ncols(&self) -> u64;

    /// Resizes the matrix to the specified number of rows and columns.
    ///
    /// # Parameters
    /// - `nrows`: The new number of rows.
    /// - `ncols`: The new number of columns.
    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    );
}

/// A trait for retrieving elements from a matrix.
pub trait Get<T> {
    /// Retrieves the element at the specified row and column.
    /// Returns `None` if the element does not exist.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    ///
    /// # Returns
    /// - `Some(T)`: The element at the specified position.
    /// - `None`: The element does not exist.
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<T>;
}

/// A trait for setting elements in a matrix.
pub trait Set<T> {
    /// Sets the element at the specified row and column to the given value.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    /// - `value`: The value to set.
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: T,
    );
}

/// A trait for deleting elements from a matrix.
pub trait Delete<T> {
    /// Deletes the element at the specified row and column.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    fn delete(
        &mut self,
        i: u64,
        j: u64,
    );
}

/// A wrapper around a GraphBLAS matrix with type safety for elements.
pub struct Matrix<T> {
    /// The underlying GraphBLAS matrix.
    m: Rc<GrB_Matrix>,
    /// Phantom data to associate the matrix with a specific type.
    phantom: PhantomData<T>,
}

/// Represents a specific row in a matrix.
pub struct Row<T> {
    /// The row index.
    row: u64,
    /// Phantom data to associate the row with a specific type.
    phantom: PhantomData<T>,
}

/// An iterator for traversing elements in a matrix.
pub struct Iter<T> {
    /// The underlying GraphBLAS iterator.
    iter: GxB_Iterator,
    /// Indicates whether the iterator is depleted.
    depleted: bool,
    /// The data associated with the iterator.
    data: T,
}

impl<T> Drop for Iter<T> {
    /// Frees the GraphBLAS iterator when the `Iter` is dropped.
    fn drop(&mut self) {
        unsafe {
            GxB_Iterator_free(addr_of_mut!(self.iter));
        }
    }
}

impl Iter<bool> {
    /// Creates a new iterator for traversing all elements in a boolean matrix.
    ///
    /// # Parameters
    /// - `m`: The matrix to iterate over.
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
    /// Creates a new iterator for traversing elements in a specific row of a matrix.
    ///
    /// # Parameters
    /// - `m`: The matrix to iterate over.
    /// - `row`: The row index.
    #[must_use]
    pub fn new(
        m: &Matrix<T>,
        row: u64,
    ) -> Self {
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

    /// Advances the iterator and returns the next element in the row.
    ///
    /// # Returns
    /// - `Some((u64, u64))`: The next element in the row.
    /// - `None`: The iterator is depleted.
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

    /// Advances the iterator and returns the next element in the matrix.
    ///
    /// # Returns
    /// - `Some((u64, u64))`: The next element in the matrix.
    /// - `None`: The iterator is depleted.
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
    /// Creates a new `Matrix` instance that shares ownership of the underlying GraphBLAS matrix.
    ///
    /// # Returns
    /// - `Matrix<T>`: A cloned `Matrix` instance.
    #[must_use]
    fn clone(&self) -> Self {
        Self {
            m: self.m.clone(),
            phantom: self.phantom,
        }
    }
}

impl<T> Drop for Matrix<T> {
    /// Frees the GraphBLAS matrix when the `Matrix` is dropped, if it is no longer shared.
    fn drop(&mut self) {
        unsafe {
            Rc::get_mut(&mut self.m).map(|m| GrB_Matrix_free(addr_of_mut!(*m)));
        }
    }
}

impl<T> Matrix<T> {
    /// Creates an iterator for traversing elements in a specific row of the matrix.
    ///
    /// # Parameters
    /// - `row`: The row index.
    ///
    /// # Returns
    /// - `Iter<Row<T>>`: An iterator for the specified row.
    #[must_use]
    pub fn iter_row(
        &self,
        row: u64,
    ) -> Iter<Row<T>> {
        Iter::<Row<T>>::new(self, row)
    }
}

impl<T> Size<T> for Matrix<T> {
    /// Returns the number of rows in the matrix.
    ///
    /// # Returns
    /// - `u64`: The number of rows in the matrix.
    #[must_use]
    fn nrows(&self) -> u64 {
        unsafe {
            let mut nrows = 0u64;
            let info = GrB_Matrix_nrows(addr_of_mut!(nrows), *self.m);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
            nrows
        }
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Returns
    /// - `u64`: The number of columns in the matrix.
    #[must_use]
    fn ncols(&self) -> u64 {
        unsafe {
            let mut ncols = 0u64;
            let info = GrB_Matrix_ncols(addr_of_mut!(ncols), *self.m);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
            ncols
        }
    }

    /// Resizes the matrix to the specified dimensions.
    ///
    /// # Parameters
    /// - `nrows`: The new number of rows.
    /// - `ncols`: The new number of columns.
    fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        unsafe {
            let info = GrB_Matrix_resize(*self.m, nrows, ncols);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl Matrix<bool> {
    /// Creates a new boolean matrix with the specified dimensions.
    ///
    /// # Parameters
    /// - `nrows`: The number of rows.
    /// - `ncols`: The number of columns.
    ///
    /// # Returns
    /// - `Matrix<bool>`: A new boolean matrix.
    #[must_use]
    pub fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        unsafe {
            let mut m: MaybeUninit<GrB_Matrix> = MaybeUninit::uninit();
            GrB_Matrix_new(m.as_mut_ptr(), GrB_BOOL, nrows, ncols);
            Self {
                m: Rc::new(m.assume_init()),
                phantom: PhantomData,
            }
        }
    }

    /// Creates an iterator for traversing all elements in the matrix.
    ///
    /// # Returns
    /// - `Iter<bool>`: An iterator for the matrix.
    #[must_use]
    pub fn iter(&self) -> Iter<bool> {
        Iter::<bool>::new(self)
    }
}

impl<T> Delete<T> for Matrix<T> {
    /// Deletes the element at the specified position in the matrix.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    fn delete(
        &mut self,
        i: u64,
        j: u64,
    ) {
        unsafe {
            let info = GrB_Matrix_removeElement(*self.m, i, j);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}

impl Get<bool> for Matrix<bool> {
    /// Retrieves the boolean value at the specified position in the matrix.
    /// Returns `None` if the element does not exist.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    ///
    /// # Returns
    /// - `Some(bool)`: The boolean value at the specified position.
    /// - `None`: The element does not exist.
    #[must_use]
    fn get(
        &self,
        i: u64,
        j: u64,
    ) -> Option<bool> {
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
    /// Sets the boolean value at the specified position in the matrix.
    ///
    /// # Parameters
    /// - `i`: The row index.
    /// - `j`: The column index.
    /// - `value`: The value to set.
    fn set(
        &mut self,
        i: u64,
        j: u64,
        value: bool,
    ) {
        unsafe {
            let info = GrB_Matrix_setElement_BOOL(*self.m, value, i, j);
            assert_eq!(info, GrB_Info::GrB_SUCCESS);
        }
    }
}
