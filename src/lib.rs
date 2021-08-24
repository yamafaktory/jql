#![forbid(rust_2018_idioms)]
#![deny(unsafe_code, nonstandard_style)]
#![warn(missing_debug_implementations, missing_docs)]

//! # A JSON query language library
//!
//! This crate is used by `jql`, the `JSON query language CLI tool`.

mod apply_filter;
mod array_walker;
mod core;
mod flatten_json_array;
mod get_selection;
mod group_walker;
mod parser;
mod range_selector;
mod truncate;
mod types;
mod utils;

use crate::types::Selection;

use serde_json::Value;

/// Process a Serde JSON Value based on the provided selectors.
///
/// # Example
///
/// ```
/// use serde_json::json;
///
/// let json_array = json!([2, 3, 5, 7, 11]);
///
/// assert_eq!(jql::walker(&json_array, Some("[4]")), Ok(json!(11)));
/// ```
pub fn walker(json: &Value, selectors: Option<&str>) -> Selection {
    core::walker(json, selectors)
}
