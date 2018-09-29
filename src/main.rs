extern crate clap;
extern crate serde_json;

mod cli;
pub mod core;
mod types;

use cli::get_matches;
use core::walker;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main() {
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
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => match serde_json::from_str(&contents) {
                Ok(valid_json) => {
                    if cli.is_present("pretty-print") {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&json).unwrap()
                        );
                    }
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
