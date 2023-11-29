use clap::{command, ArgMatches, Command};
pub fn add_two(a: i32) -> i32 {
    a + 2
}
const APPNAME: &str = env!("CARGO_CRATE_NAME");
const APPHELP: &str = "A Python Package manger written in Rust";

pub fn create_cli() -> Command {
    Command::new(APPNAME)
        .about(APPHELP)
        .subcommand_required(true)
        .disable_help_subcommand(true)
        .subcommands([command!("example")])
}

pub fn handle_first_match(matches: ArgMatches) {
    let payload = matches.subcommand();

    match payload {
        Some(("example", _matches)) => println!("did it work?"),

        // If all subcommands are defined above, anything else is unreachable!()
        _ => unreachable!(),
    };
}
