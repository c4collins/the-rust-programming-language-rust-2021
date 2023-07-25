use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let error = Self::check_for_args_errors(args);
        if error.len() > 0 {
            return Err(error);
        }

        let config = Self::get_new_config(args);

        Ok(config)
    }

    fn check_for_args_errors(args: &[String]) -> &'static str {
        // path/to/file.rs look_for in_file_path [--case_insensitive]

        let len = args.len();
        if len < 3 {
            "Not enough arguments"
        } else if args.len() > 4 {
            "Too many arguments"
        } else {
            "" // No errors, program may continue
        }
    }

    fn get_new_config(args: &[String]) -> Config {
        let parent_path = "src/chapters/c12_an_io_project/minigrep/";

        let query = args[1].clone();
        let file_path = format!("{}{}", parent_path, &args[2]);

        let mut ignore_case = false;
        if args.len() > 3 {
            if args[3] == "--case_insensitive" {
                ignore_case = true;
            }
        } else {
            ignore_case = env::var("IGNORE_CASE").is_ok()
        }

        Config {
            query,
            file_path,
            ignore_case,
        }
    }
}
