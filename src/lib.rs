use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
    // Convention is new() never fails
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        assert!(args.len() >= 3, "Less than 2 arguments received");

        Config { query, file_path }
    }

    // Let error value be a string literal that have 'static lifetime
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Less than 2 arguments received");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}