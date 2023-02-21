use tiny_grep::{run, Config};

/// For integration testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_case_insensitive() {
        run(Config::new("you", "./tests/test.txt", false)).expect("Expect to run successfully");
    }

    #[test]
    fn run_case_sensitive() {
        run(Config::new("you", "./tests/test.txt", true)).expect("Expect to run successfully");
    }
}
