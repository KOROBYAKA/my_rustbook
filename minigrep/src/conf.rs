pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        match args.len() {
            0..3 => return Err("not enough arguments"),
            _ => {
                return Ok(Config {
                    query: args.get(1).unwrap().clone(),
                    file_path: args.get(2).unwrap().clone(),
                    ignore_case: ignore_case,
                });
            }
        }
    }
}
