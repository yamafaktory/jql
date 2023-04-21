#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![doc = include_str!("../README.md")]

mod combinators;
/// Parser errors.
pub mod errors;
/// Grouping utilities.
pub mod group;
/// Parsing utilities.
pub mod parser;
/// Tokens for the parser.
pub mod tokens;
