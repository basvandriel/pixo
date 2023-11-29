use pixo::{self, create_cli};

#[test]
fn it_expected_right_about() {
    let cli = create_cli();

    let about = cli
        .get_about()
        .expect("Didn't find the about section")
        .to_string();

    let expected: String = "A Python Package manger written in Rust".into();

    assert_eq!(about, expected);
}
