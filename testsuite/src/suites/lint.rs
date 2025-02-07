use super::{Suite, Test, TestResult};
use driver::{
    consts::{EXAMPLES_DIR, L_INT_DIR},
    l_int::LIntDriver,
    Driver,
};
use std::path::PathBuf;

pub struct LIntSuite {
    driver: LIntDriver,
}

impl LIntSuite {
    pub fn new() -> LIntSuite {
        LIntSuite { driver: LIntDriver }
    }
}

impl Suite for LIntSuite {
    fn examples_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_DIR).join(L_INT_DIR)
    }

    fn name(&self) -> &str {
        "L-Int"
    }

    fn run_test(&self, test: Test) -> TestResult {
        let result = match self.driver.compile_and_eval(&test.contents, false) {
            Ok(res) => res,
            Err(err) => return TestResult::Failure(err.to_string()),
        };
        if result == test.expected {
            TestResult::Success
        } else {
            TestResult::Failure(format!("{result} != {}", test.expected))
        }
    }
}
