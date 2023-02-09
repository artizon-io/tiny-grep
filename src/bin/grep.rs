use std::env;
use std::process;

use rust_grep::Config;

fn main() {
    // env::args() will return an iterator over the arguments
    let args: Vec<String> = env::args().collect();
    // args[1] will equal to the relative path of the executable
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // dbg!() macro will move its argument, so must be placed after Config::build(&args)
    dbg!(args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    // run() will take ownership of config
    if let Err(e) = rust_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
