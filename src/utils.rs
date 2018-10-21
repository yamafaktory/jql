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
fn display_range_selector(
    (start, end): (usize, usize),
    capitalized: bool,
) -> String {
    [
        if capitalized { "Range (" } else { "range (" },
        start.to_string().as_str(),
        ":",
        end.to_string().as_str(),
        ")",
    ]
        .join(" ")
}

/// Convert a range to a readable string.
fn display_default_selector(value: String, capitalized: bool) -> String {
    [
        if capitalized { "Node (" } else { "node (" },
        value.as_str(),
        ")",
    ]
        .join(" ")
}

/// Return the node or the range of Selector as a string.
pub fn display_node_or_range(selector: &Selector, capitalized: bool) -> String {
    match selector {
        Selector::Default(value) => {
            display_default_selector(value.clone(), capitalized)
        }
        Selector::Range(range) => display_range_selector(*range, capitalized),
    }
}
