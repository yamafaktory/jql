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
mod types;
mod utils;

use serde_json::Value;

/// Process a Serde JSON Value based on the provided selectors.
pub fn process(json: &Value, selectors: Option<&str>) -> Result<Value, String> {
    core::walker(&json, selectors)
}
