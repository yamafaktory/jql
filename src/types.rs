use serde_json::Value;

#[derive(Debug)]
pub enum Selector {
    Default(String),
    Range((usize, usize)),
}

pub type Selection = Result<Vec<Value>, String>;

pub type Selectors = [Selector];
