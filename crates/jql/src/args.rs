use std::path::PathBuf;

use clap::{Parser, ValueHint};

static QUERY_HELP: &str = r#"
A query is sequence of tokens used to make a selection on a JSON input.

A query must be enclosed by single quotation marks.

The following tokens are available to build up a query:

== Separators ==

Group separator ,
    ┬
    ╰→ query '"a","b","c"' will build up an array from sub-queries

== Selectors ==

-- Arrays --

Array index selector [0,2,1]
    ┬
    ╰→ indexes can be used in arbitrary order

Array range selector [2:0]
    ┬
    ╰→ range can be in natural order [0:2], reversed [2:0],
       without lower [:2] or upper bound [0:]

Lens selector |={"a","b"=true,"c"=null,"d"=1,"e"="string"}
    ┬
    ╰→ lens can be a key only or a combination of key/value,
       a value being any of boolean | null | number | string

-- Objects --

Key selector "a"
    ┬
    ╰→ any valid JSON key

Multi key selector {"a","c","b"}
    ┬
    ╰→ keys can be used in arbitrary order

Object index selector {0,2,1}
    ┬
    ╰→ indexes can be used in arbitrary order

Object range selector {2:0}
    ┬
    ╰→ range can be in natural order {0:2}, reversed {2:0},
       without lower {:2} or upper bound {0:}

== Operators ==

Flatten operator ..
    ┬
    ╰→ flattens arrays and objects

Pipe in operator |> 
    ┬
    ╰→ applies the next tokens in parallel on each element of an array

Pipe out operator <|
    ┬
    ╰→ stops the parallelization initiated by the pipe in operator

Truncate operator !
    ┬
    ╰→ maps the output into simple JSON primitives
       boolean | null | number | string | [] | {}
"#;

#[derive(Debug, Parser)]
#[command(
    about, 
    author,
    long_about = None,
    version, 
)]
pub(crate) struct Args {
    /// Query argument.
    #[arg(
        conflicts_with = "validate",
        help = "Query to apply to the JSON data", 
        index = 1,
        long_help = QUERY_HELP,
        required_unless_present_any = ["no-query"]
    )]
    pub(crate) query: Option<String>,

    /// JSON file argument.
    #[arg(
        help = "JSON file to use", 
        index = 2, 
        value_hint = ValueHint::FilePath,
        value_name = "OPTIONAL_FILE",
    )]
    pub(crate) json_file: Option<PathBuf>,

    /// Inline JSON flag.
    #[arg(
        conflicts_with = "validate",
        help = "Inline the JSON output",
        long = "inline",
        short = 'i',
    )]
    pub(crate) inline: bool,

    /// Query from file flag.
    #[arg(
        group = "no-query",
        help = "Read the query from file",
        long = "query",
        long_help = QUERY_HELP,
        short = 'q',
        value_hint = ValueHint::FilePath,
        value_name = "FILE",
    )]
    pub(crate) query_from_file: Option<PathBuf>,

    /// Raw string flag.
    #[arg(
        help = "Write to stdout without JSON double-quotes (string only)",
        long = "raw-string",
        short = 'r',
    )]
    pub(crate) raw_string: bool,
   
    /// Stream flag.
    #[arg(
        help = "Read a stream of JSON data line by line",
        long = "stream",
        short = 's',
    )]
    pub(crate) stream: bool,

    /// Validate JSON data flag.
    #[arg(
        group = "no-query",
        help = "Validate the JSON data", 
        long = "validate",
        short = 'v'
    )]
    pub(crate) validate: bool,
}

#[test]
fn check_args() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
