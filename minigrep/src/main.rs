use minigrep::{conf::Config, search, search_case_insensitive};
use std::{env, error::Error, fs, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(conf).unwrap();
}
