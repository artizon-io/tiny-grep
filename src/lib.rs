use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub search_options: SearchOptions
}

pub struct SearchOptions {
    case_sensitive: bool
}

impl Config {
    // Convention is new() never fails
    pub fn new(args: &[String]) -> Config {
        assert!(args.len() >= 3, "Less than 2 arguments received");

        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Config { query, file_path, search_options: SearchOptions { case_sensitive } }
    }

    // Let error value be a string literal that have 'static lifetime
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Less than 2 arguments received");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // env::var() returns Result enum
        // is_ok() returns true if the Result enum is Ok variant
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config { query, file_path, search_options: SearchOptions { case_sensitive }})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents, &config.search_options) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, search_options: &SearchOptions) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query_lowercase = &query.to_lowercase();

    match search_options.case_sensitive {
        true => for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        false => for line in contents.lines() {
            if line.to_lowercase().contains(query_lowercase) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = indoc! {"
            hello world.
            fello world.
            World Hello World.
        "};

        assert_eq!(vec!["hello world.", "World Hello World."], search(query, contents, &SearchOptions { case_sensitive: false }));
    }

    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = indoc! {"
            Hello world.
            hello world.
            World Hello world.
        "};

        assert_eq!(
            vec!["hello world."],
            search(query, contents, &SearchOptions { case_sensitive: true })
        );
    }
}