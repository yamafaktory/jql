#![forbid(rust_2018_idioms)]
#![deny(unsafe_code, nonstandard_style)]

mod cli;

use jql::walker;

use anyhow::Result;
use async_std::{fs, io, path::Path, prelude::*, process::exit, task};
use clap::ArgMatches;
use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter};
use serde_json::{Deserializer, Value};

/// Try to serialize the raw JSON content, output the selection or throw an
/// error.
fn render_output(json_content: &str, cli: &ArgMatches<'_>) {
    let inline = cli.is_present("inline");
    let raw_output = cli.is_present("raw-output");
    let selectors = cli.value_of("selectors");

    Deserializer::from_str(json_content)
        .into_iter::<Value>()
        .for_each(|value| match value {
            Ok(valid_json) => {
                // Walk through the JSON content with the provided selectors as
                // input.
                match walker(&valid_json, selectors) {
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

#[async_std::main]
async fn main() -> Result<()> {
    let cli = cli::get_matches();

    match cli.value_of("JSON") {
        // JSON content coming from the CLI.
        Some(json) => {
            let path = Path::new(json);
            let contents = fs::read_to_string(path).await?;

            render_output(&contents, &cli);

            Ok(())
        }

        // JSON content coming from stdin.
        None => {
            task::block_on(async {
                let stdin = io::stdin();
                let mut stdout = io::stdout();
                let mut line = String::new();

                loop {
                    // Read a line from stdin.
                    let n = stdin.read_line(&mut line).await?;

                    // If this is the end of stdin, return.
                    if n == 0 {
                        return Ok(());
                    }

                    render_output(&line, &cli);

                    stdout.flush().await?;

                    line.clear();
                }
            })
        }
    }
}
