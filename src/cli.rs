use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

/// Get the CLI matches.
pub fn get_matches() -> ArgMatches {
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("selectors")
                .about("Selectors to apply")
                .index(1)
                .required_unless_present("check")
        )
        .arg(
           Arg::new("JSON")
                .about("JSON file to use")
                .index(2)
                .required(false),
        )
        .arg(
            Arg::new("check")
                .about("Checks if the input is valid JSON")
                .long("check")
                .short('c'),
        )
        .arg(
            Arg::new("inline")
                .about("Inlines JSON output")
                .long("inline")
                .short('i'),
        )
        .arg(
            Arg::new("raw-output")
                .about("Writes raw string selection directly to standard output without JSON double-quotes")
                .long("raw-output")
                .short('r'),
        )
        .arg(
            Arg::new("stream")
                .about("Reads a stream of JSON data line by line")
                .conflicts_with("check")
                .long("stream")
                .short('s'),
        )
        .get_matches()
}
