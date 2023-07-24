use minigrep;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::configuration::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1)
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
