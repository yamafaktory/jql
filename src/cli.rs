use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

/// Get the CLI matches.
pub fn get_matches() -> ArgMatches {
    Command::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("selectors")
                .help("Selectors to apply")
                .index(1)
                .required_unless_present("check")
        )
        .arg(
           Arg::new("JSON")
                .help("JSON file to use")
                .index(2)
                .required(false),
        )
        .arg(
            Arg::new("check")
                .help("Checks if the input is valid JSON")
                .long("check")
                .short('c'),
        )
        .arg(
            Arg::new("inline")
                .help("Inlines JSON output")
                .conflicts_with("check")
                .long("inline")
                .short('i'),
        )
        .arg(
            Arg::new("raw-output")
                .help("Writes raw string selection directly to standard output without JSON double-quotes")
                .conflicts_with("check")
                .long("raw-output")
                .short('r'),
        )
        .arg(
            Arg::new("stream")
                .help("Reads a stream of JSON data line by line")
                .conflicts_with("check")
                .long("stream")
                .short('s'),
        )
        .get_matches()
}
