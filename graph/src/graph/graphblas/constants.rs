//! Constants shared across the GraphBLAS wrappers.
//!
//! These live in their own module rather than in `matrix.rs` or `tensor.rs` so
//! that the lower-level wrappers can use them without depending on a
//! higher-level one (`tensor` already imports from `matrix`, so defining
//! shared values there would make the dependency circular).

/// Maximum GraphBLAS index value (2^60 - 1).
///
/// SuiteSparse:GraphBLAS caps matrix and vector dimensions at this value; any
/// larger dimension is rejected by `GrB_Matrix_new` and friends.
#[allow(non_upper_case_globals)]
pub const GrB_INDEX_MAX: u64 = (1u64 << 60) - 1;
