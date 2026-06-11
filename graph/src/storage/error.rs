//! The shared storage seam's error type.
//!
//! Hand-rolled rather than `thiserror`-derived: this module is ungated core
//! infrastructure, while `thiserror` is an optional dependency pulled in only by
//! the `index-falkordb` feature — so the shared seam cannot depend on it without
//! enlarging the default dependency set. The index subsystem keeps its own,
//! richer `thiserror` error ([`crate::index::falkordb::IndexError`]) and converts
//! at the boundary.

use std::fmt;

/// Result alias for the shared storage seam.
pub type Result<T> = core::result::Result<T, StorageError>;

/// Errors surfaced by the shared storage seam (residency / backend registry).
#[derive(Debug)]
#[non_exhaustive]
pub enum StorageError {
    /// A durable backend operation failed (I/O, serialization, fault-in).
    Backend(String),
    /// The resident pool could not satisfy a request (e.g. admission would
    /// exceed the budget and nothing is evictable).
    Residency(String),
}

impl fmt::Display for StorageError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Backend(m) => write!(f, "storage backend error: {m}"),
            Self::Residency(m) => write!(f, "residency error: {m}"),
        }
    }
}

impl std::error::Error for StorageError {}
