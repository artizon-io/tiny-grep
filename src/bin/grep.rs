// use std::env;
use clap::{Parser, ValueEnum};
use std::path::Path;
use std::process;

use rust_grep::Config;

/// A grep terminal utliity program written in Rust
#[derive(Parser, Debug)]
// Fill fields from Cargo.toml
#[command(author, version, about)]
#[command(next_line_help = true)]
struct Cli {
    /// The query string to search for
    // Make it a keyword argument
    // #[arg(long)]
    query: String,

    /// The file path to search in
    #[arg(value_parser = file_path_parser)]
    file_path: String,

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

fn main() {
    let cli = Cli::parse();

    let config = match cli.interactive {
        true => Config::new("", "", false),
        false => {
            // env::args() will return an iterator over the arguments
            // let args: Vec<String> = env::args().collect();
            // args[0] will equal to the relative path of the executable

            Config::new(&cli.query, &cli.file_path, cli.case_sensitive)
            // maybe_config.unwrap_or_else(|err| {
            //     eprintln!("Problem parsing arguments: {err}");
            //     process::exit(1);
            // })
        }
    };

    // dbg!() macro will move its argument, so must be placed after Config::build(&args)
    // dbg!(args);

    // println!("Searching for '{}'", config.query);
    // println!("In file '{}'", config.file_path);

    // run() will take ownership of config
    if let Err(e) = rust_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
