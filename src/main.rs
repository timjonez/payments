use payments::run_transactions;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(err) = run_transactions(&args) {
        eprintln!("Error: {}", err);
        process::exit(1)
    }
}
