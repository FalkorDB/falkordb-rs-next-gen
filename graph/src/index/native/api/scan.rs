//! The read iterator surface: [`IndexScan`] and the `'q`-scoped
//! [`IndexScanIter`].
//!
//! The iterator is **query-lifetime-scoped**, not `'static`
//! (query-api §2, §4): it owns
//! an `Arc` share of the committed index (mechanism A) — `Send`, reclaims by
//! `Arc`-drop, never holds an `AtomicRefCell` borrow guard — and is boxed as
//! `Box<dyn IndexScan + 'q>`. It never materializes an independent owned result
//! set; it reads the committed snapshot the query already pins.

use crate::index::native::error::Result;

use super::result::IndexHit;

/// A pull-based scan over an index, yielding [`IndexHit`]s lazily.
///
/// `Send` because read queries run on a thread pool. A scan-time error is
/// terminal for that scan; already-yielded hits stay valid.
pub trait IndexScan: Iterator<Item = Result<IndexHit>> + Send {
    /// Pull up to `n` hits into `out` in one call (amortizes per-call overhead;
    /// the scan ops accumulate until `BATCH_SIZE`). The bare [`Iterator::next`]
    /// is the one-at-a-time fallback. An error ends the scan.
    fn next_batch(
        &mut self,
        n: usize,
        out: &mut Vec<IndexHit>,
    ) -> Result<()>;
}

/// A boxed scan scoped to the **query lifetime** `'q`
/// (query-api §2). The scan ops
/// hold this; node-vs-edge never mix in one iterator.
pub type IndexScanIter<'q> = Box<dyn IndexScan + 'q>;
