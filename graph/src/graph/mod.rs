//! Graph data structures and storage.
//!
//! This module contains the core graph representation using sparse matrices
//! backed by GraphBLAS for efficient graph operations.
//!
//! ## Key Components
//!
//! - [`graph::Graph`]: The main graph structure holding nodes, edges, and properties
//! - [`matrix::Matrix`]: Sparse matrix for adjacency representation
//! - [`vector::Vector`]: Sparse vector for label membership
//! - [`attribute_store::AttributeStore`]: Attribute storage for nodes/edges
//! - [`mvcc_graph::MvccGraph`]: MVCC wrapper for concurrent access
//!
//! ## Storage Model
//!
//! The graph uses a sparse matrix representation where:
//! - Nodes are identified by 32-bit IDs
//! - Labels are stored as sparse vectors (node ID → boolean)
//! - Relationships are stored as sparse matrices (src × dst → edge ID)
//! - Properties are stored separately in block vectors
//!
//! ## GraphBLAS Integration
//!
//! The underlying sparse matrix operations use GraphBLAS, a high-performance
//! library for graph algorithms using linear algebra. The [`GraphBLAS`] module
//! contains the FFI bindings (auto-generated).

pub mod GraphBLAS;
pub mod attribute_store;
pub mod cow;
pub mod graph;
pub mod matrix;
pub mod mvcc_graph;
pub mod tensor;
pub mod vector;
pub mod versioned_matrix;
