use parser::parse_program;
use test_utils::{Error, load_examples, set_working_dir};

fn main() -> Result<(), Error> {
    set_working_dir()?;
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
