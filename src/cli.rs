use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction, ArgMatches,
    Command,
};

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
                .required_unless_present_any(["check", "from-file"])
        )
        .arg(
           Arg::new("JSON")
                .help("JSON file to use")
                .index(2)
                .required(false)
        )
        .arg(
            Arg::new("check")
                .action(ArgAction::SetTrue)
                .help("Checks if the input is valid JSON") .long("check")
                .short('c')
        )
        .arg(
            Arg::new("from-file")
                .conflicts_with("check")
                .help("Reads selectors from file rather than from a command line")
                .long("from-file")
                .num_args(1)
                .short('f')
                .use_value_delimiter(false)
                .value_name("FILE")
        )
        .arg(
            Arg::new("inline")
                .action(ArgAction::SetTrue)
                .conflicts_with("check")
                .help("Inlines JSON output")
                .long("inline")
                .short('i')
        )
        .arg(
            Arg::new("raw-output")
                .action(ArgAction::SetTrue)
                .conflicts_with("check")
                .help("Writes raw string selection directly to standard output without JSON double-quotes")
                .long("raw-output")
                .short('r')
        )
        .arg(
            Arg::new("stream")
                .action(ArgAction::SetTrue)
                .conflicts_with("check")
                .help("Reads a stream of JSON data line by line")
                .long("stream")
                .short('s')
        )
        .get_matches()
}
