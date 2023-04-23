#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://github.com/yamafaktory/jql/blob/main/jql.svg")]

/// Array utilities.
mod array;
/// Parser errors.
pub mod errors;
/// Object utilities.
mod object;
/// Runner utilities.
pub mod runner;
