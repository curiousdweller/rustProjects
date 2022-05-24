use std::env;
use std::process;
use minigrep::{Config, run};


fn main() {
    let collated_args = env::args();
    let config = Config::new(collated_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Looking for: {} \nIn: {}", config.data, config.filename);
    if let Err(e) = run(config) {
        eprintln!("Error in application: {}", e);
        process::exit(1);
    };

}



