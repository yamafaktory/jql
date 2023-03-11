#![deny(unsafe_code, nonstandard_style)]
#![deny(clippy::pedantic)]

mod cli;
mod panic;

use std::clone::Clone;

use anyhow::Result;
use async_std::{fs, io, path::Path, prelude::*, process::exit};
use clap::ArgMatches;
use cli::get_matches;
use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use jql::walker;
use panic::use_custom_panic_hook;
use serde_json::{Deserializer, Value};

/// Try to serialize the raw JSON content, output the selection or throw an
/// error.
async fn render_output(json_content: &str, cli: &ArgMatches) {
    let check = cli.get_flag("check");
    let inline = cli.get_flag("inline");
    let raw_output = cli.get_flag("raw-output");
    let from_file = cli.value_source("from-file").is_some();

    let selectors = if from_file {
        let file: &String = cli.get_one("from-file").unwrap();
        let path = Path::new(file);
        let contents = fs::read_to_string(path).await;

        if let Ok(selectors) = contents {
            Some(selectors)
        } else {
            eprintln!("Invalid selectors file");
            exit(1);
        }
    } else {
        cli.get_one::<String>("selectors").map(Clone::clone)
    };

    // Early check of the JSON content with matching exit code based on result.
    if check {
        if serde_json::from_str::<Value>(json_content).is_ok() {
            println!("Valid JSON file or content");
            exit(0);
        } else {
            eprintln!("Invalid JSON file or content");
            exit(1);
        }
    }

    if let Some(selectors) = selectors {
        // Get a deserializer out of the JSON content.
        let mut deserializer = Deserializer::from_str(json_content);

        // Disable recursion limit.
        // Fixes issue #120.
        // Note to be used along with the `unbounded_depth` feature.
        // https://github.com/serde-rs/json/blob/master/src/de.rs#L163-L210
        // We might eventually use `serde_stacker` but this might introduce
        // some performance cost.
        // https://github.com/dtolnay/serde-stacker/issues/1
        deserializer.disable_recursion_limit();

        deserializer.into_iter::<Value>().for_each(|value| {
            if let Ok(valid_json) = value {
                // Walk through the JSON content with the provided selectors as
                // input.
                match walker(&valid_json, &selectors) {
                    Ok(selection) => println!(
                        "{}",
                        // Inline or pretty output.
                        (if inline {
                            ColoredFormatter::new(CompactFormatter {})
                                .to_colored_json_auto(&selection)
                                .unwrap()
                        } else {
                            // If the selection is a string and the raw-output
                            // flag is passed, directly return the raw string
                            // without JSON double-quotes.
                            // https://github.com/serde-rs/json/issues/367
                            if raw_output && selection.is_string() {
                                String::from(selection.as_str().unwrap())
                            } else {
                                ColoredFormatter::new(PrettyFormatter::new())
                                    .to_colored_json_auto(&selection)
                                    .unwrap()
                            }
                        })
                    ),
                    Err(error) => {
                        eprintln!("{error}");
                        exit(1);
                    }
                }
            } else {
                eprintln!("Invalid JSON file or content");
                exit(1);
            }
        });
    } else {
        eprintln!("No selector found");
        exit(1);
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    // Use a custom panic hook.
    use_custom_panic_hook();

    let cli = get_matches();
    let check = cli.get_flag("check");
    let from_file = cli.value_source("from-file").is_some();

    // Use a hack here since we can't conditionally define indexes of
    // positional arguments with clap.
    // Assume that the first positional argument is the JSON file or content
    // if the check flag is passed, otherwise keep the default behavior.
    let json_arg = if check || from_file {
        "selectors"
    } else {
        "JSON"
    };

    // JSON content coming from the CLI.
    if let Some(json) = cli.get_one::<String>(json_arg) {
        let path = Path::new(json);
        let contents = fs::read_to_string(path).await?;

        render_output(&contents, &cli).await;

        return Ok(());
    }

    // JSON content coming from stdin.
    let stream = cli.get_flag("stream");
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // Special case for the stream option.
    // In this case, read line by line.
    if stream {
        let mut line = String::new();

        loop {
            // Read one line from stdin.
            let n = stdin.read_line(&mut line).await?;

            // Check for the EOF.
            if n == 0 {
                return Ok(());
            }

            render_output(&line, &cli).await;

            stdout.flush().await?;

            line.clear();
        }
    }

    // By default, read the whole piped content from stdin.
    let mut buffer = Vec::new();

    stdin.read_to_end(&mut buffer).await?;

    match String::from_utf8(buffer) {
        Ok(lines) => {
            render_output(&lines, &cli).await;

            Ok(())
        }
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
