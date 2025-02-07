use super::{Suite, Test, TestResult};
use chapter2::compile::compile;
use driver::consts::{EXAMPLES_DIR, L_VAR_DIR};
use std::path::PathBuf;

pub struct LVarSuite;

impl Suite for LVarSuite {
    fn examples_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_DIR).join(L_VAR_DIR)
    }

    fn name(&self) -> &str {
        "L-Var"
    }

    fn run_test(&self, test: Test) -> TestResult {
        let prog = match compile(&test.contents) {
            Ok(prog) => prog,
            Err(err) => return TestResult::Failure(err.to_string()),
        };
        if prog.to_string() == test.expected {
            TestResult::Success
        } else {
            TestResult::Failure(format!("{prog} != {}", test.expected))
        }
    }
}
