use clap::{command, ArgMatches, Command};
pub fn add_two(a: i32) -> i32 {
    a + 2
}

const APPHELP: &str = "A Python Package manger written in Rust";

pub fn create_cli() -> Command {
    Command::new(
        env!("CARGO_CRATE_NAME"), // noformat
    )
    .about(APPHELP)
    .subcommand_required(true)
    .disable_help_subcommand(true)
    .subcommands([command!("example")])
}

pub fn handle_first_match(matches: ArgMatches) {
    let payload = matches.subcommand();

    match payload {
        Some(("example", _matches)) => println!("did it work?"),

        // no match
        _ => unreachable!(),
    };
}
