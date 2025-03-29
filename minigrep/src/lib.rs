use std::error::Error;
use std::{env, fs};

#[test]
mod test {
    pub fn test_search() {}
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            None => return Err("请输入query参数"),
            Some(value) => value,
        };

        let file_path = match args.next() {
            None => return Err("请输入路径参数"),
            Some(value) => value,
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search(query: String, contents: &str) -> Vec<&str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive(query: String, contents: &str) -> Vec<&str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(config.query, contents.as_str())
    } else {
        search(config.query, contents.as_str())
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
