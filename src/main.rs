use payments::run_transactions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match run_transactions(&args) {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }
}
