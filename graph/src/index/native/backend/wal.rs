//! Durability-layer value types: the matrix [`Cell`], the metadata
//! [`Keyspace`]s, and the append-only [`WalRecord`] / [`WalOp`] shapes
//! (durability §2.2, §6.1).

use crate::index::native::id::DocKey;

/// A matrix cell value: membership (`bool`) or a packed scalar (`u64`).
/// Anything richer lives in the payload store
/// (mvcc-core §2, §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Membership posting — all the numeric POC needs.
    Bool(bool),
    /// A packed scalar payload that still fits in a cell.
    U64(u64),
}

/// Metadata keyspaces in the backend's plain-KV area
/// (durability §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyspace {
    /// `index_version`, per-shard manifest, schema-adjacent metadata.
    Meta,
    /// Doc-keyed sidecars that are plain KV (e.g. edge-endpoints overflow).
    Sidecar,
}

/// One append-only, version-tagged WAL record
/// (durability §2.2). The WAL
/// is per-shard, so the record carries no shard field.
#[derive(Debug, Clone)]
pub struct WalRecord {
    /// The graph commit version — the log position; there is no LSN of our own
    /// (mvcc-core §4).
    pub version: u64,
    /// The mutation.
    pub op: WalOp,
}

/// The mutations a WAL record can carry
/// (durability §2.2).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WalOp {
    /// Add/overwrite a posting at `(row, doc)`.
    SetCell { row: u64, doc: DocKey, cell: Cell },
    /// Tombstone every row of `doc` within this shard.
    RemoveDoc { doc: DocKey },
    /// Tombstone one posting at `(row, doc)`.
    RemoveCell { row: u64, doc: DocKey },
    /// Payload-store overlay at this version (later kinds).
    PutPayload { doc: DocKey, blob: Vec<u8> },
    /// Tombstone a doc's payload (later kinds).
    TombstonePayload { doc: DocKey },
}
