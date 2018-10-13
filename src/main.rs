extern crate clap;
extern crate regex;
extern crate serde_json;
extern crate toml;

mod cli;
mod core;
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
            Err(..) => {
                println!("File {:?} not found", &path);
                return ();
            }
            Ok(file) => file,
        };
        let mut buffer_reader = BufReader::new(file);
        let mut contents = String::new();

        match buffer_reader.read_to_string(&mut contents) {
            Ok(_) => match serde_json::from_str(&contents) {
                Ok(valid_json) => {
                    if cli.is_present("pretty-print") {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&json).unwrap()
                        );
                    }

                    // Walk through the JSON content with the provided selector.
                    match walker(&valid_json, selector) {
                        Some(items) => match items {
                            Ok(results) => println!(
                                "{}",
                                serde_json::to_string_pretty(&results.last())
                                    .unwrap()
                            ),
                            Err(error) => println!("{}", error),
                        },
                        None => println!("has no value"),
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
