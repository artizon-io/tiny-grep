use colored::*;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub search_options: SearchOptions,
}

pub struct SearchOptions {
    case_sensitive: bool,
    line_numbered: bool,
    colored: bool,
}

impl SearchOptions {
    pub fn new(case_sensitive: bool, line_numbered: bool, colored: bool) -> SearchOptions {
        SearchOptions {
            case_sensitive,
            line_numbered,
            colored,
        }
    }
}

impl Config {
    // Convention is new() never fails
    pub fn new(
        query: &str,
        file_path: &str,
        case_sensitive: bool,
        line_numbered: bool,
        colored: bool,
    ) -> Config {
        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            search_options: SearchOptions {
                case_sensitive,
                line_numbered,
                colored,
            },
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
                line_numbered: false,
                colored: false,
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

fn search(query: &str, contents: &str, search_options: &SearchOptions) -> Vec<String> {
    let mut results = Vec::new();

    match search_options.case_sensitive {
        true => {
            for (i, line) in contents.lines().enumerate() {
                if line.contains(query) {
                    add_to_result(
                        i,
                        line,
                        search_options.colored,
                        search_options.line_numbered,
                        query,
                        &mut results,
                    );
                }
            }
        }
        false => {
            let query_lowercase = &query.to_lowercase();
            for (i, line) in contents.lines().enumerate() {
                if line.to_lowercase().contains(query_lowercase) {
                    add_to_result(
                        i,
                        line,
                        search_options.colored,
                        search_options.line_numbered,
                        query,
                        &mut results,
                    );
                }
            }
        }
    }

    results
}

fn add_to_result(
    i: usize,
    line: &str,
    colored: bool,
    line_numbered: bool,
    query: &str,
    results: &mut Vec<String>,
) {
    match line_numbered {
        true => {
            match colored {
                true => {
                    results.push(format!(
                        "{}: {}",
                        (i + 1).to_string().blue(),
                        // The coloring is done inside ColoredString's deref() therefore must first cast it to a String
                        // https://stackoverflow.com/questions/52792990/why-does-replacing-a-substring-with-a-colored-string-from-the-colored-crate-not
                        line.replace(query, &query.blue().bold().to_string())
                    ));
                }
                false => {
                    // Cannot make it a &str because we will be forced to allocate a new string
                    // but it will be droppeed at the end of the function
                    // Therefore, we make results a Vec<String> instead of Vec<&'a str> with 'a being life time of line
                    results.push(format!("{}: {line}", i + 1));
                }
            }
        }
        false => match colored {
            true => {
                results.push(format!(
                    "{}",
                    line.replace(query, &query.blue().bold().to_string())
                ));
            }
            false => {
                results.push(line.to_string());
            }
        },
    }
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
                    case_sensitive: false,
                    line_numbered: false,
                    colored: false
                },
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
                    case_sensitive: true,
                    line_numbered: false,
                    colored: false
                }
            )
        );
    }
}
