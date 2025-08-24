use parser::parse_program;
use std::env::{current_dir, set_current_dir};
use test_utils::load_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_current_dir(current_dir()?.parent().unwrap())?;
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
