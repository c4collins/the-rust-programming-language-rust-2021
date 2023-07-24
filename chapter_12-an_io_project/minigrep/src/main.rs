use std::{env, fs, process};

mod configuration;

use configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = configuration::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect(
        format!(
            "Should have been able to read the file @ {}",
            config.file_path
        )
        .as_str(),
    );
    println!("With text:\n{}", contents);
}
