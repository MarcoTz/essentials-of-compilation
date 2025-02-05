use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

const EXAMPLES_DIR: &str = "examples";

mod lif;
mod lint;
mod lvar;

pub fn run_tests() {
    println!("Running L-Int Tests");
    lint::LIntSuite.run_tests();
}

struct Test {
    name: String,
    contents: String,
    expected: String,
}

enum TestResult {
    Success,
    Failure(String),
}

impl From<Box<dyn std::error::Error>> for TestResult {
    fn from(err: Box<dyn std::error::Error>) -> TestResult {
        TestResult::Failure(err.to_string())
    }
}

trait Suite {
    type ExampleResult: ToString;

    fn examples_dir(&self) -> PathBuf;
    fn name(&self) -> &str;

    fn run_test(&self, test: Test) -> TestResult;

    fn run_tests(&self) {
        let tests = load_tests(self.examples_dir());
        println!("Running {} tests for {}", tests.len(), self.name());
        println!("");

        for test in tests {
            let name = test.name.clone();
            match self.run_test(test) {
                TestResult::Success => println!("{}...... ok", name),
                TestResult::Failure(err) => {
                    println!("{}....... error", name);
                    println!("\t failed with error: {}", err.to_string());
                }
            }
        }
    }
}

fn load_tests(dir: PathBuf) -> Vec<Test> {
    let mut tests = vec![];

    println!("loading examples from {dir:?}");
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.extension().unwrap() == "res" {
            continue;
        }

        let mut expected = file_path.clone();
        expected.set_extension("res");
        let expected_contents = read_to_string(expected).unwrap().trim().to_owned();

        let contents = read_to_string(&file_path).unwrap();

        tests.push(Test {
            name: file_path.file_stem().unwrap().to_str().unwrap().to_owned(),
            contents,
            expected: expected_contents,
        });
    }
    tests
}
