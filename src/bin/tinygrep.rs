use clap::Parser;
use requestty::Question;
use std::env;
use std::path::Path;
use std::process;

use tiny_grep::{Config, Theme};

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
    file_path: Option<Box<Path>>,

    /// Interactive mode
    #[arg(long)]
    interactive: bool,

    /// Case sensitive search
    #[arg(long)]
    case_sensitive: bool,

    /// Display line number
    #[arg(long)]
    line_number: bool,

    /// Use colored output
    #[arg(long)]
    color: bool,

    /// Theme
    #[arg(long, value_enum)]
    theme: Option<Theme>,
}

fn file_path_parser(file_path_str: &str) -> Result<Box<Path>, String> {
    // Path's instance size cannot be deduced at compile time hence should be stored on heap(?)
    let file_path = Path::new(file_path_str);
    match file_path.exists() {
        true => Ok(Box::from(file_path)),
        false => Err(format!("File path {} does not exist", file_path_str)),
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

            let file_path = file_path_parser(file_path.as_string().unwrap()).unwrap();

            let case_sensitive = requestty::prompt_one(
                Question::confirm("case_sensitive")
                    .message("Case sensitive search?")
                    .default(false)
                    .build(),
            )
            .unwrap();

            let line_numbered = requestty::prompt_one(
                Question::confirm("line_numbered")
                    .message("Show line number?")
                    .default(true)
                    .build(),
            )
            .unwrap();

            let colored = requestty::prompt_one(
                Question::confirm("colored")
                    .message("Use colored output?")
                    .default(true)
                    .build(),
            )
            .unwrap();

            // Must as_string().unwrap() here in order to set the lifetime variable correctly
            Config::new(
                String::from(query.as_string().unwrap()),
                file_path,
                case_sensitive.as_bool().unwrap(),
                line_numbered.as_bool().unwrap(),
                colored.as_bool().unwrap(),
                Theme::Blue,
            )
        }
        false => {
            // Only works for single line / function
            // #[cfg(debug_assertions)]
            if cfg!(debug_assertions) {
                // env::args() will return an iterator over the arguments
                let args: Vec<String> = env::args().collect();
                dbg!(format!("Running grep binary located in: {}", &args[0]));
                // args[1..] will equal to the nth command line argument
            }

            Config::new(
                cli.query.unwrap(),
                cli.file_path.unwrap(),
                case_sensitive_env_var || cli.case_sensitive,
                // true if either env var is set or cli arg is set
                cli.line_number,
                cli.color,
                cli.theme.unwrap_or_else(|| Theme::Blue),
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
