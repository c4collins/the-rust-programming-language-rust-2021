use std::error::Error;
use std::fs;

pub mod configuration;

pub fn run(config: configuration::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{}", contents);

    Ok(())
}
