use super::{Suite, Test, TestResult, EXAMPLES_DIR};
use chapter1::{eval::interp_lint, parser::parse_l_int};
use std::path::PathBuf;

pub struct LIntSuite;

impl Suite for LIntSuite {
    fn examples_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_DIR).join("l_int")
    }

    fn name(&self) -> &str {
        "L-If"
    }

    fn run_test(&self, test: Test) -> TestResult {
        let mut input = test.contents;
        let parsed = match parse_l_int(&mut input) {
            Ok(prog) => prog,
            Err(err) => {
                return <Box<dyn std::error::Error> as Into<TestResult>>::into(Box::new(err))
            }
        };
        let result = interp_lint(parsed).to_string();
        if result == test.expected {
            TestResult::Success
        } else {
            TestResult::Failure(format!("{result} != {}", test.expected))
        }
    }
}
