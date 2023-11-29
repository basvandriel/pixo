# pixo

## Running
`cargo run`

With arguments example

`cargo run -- --name Bas`

## Preparing to create a coverage report
```bash
export RUSTFLAGS="-C instrument-coverage=all"

cargo install rustfilt
cargo install grcov

rustup component add llvm-tools-preview
```

Then, simply use `make coveragereport`


  <!-- tip: to pass '--name' as a value, use '-- --name' -->