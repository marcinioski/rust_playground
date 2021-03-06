use std::env;
use std::process;

use minigrep::Config;

mod tests;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem during parsing: {}", err);
        process::exit(1);
    });

    println!("looking for: {} from: {}", config.query, config.filename);

    if let Err(e) = minigrep::run(&config) {
        eprintln!("appearance error: {}", e);
        process::exit(1);
    }
}
