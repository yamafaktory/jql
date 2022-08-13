use serde_json::Value;

use crate::utils::{
    display_array_selector, display_default_selector, display_index_selector,
    display_object_selector, display_range_selector,
};

/// Filters.
#[derive(Debug, PartialEq, Eq)]
pub enum Filter {
    /// Default variant.
    Default(Selector),
    /// Lens variant.
    Lens(Selector),
}

impl Filter {
    pub fn get_selector(&self) -> Selector {
        match self {
            Filter::Default(selector) => selector.to_owned(),
            Filter::Lens(selector) => selector.to_owned(),
        }
    }
}

/// Selectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    /// Array variant.
    Array,
    /// Default variant.
    Default(String),
    /// Index variant.
    Index(Vec<usize>),
    /// Object variant.
    Object(Vec<InnerObject>),
    /// Range variant.
    Range((Option<usize>, Option<usize>)),
}

/// Inner objects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerObject {
    /// Array variant.
    Array,
    /// Index variant.
    Index(Vec<usize>),
    /// Key / Value variant.
    KeyValue(String, Option<String>),
    /// Range variant.
    Range((Option<usize>, Option<usize>)),
}

#[doc(hidden)]
pub trait Display {
    fn as_str(&self, capitalized: bool) -> String;
}

impl Display for Selector {
    // Return the selector as a readable string.
    fn as_str(&self, capitalized: bool) -> String {
        match self {
            Selector::Array => display_array_selector(capitalized),
            Selector::Default(value) => display_default_selector(&value.clone(), capitalized),
            Selector::Index(indexes) => display_index_selector(indexes, capitalized),
            Selector::Object(properties) => display_object_selector(properties, capitalized),
            Selector::Range(range) => display_range_selector(*range, capitalized),
        }
    }
}

impl Display for InnerObject {
    // Return the selector as a readable string.
    fn as_str(&self, capitalized: bool) -> String {
        match self {
            InnerObject::Array => display_array_selector(capitalized),
            InnerObject::Index(indexes) => display_index_selector(indexes, capitalized),
            InnerObject::KeyValue(key, _value) => key.to_string(),
            InnerObject::Range(range) => display_range_selector(*range, capitalized),
        }
    }
}

/// A Group is a set of grammar elements used to define a selection.
#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    /// Filters.
    pub filters: Vec<Filter>,
    /// Root marker.
    pub root: Option<()>,
    /// Selectors.
    pub selectors: Vec<Selector>,
    /// Spread marker.
    pub spread: Option<()>,
    /// Truncate marker.
    pub truncate: Option<()>,
}

/// Group implementations.
impl Group {
    /// Creates a new group.
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            root: None,
            selectors: Vec::new(),
            spread: None,
            truncate: None,
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub enum MaybeArray {
    Array(Vec<Value>),
    NonArray(Vec<Value>),
}

pub(crate) type Selection = Result<Value, String>;

pub(crate) type Selections = Result<Vec<Value>, String>;

pub(crate) type ExtendedSelections = Result<MaybeArray, String>;
