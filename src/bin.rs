#![deny(unsafe_code, nonstandard_style)]

mod cli;
mod panic;

use cli::get_matches;
use jql::walker;
use panic::use_custom_panic_hook;

use anyhow::Result;
use async_std::{fs, io, path::Path, prelude::*, process::exit};
use clap::ArgMatches;
use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde_json::{Deserializer, Value};

/// Try to serialize the raw JSON content, output the selection or throw an
/// error.
async fn render_output(json_content: &str, cli: &ArgMatches) {
    let check = cli.is_present("check");
    let inline = cli.is_present("inline");
    let raw_output = cli.is_present("raw-output");
    let from_file = cli.is_present("from-file");
    let selectors = if from_file {
        let file = cli.value_of("from-file").unwrap();
        let path = Path::new(file);
        let contents = fs::read_to_string(path).await;
        match contents {
            Ok(selectors) => Some(selectors),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    } else {
        cli.value_of("selectors").map(|s| s.to_string())
    };

    // Early check of the JSON content with matching exit code based on result.
    if check {
        match serde_json::from_str::<Value>(json_content) {
            Ok(_) => {
                println!("Valid JSON file or content");
                exit(0);
            }
            Err(_) => {
                eprintln!("Invalid JSON file or content");
                exit(1);
            }
        }
    }

    match selectors {
        Some(selectors) => {
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

            deserializer
                .into_iter::<Value>()
                .for_each(|value| match value {
                    Ok(valid_json) => {
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
                                eprintln!("{}", error);
                                exit(1);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Invalid JSON file or content");
                        exit(1);
                    }
                });
        }

        // No selector found.
        None => {
            eprintln!("No selector found");
            exit(1);
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    // Use a custom panic hook.
    use_custom_panic_hook();

    let cli = get_matches();
    let check = cli.is_present("check");
    let from_file = cli.is_present("from-file");

    // Use a hack here since we can't conditionally define indexes of
    // positional arguments with clap.
    // Assume that the first positional argument is the JSON file or content
    // if the check flag is passed, otherwise keep the default behavior.
    let json_arg = if check || from_file {
        "selectors"
    } else {
        "JSON"
    };
    match cli.value_of(json_arg) {
        // JSON content coming from the CLI.
        Some(json) => {
            let path = Path::new(json);
            let contents = fs::read_to_string(path).await?;

            render_output(&contents, &cli).await;

            Ok(())
        }

        // JSON content coming from stdin.
        None => {
            let stream = cli.is_present("stream");
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
                    eprintln!("{}", error);
                    exit(1);
                }
            }
        }
    }
}
