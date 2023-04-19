#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs)]
#![doc = include_str!("../../../README.md")]

mod args;
mod panic;

use std::{
    path::Path,
    process::exit,
};

use anyhow::{
    anyhow,
    Context,
    Result,
};
use args::Args;
use clap::Parser;
use colored_json::{
    ColoredFormatter,
    CompactFormatter,
    PrettyFormatter,
};
use jql_runner::runner::raw_runner;
use panic::use_custom_panic_hook;
use serde::Deserialize;
use serde_json::Value;
use serde_stacker::Deserializer;
use tokio::{
    fs::File,
    io::{
        stdin,
        stdout,
        AsyncBufReadExt,
        AsyncReadExt,
        AsyncWriteExt,
        BufReader,
    },
};

/// Reads a file from `path`.
async fn read_file(path: impl AsRef<Path>) -> Result<String> {
    let display_path = path.as_ref().display();
    let mut file = File::open(&path)
        .await
        .with_context(|| format!("Failed to open file {display_path}"))?;
    let mut contents = vec![];

    file.read_to_end(&mut contents)
        .await
        .with_context(|| format!("Failed to read from file {display_path}"))?;

    Ok(String::from_utf8_lossy(&contents).into_owned())
}

/// Renders the output or the error and exits.
fn render(result: Result<String>) {
    match result {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}

/// Processes the JSON content based on the arguments.
async fn process_json(json: &str, args: &Args) -> Result<String> {
    if args.validate {
        return serde_json::from_str::<Value>(json).map_or_else(
            |_| Err(anyhow!("Invalid JSON file or content")),
            |_| Ok("Valid JSON file or content".to_string()),
        );
    }

    let query = match args.query_from_file.as_deref() {
        Some(path) => read_file(path).await?,
        // We can safely unwrap since clap is taking care of the validation.
        None => args.query.as_deref().unwrap().to_string(),
    };

    let mut deserializer = serde_json::Deserializer::from_str(&json);

    deserializer.disable_recursion_limit();

    let deserializer = Deserializer::new(&mut deserializer);
    let value: Value = Value::deserialize(deserializer)
        .with_context(|| format!("Failed to deserialize the JSON data"))?;
    let result: Value = raw_runner(&query, &value)?;

    if args.inline {
        return Ok(ColoredFormatter::new(CompactFormatter {})
            .to_colored_json_auto(&result)
            .with_context(|| format!("Failed to inline the JSON data"))?);
    }

    if args.raw_string && result.is_string() {
        // We can safely unwrap since the result is a string.
        return Ok(String::from(result.as_str().unwrap()));
    }

    Ok(ColoredFormatter::new(PrettyFormatter::new())
        .to_colored_json_auto(&result)
        .with_context(|| format!("Failed to format the JSON data"))?)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Use a custom panic hook.
    use_custom_panic_hook();

    let args = Args::parse();

    if let Some(path) = args.json_file.as_deref() {
        let contents = read_file(path).await?;

        render(process_json(&contents, &args).await);

        return Ok(());
    }

    let mut stdout = stdout();

    if args.stream {
        let mut reader = BufReader::new(stdin()).lines();

        while let Some(mut line) = reader
            .next_line()
            .await
            .with_context(|| format!("Failed to read stream"))?
        {
            render(process_json(&line, &args).await);

            stdout
                .flush()
                .await
                .with_context(|| format!("Failed to flush stdout"))?;

            line.clear();
        }
    }

    let mut buffer = Vec::new();
    let mut stdin = stdin();

    // By default, read the whole piped content from stdin.
    stdin
        .read_to_end(&mut buffer)
        .await
        .with_context(|| format!("Failed to read piped content from stdin"))?;

    let lines = String::from_utf8(buffer)
        .with_context(|| format!("Failed to convert piped content from stdin"))?;

    render(process_json(&lines, &args).await);

    Ok(())
}
