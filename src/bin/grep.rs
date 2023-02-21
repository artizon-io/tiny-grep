use clap::{Parser, ValueEnum};
use requestty::Question;
use std::env;
use std::path::Path;
use std::process;

use tiny_grep::Config;

/// A grep terminal utliity program written in Rust
#[derive(Parser, Debug)]
// Fill fields from Cargo.toml
#[command(author, version, about)]
#[command(next_line_help = true)]
struct Cli {
    /// The query string to search for
    // To make it a keyword argument:
    // #[arg(long)]
    query: Option<String>,

    /// The file path to search in
    #[arg(value_parser = file_path_parser)]
    file_path: Option<String>,

    /// Interactive mode
    #[arg(long)]
    interactive: bool,

    /// Case sensitive search
    #[arg(long)]
    case_sensitive: bool,

    /// Theme
    #[arg(long, value_enum)]
    theme: Option<Theme>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Theme {
    /// Fancy blue color
    Blue,
    /// Fancy teal color
    Teal,
}

fn file_path_parser(file_path: &str) -> Result<String, String> {
    // Returning &str can cause lifetime issues
    match Path::new(file_path).exists() {
        true => Ok(file_path.to_string()),
        false => Err(format!("File path {} does not exist", file_path)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // env::var() returns Result enum
    // is_ok() returns true if the Result enum is Ok variant
    let case_sensitive_env_var = env::var("CASE_SENSITIVE").is_ok();

    let config = match cli.interactive {
        true => {
            let query = requestty::prompt_one(
                Question::input("query")
                    .message("String to search for?")
                    .default("Hello world")
                    .build(),
            )
            .unwrap();

            let file_path = requestty::prompt_one(
                Question::input("file_path")
                    .message("File to search in?")
                    .default("text.txt")
                    .build(),
            )
            .unwrap();

            // The &str gets the same lifetime as the argument
            let file_path = &file_path_parser(file_path.as_string().unwrap()).unwrap();

            let case_sensitive = requestty::prompt_one(
                Question::confirm("case_sensitive")
                    .message("Case sensitive search?")
                    .default(false)
                    .build(),
            )
            .unwrap();

            // Must as_string().unwrap() here in order to set the lifetime variable correctly
            Config::new(
                query.as_string().unwrap(),
                file_path,
                case_sensitive.as_bool().unwrap(),
            )
        }
        false => {
            // env::args() will return an iterator over the arguments
            let args: Vec<String> = env::args().collect();
            dbg!("Running grep binary located in: {}", &args[0]);
            // args[1..] will equal to the nth command line argument

            Config::new(
                &cli.query.unwrap(),
                &cli.file_path.unwrap(),
                case_sensitive_env_var || cli.case_sensitive,
                // true if either env var is set or cli arg is set
            )
        }
    };

    // run() will take ownership of config
    if let Err(e) = tiny_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
