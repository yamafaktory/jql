#![feature(type_ascription)]

extern crate clap;
extern crate serde_json;

use clap::{App, Arg};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Selection = Result<Vec<Value>, String>;

fn get_selection(json: &Value, selector: Option<&str>) -> Option<Selection> {
    let mut inner_json = json;
    if let Some(selector) = selector {
        let selector: Vec<&str> = selector.split('.').collect();
        // Returns Result of values or Err early on, stopping the iteration.
        let items: Selection = selector
            .iter()
            .enumerate()
            .map(|(i, s)| -> Result<Value, String> {
                if let Ok(index) = s.parse::<isize>() {
                    if (index).is_negative() {
                        Err(String::from("Invalid negative array index"))
                    } else {
                        if inner_json[index as usize] == Value::Null {
                            let error_message = match inner_json.as_array() {
                                Some(array) => [
                                    "Index (",
                                    s,
                                    ") is out of bound, node (",
                                    selector[i - 1],
                                    ") has a length of",
                                    &(array.len()).to_string(),
                                ]
                                    .join(" "),
                                // Trying to access an index on a node which
                                // is not an array.
                                None => [
                                    "Node (",
                                    selector[i - 1],
                                    ") is not an array",
                                ]
                                    .join(" "),
                            };
                            println!("# {:?} #", inner_json.as_array());
                            Err(error_message)
                        } else {
                            inner_json = &inner_json[index as usize];
                            Ok(inner_json.clone())
                        }
                    }
                } else {
                    if s.is_empty() {
                        Err(String::from("Unterminated selector found"))
                    } else {
                        if inner_json[s] == Value::Null {
                            if i == 0 {
                                Err(["Node (", s, ") is not the root element"]
                                    .join(" "))
                            } else {
                                Err([
                                    "Node (",
                                    s,
                                    ") not found on parent (",
                                    selector[i - 1],
                                    ")",
                                ]
                                    .join(" "))
                            }
                        } else {
                            inner_json = &inner_json[s];
                            Ok(inner_json.clone())
                        }
                    }
                }
            }).collect();
        Some(items)
    } else {
        None
    }
}

fn main() {
    let cli = App::new("jql")
        .version("1.0")
        .author("Davy Duperron <yamafaktory@gmail.com>")
        .about("JSON query language")
        .arg(
            Arg::with_name("JSON")
                .help("JSON file to use")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("selector")
                .help("Selector to apply")
                .index(2)
                .required(true),
        ).arg(
            Arg::with_name("pretty-print")
                .help("Pretty print the JSON input")
                .long("pretty-print")
                .short("p"),
        ).get_matches();

    if let Some(json) = cli.value_of("JSON") {
        let selector = cli.value_of("selector");
        let path = Path::new(json);
        let mut file = match File::open(&path) {
            // TODO: print and exit properly.
            Err(error) => panic!("{}", error),
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
                    match get_selection(&valid_json, selector) {
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
