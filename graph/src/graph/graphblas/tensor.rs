//! 3D sparse tensor for multi-edge relationship storage.
//!
//! This module provides [`Tensor`], which extends the adjacency matrix model
//! to support multiple edges of the same type between the same pair of nodes.
//! While a plain adjacency matrix can only record whether an edge exists,
//! the tensor stores individual edge IDs so that each edge can carry its own
//! properties.
//!
//! ## Internal Structure
//!
//! Mirrors FalkorDB C's tensor design: the forward adjacency is a UINT64
//! matrix whose entries are tagged words ([`VersionedVector`]) — either a
//! scalar edge id (MSB clear) or a pointer to a heap-allocated vector of all
//! edge ids of a multi-edge pair (MSB set).
//!
//! ```text
//!   Tensor
//!     |
//!     |-- m  (forward adjacency)   src --> dst  (UINT64 tagged word)
//!     |-- mt (backward adjacency)  dst --> src  (boolean, structure only)
//!
//!   m[s, d] = edge_id                 single edge
//!   m[s, d] = ptr | MSB               multi edge (ids live in the inner)
//! ```
//!
//! ## MVCC
//!
//! The matrices get snapshot isolation from [`VersionedMatrix`] (delta
//! tracking + `Cow`). The heap vectors behind multi-edge entries are *shared*
//! across snapshots (`GrB_Matrix_dup` bit-copies the tagged words), so they
//! carry their own version stacks — see [`VersionedVector`]. Each `Tensor`
//! value is pinned to an **epoch**: a fresh, never-reused number drawn from
//! the global counter ([`next_epoch`]) on every [`Tensor::dup`] (i.e. per
//! write transaction). Reads resolve multi-edge ids at that epoch; writes tag
//! new vector versions with it; [`Tensor::commit`] marks every inner touched
//! by the transaction as committed.
//!
//! ## Iteration
//!
//! [`Iter`] walks the forward (or backward) adjacency matrix and yields
//! `(src, dst, edge_id)` triples, expanding multi-edge entries in ascending
//! edge-id order.

use std::ptr::NonNull;

use rustc_hash::FxHashMap;

use crate::graph::graphblas::{
    matrix::{BoolExtract, Uint64Extract},
    versioned_vector::{
        IdsIter, TensorEntryVector, TensorEntryVectorRef, VersionedVector, VersionedVectorInner,
        next_epoch, release_inner,
    },
};

use super::{
    matrix::{Dup, Matrix},
    serialization::{Decode, Encode, Reader, Writer},
    versioned_matrix::{self, VersionedMatrix},
};

/// Maximum GraphBLAS index value (2^60 - 1).
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;

/// Pack a `(src, dst)` node-id pair into a compound key, used by the
/// graph-wide `edge_id → endpoints` reverse index.
///
/// The encoding `(src << 32) | dst` reserves 32 bits for each side, so both
/// values must fit in a `u32`. We check this unconditionally (not just under
/// `debug_assert!`) because silent truncation would corrupt the key and
/// conflate edges between different node pairs.
#[inline]
#[must_use]
pub fn compound_key(
    src: u64,
    dst: u64,
) -> u64 {
    assert!(
        u32::try_from(src).is_ok() && u32::try_from(dst).is_ok(),
        "Tensor compound key overflow: src={src}, dst={dst} (each must fit in u32)",
    );
    (src << 32) | dst
}

/// MSB flag used by C FalkorDB to mark multi-edge entries in the serialized
/// UINT64 forward matrix (`count | MSB`).
const MSB_MASK: u64 = 1u64 << 63;

/// Edge storage for one relationship type, with inline edge ids.
///
/// The forward adjacency `m` is UINT64-valued: each entry is a
/// [`VersionedVector`] tagged word (scalar edge id, or pointer to the pair's
/// multi-edge id vector). The backward adjacency `mt` is structure-only.
///
/// Invariants:
/// - `mt[d, s]` has an entry iff `m[s, d]` does.
/// - `count` == total edges visible at `epoch`.
/// - `multi_pair_count` == number of pairs whose entry is a vector word.
pub struct Tensor {
    /// Forward adjacency (src → dst), UINT64 tagged word.
    m: VersionedMatrix<VersionedVector>,
    /// Backward adjacency (dst → src), BOOL structure only. Edge ids are never
    /// stored here — they are recovered from `m` when iterating incoming
    /// edges, avoiding a redundant copy of every id.
    mt: VersionedMatrix<bool>,
    /// Epoch this tensor value reads/writes multi-edge vectors at. Fresh per
    /// write transaction (see [`Tensor::dup`]); never reused, even after
    /// rollback.
    epoch: u64,
    /// Inners mutated by this (writer) tensor, to be flipped to committed by
    /// [`Tensor::commit`]. Each entry holds a +1 refcount on its inner
    /// (released by commit or by [`Drop`] on rollback), so a promotion
    /// followed by a demotion in the same transaction can't leave a dangling
    /// pointer here. Never marked committed on rollback.
    touched: Vec<NonNull<VersionedVectorInner>>,
    /// Total edge count visible at `epoch`.
    count: u64,
    /// Number of pairs whose entry is a multi-edge vector word.
    multi_pair_count: u64,
}

// `touched` holds refcounted pointers to inners whose version stacks are
// mutex-guarded.
unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

impl Drop for Tensor {
    fn drop(&mut self) {
        // Rollback path: commit never drained `touched`, drop its refs.
        for inner in self.touched.drain(..) {
            unsafe { release_inner(inner) };
        }
    }
}

impl Tensor {
    #[must_use]
    pub fn new(
        nrows: u64,
        ncols: u64,
    ) -> Self {
        Self {
            m: VersionedMatrix::<VersionedVector>::new(nrows, ncols),
            mt: VersionedMatrix::<bool>::new(ncols, nrows),
            epoch: next_epoch(),
            touched: Vec::new(),
            count: 0,
            multi_pair_count: 0,
        }
    }

    /// Edge ids for the `(src, dest)` pair visible at this tensor's epoch, in
    /// ascending edge-id order. Lock-free: a multi-edge pair's visible
    /// vector is read in place (see
    /// [`VersionedVector::ids`](super::versioned_vector::VersionedVector::ids)).
    #[must_use]
    pub fn get(
        &self,
        src: u64,
        dest: u64,
    ) -> IdsIter {
        self.m
            .get(src, dest)
            .map_or_else(IdsIter::empty, |vv| vv.ids(self.epoch))
    }

    /// Record an inner mutated by this transaction, taking a +1 refcount so
    /// it stays alive until commit/rollback even if its word is later
    /// removed from the matrices.
    fn touch(
        &mut self,
        inner: NonNull<VersionedVectorInner>,
    ) {
        unsafe { inner.as_ref() }.retain();
        self.touched.push(inner);
    }

    pub fn set(
        &mut self,
        src: u64,
        dest: u64,
        id: u64,
    ) {
        match self.m.get(src, dest) {
            None => {
                // First edge for this pair: store the id inline.
                self.m.set(src, dest, VersionedVector::new_scalar(id));
                self.mt.set(dest, src, true);
            }
            Some(vv) if vv.is_scalar() => {
                // Second edge: promote scalar → vector.
                let vec = VersionedVector::new_vec(self.epoch, [vv.scalar(), id]);
                self.touch(vec.inner());
                self.m.set(src, dest, vec);
                self.multi_pair_count += 1;
            }
            Some(vv) => {
                let inner = vv.push(self.epoch, id);
                self.touch(inner);
            }
        }
        self.count += 1;
    }

    /// Set entries from parallel slices. The first edge of each pair lands
    /// inline in `m`/`mt` as a scalar word; pairs that gain additional edges
    /// get (or extend) a multi-edge vector.
    ///
    /// Membership probes run before any write to `m`, so `m` syncs pending
    /// GraphBLAS work at most once for the whole batch (a per-edge
    /// get-after-set pattern would re-sync per edge, going quadratic).
    /// In-batch duplicates are caught by batch-local maps, keeping the cost
    /// O(batch) instead of scanning all committed pairs.
    pub fn set_all_from_slices(
        &mut self,
        srcs: &[u64],
        dsts: &[u64],
        ids: &[u64],
    ) {
        debug_assert_eq!(srcs.len(), dsts.len());
        debug_assert_eq!(srcs.len(), ids.len());
        if srcs.is_empty() {
            return;
        }

        // Pairs first seen in this batch → all their ids (queued below).
        let mut new_pairs: FxHashMap<(u64, u64), Vec<u64>> = FxHashMap::default();
        // Committed/pending scalar pairs gaining edges this batch:
        // [inline first id, new ids...].
        let mut promoted: FxHashMap<(u64, u64), Vec<u64>> = FxHashMap::default();

        for ((&s, &d), &id) in srcs.iter().zip(dsts.iter()).zip(ids.iter()) {
            if let Some(pair_ids) = new_pairs.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(pair_ids) = promoted.get_mut(&(s, d)) {
                pair_ids.push(id);
            } else if let Some(vv) = self.m.get(s, d) {
                if vv.is_scalar() {
                    promoted.insert((s, d), vec![vv.scalar(), id]);
                } else {
                    let inner = vv.push(self.epoch, id);
                    self.touch(inner);
                }
            } else {
                new_pairs.insert((s, d), vec![id]);
            }
        }
        self.count += srcs.len() as u64;

        // New pairs: single-edge pairs go through the bulk scalar path;
        // in-batch multi-edge pairs get a vector word directly.
        let mut m_srcs: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut m_dsts: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut m_ids: Vec<u64> = Vec::with_capacity(new_pairs.len());
        let mut mt_entries: Vec<(u64, u64)> = Vec::with_capacity(new_pairs.len());
        for ((s, d), pair_ids) in new_pairs {
            mt_entries.push((d, s));
            if let [id] = pair_ids[..] {
                m_srcs.push(s);
                m_dsts.push(d);
                m_ids.push(id);
            } else {
                let vec = VersionedVector::new_vec(self.epoch, pair_ids);
                self.touch(vec.inner());
                // Pair was effectively absent, so the dp word is absent too
                // (no-shadow invariant) — safe to skip the release probe.
                self.m.set_fresh(s, d, vec);
                self.multi_pair_count += 1;
            }
        }
        for ((s, d), pair_ids) in promoted {
            let vec = VersionedVector::new_vec(self.epoch, pair_ids);
            self.touch(vec.inner());
            // Old effective word was a scalar, never a pointer — no release.
            self.m.set_fresh(s, d, vec);
            self.multi_pair_count += 1;
        }

        self.m.set_all(
            m_srcs
                .iter()
                .zip(m_dsts.iter())
                .zip(m_ids.iter())
                .map(|((&s, &d), &id)| (s, d, id)),
        );
        self.mt.set_all(mt_entries.into_iter());
    }

    /// Bulk-remove specific edges from this tensor.
    ///
    /// Each entry in `rels` is `(edge_id, src, dst)`.
    /// Returns the list of `(src, dst)` pairs that became completely empty
    /// in this tensor (no remaining edges of this type between those nodes).
    pub fn remove_all(
        &mut self,
        rels: &[(u64, u64, u64)],
    ) -> Vec<(u64, u64)> {
        if rels.is_empty() {
            return Vec::new();
        }

        // Fast path: no multi-edge pairs exist, so every edge is the scalar
        // entry of its pair. Bulk-remove from the forward/backward adjacency
        // in two GraphBLAS ops; every touched pair becomes empty.
        if !self.has_multi_edge() {
            let nrows = self.m.nrows();
            let ncols = self.m.ncols();
            let mut m_rows = Vec::with_capacity(rels.len());
            let mut m_cols = Vec::with_capacity(rels.len());
            let mut mt_rows = Vec::with_capacity(rels.len());
            let mut mt_cols = Vec::with_capacity(rels.len());
            for &(_, src, dst) in rels {
                m_rows.push(src);
                m_cols.push(dst);
                mt_rows.push(dst);
                mt_cols.push(src);
            }
            let mut m_mask = Matrix::<bool>::new(nrows, ncols);
            m_mask.build(&m_rows, &m_cols);
            let mut mt_mask = Matrix::<bool>::new(ncols, nrows);
            mt_mask.build(&mt_rows, &mt_cols);
            self.m.remove_mask(&m_mask);
            self.mt.remove_mask(&mt_mask);
            self.count -= rels.len() as u64;
            return rels.iter().map(|&(_, src, dst)| (src, dst)).collect();
        }

        // Slow path: some pairs have multi-edge vectors. Handle per edge:
        //  - scalar entry: the pair becomes empty.
        //  - vector entry: drop the id from the vector at this epoch; demote
        //    to a scalar word when one id remains, empty the pair at zero.
        let mut emptied = Vec::new();
        for &(id, src, dst) in rels {
            let Some(vv) = self.m.get(src, dst) else {
                debug_assert!(false, "removing edge {id} from missing pair ({src}, {dst})");
                continue;
            };
            if vv.is_scalar() {
                debug_assert_eq!(vv.scalar(), id);
                self.m.remove(src, dst);
                self.mt.remove(dst, src);
                emptied.push((src, dst));
            } else {
                let (remaining, inner) = vv.remove(self.epoch, id);
                self.touch(inner);
                if remaining == 0 {
                    self.m.remove(src, dst);
                    self.mt.remove(dst, src);
                    self.multi_pair_count -= 1;
                    emptied.push((src, dst));
                } else if remaining == 1 {
                    // Demote back to an inline scalar; old snapshots keep
                    // reading the vector word from their own matrix copy.
                    let last = vv.ids(self.epoch).next().unwrap();
                    self.m.set(src, dst, VersionedVector::new_scalar(last));
                    self.multi_pair_count -= 1;
                }
            }
            self.count -= 1;
        }
        emptied
    }

    pub fn resize(
        &mut self,
        nrows: u64,
        ncols: u64,
    ) {
        self.m.resize(nrows, ncols);
        self.mt.resize(ncols, nrows);
    }

    /// Rebuild the backward matrix as the transpose of the forward matrix.
    ///
    /// `mt` is structure-only (`bool`). The forward matrix's *effective*
    /// structure (`(m − dm) ∪ dp`) is materialized first, then transposed into
    /// a clean base with empty deltas.
    pub fn rebuild_backward(&mut self) {
        self.mt = VersionedMatrix::from_matrix(self.m.extract().transpose());
    }

    /// Snapshot this tensor for a new write transaction: `Cow`-shares the
    /// matrices and pins the copy to a fresh, never-reused epoch.
    #[must_use]
    pub fn dup(&self) -> Self {
        Self {
            m: self.m.dup(),
            mt: self.mt.dup(),
            epoch: next_epoch(),
            touched: Vec::new(),
            count: self.count,
            multi_pair_count: self.multi_pair_count,
        }
    }

    /// Mark every multi-edge vector version written by this transaction as
    /// committed, making it visible to future epochs. Called from the MVCC
    /// commit path; never called on rollback (uncommitted versions stay
    /// invisible and are pruned lazily).
    pub fn commit(&mut self) {
        for inner in self.touched.drain(..) {
            unsafe { inner.as_ref() }.mark_committed(self.epoch);
            unsafe { release_inner(inner) };
        }
    }

    /// Forward adjacency (src → dst); UINT64 tagged words. Structural
    /// consumers (`ANY_PAIR` mxm, masks) may use it directly.
    #[must_use]
    pub const fn matrix(&self) -> &VersionedMatrix<VersionedVector> {
        &self.m
    }

    /// Transposed/backward pair-level adjacency (dst → src), structure only.
    #[must_use]
    pub const fn matrix_t(&self) -> &VersionedMatrix<bool> {
        &self.mt
    }

    /// Epoch this tensor reads multi-edge vectors at.
    #[must_use]
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Total number of edges in this tensor.
    #[must_use]
    pub const fn edge_count(&self) -> u64 {
        self.count
    }

    /// Iterate every `(src, dst, edge_id)` triple in the tensor.
    ///
    /// Streams the forward matrix, expanding multi-edge vector entries at
    /// this tensor's epoch. On a single-edge graph this is one streaming pass
    /// over `m` with no per-pair work.
    pub fn iter_edges(&self) -> impl Iterator<Item = (u64, u64, u64)> + '_ {
        let epoch = self.epoch;
        self.m.iter(0, u64::MAX).flat_map(move |(src, dst, raw)| {
            VersionedVector::from_raw(raw)
                .ids(epoch)
                .map(move |id| (src, dst, id))
        })
    }

    #[must_use]
    pub fn iter(
        &self,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Iter<'_> {
        Iter::new(self, min_row, max_row, transpose)
    }

    /// Whether this tensor has any (src, dst) pair with more than one edge.
    #[must_use]
    pub const fn has_multi_edge(&self) -> bool {
        self.multi_pair_count != 0
    }

    pub fn wait(&mut self) {
        self.m.wait();
        self.mt.wait();
    }

    /// Wait on all matrices for fork safety (takes &self, not &mut self).
    pub fn wait_all(&self) {
        self.m.wait_all();
        self.mt.wait_all();
    }

    /// Returns true if every internal matrix has no pending GraphBLAS
    /// operations queued.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.m.is_synced() && self.mt.is_synced()
    }

    #[must_use]
    pub fn memory_usage(&self) -> usize {
        let mut usage = self.m.memory_usage() + self.mt.memory_usage();
        if self.multi_pair_count != 0 {
            for (_, _, raw) in self.m.iter(0, u64::MAX) {
                usage += VersionedVector::from_raw(raw).memory_usage();
            }
        }
        usage
    }
}

impl Encode<19> for Tensor {
    #[allow(clippy::similar_names)]
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        let nrows = self.m.nrows();
        let ncols = self.m.ncols();

        // Serialize the C-compatible UINT64 forward matrix from the effective
        // state. Single-edge pairs store the edge id directly; multi-edge
        // pairs store `(edge_count | MSB)` and push their full id list into
        // the tensor section below.
        let mut f_rows: Vec<u64> = Vec::new();
        let mut f_cols: Vec<u64> = Vec::new();
        let mut f_vals: Vec<u64> = Vec::new();
        let mut multi: Vec<(u64, u64, VersionedVector)> = Vec::new();
        for (src, dst, raw) in self.m.iter(0, u64::MAX) {
            let vv = VersionedVector::from_raw(raw);
            f_rows.push(src);
            f_cols.push(dst);
            if vv.is_scalar() {
                f_vals.push(vv.scalar());
            } else {
                let n = vv.count(self.epoch);
                if n == 1 {
                    f_vals.push(vv.ids(self.epoch).next().unwrap());
                } else {
                    f_vals.push(n | MSB_MASK);
                    multi.push((src, dst, vv));
                }
            }
        }

        // Forward VersionedMatrix layout: base (effective), empty delta-plus,
        // empty delta-minus. Folding dp into the base keeps the on-disk form
        // canonical and matches what decode expects.
        let empty = Matrix::<VersionedVector>::new(nrows, ncols);
        if f_rows.is_empty() {
            empty.encode(w);
        } else {
            let mut fm = Matrix::<VersionedVector>::new(nrows, ncols);
            fm.build(&f_rows, &f_cols, &f_vals);
            fm.encode(w);
        }
        empty.encode(w); // delta-plus
        empty.encode(w); // delta-minus

        // Tensor section (C v19 format): leading word = number of multi-edge
        // pairs; zero means nothing else follows. Otherwise two groups (base
        // TM, then delta-plus TDP), each a count followed by per-pair
        // (src, dst, GxB_Vector_serialize blob of the BOOL id vector). All
        // multi-edge pairs live in the base group; the delta-plus group is
        // empty since dp was folded into the base above.
        w.write_unsigned(multi.len() as u64);
        if multi.is_empty() {
            return;
        }
        w.write_unsigned(multi.len() as u64);
        for (src, dst, vv) in &multi {
            w.write_unsigned(*src);
            w.write_unsigned(*dst);
            vv.read_visible(self.epoch, |v| TensorEntryVectorRef(v).encode(w))
                .expect("multi-edge entry must have a visible version");
        }
        w.write_unsigned(0); // empty delta-plus tensor group
    }
}

impl Decode<19> for Tensor {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let forward = VersionedMatrix::<VersionedVector>::decode(r)?;
        let nrows = forward.nrows();
        let ncols = forward.ncols();

        // The on-disk forward matrix (C-compatible) stores single-edge ids
        // directly (MSB clear) and `(count | MSB)` for multi-edge pairs,
        // whose real id lists follow in the tensor section. Note `forward`
        // holds count words, not pointers, so it must stay non-owning
        // (`owns_inners` false, the decode default); the real pointer words
        // are built into the fresh, owning `m` below.
        let mut m = VersionedMatrix::<VersionedVector>::new(nrows, ncols);
        let mut count = 0u64;
        let mut multi_pair_count = 0u64;

        m.set_all(
            forward
                .iter(0, u64::MAX)
                .filter(|&(_, _, value)| value & MSB_MASK == 0)
                .inspect(|_| count += 1),
        );

        // Tensor section (C v19 format): number of multi-edge pairs, then —
        // only when non-zero — two groups (base TM, delta-plus TDP) of
        // (src, dst, GxB blob of the BOOL id vector) entries.
        let total_tensors = r.read_unsigned()?;
        if total_tensors > 0 {
            for _ in 0..2 {
                let group_count = r.read_unsigned()?;
                for _ in 0..group_count {
                    let src = r.read_unsigned()?;
                    let dst = r.read_unsigned()?;
                    let v = TensorEntryVector::decode(r)?.0;
                    count += v.nvals();
                    // Decoded state is the committed baseline: epoch 0 (below
                    // any writer epoch) marked committed, visible to everyone.
                    let vec = VersionedVector::from_committed(v);
                    m.set(src, dst, vec);
                    multi_pair_count += 1;
                }
            }
        }
        debug_assert_eq!(multi_pair_count, total_tensors);

        // Backward matrix is rebuilt from `m` by the caller (`rebuild_backward`)
        // after decode, so leave it empty here.
        Ok(Self {
            m,
            mt: VersionedMatrix::<bool>::new(0, 0),
            epoch: next_epoch(),
            touched: Vec::new(),
            count,
            multi_pair_count,
        })
    }
}

/// Base adjacency iterator. Forward iteration streams tagged words directly
/// from `m`; backward iteration streams the BOOL structure of `mt` (which
/// carries no ids) and recovers each word from `m`.
enum BaseIter {
    Forward(versioned_matrix::Iter<Uint64Extract>),
    Backward(versioned_matrix::Iter<BoolExtract>),
}

pub struct Iter<'a> {
    t: &'a Tensor,
    base: BaseIter,
    src: u64,
    dest: u64,
    /// Buffered, ascending edge ids for the current multi-edge pair.
    buf: Vec<u64>,
    buf_pos: usize,
}

impl<'a> Iter<'a> {
    fn new(
        t: &'a Tensor,
        min_row: u64,
        max_row: u64,
        transpose: bool,
    ) -> Self {
        Self {
            t,
            base: if transpose {
                BaseIter::Backward(t.mt.iter(min_row, max_row))
            } else {
                BaseIter::Forward(t.m.iter(min_row, max_row))
            },
            src: 0,
            dest: 0,
            buf: Vec::new(),
            buf_pos: 0,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        // Drain buffered (ascending) ids for the current multi-edge pair.
        if self.buf_pos < self.buf.len() {
            let id = self.buf[self.buf_pos];
            self.buf_pos += 1;
            return Some((self.src, self.dest, id));
        }

        // Next base pair, oriented as (src, dest) with its tagged word.
        let (src, dest, vv) = match &mut self.base {
            BaseIter::Forward(it) => {
                let (row, col, raw) = it.next()?;
                (row, col, VersionedVector::from_raw(raw))
            }
            BaseIter::Backward(it) => {
                let (row, col) = it.next()?;
                let (src, dest) = (col, row);
                let vv = self
                    .t
                    .m
                    .get(src, dest)
                    .unwrap_or(VersionedVector::new_scalar(0));
                (src, dest, vv)
            }
        };
        self.src = src;
        self.dest = dest;

        if vv.is_scalar() {
            return Some((src, dest, vv.scalar()));
        }
        self.buf = vv.ids(self.t.epoch).collect();
        self.buf_pos = 1;
        self.buf.first().map(|&id| (src, dest, id))
    }
}
