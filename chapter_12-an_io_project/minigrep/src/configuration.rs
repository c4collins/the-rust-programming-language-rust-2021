pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        if args.len() > 3 {
            return Err("Too many arguments");
        }
        let parent_path = "chapter_12-an_io_project/minigrep/";

        let query = args[1].clone();
        let file_path = format!("{}{}", parent_path, &args[2]);

        Ok(Config { query, file_path })
    }
}
