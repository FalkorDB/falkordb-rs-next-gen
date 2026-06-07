//! The index subsystem's error type and result alias.

/// Crate-internal result alias for the index subsystem.
pub type Result<T> = core::result::Result<T, IndexError>;

/// Errors surfaced by the index subsystem. A scan-time error is **terminal**
/// for that scan (query-api §2):
/// already-yielded hits stay valid, the op turns it into a runtime error.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum IndexError {
    /// The durable backend failed (I/O, serialization, fault-in).
    #[error("index backend error: {0}")]
    Backend(String),
    /// A persisted blob failed its integrity check → the owning index falls
    /// back to a full rebuild (durability §8).
    #[error("index corruption: {0}")]
    Corrupt(String),
    /// The scan observed its cancellation flag
    /// ([`super::ScanOptions::cancel`]).
    #[error("index scan cancelled")]
    Cancelled,
    /// A query shape the active backend/kind does not implement yet.
    #[error("unsupported index operation: {0}")]
    Unsupported(&'static str),
}
