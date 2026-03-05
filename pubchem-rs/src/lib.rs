#![doc = include_str!("../README.md")]

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("feature \"async\" and feature \"sync\" cannot be enabled at the same time (a subject to change)");

/// Pubchem API client
pub mod client;
#[doc(hidden)]
pub(crate) mod fast_formula;
pub mod errors;
pub mod compound;