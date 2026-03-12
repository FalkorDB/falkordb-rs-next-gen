//! Runtime operator implementations for the query execution engine.
//!
//! Operators process data in batches for improved throughput. The primary
//! execution path uses [`BatchOp`](crate::runtime::batch::BatchOp) which
//! processes up to [`BATCH_SIZE`](crate::runtime::batch::BATCH_SIZE) rows
//! per operator invocation.
//!
//! ```text
//!                          BatchOp (enum dispatch)
//!                                  |
//!          +-----------+-----------+-----------+-- ...
//!          |           |           |           |
//!     AggregateOp  FilterOp   SortOp    ProjectOp
//!          |           |
//!       child op    child op
//!
//! Pull model:  parent.next()  -->  child.next()  -->  ...  -->  scan.next()
//! ```
//!
//! ## Operator categories
//!
//! | Category     | Operators |
//! |--------------|-----------|
//! | **Scans**    | `NodeByLabelScan`, `NodeByIndexScan`, `NodeByIdSeek`, `NodeByLabelAndIdScan`, `NodeByFulltextScan` |
//! | **Traversal**| `CondTraverse`, `CondVarLenTraverse`, `ExpandInto` |
//! | **Filter**   | `Filter`, `Distinct`, `Skip`, `Limit`, `Sort` |
//! | **Mutation** | `Create`, `Delete`, `Set`, `Remove`, `Merge`, `Commit` |
//! | **Control**  | `Apply`, `SemiApply`, `Optional`, `OrApplyMultiplexer`, `CartesianProduct`, `Union`, `Argument` |
//! | **Transform**| `Project`, `Aggregate`, `Unwind`, `PathBuilder`, `LoadCsv`, `ProcedureCall` |

pub mod aggregate;
pub mod apply;
pub mod cartesian_product;
pub mod commit;
pub mod cond_traverse;
pub mod cond_var_len_traverse;
pub mod create;
pub mod delete;
pub mod distinct;
pub mod expand_into;
pub mod filter;
pub mod limit;
pub mod load_csv;
pub mod merge;
pub mod node_by_fulltext_scan;
pub mod node_by_id_seek;
pub mod node_by_index_scan;
pub mod node_by_label_and_id_scan;
pub mod node_by_label_scan;
pub mod optional;
pub mod or_apply_multiplexer;
pub mod path_builder;
pub mod procedure_call;
pub mod project;
pub mod remove;
pub mod semi_apply;
pub mod set;
pub mod skip;
pub mod sort;
pub mod union;
pub mod unwind;

pub use aggregate::AggregateOp;
pub use apply::ApplyOp;
pub use cartesian_product::CartesianProductOp;
pub use commit::CommitOp;
pub use cond_traverse::CondTraverseOp;
pub use cond_var_len_traverse::CondVarLenTraverseOp;
pub use create::CreateOp;
pub use delete::DeleteOp;
pub use distinct::DistinctOp;
pub use expand_into::ExpandIntoOp;
pub use filter::FilterOp;
pub use limit::LimitOp;
pub use load_csv::LoadCsvOp;
pub use merge::MergeOp;
pub use node_by_fulltext_scan::NodeByFulltextScanOp;
pub use node_by_id_seek::NodeByIdSeekOp;
pub use node_by_index_scan::NodeByIndexScanOp;
pub use node_by_label_and_id_scan::NodeByLabelAndIdScanOp;
pub use node_by_label_scan::NodeByLabelScanOp;
pub use optional::OptionalOp;
pub use or_apply_multiplexer::OrApplyMultiplexerOp;
pub use path_builder::PathBuilderOp;
pub use procedure_call::ProcedureCallOp;
pub use project::ProjectOp;
pub use remove::RemoveOp;
pub use semi_apply::SemiApplyOp;
pub use set::SetOp;
pub use skip::SkipOp;
pub use sort::SortOp;
pub use union::UnionOp;
pub use unwind::UnwindOp;
