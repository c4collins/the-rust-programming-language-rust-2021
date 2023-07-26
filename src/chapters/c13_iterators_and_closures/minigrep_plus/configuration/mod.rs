use std::env;

use num::traits::AsPrimitive;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let parent_path = "src/chapters/c13_iterators_and_closures/minigrep_plus/";

        println!("{:?}", args.next());

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => format!("{}{}", parent_path, arg),
            None => return Err("Didn't get a file path"),
        };

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for arg in args {
            match arg.as_str() {
                "--case-insensitive" | "--case_insensitive" => ignore_case = true,
                _ => {} // Anything else is ignored
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
