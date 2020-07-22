extern crate minigrep;

use std::env;
use std::process;

use minigrep::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|msg| {
        eprintln!("Problem parsing arguments: {}", msg);
        process::exit(1);
    });

    let result = minigrep::run(&config);
    if let Err(e) = result {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
