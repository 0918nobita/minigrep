extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|msg| {
        eprintln!("Problem parsing arguments: {}", msg);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
