# Payments Engine
#### A Simple payment engine written in Rust

## Usage
- [Install the rust toolchain](https://www.rust-lang.org/learn/get-started)
- `Payments` writes output to stdout ( you can pipe it into a file as needed)
- Run a development version
    - `cargo run transactions.csv`
- Build and run for/in production
    - `cargo build --release`
    - `./target/release/payments transactions.csv`
## Tests
- No tests are included at this time - this is a simple engine and requires work to be production ready

## Design Methodology
- Uses Domain Driven Design to leverage Rust's type system
    - Validates/sanitizes incoming data via the type system - see `Transaction` and `TransactionType`
- `Account` uses an associated function to handle field updates

## TODO
- Add tests
    - Unit
    - Integration
- Remove `pub` from struct fields, and expose via methods
