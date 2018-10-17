use serde_json::Value;

pub type Selection = Result<Vec<Value>, String>;

#[derive(Debug)]
pub enum Selector {
    Default(String),
    Range((usize, usize)),
}
