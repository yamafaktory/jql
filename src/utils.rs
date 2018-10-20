use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;
use toml::Value;
use types::Selector;

/// Get the binary version by reading it directly from the Cargo.toml file.
pub fn get_cargo_version() -> String {
    let cargo_file = File::open("Cargo.toml").unwrap();
    let mut buffer_reader = BufReader::new(cargo_file);
    let mut yaml = String::new();

    buffer_reader.read_to_string(&mut yaml).unwrap();
    let parsed_yaml = yaml.parse::<Value>().unwrap();
    String::from((parsed_yaml["package"]["version"]).as_str().unwrap())
}

/// Convert a range to a readable string.
fn range_to_string((start, end): (usize, usize)) -> String {
    [start.to_string().as_str(), ":", end.to_string().as_str()].join(" ")
}

/// Return the node or the range of Selector as a string.
pub fn get_node_or_range(selector: &Selector) -> String {
    match selector {
        Selector::Default(value) => value.clone(),
        Selector::Range(range) => range_to_string(*range),
    }
}
