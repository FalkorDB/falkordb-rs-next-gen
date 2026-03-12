//! Per-query object pool for `Vec<T>` buffers.
//!
//! [`Pool`] recycles `Vec<T>` buffers to amortize allocation cost across
//! millions of `Env` clones that occur during scan and traversal operators.
//!
//! ```text
//!  Pool lifecycle during a query
//!
//!  acquire()        ┌──────────┐
//!  ─────────────►   │ Pooled   │   (RAII wrapper)
//!                   │ Vec<T>   │
//!                   └────┬─────┘
//!                        │ Drop
//!  release()             ▼
//!  ◄─────────────   free_vecs.push(vec)
//!
//!  Next acquire() pops from free_vecs instead of allocating.
//! ```
//!
//! The pool uses `RefCell` for interior mutability, which is safe because
//! each query runs on a single thread. A fresh pool is created per query
//! in `graph_core.rs` / `record.rs`.

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// Per-query object pool for `Vec<T>` buffers.
///
/// Reuses previously allocated `Vec`s to amortise allocation cost
/// across millions of clones in scan/traversal loops.
pub struct Pool<T> {
    free_vecs: RefCell<Vec<Vec<T>>>,
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Pool<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            free_vecs: RefCell::new(Vec::new()),
        }
    }

    /// Acquire a raw `Vec<T>` from the pool (or allocate if empty).
    /// Prefer [`acquire`](Self::acquire) which returns a [`Pooled`] handle.
    pub(crate) fn acquire_raw(
        &self,
        capacity: usize,
    ) -> Vec<T> {
        let mut free = self.free_vecs.borrow_mut();
        free.pop().map_or_else(
            || Vec::with_capacity(capacity),
            |mut v| {
                v.clear();
                if v.capacity() < capacity {
                    v.reserve(capacity - v.capacity());
                }
                v
            },
        )
    }

    /// Acquire a [`Pooled`] handle that automatically returns the buffer on drop.
    pub fn acquire(
        &self,
        capacity: usize,
    ) -> Pooled<'_, T> {
        Pooled {
            value: self.acquire_raw(capacity),
            pool: self,
        }
    }

    /// Return a `Vec<T>` to the pool for later reuse.
    pub fn release(
        &self,
        v: Vec<T>,
    ) {
        self.free_vecs.borrow_mut().push(v);
    }
}

/// RAII wrapper around a pooled `Vec<T>`.
///
/// When dropped, the buffer is automatically returned to the originating
/// [`Pool`].
pub struct Pooled<'a, T> {
    value: Vec<T>,
    pool: &'a Pool<T>,
}

impl<T> Drop for Pooled<'_, T> {
    fn drop(&mut self) {
        let v = std::mem::take(&mut self.value);
        self.pool.release(v);
    }
}

impl<T> Deref for Pooled<'_, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Pooled<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
