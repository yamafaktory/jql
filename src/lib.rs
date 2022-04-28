#![deny(unsafe_code, nonstandard_style)]
#![warn(missing_debug_implementations, missing_docs)]
#![forbid(rust_2021_compatibility)]

//! # A JSON query language library.
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

// Expose all the types.
use serde_json::Value;

pub use crate::types::*;

/// Walks over the Serde JSON value based on the provided selectors.
///
/// # Example
///
/// ```
/// use jql::walker;
///
/// use serde_json::json;
///
/// let json_array = json!([2, 3, 5, 7, 11]);
///
/// assert_eq!(walker(&json_array, "[4]"), Ok(json!(11)));
/// ```
pub fn walker(json: &Value, selectors: &str) -> Selection {
    core::walker(json, selectors)
}

/// Walks over the Serde JSON value based on the provided groups.
///
/// # Example
///
/// ```
/// use jql::{Group, groups_walker, Selector::{Index}};
///
/// use serde_json::json;
///
/// let json_array = json!([2, 3, 5, 7, 11]);
///
/// assert_eq!(
///     groups_walker(
///         &json_array,
///         &[Group {
///             filters: vec![],
///             filter_lenses: vec![],
///             root: None,
///             selectors: vec![Index(vec![4])],
///             spread: None,
///             truncate: None,
///         }]
///     ),
///     Ok(json!(11))
/// );
/// ```
pub fn groups_walker(json: &Value, groups: &[Group]) -> Selection {
    core::groups_walker(json, groups)
}

/// Parses the provided selectors and returns a vector of group.
///
/// # Example
///
/// ```
/// use jql::{Group, selectors_parser, Selector::{Default, Range}};
///
/// let selector = r#""range".[5:3],"array".[2:1]"#;
///
/// assert_eq!(
///     selectors_parser(selector),
///     Ok(vec![
///         Group {
///             filters: vec![],
///             filter_lenses: vec![],
///             root: None,
///             selectors: vec![Default(String::from("range")), Range((Some(5), Some(3)))],
///             spread: None,
///             truncate: None,
///         },
///         Group {
///             filters: vec![],
///             filter_lenses: vec![],
///             root: None,
///             selectors: vec![Default(String::from("array")), Range((Some(2), Some(1)))],
///             spread: None,
///             truncate: None,
///         }
///     ])
/// );
/// ```
pub fn selectors_parser(selectors: &str) -> Result<Vec<Group>, String> {
    parser::selectors_parser(selectors)
}
