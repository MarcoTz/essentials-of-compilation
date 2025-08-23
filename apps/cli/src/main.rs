use parser::parse_program;
use std::fs::{read_dir, read_to_string};

const EXAMPLES_DIR: &str = "examples";

fn main() {
    let dir_contents = read_dir(EXAMPLES_DIR).unwrap();
    for entry in dir_contents {
        let entry = entry.unwrap();
        let example = read_to_string(entry.path()).unwrap();
        println!("parsing {:?}", entry.path());
        let parsed = parse_program(&example).unwrap();
        println!("{parsed:?}");
    }
}
