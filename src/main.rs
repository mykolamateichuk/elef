use std::env;
use std::process;

use elef::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem configuring arguments: {}.", err);
        process::exit(1);
    });

    if let Err(e) = elef::run(config) {
        eprintln!("Problem executing: {e}.");
        process::exit(1);
    }
}
