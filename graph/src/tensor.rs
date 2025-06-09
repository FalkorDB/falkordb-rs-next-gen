use std::{ffi::c_void, sync::Once};

use crate::{
    GraphBLAS::GrB_Vector,
    matrix::{self, Dup, ElementWiseAdd, Matrix, New, Remove, Set, Size, Transpose, UnaryOp},
    vector::{self, Vector},
};

#[allow(non_upper_case_globals)]
const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

pub struct Tensor {
    m: Matrix<u64>,
}

macro_rules! single_edge {
    ($current_edge:expr) => {
        $current_edge as u64 & (1u64 << (u64::BITS as usize - 1)) == 0
    };
    () => {};
}

macro_rules! set_msb {
    ($meid:expr) => {
        $meid as u64 | (1u64 << (u64::BITS as usize - 1))
    };
    () => {};
}

macro_rules! clear_msb {
    ($meid:expr) => {
        $meid as u64 & !(1u64 << (u64::BITS as usize - 1))
    };
    () => {};
}

static INIT: Once = Once::new();
static mut FREE_UNARYOP: UnaryOp<u64> = UnaryOp::default();
static mut DUP_UNARYOP: UnaryOp<u64> = UnaryOp::default();

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "C" fn _free_vectors(
    _z: *mut c_void,
    x: *const c_void,
) {
    let x = *(x as *const u64);
    if !single_edge!(x) {
        drop(Vector::from(clear_msb!(x) as GrB_Vector));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "C" fn _dup_vectors(
    z: *mut c_void,
    x: *const c_void,
) {
    let x = *(x as *const u64);
    if !single_edge!(x) {
        // let v = Vector::from(clear_msb!(x) as GrB_Vector);
        // z
    }
}

#[allow(static_mut_refs)]
impl Drop for Tensor {
    fn drop(&mut self) {
        unsafe {
            INIT.call_once(|| {
                FREE_UNARYOP.set(Some(_free_vectors));
                DUP_UNARYOP.set(Some(_dup_vectors));
            });

            self.m.apply(&FREE_UNARYOP);
        }
    }
}

impl New for Tensor {
    fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: Matrix::<u64>::new(nrows, ncols),
        }
    }
}

impl ElementWiseAdd<u64> for Tensor {
    fn element_wise_add(
        &mut self,
        other: &Self,
    ) {
        self.m.element_wise_add(&other.m);
    }
}

impl Tensor {
    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        // if let Some(current_edge) = self.m.get(src, dest) {
        //     let mut v = if single_edge!(current_edge) {
        //         let mut v = Vector::new(GrB_INDEX_MAX);
        //         self.m.set(src, dest, set_msb!(v.ptr()));
        //         v.set(current_edge, true);
        //         v
        //     } else {
        //         Vector::from(clear_msb!(current_edge) as GrB_Vector)
        //     };
        //     v.set(id, true);
        //     v.wait();
        // } else {
        self.m.set(src, dest, id);
        // }
    }

    pub fn remove(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        // if let Some(current_edge) = self.m.get(src, dest) {
        //     if single_edge!(current_edge) {
        //         let mut v = Vector::from(clear_msb!(current_edge) as GrB_Vector);
        //         v.set(id, false);
        //         v.wait();
        //     } else {
        self.m.remove(src, dest);
        //     }
        // }
    }

    pub fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.m.resize(nrows, ncols);
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        Self {
            m: self.m.transpose(),
        }
    }

    #[allow(static_mut_refs)]
    #[must_use]
    pub fn dup(&self) -> Self {
        unsafe {
            INIT.call_once(|| {
                FREE_UNARYOP.set(Some(_free_vectors));
                DUP_UNARYOP.set(Some(_dup_vectors));
            });
        }
        let mut m = self.m.dup();
        m.apply(unsafe { &DUP_UNARYOP });
        Self { m }
    }

    #[must_use]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
    ) -> Iter {
        Iter::new(self, min_row, max_row)
    }

    pub fn wait(&self) {
        self.m.wait();
    }
}

pub struct Iter {
    mit: matrix::Iter<u64>,
    vit: Option<vector::Iter<bool>>,
    src: u64,
    dest: u64,
}

impl Iter {
    fn new(
        m: &Tensor,
        min_row: u64,
        max_row: u64,
    ) -> Self {
        Self {
            mit: m.m.iter(min_row, max_row),
            vit: None,
            src: 0,
            dest: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vit) = &mut self.vit {
            if let Some(id) = vit.next() {
                return Some((self.src, self.dest, id));
            }
            self.vit = None;
        }

        if let Some((src, dest, id)) = self.mit.next() {
            if single_edge!(id) {
                return Some((src, dest, id));
            }
            self.src = src;
            self.dest = dest;
            let v = Vector::from(clear_msb!(id) as GrB_Vector);
            self.vit = Some(v.iter());
            return self.next();
        }

        None
    }
}
