//! The numeric / range index — the POC kind
//! (mvcc-core,
//! query-api).
//!
//! [`NumericIndex`] is one structure (index × field × entity-kind) over the
//! in-RAM [`MatrixStore`] core — an order-preserving banded set of GraphBLAS
//! `VersionedMatrix`es. It encodes values to sortable `u64` row keys via
//! [`NumericEncoder`], applies graph-commit mutations through the store's
//! copy-on-write `commit`, and answers [`crate::index::IndexQuery`] predicates
//! by lowering them onto inclusive encoded-key ranges, which the store resolves
//! to matrix rows scanned by [`MatrixRangeCursor`].
//!
//! Reads follow mechanism A: [`query`](NumericIndex::query) pins an `Arc` share
//! of the latest committed matrix, then hands back a [`NumericScan`] that owns
//! the share — `'q`-scoped, `Send`, reclaimed by `Arc`-drop, holding no lock
//! across the scan. The disk backend (WAL/checkpoint) and RAM↔disk tiering are
//! enterprise concerns layered on in later milestones; this is the OSS in-RAM
//! path (rebuilds on restart).

mod encoder;
mod store;

pub use encoder::NumericEncoder;

use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Bound;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use roaring::RoaringTreemap;

use crate::index::{Field, IndexInfo, IndexQuery, IndexType};
use crate::runtime::value::Value;

use super::api::{
    BoundSide, EntityKind, Index, IndexHit, IndexScan, IndexScanIter, IndexSchema, MemoryBreakdown,
    RowEncoder, ScanOptions,
};
use super::backend::StorageBackend;
use super::error::{IndexError, Result};
use super::id::DocKey;

use self::store::{MatrixRangeCursor, MatrixStore, Snapshot};

/// Poll the cancellation flag once every this many yielded hits — coarse enough
/// not to cost per-hit, fine enough to abort a runaway scan promptly.
const CANCEL_POLL_INTERVAL: u32 = 1024;

/// A numeric/range index structure over the logical-MVCC ordered store.
pub struct NumericIndex {
    schema: IndexSchema,
    encoder: NumericEncoder,
    store: MatrixStore,
    /// The OSS↔enterprise durable-bytes seam. `NullBackend` in OSS; the WAL/
    /// checkpoint wiring that uses it lands in the enterprise disk milestone.
    _backend: Arc<dyn StorageBackend>,
    progress: AtomicU64,
    total: AtomicU64,
    cancel: AtomicBool,
}

impl NumericIndex {
    /// Lower one query predicate onto the cursors that evaluate it against
    /// `matrix` (a pinned committed version). Each predicate becomes one or more
    /// inclusive encoded-key ranges `[lo, hi]`, which the cursor walks as
    /// contiguous, key-ordered matrix row-sweeps. Multiple cursors (from
    /// `InList` / `Or`) are unioned by the scan.
    fn lower(
        &self,
        q: &IndexQuery<Value>,
        matrix: &Snapshot,
        cursors: &mut Vec<MatrixRangeCursor>,
    ) -> Result<()> {
        match q {
            // Equality and array-contains are both point lookups at `encode(v)`:
            // an array doc occupies one row per element, so membership of `v` is
            // exactly the presence of the doc at the row for key `encode(v)`.
            IndexQuery::Equal { value, .. } | IndexQuery::ArrayContains { value, .. } => {
                if let Some(k) = NumericEncoder::encode_value(value) {
                    cursors.push(MatrixRangeCursor::new(Arc::clone(matrix), k, k));
                }
                // Non-numeric value against a numeric index → no matches.
                Ok(())
            }
            IndexQuery::Range {
                min,
                max,
                include_min,
                include_max,
                ..
            } => {
                let lo = self
                    .encoder
                    .encode_bound(opt_bound(min.as_ref(), *include_min), BoundSide::Lower);
                let hi = self
                    .encoder
                    .encode_bound(opt_bound(max.as_ref(), *include_max), BoundSide::Upper);
                cursors.push(MatrixRangeCursor::new(Arc::clone(matrix), lo, hi));
                Ok(())
            }
            // `IN [..]` — a union of point lookups, one per numeric element.
            IndexQuery::InList { list, .. } => {
                if let Value::List(items) = list {
                    for item in items.iter() {
                        if let Some(k) = NumericEncoder::encode_value(item) {
                            cursors.push(MatrixRangeCursor::new(Arc::clone(matrix), k, k));
                        }
                    }
                }
                Ok(())
            }
            // Disjunction — flatten each arm onto the same cursor union.
            IndexQuery::Or(subs) => {
                for sub in subs {
                    self.lower(sub, matrix, cursors)?;
                }
                Ok(())
            }
            // Conjunction and geo are not part of the numeric POC: `And` needs
            // row-range intersection, `Point` is the geo kind.
            IndexQuery::And(_) => Err(IndexError::Unsupported("numeric index: And predicate")),
            IndexQuery::Point { .. } => {
                Err(IndexError::Unsupported("numeric index: Point predicate"))
            }
        }
    }
}

/// Build a `Bound<&Value>` from an optional range endpoint: absent → open,
/// present → inclusive or exclusive per the predicate's flag.
fn opt_bound(
    v: Option<&Value>,
    inclusive: bool,
) -> Bound<&Value> {
    match v {
        None => Bound::Unbounded,
        Some(v) if inclusive => Bound::Included(v),
        Some(v) => Bound::Excluded(v),
    }
}

impl Index for NumericIndex {
    fn create(
        schema: &IndexSchema,
        backend: Arc<dyn StorageBackend>,
    ) -> Self {
        Self {
            schema: schema.clone(),
            encoder: NumericEncoder,
            store: MatrixStore::new(),
            _backend: backend,
            progress: AtomicU64::new(0),
            total: AtomicU64::new(0),
            cancel: AtomicBool::new(false),
        }
    }

    fn info(&self) -> IndexInfo {
        let mut fields: HashMap<Arc<String>, Vec<Arc<Field>>> = HashMap::new();
        let mut field_order = Vec::with_capacity(self.schema.fields.len());
        for name in &self.schema.fields {
            let cname = CString::new(name.as_str()).unwrap_or_default();
            let field = Arc::new(Field::new(cname, IndexType::Range, None));
            fields.insert(Arc::clone(name), vec![field]);
            field_order.push(Arc::clone(name));
        }
        IndexInfo {
            label: Arc::clone(&self.schema.label),
            pending: 0,
            progress: self.progress.load(Ordering::Relaxed),
            total: self.total.load(Ordering::Relaxed),
            fields,
            field_order,
            language: None,
            stopwords: None,
            entity_type: match self.schema.entity {
                EntityKind::Node => "NODE",
                EntityKind::Edge => "RELATIONSHIP",
            }
            .to_string(),
        }
    }

    fn commit(
        &self,
        version: u64,
        add: &[(DocKey, Value)],
        remove: &[DocKey],
    ) -> Result<()> {
        let mut adds: Vec<(DocKey, Vec<u64>)> = Vec::with_capacity(add.len());
        for (doc, value) in add {
            let mut keys = Vec::new();
            self.encoder.encode(value, &mut keys);
            adds.push((*doc, keys));
        }
        self.store.commit(version, &adds, remove);
        Ok(())
    }

    fn query<'q>(
        &'q self,
        q: &IndexQuery<Value>,
        opts: ScanOptions,
    ) -> Result<IndexScanIter<'q>> {
        // Pin the latest committed matrix version; the scan owns the `Arc` share
        // and never holds a lock across iteration (mechanism A).
        let matrix = self.store.snapshot();
        let mut cursors = Vec::new();
        self.lower(q, &matrix, &mut cursors)?;
        Ok(Box::new(NumericScan::new(cursors, opts)))
    }

    fn memory_usage(&self) -> MemoryBreakdown {
        MemoryBreakdown {
            matrix_bytes: self.store.memory_usage(),
            payload_bytes: 0,
        }
    }

    fn update_progress(
        &self,
        done: u64,
        total: u64,
    ) {
        self.progress.store(done, Ordering::Relaxed);
        self.total.store(total, Ordering::Relaxed);
    }

    fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }

    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }

    fn try_adopt(
        &self,
        _v_graph: u64,
    ) -> Result<bool> {
        // OSS keeps nothing durable, so there is never persisted state to adopt:
        // the caller must repopulate from the graph. The enterprise build adopts
        // via its `FjallBackend` in a later milestone.
        Ok(false)
    }
}

/// A `'q`-scoped pull scan over one or more [`RangeCursor`]s, unioning their doc
/// ids (deduplicated when there is more than one cursor) into [`IndexHit`]s.
pub struct NumericScan {
    cursors: Vec<MatrixRangeCursor>,
    idx: usize,
    /// Dedup set across cursors — allocated only when a union can produce the
    /// same doc twice (`InList` / `Or`). A single cursor never repeats a doc.
    seen: Option<RoaringTreemap>,
    yielded: usize,
    max_results: Option<usize>,
    cancel: Option<Arc<AtomicBool>>,
    cancel_tick: u32,
    done: bool,
}

impl NumericScan {
    fn new(
        cursors: Vec<MatrixRangeCursor>,
        opts: ScanOptions,
    ) -> Self {
        let seen = (cursors.len() > 1).then(RoaringTreemap::new);
        Self {
            cursors,
            idx: 0,
            seen,
            yielded: 0,
            max_results: opts.max_results,
            cancel: opts.cancel,
            cancel_tick: 0,
            done: false,
        }
    }

    /// Coarse cancellation poll — checked once per [`CANCEL_POLL_INTERVAL`].
    fn cancelled(&mut self) -> bool {
        let Some(flag) = &self.cancel else {
            return false;
        };
        self.cancel_tick = self.cancel_tick.wrapping_add(1);
        self.cancel_tick.is_multiple_of(CANCEL_POLL_INTERVAL) && flag.load(Ordering::Relaxed)
    }
}

impl Iterator for NumericScan {
    type Item = Result<IndexHit>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.max_results.is_some_and(|max| self.yielded >= max) {
            self.done = true;
            return None;
        }
        loop {
            if self.cancelled() {
                self.done = true;
                return Some(Err(IndexError::Cancelled));
            }
            let Some(cursor) = self.cursors.get_mut(self.idx) else {
                self.done = true;
                return None;
            };
            match cursor.next_id() {
                Some(id) => {
                    if let Some(seen) = &mut self.seen {
                        // `insert` returns false if the id was already present.
                        if !seen.insert(id) {
                            continue;
                        }
                    }
                    self.yielded += 1;
                    return Some(Ok(IndexHit { id, score: None }));
                }
                None => self.idx += 1, // this cursor is exhausted; try the next
            }
        }
    }
}

impl IndexScan for NumericScan {
    fn next_batch(
        &mut self,
        n: usize,
        out: &mut Vec<IndexHit>,
    ) -> Result<()> {
        for _ in 0..n {
            match self.next() {
                Some(Ok(hit)) => out.push(hit),
                Some(Err(e)) => return Err(e),
                None => break,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use thin_vec::ThinVec;

    use crate::index::native::backend::NullBackend;

    fn key(name: &str) -> Arc<String> {
        Arc::new(name.to_string())
    }

    fn schema() -> IndexSchema {
        IndexSchema {
            index_id: 1,
            entity: EntityKind::Node,
            label: key("Person"),
            fields: vec![key("age")],
        }
    }

    fn idx() -> NumericIndex {
        crate::index::native::test_init_graphblas();
        NumericIndex::create(&schema(), Arc::new(NullBackend))
    }

    fn ids(
        index: &NumericIndex,
        q: &IndexQuery<Value>,
    ) -> Vec<DocKey> {
        let mut scan = index.query(q, ScanOptions::default()).unwrap();
        let mut out = Vec::new();
        for hit in scan.by_ref() {
            out.push(hit.unwrap().id);
        }
        out.sort_unstable();
        out
    }

    fn equal(v: Value) -> IndexQuery<Value> {
        IndexQuery::Equal {
            key: key("age"),
            value: v,
        }
    }

    fn range(
        min: Option<Value>,
        max: Option<Value>,
        include_min: bool,
        include_max: bool,
    ) -> IndexQuery<Value> {
        IndexQuery::Range {
            key: key("age"),
            min,
            max,
            include_min,
            include_max,
        }
    }

    #[test]
    fn equality_finds_matching_docs_int_float_interleave() {
        let index = idx();
        index
            .commit(
                1,
                &[
                    (1, Value::Int(30)),
                    (2, Value::Float(30.0)), // same row as Int(30)
                    (3, Value::Int(40)),
                ],
                &[],
            )
            .unwrap();
        assert_eq!(ids(&index, &equal(Value::Int(30))), vec![1, 2]);
        assert_eq!(ids(&index, &equal(Value::Float(30.0))), vec![1, 2]);
        assert_eq!(ids(&index, &equal(Value::Int(40))), vec![3]);
        assert_eq!(ids(&index, &equal(Value::Int(99))), Vec::<DocKey>::new());
    }

    #[test]
    fn closed_and_half_open_ranges() {
        let index = idx();
        for (doc, v) in [(1, 10), (2, 20), (3, 30), (4, 40)] {
            index.commit(doc, &[(doc, Value::Int(v))], &[]).unwrap();
        }
        // [20, 30] inclusive
        assert_eq!(
            ids(
                &index,
                &range(Some(Value::Int(20)), Some(Value::Int(30)), true, true)
            ),
            vec![2, 3]
        );
        // (20, 40] — drop the 20 endpoint
        assert_eq!(
            ids(
                &index,
                &range(Some(Value::Int(20)), Some(Value::Int(40)), false, true)
            ),
            vec![3, 4]
        );
        // open lower: <= 20
        assert_eq!(
            ids(&index, &range(None, Some(Value::Int(20)), true, true)),
            vec![1, 2]
        );
        // open upper: > 20
        assert_eq!(
            ids(&index, &range(Some(Value::Int(20)), None, false, true)),
            vec![3, 4]
        );
    }

    #[test]
    fn in_list_unions_and_dedups() {
        let index = idx();
        index
            .commit(
                1,
                &[
                    (1, Value::Int(10)),
                    (2, Value::Int(20)),
                    (3, Value::Int(30)),
                ],
                &[],
            )
            .unwrap();
        let q = IndexQuery::InList {
            key: key("age"),
            // 10 listed twice — the doc must still appear once.
            list: Value::List(Arc::new(ThinVec::from(vec![
                Value::Int(10),
                Value::Int(30),
                Value::Int(10),
            ]))),
        };
        assert_eq!(ids(&index, &q), vec![1, 3]);
    }

    #[test]
    fn array_property_and_array_contains() {
        let index = idx();
        // doc 1 has [10, 20, 30]; doc 2 has [20].
        index
            .commit(
                1,
                &[
                    (
                        1,
                        Value::List(Arc::new(ThinVec::from(vec![
                            Value::Int(10),
                            Value::Int(20),
                            Value::Int(30),
                        ]))),
                    ),
                    (
                        2,
                        Value::List(Arc::new(ThinVec::from(vec![Value::Int(20)]))),
                    ),
                ],
                &[],
            )
            .unwrap();
        let contains = |v: i64| IndexQuery::ArrayContains {
            key: key("age"),
            value: Value::Int(v),
        };
        assert_eq!(ids(&index, &contains(20)), vec![1, 2]);
        assert_eq!(ids(&index, &contains(10)), vec![1]);
        assert_eq!(ids(&index, &contains(99)), Vec::<DocKey>::new());
    }

    #[test]
    fn update_and_remove() {
        let index = idx();
        index.commit(1, &[(1, Value::Int(10))], &[]).unwrap();
        // update doc 1: 10 -> 50
        index.commit(2, &[(1, Value::Int(50))], &[]).unwrap();
        assert_eq!(ids(&index, &equal(Value::Int(10))), Vec::<DocKey>::new());
        assert_eq!(ids(&index, &equal(Value::Int(50))), vec![1]);
        // remove doc 1
        index.commit(3, &[], &[1]).unwrap();
        assert_eq!(ids(&index, &equal(Value::Int(50))), Vec::<DocKey>::new());
    }

    #[test]
    fn scan_pinned_before_commit_is_isolated() {
        let index = idx();
        index.commit(1, &[(1, Value::Int(10))], &[]).unwrap();
        // Build the scan (pins version 1) BEFORE the next commit.
        let mut scan = index
            .query(&equal(Value::Int(10)), ScanOptions::default())
            .unwrap();
        index.commit(2, &[(2, Value::Int(10))], &[]).unwrap();
        // The pinned scan must not see doc 2 added after it was created.
        let seen: Vec<DocKey> = scan.by_ref().map(|h| h.unwrap().id).collect();
        assert_eq!(seen, vec![1]);
    }

    #[test]
    fn max_results_caps_the_scan() {
        let index = idx();
        for d in 1..=10u64 {
            index.commit(d, &[(d, Value::Int(5))], &[]).unwrap();
        }
        let q = equal(Value::Int(5));
        let opts = ScanOptions {
            max_results: Some(3),
            ..Default::default()
        };
        let scan = index.query(&q, opts).unwrap();
        assert_eq!(scan.count(), 3);
    }

    #[test]
    fn cancelled_scan_yields_error_then_stops() {
        use std::sync::atomic::AtomicBool;
        let index = idx();
        // Enough docs that the cancel poll (every 1024) trips.
        for d in 1..=3000u64 {
            index.commit(d, &[(d, Value::Int(5))], &[]).unwrap();
        }
        let flag = Arc::new(AtomicBool::new(true));
        let opts = ScanOptions {
            cancel: Some(Arc::clone(&flag)),
            ..Default::default()
        };
        let mut scan = index.query(&equal(Value::Int(5)), opts).unwrap();
        let mut saw_cancel = false;
        for item in scan.by_ref() {
            if item.is_err() {
                saw_cancel = true;
                break;
            }
        }
        assert!(saw_cancel, "expected a Cancelled error from the scan");
        assert!(scan.next().is_none(), "scan must be terminal after cancel");
    }

    #[test]
    fn unsupported_predicates_error() {
        let index = idx();
        let and = IndexQuery::And(vec![equal(Value::Int(1))]);
        assert!(matches!(
            index.query(&and, ScanOptions::default()),
            Err(IndexError::Unsupported(_))
        ));
    }

    #[test]
    fn memory_usage_grows_with_postings() {
        let index = idx();
        // A GraphBLAS matrix reports nonzero header bytes even when empty, so
        // compare before/after rather than against zero.
        let before = index.memory_usage().matrix_bytes;
        index.commit(1, &[(1, Value::Int(10))], &[]).unwrap();
        assert!(index.memory_usage().matrix_bytes >= before);
        assert!(index.memory_usage().matrix_bytes > 0);
    }
}
