use pixo::add::install_pip_module;

#[test]
fn it_expects_pip_info() {
    let _ = install_pip_module("pytest", true);
}
