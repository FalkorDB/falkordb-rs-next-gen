//! Identifiers used across the index subsystem: the document key and the
//! structure / shard / snapshot keys that scope durable state.

/// A document key: a node id, **or an edge id alone** (not the
/// `[src, dst, edge_id]` triple). Uniform across node and edge indexes;
/// edge endpoints are recovered from the `edge_endpoints` mapping at the
/// scan-op boundary (mvcc-core §5).
pub type DocKey = u64;

/// Identifies one **structure** — a single (index, field, entity-kind) matrix.
/// `SnapKey`/`ShardId` are scoped beneath it
/// (durability §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructureId {
    /// The owning index's id (one index may declare several fields).
    pub index_id: u64,
    /// The indexed field, as a stable small id within the index.
    pub field: u32,
    /// `true` for an edge index, `false` for a node index.
    pub edge: bool,
}

/// Identifies one shard: a contiguous row-value range within a structure
/// (mvcc-core §8). Sharding is
/// the unit of residency and parallelism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShardId {
    /// The structure this shard belongs to.
    pub structure: StructureId,
    /// Shard ordinal within the structure.
    pub shard: u32,
}

/// Key for a base-snapshot blob: a shard at a given logical version
/// (durability §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnapKey {
    /// Which shard the blob is a base for.
    pub shard: ShardId,
    /// `V_base` — the logical version the base is complete as of.
    pub version: u64,
}
