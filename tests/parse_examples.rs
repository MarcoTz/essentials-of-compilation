use parser::parse_program;
use test_utils::{Error, load_examples, set_working_dir};

fn main() -> Result<(), Error> {
    set_working_dir()?;
    let examples = load_examples()?;
    for example in examples {
        println!("Parsing {}", example.name);
        let parsed = parse_program(&example.source)?;
        println!("\t...Ok");
        println!("Reparsing {}", example.name);
        let parsed_str = parsed.to_string();
        parse_program(&parsed_str)?;
        println!("\t...Ok");
    }
    Ok(())
}
