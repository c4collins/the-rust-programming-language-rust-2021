use std::{env, process};

mod iterators;
mod minigrep_plus;
mod shirt_company;

pub fn run() {
    shirt_company::main();
    iterators::main();

    let config = minigrep_plus::configuration::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1)
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    if let Err(e) = minigrep_plus::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
