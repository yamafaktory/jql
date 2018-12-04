use serde_json::Value;

#[derive(Debug)]
pub enum Selector {
    Default(String),
    Index(usize),
    Range((usize, usize)),
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
