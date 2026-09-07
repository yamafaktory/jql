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
    Ok(String::from_utf8_lossy(&read_file_bytes(path)?).into_owned())
}

/// Reads a file from `path` as bytes, replacing any invalid UTF-8 sequence the
/// way [`read_file`] does. Valid UTF-8 — every JSON file in practice — is handed
/// over as read, without the copy a `String` round-trip costs.
fn read_file_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let display_path = path.as_ref().display();
    let contents =
        fs::read(&path).with_context(|| format!("Failed to read from file {display_path}"))?;

    if std::str::from_utf8(&contents).is_ok() {
        return Ok(contents);
    }

    Ok(String::from_utf8_lossy(&contents).into_owned().into_bytes())
}

/// Renders the outputs or the error and exits.
fn render(result: Result<Vec<String>>) {
    match result {
        Ok(outputs) => {
            for output in outputs {
                println!("{output}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}

/// Returns the query, read from the file the arguments point at if there is one.
fn resolve_query(args: &Args) -> Result<String> {
    match args.query_from_file.as_deref() {
        Some(path) => read_file(path),
        // We can safely unwrap since clap is taking care of the validation.
        None => Ok(args.query.as_deref().unwrap().to_string()),
    }
}

/// Processes the JSON content based on the arguments.
/// Returns one rendered output per JSON document in `json`.
fn process_json(
    json: &mut [u8],
    query: &str,
    args: &Args,
    evaluator: &mut lazy::Evaluator,
) -> Result<Vec<String>> {
    if args.validate {
        return serde_json::from_slice::<Value>(json).map_or_else(
            |_| Err(anyhow!("Invalid JSON file or content")),
            |_| Ok(vec!["Valid JSON file or content".to_string()]),
        );
    }

    evaluator
        .raw_all(query, json)?
        .into_iter()
        .map(|result| format_json(result, args))
        .collect()
}

/// Renders one query result according to the output arguments.
fn format_json(mut result: Value, args: &Args) -> Result<String> {
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

    let args = Args::get();
    let query = if args.validate {
        String::new()
    } else {
        resolve_query(&args)?
    };
    // Reused across every document of a stream, so the scratch space a tape
    // build needs is allocated once instead of per line.
    let mut evaluator = lazy::Evaluator::new();

    if let Some(path) = args.json_file.as_deref() {
        let mut contents = read_file_bytes(path)?;

        render(process_json(&mut contents, &query, &args, &mut evaluator));

        return Ok(());
    }

    if args.stream {
        let mut stdout = stdout().lock();

        for line in stdin().lock().lines() {
            let mut line = line
                .with_context(|| "Failed to read stream".to_string())?
                .into_bytes();

            render(process_json(&mut line, &query, &args, &mut evaluator));

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

    render(process_json(&mut buffer, &query, &args, &mut evaluator));

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[test]
    fn sort_keys_flag_sorts_objects_recursively() {
        let mut json = br#"{ "root": { "d": 1, "b": { "y": 1, "x": 2 }, "a": 3 } }"#.to_vec();
        let args = Args::parse_from(["jql", "--sort-keys", "--inline", r#""root""#]);
        let query = resolve_query(&args).unwrap();

        assert_eq!(
            process_json(&mut json, &query, &args, &mut lazy::Evaluator::new()).unwrap(),
            [r#"{"a":3,"b":{"x":2,"y":1},"d":1}"#]
        );
    }

    #[test]
    fn output_keeps_source_order_without_the_flag() {
        let mut json = br#"{ "root": { "d": 1, "a": 3 } }"#.to_vec();
        let args = Args::parse_from(["jql", "--inline", r#""root""#]);
        let query = resolve_query(&args).unwrap();

        assert_eq!(
            process_json(&mut json, &query, &args, &mut lazy::Evaluator::new()).unwrap(),
            [r#"{"d":1,"a":3}"#]
        );
    }
}
