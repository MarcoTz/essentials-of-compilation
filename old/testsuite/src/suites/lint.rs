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
        let parse_res = self.driver.parse(&test.contents);
        let parsed = match parse_res {
            Err(err) => return TestResult::Failure(err.to_string()),
            Ok(p) => p,
        };
        let comp_res = self.driver.compile(parsed, test.name);
        let compiled = match comp_res {
            Err(err) => return TestResult::Failure(err.to_string()),
            Ok(c) => c,
        };
        let eval_res = self.driver.evaluate(compiled);
        let evaled = match eval_res {
            Err(err) => return TestResult::Failure(err.to_string()),
            Ok(e) => e,
        };

        if evaled == test.expected {
            TestResult::Success
        } else {
            TestResult::Failure(format!("{evaled} != {}", test.expected))
        }
    }
}
