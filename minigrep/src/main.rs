use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // Instead of creating a vector of strings and a reference to it, we can just pass
    // the entire iterator instead
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("A system error occured, {}", e);
        process::exit(1);
    };
}