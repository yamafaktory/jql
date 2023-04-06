#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! TODO

/// Parser errors.
pub mod errors;
/// JSON utilities.
mod json;
/// Runner utilities.
pub mod runner;
