use pixo::{create_cli, handle_first_match};

fn main() {
    let cli = create_cli();
    let matches = cli.get_matches();

    handle_first_match(matches)
}
