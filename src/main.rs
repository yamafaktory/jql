extern crate clap;
extern crate lazy_static;
extern crate regex;
extern crate serde_json;
extern crate toml;

mod apply_filter;
mod array_walker;
mod cli;
mod core;
mod get_selection;
mod get_selector;
mod group_walker;
mod range_selector;
mod types;
mod utils;

use cli::get_matches;
use core::walker;
use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let cli = get_matches();

    if let Some(json) = cli.value_of("JSON") {
        let selector = cli.value_of("selector");
        let path = Path::new(json);
        let mut file = match File::open(&path) {
            Err(_) => {
                println!("File {:?} not found", &path);
                return;
            }
            Ok(file) => file,
        };
        let mut buffer_reader = BufReader::new(file);
        let mut contents = String::new();

        match buffer_reader.read_to_string(&mut contents) {
            Ok(_) => match serde_json::from_str(&contents) {
                Ok(valid_json) => {
                    // Walk through the JSON content with the provided selector.
                    match walker(&valid_json, selector) {
                        Ok(selection) => println!(
                            "{}",
                            serde_json::to_string_pretty(&selection).unwrap()
                        ),
                        Err(error) => println!("{}", error),
                    }
                }
                Err(_) => println!("Invalid JSON file"),
            },
            Err(error) => panic!(
                "Couldn't read {}: {}",
                path.display(),
                error.description()
            ),
        }
    }
}
