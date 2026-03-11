//! Query execution runtime.
//!
//! This module contains the query execution engine that evaluates execution
//! plans against the graph. It includes:
//!
//! ## Key Components
//!
//! - [`runtime::Runtime`]: The main execution engine that processes plan operators
//! - [`value::Value`]: Runtime representation of Cypher values
//! - [`functions`]: Built-in Cypher function implementations
//! - [`pending`]: Deferred operations for write batching
//!
//! ## Execution Model
//!
//! The runtime uses a pull-based execution model where each operator pulls
//! tuples from its children. This enables lazy evaluation and early termination
//! for LIMIT clauses.
//!
//! ## Data Structures
//!
//! - [`ordermap::OrderMap`]: Insertion-ordered map for consistent iteration
//! - [`orderset::OrderSet`]: Insertion-ordered set for label/type collections

pub mod bitset;
pub mod env;
pub mod functions;
pub mod object_pool;
pub mod ops;
pub mod ordermap;
pub mod orderset;
pub mod pending;
pub mod runtime;
pub mod value;
