#![doc = include_str!("../README.md")]

mod args;
mod panic;

#[cfg(not(target_arch = "loongarch64"))]
use mimalloc::MiMalloc;

#[cfg(not(target_arch = "loongarch64"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::{
    fs,
    io::{
        BufRead,
        Read,
        Write,
        stdin,
        stdout,
    },
    path::Path,
    process::exit,
};

use anyhow::{
    Context,
    Result,
    anyhow,
};
use args::Args;
use clap::Parser;
use colored_json::{
    ColoredFormatter,
    CompactFormatter,
    PrettyFormatter,
};
use jql_runner::lazy;
use panic::use_custom_panic_hook;
use serde_json::Value;

/// Reads a file from `path`.
fn read_file(path: impl AsRef<Path>) -> Result<String> {
    let display_path = path.as_ref().display();
    let contents =
        fs::read(&path).with_context(|| format!("Failed to read from file {display_path}"))?;

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
fn process_json(json: &[u8], args: &Args) -> Result<String> {
    if args.validate {
        return serde_json::from_slice::<Value>(json).map_or_else(
            |_| Err(anyhow!("Invalid JSON file or content")),
            |_| Ok("Valid JSON file or content".to_string()),
        );
    }

    let query = match args.query_from_file.as_deref() {
        Some(path) => read_file(path)?,
        // We can safely unwrap since clap is taking care of the validation.
        None => args.query.as_deref().unwrap().to_string(),
    };

    let mut result: Value = lazy::raw(&query, json)?;

    if args.sort_keys {
        result.sort_all_objects();
    }

    if args.inline {
        return ColoredFormatter::new(CompactFormatter {})
            .to_colored_json_auto(&result)
            .with_context(|| "Failed to inline the JSON data".to_string());
    }

    if args.raw_string && result.is_string() {
        // We can safely unwrap since the result is a string.
        return Ok(String::from(result.as_str().unwrap()));
    }

    ColoredFormatter::new(PrettyFormatter::new())
        .to_colored_json_auto(&result)
        .with_context(|| "Failed to format the JSON data".to_string())
}

fn main() -> Result<()> {
    // Use a custom panic hook.
    use_custom_panic_hook();

    let args = Args::parse();

    if let Some(path) = args.json_file.as_deref() {
        let contents = read_file(path)?.into_bytes();

        render(process_json(&contents, &args));

        return Ok(());
    }

    if args.stream {
        let mut stdout = stdout().lock();

        for line in stdin().lock().lines() {
            let line = line
                .with_context(|| "Failed to read stream".to_string())?
                .into_bytes();

            render(process_json(&line, &args));

            stdout
                .flush()
                .with_context(|| "Failed to flush stdout".to_string())?;
        }

        return Ok(());
    }

    let mut buffer = Vec::new();

    // By default, read the whole piped content from stdin.
    stdin()
        .lock()
        .read_to_end(&mut buffer)
        .with_context(|| "Failed to read piped content from stdin".to_string())?;

    render(process_json(&buffer, &args));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_keys_flag_sorts_objects_recursively() {
        let json = br#"{ "root": { "d": 1, "b": { "y": 1, "x": 2 }, "a": 3 } }"#.to_vec();
        let args = Args::parse_from(["jql", "--sort-keys", "--inline", r#""root""#]);

        assert_eq!(
            process_json(&json, &args).unwrap(),
            r#"{"a":3,"b":{"x":2,"y":1},"d":1}"#
        );
    }

    #[test]
    fn output_keeps_source_order_without_the_flag() {
        let json = br#"{ "root": { "d": 1, "a": 3 } }"#.to_vec();
        let args = Args::parse_from(["jql", "--inline", r#""root""#]);

        assert_eq!(process_json(&json, &args).unwrap(), r#"{"d":1,"a":3}"#);
    }
}
