//! Memory accounting for `CALL db.indexes()` and the resident-pool budget
//! (durability §6.3).

/// Resident-byte breakdown reported by an index.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MemoryBreakdown {
    /// Bytes held by resident matrix shards.
    pub matrix_bytes: usize,
    /// Bytes held by resident payload-store entries.
    pub payload_bytes: usize,
}

impl MemoryBreakdown {
    /// Total resident bytes.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.matrix_bytes + self.payload_bytes
    }
}
