use parser::parse_program;
use std::fs::{read_dir, read_to_string};

const EXAMPLES_DIR: &str = "../examples";

struct Example {
    name: String,
    source: String,
}

fn load_examples() -> Result<Vec<Example>, Box<dyn std::error::Error>> {
    let dir_contents = read_dir(EXAMPLES_DIR)?;
    let mut examples = vec![];
    for entry in dir_contents {
        let entry = entry?;
        let example_contents = read_to_string(entry.path())?;
        let example_name = entry
            .path()
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not get file name as string")
            .to_owned();
        examples.push(Example {
            name: example_name,
            source: example_contents,
        });
    }
    Ok(examples)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading examples from {EXAMPLES_DIR}\n");
    let examples = load_examples()?;
    for example in examples {
        print!("Parsing {}", example.name);
        let parsed = parse_program(&example.source)?;
        println!(".... Ok");
        print!("Reparsing {}", example.name);
        let parsed_str = parsed.to_string();
        let _ = parse_program(&parsed_str)?;
        println!(".... Ok");
    }
    Ok(())
}
