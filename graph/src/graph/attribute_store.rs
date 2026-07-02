//! Attribute storage for graph entities (nodes and relationships).
//!
//! This module provides [`AttributeStore`], a **columnar** in-memory store: each
//! attribute is a separate column, and a column stores dense `Value`s keyed
//! directly by entity id (a `Null` slot means "absent"). This drops the two
//! per-entity overheads of an entity-major layout — the per-entity pointer and a
//! repeated `attr_id` next to every value — so only the `Value`s are stored.
//!
//! ## Read Path
//!
//! `get_attr_by_idx` indexes straight into one column (`O(1)`), and the batch
//! path (`get_attrs_by_idx_batch_into`) is a single contiguous column walk.
//! Per-entity materialization (`get_all_attrs*`) gathers across columns.
//!
//! ## MVCC Integration
//!
//! Snapshot isolation is provided by **copy-on-write** at chunk granularity.
//! Each column is a vector of `Arc<Chunk>` blocks; `new_version()` clones the
//! store cheaply (the parent keeps its `Arc`s), and the first write to a chunk
//! in a new version calls `Arc::make_mut`, cloning only that chunk. A reader on
//! an older version keeps the original `Arc`, so it never observes a writer's
//! changes — and rollback is simply dropping the discarded version. No locks,
//! no per-entry version stamps, and no `unsafe`.

use std::sync::Arc;

use rustc_hash::FxHashMap;

use roaring::RoaringTreemap;

use super::graphblas::serialization::{Decode, Encode, Reader, Writer};
use crate::runtime::{
    ordermap::OrderMap,
    value::{Point, Value},
};

/// Insertion-ordered map of attribute names to attribute indices.
///
/// Maintains both a `Vec<Arc<String>>` (for stable index → name lookup and
/// deterministic iteration order) and a `FxHashMap<Arc<String>, u16>` for
/// O(1) name → index resolution on the hot read path.
#[derive(Default, Clone)]
pub struct AttrNameMap {
    vec: Vec<Arc<String>>,
    index: FxHashMap<Arc<String>, u16>,
}

impl AttrNameMap {
    #[must_use]
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[must_use]
    pub fn get(
        &self,
        idx: usize,
    ) -> Option<&Arc<String>> {
        self.vec.get(idx)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Arc<String>> {
        self.vec.iter()
    }

    #[must_use]
    pub fn get_index_of(
        &self,
        name: &Arc<String>,
    ) -> Option<usize> {
        self.index.get(name).map(|&i| i as usize)
    }

    pub fn insert(
        &mut self,
        name: Arc<String>,
    ) {
        if self.index.contains_key(&name) {
            return;
        }
        // Column indices are `u16`; more than `u16::MAX` distinct names would
        // silently alias two attributes to one column, so fail loudly instead.
        assert!(
            self.vec.len() < u16::MAX as usize,
            "attribute schema exceeded {} distinct names",
            u16::MAX
        );
        let idx = self.vec.len() as u16;
        self.vec.push(name.clone());
        self.index.insert(name, idx);
    }
}

impl std::ops::Index<usize> for AttrNameMap {
    type Output = Arc<String>;

    fn index(
        &self,
        idx: usize,
    ) -> &Arc<String> {
        &self.vec[idx]
    }
}

impl<'a> IntoIterator for &'a AttrNameMap {
    type Item = &'a Arc<String>;
    type IntoIter = std::slice::Iter<'a, Arc<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

/// Columnar attribute storage for graph entities.
///
/// Uses a shared [`AttributeStorage`] as the sole, in-memory source of truth for
/// entity attributes.  A cache miss means the entity has no attributes.
#[derive(Clone)]
pub struct AttributeStore {
    /// Attribute names in insertion order (name → column index)
    pub attrs_name: AttrNameMap,
    /// Columnar storage. Owned per version; columns share `Arc<Chunk>` blocks
    /// with the parent version until a write copies a chunk (copy-on-write).
    storage: AttributeStorage,
    /// MVCC version of this store's snapshot (informational; visibility is
    /// enforced structurally by copy-on-write, not by version comparison).
    version: u64,
}

impl AttributeStore {
    #[must_use]
    pub fn new(version: u64) -> Self {
        Self {
            attrs_name: AttrNameMap::default(),
            storage: AttributeStorage::new(),
            version,
        }
    }

    #[must_use]
    pub fn new_version(
        &self,
        version: u64,
    ) -> Self {
        // Copy-on-write: the clone shares `Arc<Chunk>` blocks with `self`; the
        // first write to any chunk in the new version clones just that chunk.
        Self {
            attrs_name: self.attrs_name.clone(),
            storage: self.storage.clone(),
            version,
        }
    }

    // ---- helpers --------------------------------------------------------

    #[must_use]
    pub fn get_attr(
        &self,
        key: u64,
        attr: &Arc<String>,
    ) -> Option<Value> {
        let idx = self.attrs_name.get_index_of(attr)? as u16;
        self.get_attr_by_idx(key, idx)
    }

    #[must_use]
    pub fn get_attr_by_idx(
        &self,
        key: u64,
        attr_idx: u16,
    ) -> Option<Value> {
        // A miss (outer or inner `None`) means the entity has no such attribute.
        self.storage.get_attr(key, attr_idx, self.version).flatten()
    }

    /// Batch variant of `get_attr_by_idx` for a list of keys with the same
    /// `attr_idx`. A single contiguous column walk; each absent key carries
    /// `default`.
    pub fn get_attrs_by_idx_batch_into(
        &self,
        keys: &[u64],
        attr_idx: u16,
        default: &Value,
        out: &mut Vec<Value>,
    ) {
        out.reserve(keys.len());
        let mut missing: Vec<usize> = Vec::new();
        self.storage
            .get_attrs_batch_into(keys, attr_idx, self.version, default, out, &mut missing);
    }

    #[must_use]
    pub fn has_attributes(
        &self,
        key: u64,
    ) -> bool {
        self.storage.has_entity(key, self.version).unwrap_or(false)
    }

    #[must_use]
    pub fn get_attrs(
        &self,
        key: u64,
    ) -> AttrArrayView<'_> {
        AttrArrayView {
            store: self,
            entity_id: key,
        }
    }

    // ---- write path (cache only) ----------------------------------------

    pub fn remove_attr(
        &mut self,
        key: u64,
        attr: &Arc<String>,
    ) -> Result<bool, String> {
        if let Some(idx) = self.attrs_name.get_index_of(attr) {
            return Ok(self.storage.clear(key, idx as u16));
        }
        Ok(false)
    }

    pub fn remove_all(
        &mut self,
        keys: &RoaringTreemap,
    ) {
        // Clear every column at these ids immediately. Copy-on-write means this
        // only touches this version's chunks; a failed transaction is discarded
        // by dropping the version, so no deferred delete bookkeeping is needed.
        self.storage.invalidate_batch(keys);
    }

    /// Batch insert/update multiple attributes for entities.
    ///
    /// Non-null values are written to their column; nulls clear the column at
    /// that entity. Returns `(nremoved, nset)`: the number of previously-present
    /// attributes overwritten or cleared, and the number of non-null values set.
    pub fn insert_attrs(
        &mut self,
        attrs: &FxHashMap<u64, OrderMap<Arc<String>, Value>>,
    ) -> Result<(usize, usize), String> {
        let mut nremoved = 0;
        let mut nset = 0;

        // Pre-resolve all unique attribute names → indices ONCE.
        // Uses Arc pointer identity as key to avoid rehashing strings.
        let mut name_to_idx: FxHashMap<*const String, u16> = FxHashMap::default();
        for entity_attrs in attrs.values() {
            for (attr, _) in entity_attrs.iter() {
                let ptr = Arc::as_ptr(attr);
                if let std::collections::hash_map::Entry::Vacant(e) = name_to_idx.entry(ptr) {
                    let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
                        self.attrs_name.insert(attr.clone());
                        self.attrs_name.len() - 1
                    }) as u16;
                    e.insert(idx);
                }
            }
        }

        for (key, entity_attrs) in attrs {
            for (attr, value) in entity_attrs.iter() {
                let idx = name_to_idx[&Arc::as_ptr(attr)];
                if matches!(value, Value::Null) {
                    // Clearing an absent attribute is a no-op; count real removals.
                    if self.storage.clear(*key, idx) {
                        nremoved += 1;
                    }
                } else {
                    // Overwriting an existing value counts as a replacement.
                    if self.storage.set(*key, idx, value.clone()) {
                        nremoved += 1;
                    }
                    nset += 1;
                }
            }
        }

        Ok((nremoved, nset))
    }

    /// Bulk import attributes for entities known to be new (no prior state).
    ///
    /// Optimized for RDB decode: writes each non-null value directly to its
    /// column. Returns the number of non-null attributes imported.
    pub fn import_attrs(
        &mut self,
        attrs: &FxHashMap<u64, OrderMap<Arc<String>, Value>>,
    ) -> usize {
        // Pre-resolve all unique attribute names → indices ONCE.
        let mut name_to_idx: FxHashMap<*const String, u16> = FxHashMap::default();
        for entity_attrs in attrs.values() {
            for (attr, _) in entity_attrs.iter() {
                let ptr = Arc::as_ptr(attr);
                if let std::collections::hash_map::Entry::Vacant(e) = name_to_idx.entry(ptr) {
                    let idx = self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
                        self.attrs_name.insert(attr.clone());
                        self.attrs_name.len() - 1
                    }) as u16;
                    e.insert(idx);
                }
            }
        }

        let mut nset = 0;
        for (key, entity_attrs) in attrs {
            for (attr, value) in entity_attrs.iter() {
                if matches!(value, Value::Null) {
                    continue;
                }
                let idx = name_to_idx[&Arc::as_ptr(attr)];
                self.storage.set(*key, idx, value.clone());
                nset += 1;
            }
        }
        nset
    }

    /// Import pre-resolved attribute data directly into the columns.
    /// Skips name resolution and OrderMap construction; used by bulk insert.
    pub fn import_attrs_resolved(
        &mut self,
        data: &mut Vec<(u64, Vec<(u16, Value)>)>,
    ) -> usize {
        let mut nset = 0;
        for (entity_id, entries) in data.drain(..) {
            for (idx, value) in entries {
                self.storage.set(entity_id, idx, value);
                nset += 1;
            }
        }
        nset
    }

    /// Resolve an attribute name to its index, creating a new mapping if needed.
    pub fn get_or_create_attr_id(
        &mut self,
        attr: &Arc<String>,
    ) -> u16 {
        self.attrs_name.get_index_of(attr).unwrap_or_else(|| {
            self.attrs_name.insert(attr.clone());
            self.attrs_name.len() - 1
        }) as u16
    }

    #[must_use]
    pub fn get_attr_id(
        &self,
        attr: &Arc<String>,
    ) -> Option<usize> {
        self.attrs_name.get_index_of(attr)
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.storage.memory_usage()
    }

    /// Structural slot-storage overhead, excluding attribute payload heap.
    #[must_use]
    pub fn structural_memory_usage(&self) -> usize {
        self.storage.structural_memory_usage()
    }

    /// Encode a range of entities, reading attributes from the in-memory cache.
    pub fn encode_with_range(
        &self,
        w: &mut dyn Writer,
        deleted: &RoaringTreemap,
        max_id: u64,
        global_attrs: &[Arc<String>],
        count: u64,
        offset: u64,
    ) {
        // Build attr remap inline.
        let global_index: FxHashMap<&Arc<String>, usize> = global_attrs
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect();

        let mut remap = vec![u16::MAX; self.attrs_name.len()];
        for (local_id, local_name) in self.attrs_name.iter().enumerate() {
            if let Some(&global_id) = global_index.get(local_name) {
                remap[local_id] = global_id as u16;
            }
        }

        let mut skipped = 0u64;
        let mut encoded = 0u64;

        for id in 0..=max_id {
            if deleted.contains(id) {
                continue;
            }
            if skipped < offset {
                skipped += 1;
                continue;
            }

            w.write_unsigned(id);

            // Two cheap column scans (count, then encode) avoid allocating a
            // per-entity snapshot for every entity in the graph.
            let mut n_props = 0u64;
            self.storage.for_each_attr(id, |_, _| n_props += 1);
            w.write_unsigned(n_props);
            self.storage.for_each_attr(id, |local_attr_id, value| {
                let global_attr_id = if (local_attr_id as usize) < remap.len() {
                    remap[local_attr_id as usize]
                } else {
                    local_attr_id
                };
                w.write_unsigned(global_attr_id as u64);
                value.encode(w);
            });

            encoded += 1;
            if encoded >= count {
                break;
            }
        }
    }
}

// `AttributeStore` is automatically `Send + Sync`: every field is `Send + Sync`
// (columns are plain `Vec`/`Arc<Chunk>` over `Send + Sync` `Value`s), so no
// manual impl — and no `unsafe` — is needed here.

impl Decode<19> for AttributeStore {
    fn decode(_r: &mut dyn Reader) -> Result<Self, String> {
        unimplemented!("use decode_with_count for AttributeStore")
    }

    fn decode_with_count(
        &mut self,
        r: &mut dyn Reader,
        count: u64,
    ) -> Result<(), String> {
        for _ in 0..count {
            let entity_id = r.read_unsigned()?;
            let attr_count = r.read_unsigned()?;

            for _ in 0..attr_count {
                let attr_id = r.read_unsigned()? as u16;
                let value = Value::decode(r)?;

                if (attr_id as usize) < self.attrs_name.len() && !matches!(value, Value::Null) {
                    self.storage.set(entity_id, attr_id, value);
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Columnar storage engine
// ============================================================================

/// Borrowing view of one entity's attributes, gathered lazily from the columns.
///
/// Returned by the per-entity read APIs (`get_all_attrs`). Holds no owned
/// data and clones no `Value`s: `iter()` walks the columns on demand in ascending
/// attribute-index order, so the view must be consumed while the store borrow is
/// alive. A prop-less entity simply yields an empty iterator.
pub struct AttrArrayView<'a> {
    store: &'a AttributeStore,
    entity_id: u64,
}

impl<'a> AttrArrayView<'a> {
    /// Number of present attributes for this entity.
    #[must_use]
    pub fn len(&self) -> usize {
        let (pidx, cin, slot) = decompose_id(self.entity_id);
        self.store
            .storage
            .columns
            .iter()
            .filter(|c| c.get_at(pidx, cin, slot).is_some())
            .count()
    }

    /// Whether the entity has no attributes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        let (pidx, cin, slot) = decompose_id(self.entity_id);
        !self
            .store
            .storage
            .columns
            .iter()
            .any(|c| c.get_at(pidx, cin, slot).is_some())
    }

    /// Iterate `(attr_idx, value)` pairs in ascending index order. Mixed chunks
    /// yield a borrow; packed scalar chunks materialize a cheap owned value.
    pub fn iter(&self) -> impl Iterator<Item = (u16, ValueRef<'a>)> {
        let store = self.store;
        let (pidx, cin, slot) = decompose_id(self.entity_id);
        store
            .storage
            .columns
            .iter()
            .enumerate()
            .filter_map(move |(i, col)| col.get_at(pidx, cin, slot).map(|v| (i as u16, v)))
    }

    /// Iterate `(&attr_name, value)` pairs in ascending index order. Like
    /// [`iter`](Self::iter) but resolves each column index to its attribute name.
    pub fn iter_named(&self) -> impl Iterator<Item = (&'a Arc<String>, ValueRef<'a>)> {
        let store = self.store;
        let (pidx, cin, slot) = decompose_id(self.entity_id);
        store
            .storage
            .columns
            .iter()
            .enumerate()
            .filter_map(move |(i, col)| {
                col.get_at(pidx, cin, slot)
                    .and_then(|v| store.attrs_name.get(i).map(|n| (n, v)))
            })
    }

    /// Materialize an owned `Vec` of `(name, value)` pairs. For callers that need
    /// the attributes to outlive the store borrow.
    #[must_use]
    pub fn to_pairs(&self) -> Vec<(Arc<String>, Value)> {
        self.iter_named()
            .map(|(n, v)| (n.clone(), v.into_owned()))
            .collect()
    }
}

/// Decompose an entity id into `(page index, chunk-in-page, value slot)` for
/// direct column addressing.
#[inline]
fn decompose_id(id: u64) -> (usize, usize, usize) {
    let cid = (id >> CHUNK_SHIFT) as usize;
    (
        cid >> PAGE_SHIFT,
        cid & PAGE_MASK,
        (id & CHUNK_MASK) as usize,
    )
}

/// Number of consecutive entity ids whose values live in one [`Chunk`].
/// A power of two so the id split is shift/mask. Copy-on-write clones at this
/// granularity, bounding write amplification (a single-value write copies at
/// most `CHUNK_LEN` values) while amortizing the per-chunk `Arc` overhead.
const CHUNK_LEN: usize = 128;
const CHUNK_SHIFT: u32 = CHUNK_LEN.trailing_zeros();
const CHUNK_MASK: u64 = CHUNK_LEN as u64 - 1;

/// Chunk-pointers per [`Page`]. The chunk directory is paged so a write
/// copies at most one page of pointers (`PAGE_LEN`) plus the touched chunk,
/// instead of the whole directory — bounding per-write copy cost for large
/// graphs.
const PAGE_LEN: usize = 64;
const PAGE_SHIFT: u32 = PAGE_LEN.trailing_zeros();
const PAGE_MASK: usize = PAGE_LEN - 1;

/// A borrowed or owned handle to a stored [`Value`].
///
/// Mixed chunks store real `Value`s and hand out a borrow (`Borrowed`); packed
/// scalar chunks keep no `Value` in memory, so they materialize a cheap
/// (`Copy`-sized) owned value (`Owned`). Dereferences to `Value`, so most call
/// sites are agnostic to which case they hold.
pub enum ValueRef<'a> {
    Borrowed(&'a Value),
    Owned(Value),
}

impl std::ops::Deref for ValueRef<'_> {
    type Target = Value;

    #[inline]
    fn deref(&self) -> &Value {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => v,
        }
    }
}

impl ValueRef<'_> {
    /// Take ownership of the value, cloning only in the borrowed case.
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> Value {
        match self {
            Self::Borrowed(v) => v.clone(),
            Self::Owned(v) => v,
        }
    }
}

/// Number of 64-bit words in a chunk's presence bitmap.
const PRESENT_WORDS: usize = CHUNK_LEN / 64;

#[inline]
const fn present_get(
    present: &[u64; PRESENT_WORDS],
    slot: usize,
) -> bool {
    (present[slot >> 6] >> (slot & 63)) & 1 != 0
}

#[inline]
const fn present_set(
    present: &mut [u64; PRESENT_WORDS],
    slot: usize,
) {
    present[slot >> 6] |= 1u64 << (slot & 63);
}

#[inline]
const fn present_clear(
    present: &mut [u64; PRESENT_WORDS],
    slot: usize,
) {
    present[slot >> 6] &= !(1u64 << (slot & 63));
}

/// Allocate a boxed fixed array of `CHUNK_LEN` copies of `fill`.
fn boxed_array<T: Clone>(fill: T) -> Box<[T; CHUNK_LEN]> {
    vec![fill; CHUNK_LEN]
        .into_boxed_slice()
        .try_into()
        .unwrap_or_else(|_| unreachable!("vec built with exactly CHUNK_LEN elements"))
}

/// The scalar attribute types that can be stored *unboxed* in a packed,
/// type-homogeneous chunk. Each is `Copy`, at most 8 bytes, and owns no heap.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    Bool,
    Int,
    Float,
    Point,
    Datetime,
    Date,
    Time,
    Duration,
}

impl ScalarKind {
    /// The packable scalar kind of `v`, or `None` for `Null` and the
    /// pointer-backed types (`String`/`List`/`VecF32`) which stay boxed as
    /// `Value`s in a mixed chunk.
    #[inline]
    const fn of(v: &Value) -> Option<Self> {
        match v {
            Value::Bool(_) => Some(Self::Bool),
            Value::Int(_) => Some(Self::Int),
            Value::Float(_) => Some(Self::Float),
            Value::Point(_) => Some(Self::Point),
            Value::Datetime(_) => Some(Self::Datetime),
            Value::Date(_) => Some(Self::Date),
            Value::Time(_) => Some(Self::Time),
            Value::Duration(_) => Some(Self::Duration),
            _ => None,
        }
    }

    /// Bytes per stored element for this kind's packed array.
    const fn elem_size(self) -> usize {
        match self {
            Self::Bool => std::mem::size_of::<bool>(),
            Self::Int | Self::Datetime | Self::Date | Self::Time | Self::Duration => {
                std::mem::size_of::<i64>()
            }
            Self::Float => std::mem::size_of::<f64>(),
            Self::Point => std::mem::size_of::<Point>(),
        }
    }
}

/// Packed, type-homogeneous storage for one chunk's scalar values. Absent slots
/// are tracked by the chunk's presence bitmap, so the padding in unused slots is
/// never observed.
#[derive(Clone)]
enum ScalarData {
    Bool(Box<[bool; CHUNK_LEN]>),
    Int(Box<[i64; CHUNK_LEN]>),
    Float(Box<[f64; CHUNK_LEN]>),
    Point(Box<[Point; CHUNK_LEN]>),
    Datetime(Box<[i64; CHUNK_LEN]>),
    Date(Box<[i64; CHUNK_LEN]>),
    Time(Box<[i64; CHUNK_LEN]>),
    Duration(Box<[i64; CHUNK_LEN]>),
}

impl ScalarData {
    /// A zero-filled packed array for `kind` (padding for absent slots).
    fn empty(kind: ScalarKind) -> Self {
        match kind {
            ScalarKind::Bool => Self::Bool(boxed_array(false)),
            ScalarKind::Int => Self::Int(boxed_array(0)),
            ScalarKind::Float => Self::Float(boxed_array(0.0)),
            ScalarKind::Point => Self::Point(boxed_array(Point::new(0.0, 0.0))),
            ScalarKind::Datetime => Self::Datetime(boxed_array(0)),
            ScalarKind::Date => Self::Date(boxed_array(0)),
            ScalarKind::Time => Self::Time(boxed_array(0)),
            ScalarKind::Duration => Self::Duration(boxed_array(0)),
        }
    }

    #[inline]
    const fn kind(&self) -> ScalarKind {
        match self {
            Self::Bool(_) => ScalarKind::Bool,
            Self::Int(_) => ScalarKind::Int,
            Self::Float(_) => ScalarKind::Float,
            Self::Point(_) => ScalarKind::Point,
            Self::Datetime(_) => ScalarKind::Datetime,
            Self::Date(_) => ScalarKind::Date,
            Self::Time(_) => ScalarKind::Time,
            Self::Duration(_) => ScalarKind::Duration,
        }
    }

    /// Write a value known to match this data's kind into `slot`.
    #[inline]
    #[allow(clippy::match_same_arms)]
    fn write(
        &mut self,
        slot: usize,
        value: Value,
    ) {
        match (self, value) {
            (Self::Bool(a), Value::Bool(b)) => a[slot] = b,
            (Self::Int(a), Value::Int(i)) => a[slot] = i,
            (Self::Float(a), Value::Float(f)) => a[slot] = f,
            (Self::Point(a), Value::Point(p)) => a[slot] = p,
            (Self::Datetime(a), Value::Datetime(t)) => a[slot] = t,
            (Self::Date(a), Value::Date(t)) => a[slot] = t,
            (Self::Time(a), Value::Time(t)) => a[slot] = t,
            (Self::Duration(a), Value::Duration(t)) => a[slot] = t,
            _ => unreachable!("ScalarData::write called with mismatched value type"),
        }
    }

    /// Reconstruct the `Value` stored at `slot` (caller ensures presence).
    #[inline]
    fn value_at(
        &self,
        slot: usize,
    ) -> Value {
        match self {
            Self::Bool(a) => Value::Bool(a[slot]),
            Self::Int(a) => Value::Int(a[slot]),
            Self::Float(a) => Value::Float(a[slot]),
            Self::Point(a) => Value::Point(a[slot].clone()),
            Self::Datetime(a) => Value::Datetime(a[slot]),
            Self::Date(a) => Value::Date(a[slot]),
            Self::Time(a) => Value::Time(a[slot]),
            Self::Duration(a) => Value::Duration(a[slot]),
        }
    }
}

/// A dense block of `CHUNK_LEN` values for a contiguous id range.
///
/// A chunk is either **packed** — a type-homogeneous [`ScalarData`] array plus a
/// presence bitmap, roughly halving memory for the common case of a column whose
/// values are all the same scalar type — or **mixed**, a `Box<[Value; CHUNK_LEN]>`
/// where `Value::Null` marks absent slots (used for pointer-typed values like
/// `String`/`List`/`VecF32` and for columns that mix types). `count` tracks the
/// number of present values so an emptied chunk can be reclaimed in O(1).
#[derive(Clone)]
enum Chunk {
    Mixed {
        values: Box<[Value; CHUNK_LEN]>,
        count: u32,
    },
    Scalar {
        data: ScalarData,
        present: [u64; PRESENT_WORDS],
        count: u32,
    },
}

impl Chunk {
    /// Build a chunk seeded with a single non-null `value` at `slot`, choosing
    /// the packed representation when the value is a packable scalar.
    fn with_first(
        slot: usize,
        value: Value,
    ) -> Self {
        if let Some(kind) = ScalarKind::of(&value) {
            let mut data = ScalarData::empty(kind);
            data.write(slot, value);
            let mut present = [0u64; PRESENT_WORDS];
            present_set(&mut present, slot);
            Self::Scalar {
                data,
                present,
                count: 1,
            }
        } else {
            let mut values = boxed_array(Value::Null);
            values[slot] = value;
            Self::Mixed { values, count: 1 }
        }
    }

    /// Read `slot`, borrowing for mixed chunks and materializing a cheap owned
    /// `Value` for packed scalar chunks.
    #[inline]
    fn get(
        &self,
        slot: usize,
    ) -> Option<ValueRef<'_>> {
        match self {
            Self::Mixed { values, .. } => {
                let v = &values[slot];
                if matches!(v, Value::Null) {
                    None
                } else {
                    Some(ValueRef::Borrowed(v))
                }
            }
            Self::Scalar { data, present, .. } => {
                if present_get(present, slot) {
                    Some(ValueRef::Owned(data.value_at(slot)))
                } else {
                    None
                }
            }
        }
    }

    /// Set a non-null `value` at `slot`. Returns whether a value was already
    /// present. A packed chunk that receives a differing scalar type is promoted
    /// to a mixed chunk first.
    fn set(
        &mut self,
        slot: usize,
        value: Value,
    ) -> bool {
        debug_assert!(
            !matches!(value, Value::Null),
            "Chunk::set is only called with non-null values; nulls go through clear"
        );
        match self {
            Self::Mixed { values, count } => {
                let had = !matches!(values[slot], Value::Null);
                if !had {
                    *count += 1;
                }
                values[slot] = value;
                return had;
            }
            Self::Scalar {
                data,
                present,
                count,
            } if ScalarKind::of(&value) == Some(data.kind()) => {
                let had = present_get(present, slot);
                if !had {
                    present_set(present, slot);
                    *count += 1;
                }
                data.write(slot, value);
                return had;
            }
            Self::Scalar { .. } => {}
        }
        // Promotion: a packed chunk received a different type — rebuild as mixed.
        let Self::Scalar {
            data,
            present,
            count,
        } = self
        else {
            unreachable!("promotion is only reached from a scalar chunk");
        };
        let mut values = boxed_array(Value::Null);
        for s in 0..CHUNK_LEN {
            if present_get(present, s) {
                values[s] = data.value_at(s);
            }
        }
        let mut new_count = *count;
        let had = present_get(present, slot);
        if !had {
            new_count += 1;
        }
        values[slot] = value;
        *self = Self::Mixed {
            values,
            count: new_count,
        };
        had
    }

    /// Clear `slot` (caller ensures it is present). Returns whether the chunk is
    /// now empty and can be reclaimed.
    fn clear(
        &mut self,
        slot: usize,
    ) -> bool {
        match self {
            Self::Mixed { values, count } => {
                values[slot] = Value::Null;
                *count -= 1;
                *count == 0
            }
            Self::Scalar { present, count, .. } => {
                present_clear(present, slot);
                *count -= 1;
                *count == 0
            }
        }
    }

    /// Structural bytes of this chunk's value array plus presence bitmap.
    const fn structural_bytes(&self) -> usize {
        match self {
            Self::Mixed { .. } => CHUNK_LEN * std::mem::size_of::<Value>(),
            Self::Scalar { data, .. } => {
                CHUNK_LEN * data.kind().elem_size() + std::mem::size_of::<[u64; PRESENT_WORDS]>()
            }
        }
    }

    /// Out-of-line heap owned by this chunk's values. Packed scalars own none.
    fn heap_payload_bytes(&self) -> usize {
        match self {
            Self::Mixed { values, .. } => values.iter().map(Value::heap_size).sum(),
            Self::Scalar { .. } => 0,
        }
    }
}

/// A page of `PAGE_LEN` chunk pointers — the copy-on-write unit for the
/// column's chunk directory.
#[derive(Clone)]
struct Page {
    chunks: [Option<Arc<Chunk>>; PAGE_LEN],
}

impl Default for Page {
    fn default() -> Self {
        Self {
            chunks: std::array::from_fn(|_| None),
        }
    }
}

/// One attribute's values, stored densely and chunked by id, with a paged
/// chunk directory.
///
/// Three `Arc` layers give copy-on-write sharing across MVCC versions at
/// bounded cost: cloning a column shares the directory; a write clones only
/// the directory's page vector (once), the touched page (`PAGE_LEN` pointers,
/// once), and the touched chunk (`CHUNK_LEN` values, once) per version.
#[derive(Clone, Default)]
struct Column {
    pages: Arc<Vec<Option<Arc<Page>>>>,
}

impl Column {
    #[inline]
    fn get(
        &self,
        id: u64,
    ) -> Option<ValueRef<'_>> {
        let cid = (id >> CHUNK_SHIFT) as usize;
        self.get_at(
            cid >> PAGE_SHIFT,
            cid & PAGE_MASK,
            (id & CHUNK_MASK) as usize,
        )
    }

    /// Lookup with a pre-decomposed id (`page`, `chunk-in-page`, `value slot`).
    /// Reused across every column when materializing one entity, so the id
    /// shift/mask math runs once rather than per column.
    #[inline]
    fn get_at(
        &self,
        pidx: usize,
        cin: usize,
        slot: usize,
    ) -> Option<ValueRef<'_>> {
        let page = self.pages.get(pidx)?.as_ref()?;
        let chunk = page.chunks[cin].as_ref()?;
        chunk.get(slot)
    }

    /// Set `id`'s value. Returns whether a non-null value was already present.
    fn set(
        &mut self,
        id: u64,
        value: Value,
    ) -> bool {
        let cid = (id >> CHUNK_SHIFT) as usize;
        let pidx = cid >> PAGE_SHIFT;
        // Copy-on-write the (small) page directory if shared with another version.
        let pages = Arc::make_mut(&mut self.pages);
        if pidx >= pages.len() {
            pages.resize(pidx + 1, None);
        }
        let page_arc = pages[pidx].get_or_insert_with(|| Arc::new(Page::default()));
        // Copy-on-write the touched page (PAGE_LEN pointers) if shared.
        let page = Arc::make_mut(page_arc);
        let slot = (id & CHUNK_MASK) as usize;
        // Copy-on-write the touched chunk (CHUNK_LEN values) if shared; a brand
        // new chunk picks its packed/mixed representation from the first value.
        let entry = &mut page.chunks[cid & PAGE_MASK];
        if let Some(chunk_arc) = entry {
            Arc::make_mut(chunk_arc).set(slot, value)
        } else {
            *entry = Some(Arc::new(Chunk::with_first(slot, value)));
            false
        }
    }

    /// Clear `id`'s value. Returns whether a non-null value was present.
    fn clear(
        &mut self,
        id: u64,
    ) -> bool {
        // Peek first so an absent value never triggers a copy-on-write clone.
        if self.get(id).is_none() {
            return false;
        }
        let cid = (id >> CHUNK_SHIFT) as usize;
        let pidx = cid >> PAGE_SHIFT;
        let cin = cid & PAGE_MASK;
        let slot = (id & CHUNK_MASK) as usize;
        let pages = Arc::make_mut(&mut self.pages);
        // Mutate the chunk, and note whether it (and then its page) emptied. The
        // inner borrow is scoped so the page slot can be reclaimed afterwards.
        let page_emptied = {
            let page = Arc::make_mut(pages[pidx].as_mut().expect("present page"));
            let chunk = Arc::make_mut(page.chunks[cin].as_mut().expect("present chunk"));
            if chunk.clear(slot) {
                // Reclaim the emptied chunk: drops this version's `Arc` (shared
                // readers keep theirs), freeing the dense value block.
                page.chunks[cin] = None;
                page.chunks.iter().all(Option::is_none)
            } else {
                false
            }
        };
        if page_emptied {
            pages[pidx] = None;
            // Trim trailing empty directory slots so the directory can shrink.
            while matches!(pages.last(), Some(None)) {
                pages.pop();
            }
        }
        true
    }

    fn chunks(&self) -> impl Iterator<Item = &Arc<Chunk>> + '_ {
        self.pages
            .iter()
            .flatten()
            .flat_map(|p| p.chunks.iter().flatten())
    }

    fn num_chunk_slots(&self) -> usize {
        self.pages.len() * PAGE_LEN
    }

    fn heap_bytes(&self) -> usize {
        self.num_chunk_slots() * std::mem::size_of::<Option<Arc<Chunk>>>()
            + self.pages.len() * std::mem::size_of::<Option<Arc<Page>>>()
            + self
                .chunks()
                .map(|c| c.structural_bytes() + c.heap_payload_bytes())
                .sum::<usize>()
    }

    fn structural_bytes(&self) -> usize {
        self.num_chunk_slots() * std::mem::size_of::<Option<Arc<Chunk>>>()
            + self.pages.len() * std::mem::size_of::<Option<Arc<Page>>>()
            + self.chunks().map(|c| c.structural_bytes()).sum::<usize>()
    }
}

/// Columnar attribute storage: one [`Column`] per attribute index.
///
/// Owned per MVCC version; `Clone` shares `Arc<Chunk>` blocks with the parent
/// (copy-on-write). No locks and no `unsafe`: readers hold an older version's
/// `Arc`s while a writer's `make_mut` copies only the chunks it touches.
#[derive(Clone, Default)]
pub struct AttributeStorage {
    columns: Vec<Column>,
}

impl AttributeStorage {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn column(
        &self,
        attr_idx: u16,
    ) -> Option<&Column> {
        self.columns.get(attr_idx as usize)
    }

    #[inline]
    fn column_mut(
        &mut self,
        attr_idx: u16,
    ) -> &mut Column {
        let i = attr_idx as usize;
        if i >= self.columns.len() {
            self.columns.resize_with(i + 1, Column::default);
        }
        &mut self.columns[i]
    }

    /// Set an attribute for an entity. Returns whether a non-null value was
    /// already present (i.e. this was a replacement).
    fn set(
        &mut self,
        id: u64,
        attr_idx: u16,
        value: Value,
    ) -> bool {
        self.column_mut(attr_idx).set(id, value)
    }

    /// Clear an attribute for an entity. Returns whether a value was present.
    fn clear(
        &mut self,
        id: u64,
        attr_idx: u16,
    ) -> bool {
        self.columns
            .get_mut(attr_idx as usize)
            .is_some_and(|c| c.clear(id))
    }

    /// Look up a single attribute for an entity by index.
    ///
    /// Returns `Some(Some(value))` when present and `None` when absent; the
    /// `version` argument is accepted for API compatibility and unused, since
    /// snapshot isolation is structural (copy-on-write).
    #[must_use]
    pub fn get_attr(
        &self,
        entity_id: u64,
        attr_idx: u16,
        _version: u64,
    ) -> Option<Option<Value>> {
        self.column(attr_idx)?
            .get(entity_id)
            .map(|v| Some(v.into_owned()))
    }

    /// Fused batch lookup that writes resolved `Value`s straight into `out`,
    /// substituting `default` for absent ids and recording their absolute
    /// positions in `missing`.
    pub fn get_attrs_batch_into(
        &self,
        keys: &[u64],
        attr_idx: u16,
        _version: u64,
        default: &Value,
        out: &mut Vec<Value>,
        missing: &mut Vec<usize>,
    ) {
        let base = out.len();
        let col = self.column(attr_idx);
        for (pos, &id) in keys.iter().enumerate() {
            if let Some(v) = col.and_then(|c| c.get(id)) {
                out.push(v.into_owned());
            } else {
                out.push(default.clone());
                missing.push(base + pos);
            }
        }
    }

    /// Visit each present `(attr_idx, &Value)` for an entity in ascending index
    /// order, without allocating a snapshot. The id is decomposed once and the
    /// page/chunk/slot indices are reused across every column.
    #[inline]
    fn for_each_attr(
        &self,
        entity_id: u64,
        mut f: impl FnMut(u16, &Value),
    ) {
        let cid = (entity_id >> CHUNK_SHIFT) as usize;
        let pidx = cid >> PAGE_SHIFT;
        let cin = cid & PAGE_MASK;
        let slot = (entity_id & CHUNK_MASK) as usize;
        for (idx, col) in self.columns.iter().enumerate() {
            if let Some(v) = col.get_at(pidx, cin, slot) {
                f(idx as u16, &v);
            }
        }
    }

    /// Whether an entity has any attribute.
    #[must_use]
    pub fn has_entity(
        &self,
        entity_id: u64,
        _version: u64,
    ) -> Option<bool> {
        let cid = (entity_id >> CHUNK_SHIFT) as usize;
        let pidx = cid >> PAGE_SHIFT;
        let cin = cid & PAGE_MASK;
        let slot = (entity_id & CHUNK_MASK) as usize;
        Some(
            self.columns
                .iter()
                .any(|c| c.get_at(pidx, cin, slot).is_some()),
        )
    }

    /// Clear every attribute of each id (used on commit for deleted entities).
    pub fn invalidate_batch(
        &mut self,
        entity_ids: &RoaringTreemap,
    ) {
        for col in &mut self.columns {
            for id in entity_ids {
                col.clear(id);
            }
        }
    }

    /// Estimated heap bytes of stored values across all columns.
    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.columns.iter().map(Column::heap_bytes).sum()
    }

    /// Structural overhead (chunk pointers + dense value slots), excluding the
    /// out-of-line heap owned by each `Value`.
    #[must_use]
    pub fn structural_memory_usage(&self) -> usize {
        self.columns.iter().map(Column::structural_bytes).sum()
    }
}

#[cfg(test)]
mod pack_tests {
    use super::*;

    #[test]
    fn packed_scalar_roundtrip() {
        let mut col = Column::default();
        // Values spanning several chunks stay type-homogeneous → packed.
        for id in 0..300u64 {
            col.set(id, Value::Int(id as i64));
        }
        for id in 0..300u64 {
            assert!(
                matches!(col.get(id).as_deref(), Some(Value::Int(v)) if *v == id as i64),
                "int {id} round-trip"
            );
        }
        assert!(col.get(300).is_none());
    }

    #[test]
    fn promotes_to_mixed_on_type_change() {
        let mut col = Column::default();
        for id in 0..10u64 {
            col.set(id, Value::Int(id as i64));
        }
        let packed_bytes = col.structural_bytes();

        // A differing type in the same chunk promotes it to a mixed value array.
        col.set(3, Value::String(Arc::new("x".to_owned())));
        assert!(matches!(col.get(3).as_deref(), Some(Value::String(s)) if s.as_str() == "x"));
        // The other ints in the promoted chunk survive the rebuild.
        assert!(matches!(col.get(7).as_deref(), Some(Value::Int(7))));
        // A promoted chunk (16 B/slot) is larger than the packed one (8 B/slot).
        assert!(col.structural_bytes() > packed_bytes);
    }

    #[test]
    fn clear_reclaims_and_updates() {
        let mut col = Column::default();
        col.set(10, Value::Float(1.5));
        col.set(11, Value::Float(2.5));
        assert!(col.clear(10));
        assert!(col.get(10).is_none());
        assert!(matches!(col.get(11).as_deref(), Some(Value::Float(f)) if *f == 2.5));
        // Clearing the last value reclaims the chunk.
        assert!(col.clear(11));
        assert!(col.get(11).is_none());
    }

    #[test]
    fn packed_scalar_uses_less_memory_than_mixed() {
        let n = 4096u64;
        let mut ints = Column::default();
        let mut strings = Column::default();
        for id in 0..n {
            ints.set(id, Value::Int(id as i64));
            strings.set(id, Value::String(Arc::new(format!("{id}"))));
        }
        // Same slot count, but packed i64 slots are 8 B vs the 16 B `Value` slots
        // a mixed (pointer-typed) column must use.
        assert!(
            ints.structural_bytes() < strings.structural_bytes(),
            "packed {} should be < mixed {}",
            ints.structural_bytes(),
            strings.structural_bytes()
        );
    }
}
