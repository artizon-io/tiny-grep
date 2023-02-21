use clap::ValueEnum;
use colored::*;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub file_path: Box<Path>,
    pub search_options: SearchOptions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Theme {
    /// Fancy blue color
    Blue,
    /// Fancy green color
    Green,
    /// Fancy purple color
    Purple,
}

pub struct SearchOptions {
    case_sensitive: bool,
    line_numbered: bool,
    colored: bool,
    theme: Theme,
}

impl SearchOptions {
    pub fn new(
        case_sensitive: bool,
        line_numbered: bool,
        colored: bool,
        theme: Theme,
    ) -> SearchOptions {
        SearchOptions {
            case_sensitive,
            line_numbered,
            colored,
            theme,
        }
    }
}

impl Config {
    // Convention is new() never fails
    pub fn new(
        query: String,
        file_path: Box<Path>,
        case_sensitive: bool,
        line_numbered: bool,
        colored: bool,
        theme: Theme,
    ) -> Config {
        Config {
            query: query.to_string(),
            file_path,
            search_options: SearchOptions {
                case_sensitive,
                line_numbered,
                colored,
                theme,
            },
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for line in search(&config.query, &config.file_path, &config.search_options)? {
        println!("{line}");
    }

    Ok(())
}

fn search(
    query: &str,
    file_path: &Box<Path>,
    search_options: &SearchOptions,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut results = Vec::new();

    if file_path.is_dir() {
        for child_entry in file_path.read_dir()? {
            let child_entry = child_entry.unwrap();
            let mut child_results = search(query, &Box::from(child_entry.path()), search_options)?;
            if child_results.len() > 0 {
                let heading = String::from(child_entry.path().to_str().unwrap());

                let colored_heading = match search_options.theme {
                    Theme::Blue => heading.blue(),
                    Theme::Green => heading.green(),
                    Theme::Purple => heading.purple(),
                };

                results.push(String::from(""));

                results.push(if search_options.colored {
                    colored_heading.to_string()
                } else {
                    heading
                });
                results.append(&mut child_results);
            }
        }
        Ok(results)
    } else if file_path.is_file() {
        let contents = fs::read_to_string(file_path)?;
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
                            search_options.theme,
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
                            search_options.theme,
                            &mut results,
                        );
                    }
                }
            }
        }

        Ok(results)
    } else {
        Err("Path is neither a file nor a directory".into())
    }
}

fn add_to_result(
    i: usize,
    line: &str,
    colored: bool,
    line_numbered: bool,
    query: &str,
    theme: Theme,
    results: &mut Vec<String>,
) {
    let (colored_query, colored_line_index) = match theme {
        Theme::Blue => (query.blue().bold(), (i + 1).to_string().blue()),
        Theme::Green => (query.green().bold(), (i + 1).to_string().green()),
        Theme::Purple => (query.purple().bold(), (i + 1).to_string().purple()),
    };

    match line_numbered {
        true => {
            match colored {
                true => {
                    results.push(format!(
                        "{}: {}",
                        colored_line_index,
                        // The coloring is done inside ColoredString's deref() therefore must first cast it to a String
                        // https://stackoverflow.com/questions/52792990/why-does-replacing-a-substring-with-a-colored-string-from-the-colored-crate-not
                        line.replace(query, &colored_query.to_string())
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
                    line.replace(query, &colored_query.to_string())
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
    // use indoc::indoc;

    #[test]
    fn search_single_file_case_insensitive() {
        let query = "hello";
        // let contents = indoc! {"
        //     hello world.
        //     fello world.
        //     World Hello World.
        // "};

        assert_eq!(
            vec![
                "hello world.",
                "World Hello World.",
                "hello you hello world."
            ],
            // Can directly use private functions in tests
            search(
                query,
                &Box::from(Path::new("./tests/hello_world.txt")),
                &SearchOptions {
                    case_sensitive: false,
                    line_numbered: false,
                    colored: false,
                    theme: Theme::Blue
                },
            )
            .expect("Should not fail")
        );
    }

    #[test]
    fn search_single_file_case_sensitive() {
        let query = "hello";

        assert_eq!(
            vec!["hello world.", "hello you hello world."],
            search(
                query,
                &Box::from(Path::new("./tests/hello_world.txt")),
                &SearchOptions {
                    case_sensitive: true,
                    line_numbered: false,
                    colored: false,
                    theme: Theme::Blue
                }
            )
            .expect("Should not fail")
        );
    }
}
