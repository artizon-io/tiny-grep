use tiny_grep::{run, Config};

/// For integration testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_case_insensitive() {
        run(Config::new("you", "./tests/test.txt", false, false, false)).expect("Expect to run successfully");
    }

    #[test]
    fn run_case_sensitive() {
        run(Config::new("you", "./tests/test.txt", true, false, false)).expect("Expect to run successfully");
    }
}
