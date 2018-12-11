use crate::utils::{
    display_array_selector, display_default_selector, display_index_selector,
    display_range_selector,
};
use serde_json::Value;

#[derive(Debug)]
pub enum Selector {
    Default(String),
    Array,
    Index(usize),
    Range((usize, usize)),
}

pub trait Display {
    fn as_str(&self, capitalized: bool) -> String;
}

impl Display for Selector {
    // Return the selector as a readable string.
    fn as_str(&self, capitalized: bool) -> String {
        match self {
            Selector::Array => display_array_selector(capitalized),
            Selector::Default(value) => {
                display_default_selector(&value.clone(), capitalized)
            }
            Selector::Range(range) => {
                display_range_selector(*range, capitalized)
            }
            Selector::Index(index) => {
                display_index_selector(*index, capitalized)
            }
        }
    }
}

#[derive(Debug)]
pub enum MaybeArray {
    Array(Vec<Value>),
    NonArray(Vec<Value>),
}

pub type Selection = Result<Vec<Value>, String>;

pub type ExtendedSelection = Result<MaybeArray, String>;

pub type Selectors = [Selector];

pub type Group = (
    // Spread part.
    Option<()>,
    // Root part.
    Option<()>,
    // Selectors part.
    Vec<Selector>,
    // Filters part.
    Vec<Selector>,
);

pub type Groups = Vec<Group>;
