use std::io::{self, Write};
use std::process::Command as ShellCommand;

const DEFAULT_PIP_EXECUTABLE: &str = "pip3";

pub fn install_pip_module(module: &str, upgrade: bool) {
    let mut x = ShellCommand::new(DEFAULT_PIP_EXECUTABLE);
    let install = x.arg("install");

    println!("Installing {module}...");

    if upgrade == true {
        install.arg("--upgrade");
    }
    install.arg(module);

    let output = install.output().expect("Failed to execute");
    let stdout = output.stdout;
    let _stderror = output.stderr;

    // ...
    io::stdout().write_all(&stdout).unwrap();

    println!("whoknows")
}
