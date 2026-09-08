use std::path::PathBuf;

use clap::{
    CommandFactory,
    Parser,
    ValueHint,
    error::ErrorKind,
};

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
    ╰→ lens can be a combination of one or more selectors with an optional value,
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

Keys operator @
    ┬
    ╰→ returns the keys of an object or the indexes of an array,
       other primitives are returned as is

Pipe in operator |> 
    ┬
    ╰→ applies the next tokens on each element of an array

Pipe out operator <|
    ┬
    ╰→ stops the iteration initiated by the pipe in operator

Truncate operator !
    ┬
    ╰→ maps the output into simple JSON primitives
       boolean | null | number | string | [] | {}
"#;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(
    about,
    author,
    long_about = None,
    version
)]
pub(crate) struct Args {
    /// Query argument.
    #[arg(
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
        value_name = "OPTIONAL_FILE"
    )]
    pub(crate) json_file: Option<PathBuf>,

    /// Inline JSON flag.
    #[arg(
        conflicts_with = "validate",
        help = "Inline the JSON output",
        long = "inline",
        short = 'i'
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
        value_name = "FILE"
    )]
    pub(crate) query_from_file: Option<PathBuf>,

    /// Raw string flag.
    #[arg(
        help = "Write to stdout without JSON double-quotes (string only)",
        long = "raw-string",
        short = 'r'
    )]
    pub(crate) raw_string: bool,

    /// Sort keys flag.
    #[arg(
        conflicts_with = "validate",
        help = "Sort the keys of every object in the JSON output",
        long = "sort-keys",
        short = 'S'
    )]
    pub(crate) sort_keys: bool,

    /// Stream flag.
    #[arg(
        help = "Read a stream of JSON data line by line",
        long = "stream",
        short = 's'
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

impl Args {
    /// Parses the command-line arguments, reporting misuse the way clap does.
    pub(crate) fn get() -> Self {
        match Self::parse().with_positionals_in_place() {
            Ok(args) => args,
            Err(message) => Self::command()
                .error(ErrorKind::TooManyValues, message)
                .exit(),
        }
    }

    /// Moves the lone positional into the file slot when no query is read from
    /// the command line.
    ///
    /// `--validate` and `--query` take no query, so `jql -v input.json` means
    /// the file. clap fills the positionals by their index either way — it
    /// cannot renumber one per flag — leaving the path in the query slot.
    fn with_positionals_in_place(mut self) -> Result<Self, String> {
        if !self.validate && self.query_from_file.is_none() {
            return Ok(self);
        }

        if let Some(first) = self.query.take() {
            if let Some(second) = &self.json_file {
                return Err(format!(
                    "'{first}' and '{}' were both provided, but only one file is read with --validate or --query",
                    second.display()
                ));
            }

            self.json_file = Some(PathBuf::from(first));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(arguments: &[&str]) -> Result<Args, String> {
        Args::parse_from(arguments).with_positionals_in_place()
    }

    #[test]
    fn check_args() {
        Args::command().debug_assert();
    }

    #[test]
    fn check_file_is_read_without_a_query() {
        let args = parse(&["jql", "--validate", "input.json"]).unwrap();

        assert_eq!(args.json_file, Some(PathBuf::from("input.json")));
        assert_eq!(args.query, None);

        let args = parse(&["jql", "--query", "query.jql", "input.json"]).unwrap();

        assert_eq!(args.json_file, Some(PathBuf::from("input.json")));
        assert_eq!(args.query, None);
    }

    #[test]
    fn check_query_and_file_are_left_alone() {
        let args = parse(&["jql", r#""a""#, "input.json"]).unwrap();

        assert_eq!(args.query.as_deref(), Some(r#""a""#));
        assert_eq!(args.json_file, Some(PathBuf::from("input.json")));

        let args = parse(&["jql", r#""a""#]).unwrap();

        assert_eq!(args.query.as_deref(), Some(r#""a""#));
        assert_eq!(args.json_file, None);
    }

    #[test]
    fn check_stdin_is_left_alone_without_a_query() {
        let args = parse(&["jql", "--validate"]).unwrap();

        assert_eq!(args.json_file, None);
        assert_eq!(args.query, None);
    }

    #[test]
    fn check_two_files_are_rejected_without_a_query() {
        assert!(parse(&["jql", "--validate", "one.json", "two.json"]).is_err());
        assert!(parse(&["jql", "--query", "query.jql", "one.json", "two.json"]).is_err());
    }
}
