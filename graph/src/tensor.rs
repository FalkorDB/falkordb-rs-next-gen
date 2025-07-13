use crate::matrix::{self, Dup, ElementWiseAdd, Get, Matrix, New, Remove, Set, Size};

#[allow(non_upper_case_globals)]
const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

pub struct Tensor {
    m: Matrix<bool>,
    mt: Matrix<bool>,
    me: Matrix<bool>,
}

impl New for Tensor {
    fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: Matrix::<bool>::new(nrows, ncols),
            mt: Matrix::<bool>::new(ncols, nrows),
            me: Matrix::<bool>::new(GrB_INDEX_MAX, GrB_INDEX_MAX),
        }
    }
}

impl ElementWiseAdd<u64> for Tensor {
    fn element_wise_add(
        &mut self,
        other: &Self,
    ) {
        self.m.element_wise_add(&other.m);
        self.me.element_wise_add(&other.me);
    }
}

impl Tensor {
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> Vec<u64> {
        if self.m.get(src, dest).is_some() {
            let row = src << 32 | dest;
            self.me.iter(row, row).map(|(_, j)| j).collect()
        } else {
            vec![]
        }
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        self.m.set(src, dest, true);
        self.mt.set(dest, src, true);
        self.me.set(src << 32 | dest, id, true);
    }

    pub fn remove(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        self.me.remove(src << 32 | dest, id);
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
    pub fn dup_bool(&self) -> Matrix<bool> {
        self.m.dup()
    }

    #[must_use]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Iter {
        Iter::new(self, min_row, max_row, transpose)
    }

    pub fn wait(&self) {
        self.m.wait();
        self.mt.wait();
        self.me.wait();
    }
}

pub struct Iter<'a> {
    t: &'a Tensor,
    mit: matrix::Iter<bool>,
    vit: Option<matrix::Iter<bool>>,
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
            let row = self.src << 32 | self.dest;
            self.vit = Some(self.t.me.iter(row, row));
            return self.next();
        }

        None
    }
}
