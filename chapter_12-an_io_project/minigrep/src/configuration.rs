pub struct Config {
    pub query: String,
    pub file_path: String,
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
        if args.len() < 3 {
            "Not enough arguments"
        } else if args.len() > 3 {
            "Too many arguments"
        } else {
            "" // No errors, program may continue
        }
    }

    fn get_new_config(args: &[String]) -> Config {
        let parent_path = "chapter_12-an_io_project/minigrep/";

        let query = args[1].clone();
        let file_path = format!("{}{}", parent_path, &args[2]);

        Config { query, file_path }
    }
}
