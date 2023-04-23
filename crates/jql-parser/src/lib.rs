#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://github.com/yamafaktory/jql/blob/main/jql.svg")]

mod combinators;
/// Parser errors.
pub mod errors;
/// Grouping utilities.
pub mod group;
/// Parsing utilities.
pub mod parser;
/// Tokens for the parser.
pub mod tokens;
