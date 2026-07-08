//! Attribute storage for graph entities (nodes and relationships).
//!
//! This module provides [`AttributeStore`], a **row-oriented** in-memory
//! store: all attributes of one entity form a *row* (a record of
//! `(attr_idx, value)` pairs sorted by attribute index), and rows live in a
//! **paged, directly-indexed clustered heap** keyed by entity id. It keeps the
//! clustered-record layout of a MySQL/InnoDB table but replaces the B+tree
//! with radix-style direct addressing, because FalkorDB entity ids are dense
//! sequential integers (allocated from 0, recycled on delete): the id *is*
//! the address, so ordering machinery and key comparisons are pure overhead.
//! Profiling the B+tree variant showed the descent alone was ~43% of CPU.
//!
//! ## Addressing
//!
//! An id splits into three digits — `root[id >> 12]` → directory page,
//! `(id >> 6) & 63` → row page, `id & 63` → slot:
//!
//! ```text
//!  root: Vec<Option<Arc<DirPage>>>     (grows with max id; ~2 KB / 512k ids)
//!    └─ DirPage: [Option<Arc<Page>>; 64]          (512 B, COW unit)
//!         └─ Page: [Option<Row>; 64] + count      (~1 KB, COW unit)
//!              └─ Row: Arc<[(u16, Value)]>        (one entity's record)
//! ```
//!
//! ## Read Path
//!
//! `get_attr_by_idx` is three dependent loads and zero comparisons — O(1)
//! like an array, not O(log n) like a tree. Per-entity materialization
//! (`get_attrs`) lands on the whole contiguous record at once, and the
//! clustered scan (`iter_rows`) walks pages in id order with no lookups at
//! all. Batch reads keep a one-page cursor so runs of ids in the same page
//! skip even the directory hops.
//!
//! ## Write Path
//!
//! No splits, no separators, no rebalancing: a write resolves its page by
//! address arithmetic and rewrites one row. Emptied pages are unlinked from
//! their directory; trailing empty directory slots are trimmed.
//!
//! ## MVCC Integration
//!
//! Snapshot isolation is provided by **copy-on-write** at page granularity.
//! `new_version()` clones the store cheaply (one `Arc` bump of the root); a
//! write clones only the root pointer vector (once per version), the touched
//! 512 B directory page, and the touched ~1 KB row page (`Arc::make_mut`).
//! Rows themselves are `Arc`-shared records, so copying a page is
//! pointer-width work and only the row being edited is rebuilt. A reader on
//! an older version keeps the original root, so it never observes a writer's
//! changes — and rollback is simply dropping the discarded version. No locks,
//! no per-entry version stamps, and no `unsafe`.

use std::{borrow::Cow, sync::Arc};

use rustc_hash::FxHashMap;

use roaring::RoaringTreemap;

use super::super::graphblas::serialization::{Decode, Encode, Reader, Writer};
use crate::runtime::{ordermap::OrderMap, value::Value};

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
        // Attribute indices are `u16`; more than `u16::MAX` distinct names would
        // silently alias two attributes to one index, so fail loudly instead.
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

/// One entity's record: its attributes as `(attr_idx, value)` pairs sorted by
/// attribute index. Stored rows are never empty — a row whose last attribute
/// is cleared is removed from its page.
///
/// Rows are `Arc`-shared records: when copy-on-write clones a page, the clone
/// is `PAGE_LEN` pointer bumps, and only a row actually being edited is
/// rebuilt. Two MVCC versions therefore share every untouched record.
type Row = Arc<[(u16, Value)]>;

/// A pending change to one attribute of a row: `Some` sets, `None` clears.
type AttrOp = (u16, Option<Value>);

/// Row-oriented attribute storage for graph entities.
///
/// A paged clustered heap ([`PagedRows`]) directly indexed by entity id is the
/// sole, in-memory source of truth for entity attributes. A lookup miss means
/// the entity has no attributes.
#[derive(Clone)]
pub struct AttributeStore {
    /// Attribute names in insertion order (name → attribute index)
    pub attrs_name: AttrNameMap,
    /// Clustered row storage. Owned per version; pages share `Arc`s with the
    /// parent version until a write copies them (copy-on-write).
    rows: PagedRows,
    /// MVCC version of this store's snapshot (informational; visibility is
    /// enforced structurally by copy-on-write, not by version comparison).
    #[allow(dead_code)]
    version: u64,
}

impl AttributeStore {
    #[must_use]
    pub fn new(version: u64) -> Self {
        Self {
            attrs_name: AttrNameMap::default(),
            rows: PagedRows::default(),
            version,
        }
    }

    #[must_use]
    pub fn new_version(
        &self,
        version: u64,
    ) -> Self {
        // Copy-on-write: the clone shares the heap's `Arc` pages with `self`;
        // a write in the new version copies only the pages it touches.
        Self {
            attrs_name: self.attrs_name.clone(),
            rows: self.rows.clone(),
            version,
        }
    }

    // ---- read path -------------------------------------------------------

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
        // A miss (no row, or no such attr in the row) means the entity has no
        // such attribute.
        row_get(self.rows.get_row(key)?, attr_idx).cloned()
    }

    /// Batch variant of `get_attr_by_idx` for a list of keys with the same
    /// `attr_idx`. A page cursor makes runs of keys that land in one page skip
    /// the directory hops; each absent key carries `default`.
    pub fn get_attrs_by_idx_batch_into(
        &self,
        keys: &[u64],
        attr_idx: u16,
        default: &Value,
        out: &mut Vec<Value>,
    ) {
        out.reserve(keys.len());
        let mut cursor = PageCursor::new(&self.rows);
        for &key in keys {
            match cursor.find(key).and_then(|row| row_get(row, attr_idx)) {
                Some(v) => out.push(v.clone()),
                None => out.push(default.clone()),
            }
        }
    }

    #[must_use]
    pub fn has_attributes(
        &self,
        key: u64,
    ) -> bool {
        // Stored rows are never empty, so row presence == "has attributes".
        self.rows.get_row(key).is_some()
    }

    #[must_use]
    pub fn get_attrs(
        &self,
        key: u64,
    ) -> AttrArrayView<'_> {
        AttrArrayView {
            store: self,
            row: self.rows.get_row(key).map(|r| &r[..]),
        }
    }

    /// Clustered full scan: iterate every `(entity_id, attrs)` in ascending id
    /// order by walking the row pages directly — the row-store equivalent of
    /// an InnoDB full table scan. No per-entity lookup: each row costs one
    /// step through a contiguous page, so this is the fastest way to *return
    /// rows* (whole entities) in bulk.
    pub fn iter_rows(&self) -> impl Iterator<Item = (u64, AttrArrayView<'_>)> {
        RowScan {
            store: self,
            root: &self.rows.root,
            di: 0,
            dir: None,
            pi: 0,
            page: None,
            base: 0,
            si: 0,
        }
    }

    // ---- write path -------------------------------------------------------

    pub fn remove_attr(
        &mut self,
        key: u64,
        attr: &Arc<String>,
    ) -> Result<bool, String> {
        if let Some(idx) = self.attrs_name.get_index_of(attr) {
            return Ok(self.rows.delete(key, Some(idx as u16)));
        }
        Ok(false)
    }

    pub fn remove_all(
        &mut self,
        keys: &RoaringTreemap,
    ) {
        // Remove each row immediately. Copy-on-write means this only touches
        // this version's pages; a failed transaction is discarded by dropping
        // the version, so no deferred delete bookkeeping is needed.
        for key in keys {
            self.rows.delete(key, None);
        }
    }

    /// Batch insert/update multiple attributes for entities.
    ///
    /// Non-null values are written into the entity's row; nulls clear the
    /// attribute. Each entity costs one page lookup regardless of how many of
    /// its attributes change. Returns `(nremoved, nset)`: the number of
    /// previously-present attributes overwritten or cleared, and the number of
    /// non-null values set.
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
                    e.insert(self.get_or_create_attr_id(attr));
                }
            }
        }

        // One reusable op buffer for the whole batch; `apply_ops` drains it.
        // Entities are applied in ascending id order: hash-map iteration order
        // is random, while consecutive ids share pages — sorting lets one COW
        // page copy serve a whole run of neighbours.
        let mut order: Vec<(u64, &OrderMap<Arc<String>, Value>)> =
            attrs.iter().map(|(k, v)| (*k, v)).collect();
        order.sort_unstable_by_key(|&(k, _)| k);
        let mut ops: Vec<AttrOp> = Vec::new();
        let mut row_buf: Vec<(u16, Value)> = Vec::new();
        for (key, entity_attrs) in order {
            for (attr, value) in entity_attrs.iter() {
                let idx = name_to_idx[&Arc::as_ptr(attr)];
                if matches!(value, Value::Null) {
                    // A null clears; clearing an absent attribute is a no-op
                    // (merge counts only real removals).
                    ops.push((idx, None));
                } else {
                    ops.push((idx, Some(value.clone())));
                }
            }
            let (removed, set) = self.rows.apply_ops(key, &mut ops, &mut row_buf);
            nremoved += removed;
            nset += set;
        }

        Ok((nremoved, nset))
    }

    /// Bulk import attributes for entities known to be new (no prior state).
    ///
    /// Optimized for RDB decode: builds each row in one page lookup. Returns
    /// the number of non-null attributes imported.
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
                    e.insert(self.get_or_create_attr_id(attr));
                }
            }
        }

        let mut nset = 0;
        // Ascending id order: see `insert_attrs` for why batches are sorted.
        let mut order: Vec<(u64, &OrderMap<Arc<String>, Value>)> =
            attrs.iter().map(|(k, v)| (*k, v)).collect();
        order.sort_unstable_by_key(|&(k, _)| k);
        let mut ops: Vec<AttrOp> = Vec::new();
        let mut row_buf: Vec<(u16, Value)> = Vec::new();
        for (key, entity_attrs) in order {
            for (attr, value) in entity_attrs.iter() {
                if matches!(value, Value::Null) {
                    continue;
                }
                ops.push((name_to_idx[&Arc::as_ptr(attr)], Some(value.clone())));
            }
            nset += self.rows.apply_ops(key, &mut ops, &mut row_buf).1;
        }
        nset
    }

    /// Import pre-resolved attribute data directly into rows.
    /// Skips name resolution and OrderMap construction; used by bulk insert.
    pub fn import_attrs_resolved(
        &mut self,
        data: &mut Vec<(u64, Vec<(u16, Value)>)>,
    ) -> usize {
        let mut nset = 0;
        // Ascending id order: see `insert_attrs` for why batches are sorted.
        data.sort_unstable_by_key(|&(id, _)| id);
        let mut ops: Vec<AttrOp> = Vec::new();
        let mut row_buf: Vec<(u16, Value)> = Vec::new();
        for (entity_id, entries) in data.drain(..) {
            ops.extend(entries.into_iter().map(|(idx, v)| (idx, Some(v))));
            nset += self.rows.apply_ops(entity_id, &mut ops, &mut row_buf).1;
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
        let (structural, payload) = self.rows.memory_usage();
        structural + payload
    }

    /// Structural page/row-storage overhead, excluding attribute payload heap.
    #[must_use]
    pub fn structural_memory_usage(&self) -> usize {
        self.rows.memory_usage().0
    }

    /// Encode a range of entities, reading attributes from the row heap.
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
        let mut cursor = PageCursor::new(&self.rows);

        for id in 0..=max_id {
            if deleted.contains(id) {
                continue;
            }
            if skipped < offset {
                skipped += 1;
                continue;
            }

            w.write_unsigned(id);

            // The row is one contiguous record: a single cursor probe yields
            // both the count and the values.
            let row = cursor.find(id).map_or(&[][..], |r| &r[..]);
            w.write_unsigned(row.len() as u64);
            for (local_attr_id, value) in row {
                let global_attr_id = if (*local_attr_id as usize) < remap.len() {
                    remap[*local_attr_id as usize]
                } else {
                    *local_attr_id
                };
                w.write_unsigned(global_attr_id as u64);
                value.encode(w);
            }

            encoded += 1;
            if encoded >= count {
                break;
            }
        }
    }
}

// `AttributeStore` is automatically `Send + Sync`: every field is `Send + Sync`
// (pages are plain arrays/`Arc`s over `Send + Sync` `Value`s), so no manual
// impl — and no `unsafe` — is needed here.

impl Decode<19> for AttributeStore {
    fn decode(_r: &mut dyn Reader) -> Result<Self, String> {
        unimplemented!("use decode_with_count for AttributeStore")
    }

    fn decode_with_count(
        &mut self,
        r: &mut dyn Reader,
        count: u64,
    ) -> Result<(), String> {
        let mut ops: Vec<AttrOp> = Vec::new();
        let mut row_buf: Vec<(u16, Value)> = Vec::new();
        for _ in 0..count {
            let entity_id = r.read_unsigned()?;
            let attr_count = r.read_unsigned()?;

            for _ in 0..attr_count {
                let attr_id = r.read_unsigned()? as u16;
                let value = Value::decode(r)?;

                if (attr_id as usize) < self.attrs_name.len() && !matches!(value, Value::Null) {
                    ops.push((attr_id, Some(value)));
                }
            }
            // Entities arrive grouped, so each costs one page lookup.
            self.rows.apply_ops(entity_id, &mut ops, &mut row_buf);
        }
        Ok(())
    }
}

/// Find one attribute in a sorted row. Rows are tiny (a handful of attrs), so
/// a forward scan with early exit beats binary search: the branches are
/// near-perfectly predicted, while binary probes on 4-8 elements mispredict.
#[inline]
fn row_get(
    row: &Row,
    attr_idx: u16,
) -> Option<&Value> {
    for (i, v) in row.iter() {
        if *i >= attr_idx {
            return (*i == attr_idx).then_some(v);
        }
    }
    None
}

/// Borrowing view of one entity's attributes.
///
/// Returned by the per-entity read APIs (`get_attrs`). Holds the resolved row
/// slice (one O(1) lookup at construction) and clones no `Value`s: `iter()`
/// walks the contiguous record in ascending attribute-index order, so the view
/// must be consumed while the store borrow is alive. A prop-less entity simply
/// yields an empty iterator.
pub struct AttrArrayView<'a> {
    store: &'a AttributeStore,
    row: Option<&'a [(u16, Value)]>,
}

impl<'a> AttrArrayView<'a> {
    /// Number of present attributes for this entity.
    #[must_use]
    pub fn len(&self) -> usize {
        self.row.map_or(0, <[_]>::len)
    }

    /// Whether the entity has no attributes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.row.is_none()
    }

    /// Iterate `(attr_idx, value)` pairs in ascending index order. Values are
    /// always borrowed straight out of the row.
    pub fn iter(&self) -> impl Iterator<Item = (u16, Cow<'a, Value>)> {
        self.row
            .unwrap_or(&[])
            .iter()
            .map(|(i, v)| (*i, Cow::Borrowed(v)))
    }

    /// Iterate `(&attr_name, value)` pairs in ascending index order. Like
    /// [`iter`](Self::iter) but resolves each attribute index to its name.
    pub fn iter_named(&self) -> impl Iterator<Item = (&'a Arc<String>, Cow<'a, Value>)> {
        let names = &self.store.attrs_name;
        self.row
            .unwrap_or(&[])
            .iter()
            .filter_map(move |(i, v)| names.get(*i as usize).map(|n| (n, Cow::Borrowed(v))))
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

// ============================================================================
// Paged clustered heap
// ============================================================================

/// Rows per page (`id & PAGE_MASK` selects the slot). The page is the COW
/// unit: the first write to a page in a new version copies `PAGE_LEN`
/// `Option<Row>` slots and bumps one refcount per present row, so the size
/// balances copy cost against per-page overhead. 64 slots ≈ 1 KB.
const PAGE_LEN: usize = 64;
const PAGE_SHIFT: u32 = PAGE_LEN.trailing_zeros();
const PAGE_MASK: u64 = PAGE_LEN as u64 - 1;

/// Pages per directory page (`(id >> PAGE_SHIFT) & DIR_MASK` selects the
/// page pointer). Keeps the per-version root copy small: the root holds one
/// pointer per `PAGE_LEN * DIR_LEN = 4096` ids.
const DIR_LEN: usize = 64;
const DIR_SHIFT: u32 = DIR_LEN.trailing_zeros();
const DIR_MASK: u64 = DIR_LEN as u64 - 1;

/// Decompose an id into `(root index, dir slot, page slot)`.
#[inline]
const fn decompose(id: u64) -> (usize, usize, usize) {
    (
        (id >> (PAGE_SHIFT + DIR_SHIFT)) as usize,
        ((id >> PAGE_SHIFT) & DIR_MASK) as usize,
        (id & PAGE_MASK) as usize,
    )
}

/// A fixed block of `PAGE_LEN` row slots for one aligned id range. `count`
/// tracks present rows so an emptied page can be unlinked in O(1).
#[derive(Clone)]
struct Page {
    rows: [Option<Row>; PAGE_LEN],
    count: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            rows: std::array::from_fn(|_| None),
            count: 0,
        }
    }
}

/// A directory of `DIR_LEN` page pointers — the middle radix level.
#[derive(Clone)]
struct DirPage {
    pages: [Option<Arc<Page>>; DIR_LEN],
}

impl Default for DirPage {
    fn default() -> Self {
        Self {
            pages: std::array::from_fn(|_| None),
        }
    }
}

/// Paged clustered heap of rows, copy-on-write at page granularity.
///
/// `Clone` shares all pages with the parent version (one `Arc` bump); a write
/// clones only the root vector, the touched directory page, and the touched
/// row page via `Arc::make_mut`. Readers on an older version keep the
/// original root, so no locks and no `unsafe`.
#[derive(Clone, Default)]
struct PagedRows {
    root: Arc<Vec<Option<Arc<DirPage>>>>,
}

impl PagedRows {
    /// Point lookup: three dependent loads, zero comparisons.
    #[inline]
    fn get_row(
        &self,
        id: u64,
    ) -> Option<&Row> {
        let (ri, di, si) = decompose(id);
        self.root.get(ri)?.as_ref()?.pages[di].as_ref()?.rows[si].as_ref()
    }

    /// Apply a batch of attribute ops to one entity's row in a single page
    /// lookup, draining `ops` (sorted + deduped here). `row_buf` is a reusable
    /// merge scratch buffer. Returns `(nremoved, nset)`.
    fn apply_ops(
        &mut self,
        key: u64,
        ops: &mut Vec<AttrOp>,
        row_buf: &mut Vec<(u16, Value)>,
    ) -> (usize, usize) {
        if ops.is_empty() {
            return (0, 0);
        }
        // Pure clears on an absent row must not materialize pages.
        if self.get_row(key).is_none() && ops.iter().all(|(_, v)| v.is_none()) {
            ops.clear();
            return (0, 0);
        }
        ops.sort_unstable_by_key(|&(i, _)| i);
        // Defensive: collapse duplicate indices, keeping the last op (callers
        // normally guarantee uniqueness — OrderMap keys, RDB rows).
        ops.dedup_by(|later, earlier| {
            if later.0 == earlier.0 {
                std::mem::swap(&mut earlier.1, &mut later.1);
                true
            } else {
                false
            }
        });

        let (ri, di, si) = decompose(key);
        let root = Arc::make_mut(&mut self.root);
        if ri >= root.len() {
            root.resize(ri + 1, None);
        }
        let dir = Arc::make_mut(root[ri].get_or_insert_default());
        let page = Arc::make_mut(dir.pages[di].get_or_insert_default());

        let mut nremoved = 0;
        let mut nset = 0;
        let old = page.rows[si].as_deref().unwrap_or(&[]);
        merge_row(old, ops, row_buf, &mut nremoved, &mut nset);
        if row_buf.is_empty() {
            // Every attribute cleared — drop the row and unlink emptied pages.
            if page.rows[si].take().is_some() {
                page.count -= 1;
                if page.count == 0 {
                    dir.pages[di] = None;
                    Self::reclaim(root, ri);
                }
            }
        } else {
            if page.rows[si].is_none() {
                page.count += 1;
            }
            // Draining the scratch into `Arc<[_]>` is one allocation
            // (`TrustedLen`), with the values moved, not re-cloned. The old
            // record stays alive for any older version still holding it.
            page.rows[si] = Some(row_buf.drain(..).collect());
        }
        (nremoved, nset)
    }

    /// Clear one attribute (`Some`) or drop the whole row (`None`).
    /// Returns whether anything was removed.
    fn delete(
        &mut self,
        key: u64,
        attr: Option<u16>,
    ) -> bool {
        // Peek first so an absent row never triggers a copy-on-write clone.
        match (self.get_row(key), attr) {
            (None, _) => return false,
            (Some(row), Some(idx)) if row_get(row, idx).is_none() => return false,
            _ => {}
        }
        let (ri, di, si) = decompose(key);
        let root = Arc::make_mut(&mut self.root);
        let dir = Arc::make_mut(root[ri].as_mut().expect("peeked directory"));
        let page = Arc::make_mut(dir.pages[di].as_mut().expect("peeked page"));

        let drop_row = match attr {
            None => true,
            Some(idx) => {
                let row = page.rows[si].as_ref().expect("peeked row");
                if row.len() == 1 {
                    true
                } else {
                    // Rewrite the record without the cleared attribute.
                    let mut v: Vec<(u16, Value)> = row.to_vec();
                    let j = v
                        .iter()
                        .position(|&(a, _)| a == idx)
                        .expect("peeked attribute");
                    v.remove(j);
                    page.rows[si] = Some(v.into());
                    false
                }
            }
        };
        if drop_row {
            page.rows[si] = None;
            page.count -= 1;
            if page.count == 0 {
                dir.pages[di] = None;
                Self::reclaim(root, ri);
            }
        }
        true
    }

    /// Unlink a fully-empty directory page and trim trailing empty root
    /// slots so the directory shrinks with the id space.
    fn reclaim(
        root: &mut Vec<Option<Arc<DirPage>>>,
        ri: usize,
    ) {
        if root[ri]
            .as_ref()
            .is_some_and(|d| d.pages.iter().all(Option::is_none))
        {
            root[ri] = None;
        }
        while matches!(root.last(), Some(None)) {
            root.pop();
        }
    }

    /// `(structural bytes, value payload heap bytes)` across all pages.
    fn memory_usage(&self) -> (usize, usize) {
        // Arc allocations carry a strong + weak refcount header.
        const ARC_HDR: usize = 2 * std::mem::size_of::<usize>();
        let mut structural = self.root.capacity() * std::mem::size_of::<Option<Arc<DirPage>>>();
        let mut payload = 0;
        for dir in self.root.iter().flatten() {
            structural += ARC_HDR + std::mem::size_of::<DirPage>();
            for page in dir.pages.iter().flatten() {
                structural += ARC_HDR + std::mem::size_of::<Page>();
                for row in page.rows.iter().flatten() {
                    structural += ARC_HDR + row.len() * std::mem::size_of::<(u16, Value)>();
                    for (_, v) in row.iter() {
                        payload += v.heap_size();
                    }
                }
            }
        }
        (structural, payload)
    }
}

/// Merge a row's existing attributes with a drained batch of ops (both sorted
/// by attribute index; op indices unique) into `out` (cleared first). `Some`
/// sets, `None` clears. Kept values are cloned (cheap: scalars copy, heap
/// values bump an `Arc`); `out` is a reusable scratch buffer so steady-state
/// merging allocates nothing. Counts into `(nremoved, nset)` with the store's
/// semantics: overwriting or clearing a present attribute increments
/// `nremoved`; every non-null set increments `nset`; clearing an absent
/// attribute is a no-op.
fn merge_row(
    old: &[(u16, Value)],
    ops: &mut Vec<AttrOp>,
    out: &mut Vec<(u16, Value)>,
    nremoved: &mut usize,
    nset: &mut usize,
) {
    out.clear();
    out.reserve(old.len() + ops.len());
    let mut old_it = old.iter().peekable();
    let mut op_it = ops.drain(..).peekable();
    loop {
        let oi = old_it.peek().map(|&&(i, _)| i);
        let pi = op_it.peek().map(|(i, _)| *i);
        match (oi, pi) {
            (Some(o), Some(p)) if o < p => out.push(old_it.next().expect("peeked").clone()),
            (Some(o), Some(p)) if o == p => {
                let _ = old_it.next();
                *nremoved += 1;
                if let (idx, Some(v)) = op_it.next().expect("peeked") {
                    out.push((idx, v));
                    *nset += 1;
                }
            }
            (_, Some(_)) => {
                if let (idx, Some(v)) = op_it.next().expect("peeked") {
                    out.push((idx, v));
                    *nset += 1;
                }
            }
            (Some(_), None) => out.push(old_it.next().expect("peeked").clone()),
            (None, None) => break,
        }
    }
}

/// Page cursor for batched point lookups: while consecutive ids fall in the
/// same page, only the final slot index is recomputed. Correct for any id
/// order — the cache is keyed by the id's global page number.
struct PageCursor<'a> {
    root: &'a [Option<Arc<DirPage>>],
    page_no: Option<u64>,
    page: Option<&'a Page>,
}

impl<'a> PageCursor<'a> {
    fn new(rows: &'a PagedRows) -> Self {
        Self {
            root: &rows.root,
            page_no: None,
            page: None,
        }
    }

    fn find(
        &mut self,
        id: u64,
    ) -> Option<&'a Row> {
        let page_no = id >> PAGE_SHIFT;
        if self.page_no != Some(page_no) {
            let (ri, di, _) = decompose(id);
            self.page_no = Some(page_no);
            self.page = self
                .root
                .get(ri)
                .and_then(Option::as_ref)
                .and_then(|dir| dir.pages[di].as_deref());
        }
        self.page?.rows[(id & PAGE_MASK) as usize].as_ref()
    }
}

/// In-order walk over every row in the heap (see
/// [`AttributeStore::iter_rows`]). Iterates the directory levels with plain
/// indices; `base` is the first id of the current page.
struct RowScan<'a> {
    store: &'a AttributeStore,
    root: &'a [Option<Arc<DirPage>>],
    /// Next root slot to visit.
    di: usize,
    dir: Option<&'a DirPage>,
    /// Next slot within `dir`.
    pi: usize,
    page: Option<&'a Page>,
    /// First id of the current `page`.
    base: u64,
    /// Next slot within `page`.
    si: usize,
}

impl<'a> Iterator for RowScan<'a> {
    type Item = (u64, AttrArrayView<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(page) = self.page {
                while self.si < PAGE_LEN {
                    let s = self.si;
                    self.si += 1;
                    if let Some(row) = page.rows[s].as_ref() {
                        return Some((
                            self.base + s as u64,
                            AttrArrayView {
                                store: self.store,
                                row: Some(&row[..]),
                            },
                        ));
                    }
                }
                self.page = None;
            }
            if let Some(dir) = self.dir {
                if self.pi < DIR_LEN {
                    let p = self.pi;
                    self.pi += 1;
                    if let Some(page) = dir.pages[p].as_deref() {
                        // di was already advanced past this directory.
                        let dir_first = ((self.di - 1) as u64) << (PAGE_SHIFT + DIR_SHIFT);
                        self.base = dir_first + ((p as u64) << PAGE_SHIFT);
                        self.page = Some(page);
                        self.si = 0;
                    }
                    continue;
                }
                self.dir = None;
            }
            loop {
                if self.di >= self.root.len() {
                    return None;
                }
                let d = self.di;
                self.di += 1;
                if let Some(dir) = self.root[d].as_deref() {
                    self.dir = Some(dir);
                    self.pi = 0;
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod paged_rows_tests {
    use super::*;

    fn ops_of(pairs: &[(u16, Option<Value>)]) -> Vec<AttrOp> {
        pairs.to_vec()
    }

    #[test]
    fn roundtrip_across_pages() {
        let mut heap = PagedRows::default();
        let mut row_buf = Vec::new();
        // Spans many pages and several directory pages (4096 ids each).
        const N: u64 = 100_000;
        for id in 0..N {
            let mut ops = ops_of(&[(0, Some(Value::Int(id as i64)))]);
            heap.apply_ops(id, &mut ops, &mut row_buf);
        }
        for id in 0..N {
            let row = heap.get_row(id).expect("row present");
            assert!(
                matches!(row_get(row, 0), Some(Value::Int(v)) if *v == id as i64),
                "int {id} round-trip"
            );
        }
        assert!(heap.get_row(N).is_none());
    }

    #[test]
    fn sparse_and_random_order_inserts() {
        let mut heap = PagedRows::default();
        let mut row_buf = Vec::new();
        // Descending order, sparse ids (every 7th) — order must not matter.
        for id in (0..35_000u64).rev().filter(|i| i % 7 == 0) {
            let mut ops = ops_of(&[(3, Some(Value::Int(id as i64)))]);
            heap.apply_ops(id, &mut ops, &mut row_buf);
        }
        // Pseudo-random overwrites on top.
        let mut x = 0x2545_F491_4F6C_DD1Du64;
        for _ in 0..5_000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let id = (x % 5_000) * 7;
            let mut ops = ops_of(&[(3, Some(Value::Int(-(id as i64))))]);
            heap.apply_ops(id, &mut ops, &mut row_buf);
        }
        for id in (0..35_000u64).filter(|i| i % 7 == 0) {
            let row = heap.get_row(id).expect("row present");
            let got = match row_get(row, 3) {
                Some(Value::Int(v)) => *v,
                other => panic!("unexpected {other:?}"),
            };
            assert!(got == id as i64 || got == -(id as i64));
        }
        assert!(heap.get_row(1).is_none(), "non-multiple-of-7 id absent");
    }

    #[test]
    fn merge_counts_and_clear_semantics() {
        let mut store = AttributeStore::new(0);
        let a = Arc::new("a".to_owned());
        let b = Arc::new("b".to_owned());

        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(a.clone(), Value::Int(1));
        m.insert(b.clone(), Value::Int(2));
        attrs.insert(7u64, m);
        // Fresh sets: nothing removed, two set.
        assert_eq!(store.insert_attrs(&attrs).unwrap(), (0, 2));

        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(a.clone(), Value::Int(10)); // overwrite: removed+1, set+1
        m.insert(b.clone(), Value::Null); // clear present: removed+1
        attrs.insert(7u64, m);
        assert_eq!(store.insert_attrs(&attrs).unwrap(), (2, 1));

        // Clearing an absent attribute is a no-op.
        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(b.clone(), Value::Null);
        attrs.insert(7u64, m);
        assert_eq!(store.insert_attrs(&attrs).unwrap(), (0, 0));

        // Pure clears on an entity that never existed create no pages.
        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(b.clone(), Value::Null);
        attrs.insert(1_000_000u64, m);
        assert_eq!(store.insert_attrs(&attrs).unwrap(), (0, 0));
        assert!(!store.has_attributes(1_000_000));

        assert_eq!(store.get_attr(7, &a), Some(Value::Int(10)));
        assert_eq!(store.get_attr(7, &b), None);

        // Clearing the last attribute drops the row entirely.
        assert!(store.remove_attr(7, &a).unwrap());
        assert!(!store.has_attributes(7));
    }

    #[test]
    fn snapshot_isolation_across_versions() {
        let mut v1 = AttributeStore::new(1);
        let name = Arc::new("x".to_owned());
        let mut attrs = FxHashMap::default();
        for id in 0..1_000u64 {
            let mut m = OrderMap::default();
            m.insert(name.clone(), Value::Int(id as i64));
            attrs.insert(id, m);
        }
        v1.insert_attrs(&attrs).unwrap();

        let mut v2 = v1.new_version(2);
        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(name.clone(), Value::Int(-42));
        attrs.insert(500u64, m);
        v2.insert_attrs(&attrs).unwrap();
        v2.remove_all(&(0..10u64).collect::<RoaringTreemap>());

        // v2 sees its writes...
        assert_eq!(v2.get_attr(500, &name), Some(Value::Int(-42)));
        assert!(!v2.has_attributes(3));
        // ...v1 does not.
        assert_eq!(v1.get_attr(500, &name), Some(Value::Int(500)));
        assert_eq!(v1.get_attr(3, &name), Some(Value::Int(3)));
    }

    #[test]
    fn remove_all_unlinks_and_shrinks() {
        let mut store = AttributeStore::new(0);
        let name = Arc::new("x".to_owned());
        let mut attrs = FxHashMap::default();
        for id in 0..10_000u64 {
            let mut m = OrderMap::default();
            m.insert(name.clone(), Value::Int(id as i64));
            attrs.insert(id, m);
        }
        store.insert_attrs(&attrs).unwrap();

        // Delete everything but one row; pages and directory slots must be
        // reclaimed and the heap stay fully usable.
        let victims: RoaringTreemap = (0..10_000u64).filter(|id| *id != 9_999).collect();
        store.remove_all(&victims);
        assert!(store.has_attributes(9_999));
        assert_eq!(store.get_attr(9_999, &name), Some(Value::Int(9_999)));
        for id in [0u64, 1, 4_000, 9_998] {
            assert!(!store.has_attributes(id));
        }
        // Only the surviving row's page + directory remain.
        assert_eq!(store.iter_rows().count(), 1);
        // And the emptied heap accepts fresh inserts.
        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        m.insert(name.clone(), Value::Int(1));
        attrs.insert(5u64, m);
        store.insert_attrs(&attrs).unwrap();
        assert_eq!(store.get_attr(5, &name), Some(Value::Int(1)));

        // Removing the last rows trims the root directory entirely: no pages
        // remain, only the root vector's retained capacity (a few pointers).
        store.remove_all(&[5u64, 9_999].into_iter().collect::<RoaringTreemap>());
        assert_eq!(store.iter_rows().count(), 0);
        assert!(store.structural_memory_usage() < 1024);
    }

    #[test]
    fn view_iterates_in_attr_index_order() {
        let mut store = AttributeStore::new(0);
        let names: Vec<Arc<String>> = ["a", "b", "c"]
            .iter()
            .map(|s| Arc::new((*s).to_owned()))
            .collect();
        let mut attrs = FxHashMap::default();
        let mut m = OrderMap::default();
        // Insert in non-index order; the row keeps ascending index order.
        m.insert(names[2].clone(), Value::Int(3));
        m.insert(names[0].clone(), Value::Int(1));
        m.insert(names[1].clone(), Value::Int(2));
        attrs.insert(1u64, m);
        // Register names in a/b/c order first so indices are 0/1/2.
        for n in &names {
            store.get_or_create_attr_id(n);
        }
        store.insert_attrs(&attrs).unwrap();

        let view = store.get_attrs(1);
        assert_eq!(view.len(), 3);
        let got: Vec<(u16, Value)> = view.iter().map(|(i, v)| (i, v.into_owned())).collect();
        assert_eq!(
            got,
            vec![(0, Value::Int(1)), (1, Value::Int(2)), (2, Value::Int(3))]
        );
        let named: Vec<String> = view.iter_named().map(|(n, _)| (**n).clone()).collect();
        assert_eq!(named, vec!["a", "b", "c"]);
    }

    #[test]
    fn iter_rows_is_complete_and_ordered() {
        let mut store = AttributeStore::new(0);
        let name = Arc::new("x".to_owned());
        let mut attrs = FxHashMap::default();
        // Sparse ids across several directory pages, inserted via hash-map
        // (random) order.
        for id in (0..30_000u64).step_by(3) {
            let mut m = OrderMap::default();
            m.insert(name.clone(), Value::Int(id as i64));
            attrs.insert(id, m);
        }
        store.insert_attrs(&attrs).unwrap();

        let mut seen = 0u64;
        let mut prev = None;
        for (id, view) in store.iter_rows() {
            assert!(prev < Some(id), "ascending id order");
            prev = Some(id);
            assert_eq!(view.len(), 1);
            assert!(matches!(
                view.iter().next().map(|(_, v)| v.into_owned()),
                Some(Value::Int(v)) if v == id as i64
            ));
            seen += 1;
        }
        assert_eq!(seen, 10_000);

        // Empty store yields nothing.
        assert_eq!(AttributeStore::new(0).iter_rows().count(), 0);
    }

    #[test]
    fn batch_read_with_defaults() {
        let mut store = AttributeStore::new(0);
        let name = Arc::new("x".to_owned());
        let mut attrs = FxHashMap::default();
        for id in (0..1_000u64).step_by(2) {
            let mut m = OrderMap::default();
            m.insert(name.clone(), Value::Int(id as i64));
            attrs.insert(id, m);
        }
        store.insert_attrs(&attrs).unwrap();
        let idx = store.get_attr_id(&name).unwrap() as u16;

        let keys: Vec<u64> = (0..1_000).collect();
        let mut out = Vec::new();
        store.get_attrs_by_idx_batch_into(&keys, idx, &Value::Null, &mut out);
        assert_eq!(out.len(), 1_000);
        for (i, v) in out.iter().enumerate() {
            if i % 2 == 0 {
                assert!(matches!(v, Value::Int(x) if *x == i as i64));
            } else {
                assert!(matches!(v, Value::Null));
            }
        }
    }

    #[test]
    fn structural_memory_accounts_rows() {
        let mut store = AttributeStore::new(0);
        let name = Arc::new("x".to_owned());
        let empty = store.structural_memory_usage();
        let mut attrs = FxHashMap::default();
        for id in 0..1_000u64 {
            let mut m = OrderMap::default();
            m.insert(name.clone(), Value::String(Arc::new(format!("v{id}"))));
            attrs.insert(id, m);
        }
        store.insert_attrs(&attrs).unwrap();
        assert!(store.structural_memory_usage() > empty);
        // String payload heap is counted in memory_usage but not structural.
        assert!(store.memory_usage() > store.structural_memory_usage());
    }
}
