use clap::{App, Arg, ArgMatches};
use utils::get_cargo_version;


/// Get the CLI matches.
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("jql")
        .version(get_cargo_version().as_str())
        .author("Davy Duperron <yamafaktory@gmail.com>")
        .about("JSON query language")
        .arg(
            Arg::with_name("JSON")
                .help("JSON file to use")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("selector")
                .help("Selector to apply")
                .index(2)
                .required(true),
        ).arg(
            Arg::with_name("pretty-print")
                .help("Pretty print the JSON input")
                .long("pretty-print")
                .short("p"),
        ).get_matches()
}
