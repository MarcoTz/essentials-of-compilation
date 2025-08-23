use super::{Suite, Test, TestResult};
use driver::{
    consts::{EXAMPLES_DIR, L_VAR_DIR},
    l_var::LVarDriver,
    Driver,
};
use std::path::PathBuf;

pub struct LVarSuite {
    driver: LVarDriver,
}

impl LVarSuite {
    pub fn new() -> LVarSuite {
        LVarSuite {
            driver: LVarDriver::new(false),
        }
    }
}

impl Suite for LVarSuite {
    fn examples_dir(&self) -> PathBuf {
        PathBuf::from(EXAMPLES_DIR).join(L_VAR_DIR)
    }

    fn name(&self) -> &str {
        "L-Var"
    }

    fn run_test(&self, test: Test) -> TestResult {
        let parse_res = self.driver.parse(&test.contents);
        let parsed = match parse_res {
            Err(err) => return TestResult::Failure(err.to_string()),
            Ok(p) => p,
        };
        let compiled = self.driver.compile(parsed, test.name);
        match compiled {
            Err(err) => TestResult::Failure(err.to_string()),
            Ok(_) => TestResult::Success,
        }
    }
}
