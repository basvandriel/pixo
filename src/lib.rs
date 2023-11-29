use clap::{command, ArgMatches, Command};
pub fn add_two(a: i32) -> i32 {
    a + 2
}
const APPNAME: &str = env!("CARGO_CRATE_NAME");

pub fn create_cli() -> Command {
    let commands = [command!("example")];
    let base = Command::new(APPNAME).subcommands(commands);

    base
}

pub fn handle_first_match(matches: ArgMatches) {
    let payload = matches.subcommand();

    match payload {
        Some(("example", _matches)) => println!("did it work?"),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
