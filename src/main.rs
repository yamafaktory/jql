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
    let throw = |s, selector: &Vec<&str>, i: usize| -> String {
        ["Node (", s, ") not found on parent (", selector[i - 1], ")"].join(" ")
    };
    if let Some(selector) = selector {
        let selector: Vec<&str> = selector.split('.').collect();
        // Returns Result of values or Err early on, stopping the iteration.
        let items: Selection = selector
            .iter()
            .enumerate()
            .map(|(i, s)| -> Result<Value, String> {
                // let iter_index = i.to_string();
                if let Ok(index) = s.parse::<usize>() {
                    println!("wfwef{}", (index as isize).is_negative());
                    if (index as isize).is_negative() {
                        Err("Negative index".to_string())
                    } else {
                        if inner_json[index] == Value::Null {
                            Err(throw(s, &selector, i))
                        } else {
                            inner_json = &inner_json[index];
                            Ok(inner_json.clone())
                        }
                    }
                } else {
                    if inner_json[s] == Value::Null {
                        if i == 0 {
                            Err(["Node (", s, ") is not the root element"]
                                .join(" "))
                        } else {
                            Err(throw(s, &selector, i))
                        }
                    } else {
                        inner_json = &inner_json[s];
                        Ok(inner_json.clone())
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
            Ok(_) => {
                match serde_json::from_str(&contents) {
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
                                    serde_json::to_string_pretty(
                                        &results.last()
                                    ).unwrap()
                                ),
                                Err(error) => println!("{}", error),
                            },
                            None => println!("has no value"),
                        }
                    }
                    Err(_) => println!("Invalid JSON file!"),
                }
            }
            Err(error) => panic!(
                "Couldn't read {}: {}",
                path.display(),
                error.description()
            ),
        }
    }
}
