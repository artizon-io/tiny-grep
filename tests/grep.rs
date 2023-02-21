use tiny_grep::{run, Config, Theme};

/// For integration testing
#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn run_single_file_case_insensitive() {
        run(Config::new(
            String::from("you"),
            Box::from(Path::new("./tests/poem.txt")),
            false,
            false,
            false,
            Theme::Blue,
        ))
        .expect("Expect to run successfully");
    }

    #[test]
    fn run_single_file_case_sensitive() {
        run(Config::new(
            String::from("you"),
            Box::from(Path::new("./tests/poem.txt")),
            true,
            false,
            false,
            Theme::Blue,
        ))
        .expect("Expect to run successfully");
    }

    #[test]
    fn run_directory() {
        run(Config::new(
            String::from("you"),
            Box::from(Path::new("./tests")),
            true,
            false,
            false,
            Theme::Blue,
        ))
        .expect("Expect to run successfully");
    }
}
