use pixo;

#[test]
fn it_adds_two() {
    let actual = pixo::add_two(2);
    assert_eq!(4, actual);
}
