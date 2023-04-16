use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Query argument.
    #[arg(
        help = "Query to apply to the JSON data", 
        index = 1, 
    )]
    pub(crate) query: String,

    /// JSON file argument.
    #[arg(
        help = "JSON file to use", 
        index = 2, 
        value_hint = clap::ValueHint::FilePath,
        value_name = "OPTIONAL_FILE",
    )]
    pub(crate) json_file: Option<PathBuf>,

    /// Validate JSON data flag.
    #[arg(
        help = "Validate the JSON data", 
        long = "validate",
        short = 'v'
    )]
    pub(crate) validate: bool,

    /// Query from file flag.
    #[arg(
        help = "Read the query from file",
        long = "query-from-file",
        short = 'q',
        value_hint = clap::ValueHint::FilePath,
        value_name = "FILE",
    )]
    pub(crate) from_file: Option<PathBuf>,

    /// Inline JSON output flag.
    #[arg(
        conflicts_with = "validate",
        help = "Inline the JSON output",
        long = "inline",
        short = 'i',
    )]
    pub(crate) inline: bool,
    
    /// Raw string output flag.
    #[arg(
        help = "Write to stdout without JSON double-quotes (string only)",
        long = "raw-string-output",
        short = 'r',
    )]
    pub(crate) raw_string_output: bool,
   
    /// Stream flag.
    #[arg(
        help = "Reads a stream of JSON data line by line",
        long = "stream",
        short = 's',
    )]
    pub(crate) stream: bool,
}
