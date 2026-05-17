use std::env; // brings env module into scope
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // converts the iterator into a vector of strings

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Probelm parsing the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
    dbg!(args);
}
