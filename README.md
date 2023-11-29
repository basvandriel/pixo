# pixo

## Running
`cargo run`

## Preparing to create a coverage report
```bash
export RUSTFLAGS="-C instrument-coverage=all"

cargo install rustfilt
cargo install grcov

rustup component add llvm-tools-preview
```

Then, simply use `make coveragereport`