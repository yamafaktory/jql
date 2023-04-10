#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! TODO

/// Array utilities.
mod array;
/// Parser errors.
pub mod errors;
/// Object utilities.
mod object;
/// Runner utilities.
pub mod runner;
