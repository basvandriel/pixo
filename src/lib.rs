use clap::{command, ArgMatches, Command};

const APPHELP: &str = "A Python Package manger written in Rust";

// CommandHandler class -> create(), handle()
// CommandCollecter -> get_commands(), match()

pub fn create_cli() -> Command {
    Command::new(
        env!("CARGO_CRATE_NAME"), // noformat
    )
    .about(APPHELP)
    .subcommand_required(true)
    .disable_help_subcommand(true)
    .subcommands([command!("example"), command!("add")])
}

pub fn handle_first_match(matches: ArgMatches) {
    let payload = matches.subcommand();

    match payload {
        Some(("example", _matches)) => println!("did it work?"),
        Some(("add", _matches)) => println!("hjadf"),

        // no match
        _ => unreachable!(),
    };
}
