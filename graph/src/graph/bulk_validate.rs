//! Validation of `GRAPH.BULK` declared entity counts.
//!
//! Lives in the `graph` crate (rather than next to the command handler) so
//! that its unit tests run under `cargo test -p graph`; the root crate's
//! global allocator requires a live Redis and cannot host runnable tests.

/// Rejection of `GRAPH.BULK` declared entity counts that cannot possibly match
/// the accompanying payload. The counts drive up-front ID reservations (and a
/// `Vec` of that many IDs), so accepting an inflated count would allow a
/// capacity-overflow panic / OOM from a single malicious command.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BulkInsertCountError {
    /// The declared node count exceeds the node token payload size.
    #[error("Bulk insert format error, node count exceeds payload size.")]
    NodeCountExceedsPayload,
    /// The declared relation count exceeds the relation token payload size.
    #[error("Bulk insert format error, relation count exceeds payload size.")]
    EdgeCountExceedsPayload,
}

/// Validate declared entity counts against the payload sizes carrying them.
/// Each created entity needs at least one byte in its token payload, so a
/// declared count can never legitimately exceed the total payload size.
///
/// # Errors
///
/// Returns a [`BulkInsertCountError`] when either declared count exceeds the
/// corresponding payload size.
pub const fn validate_declared_counts(
    node_count: usize,
    edge_count: usize,
    node_payload_bytes: usize,
    edge_payload_bytes: usize,
) -> Result<(), BulkInsertCountError> {
    if node_count > node_payload_bytes {
        return Err(BulkInsertCountError::NodeCountExceedsPayload);
    }
    if edge_count > edge_payload_bytes {
        return Err(BulkInsertCountError::EdgeCountExceedsPayload);
    }
    Ok(())
}

#[cfg(test)]
mod count_validation_tests {
    use super::*;

    #[test]
    fn accepts_counts_within_payload() {
        assert_eq!(validate_declared_counts(10, 5, 100, 50), Ok(()));
        assert_eq!(validate_declared_counts(0, 0, 0, 0), Ok(()));
        // Exactly one byte per entity is the minimum legitimate payload.
        assert_eq!(validate_declared_counts(7, 3, 7, 3), Ok(()));
    }

    #[test]
    fn rejects_node_count_exceeding_payload() {
        assert_eq!(
            validate_declared_counts(101, 0, 100, 0),
            Err(BulkInsertCountError::NodeCountExceedsPayload)
        );
        // A huge declared count with an empty payload must not reach the
        // ID-reservation path (capacity overflow / OOM).
        assert_eq!(
            validate_declared_counts(usize::MAX, 0, 0, 0),
            Err(BulkInsertCountError::NodeCountExceedsPayload)
        );
    }

    #[test]
    fn rejects_edge_count_exceeding_payload() {
        assert_eq!(
            validate_declared_counts(0, 51, 0, 50),
            Err(BulkInsertCountError::EdgeCountExceedsPayload)
        );
        assert_eq!(
            validate_declared_counts(0, usize::MAX, 100, 0),
            Err(BulkInsertCountError::EdgeCountExceedsPayload)
        );
    }
}
