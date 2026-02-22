#![allow(clippy::arc_with_non_send_sync)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]

//! # FalkorDB Graph Engine
//!
//! This crate contains the core graph database engine for FalkorDB.
//! It provides Cypher query parsing, planning, optimization, and execution
//! over a sparse matrix-based graph representation using GraphBLAS.
//!
//! ## Query Processing Pipeline
//!
//! ```text
//! Cypher Query String
//!        │
//!        ▼
//! ┌─────────────┐
//! │   cypher    │  Parse query into AST (ANTLR-generated parser)
//! └─────────────┘
//!        │
//!        ▼
//! ┌─────────────┐
//! │    ast      │  Abstract Syntax Tree nodes
//! └─────────────┘
//!        │
//!        ▼
//! ┌─────────────┐
//! │   binder    │  Semantic analysis: resolve names, check types
//! └─────────────┘
//!        │
//!        ▼
//! ┌─────────────┐
//! │  planner    │  Convert AST to logical execution plan (IR)
//! └─────────────┘
//!        │
//!        ▼
//! ┌─────────────┐
//! │ optimizer   │  Optimize the execution plan
//! └─────────────┘
//!        │
//!        ▼
//! ┌─────────────┐
//! │  runtime    │  Execute plan against the graph
//! └─────────────┘
//! ```
//!
//! ## Module Overview
//!
//! - [`ast`]: Abstract Syntax Tree definitions for Cypher queries
//! - [`binder`]: Semantic analysis and name resolution
//! - [`cypher`]: Cypher parser (visitor pattern over ANTLR-generated parser)
//! - [`graph`]: Graph data structures (sparse matrices, vectors, MVCC)
//! - [`indexer`]: Full-text and property index management
//! - [`optimizer`]: Query plan optimization passes
//! - [`planner`]: Logical plan generation from bound AST
//! - [`redisearch`]: RediSearch integration for full-text indexing
//! - [`runtime`]: Query execution engine and built-in functions
//! - [`threadpool`]: Thread pool for parallel query execution
//! - [`tree`]: Tree utility functions

pub mod ast;
pub mod binder;
pub mod cypher;
pub mod graph;
pub mod indexer;
pub mod optimizer;
pub mod planner;
pub mod redisearch;
pub mod runtime;
pub mod string_escape;
pub mod threadpool;
pub mod tree;
