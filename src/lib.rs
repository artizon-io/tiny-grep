use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub search_options: SearchOptions,
}

pub struct SearchOptions {
    case_sensitive: bool,
}

impl Config {
    // Convention is new() never fails
    pub fn new(query: &str, file_path: &str, case_sensitive: bool) -> Config {
        Config {
            query: query.clone().to_string(),
            file_path: file_path.clone().to_string(),
            search_options: SearchOptions { case_sensitive },
        }
    }

    /// Build a new Config struct using the array of command line arguments
    // Let error value be a string literal that have 'static lifetime (for now)
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Less than 2 arguments received");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path,
            search_options: SearchOptions {
                case_sensitive: false,
            },
        })
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

    match search_options.case_sensitive {
        true => {
            for line in contents.lines() {
                if line.contains(query) {
                    results.push(line);
                }
            }
        }
        false => {
            let query_lowercase = &query.to_lowercase();
            for line in contents.lines() {
                if line.to_lowercase().contains(query_lowercase) {
                    results.push(line);
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn search_case_insensitive() {
        let query = "hello";
        let contents = indoc! {"
            hello world.
            fello world.
            World Hello World.
        "};

        assert_eq!(
            vec!["hello world.", "World Hello World."],
            // Can directly use private functions in tests
            search(
                query,
                contents,
                &SearchOptions {
                    case_sensitive: false
                }
            )
        );
    }

    #[test]
    fn search_case_sensitive() {
        let query = "hello";
        let contents = indoc! {"
            Hello world.
            hello world.
            World Hello world.
        "};

        assert_eq!(
            vec!["hello world."],
            search(
                query,
                contents,
                &SearchOptions {
                    case_sensitive: true
                }
            )
        );
    }
}
