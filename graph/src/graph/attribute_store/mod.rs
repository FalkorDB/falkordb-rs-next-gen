//! Attribute storage for graph entities — feature-selected engine.
//!
//! Two complete, API-identical implementations live side by side; a Cargo
//! feature picks which one backs [`AttributeStore`] at compile time:
//!
//! - **`columnar`** (default): each attribute is a column of packed,
//!   type-homogeneous `Arc<Chunk>` blocks with copy-on-write at chunk
//!   granularity. Strongest at single-attribute point reads, narrow update
//!   churn, and memory (packed scalars).
//! - **`heap`** (enable the `attr-store-heap` feature): row-oriented paged
//!   clustered heap — each entity's attributes form one `Arc` record, stored
//!   in a copy-on-write radix page directory addressed directly by the dense
//!   entity id. Strongest at whole-row returns (`get_attrs`/`iter_rows`
//!   clustered scans) and bulk deletes.
//!
//! Both were benchmarked head-to-head (micro + engine-level) on 2026-07-08;
//! see `graph/tests/attr_store_bench.rs` for the harness. The engines expose
//! the same public items (`AttrNameMap`, `AttributeStore`, `AttrArrayView`),
//! so switching the feature requires no call-site changes. `iter_rows` (the
//! clustered full scan) is heap-only.

#[cfg(not(feature = "attr-store-heap"))]
mod columnar;
#[cfg(not(feature = "attr-store-heap"))]
pub use columnar::*;

#[cfg(feature = "attr-store-heap")]
mod heap;
#[cfg(feature = "attr-store-heap")]
pub use heap::*;
