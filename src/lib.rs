use clap::{command, Command};

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// env!("CARGO_CRATE_NAME")
const APPNAME: &str = env!("CARGO_CRATE_NAME");

pub fn create_cli() {
    let base = Command::new(APPNAME).subcommand(command!("example"));

    let matches = base.get_matches();
    match matches.subcommand() {
        Some(("example", _matches)) => println!("did it work?"),
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
