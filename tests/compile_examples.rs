use compile::Compiler;
use std::env::{current_dir, set_current_dir};
use test_utils::load_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_current_dir(current_dir()?.parent().unwrap())?;
    println!("{:?}", current_dir().unwrap());
    let examples = load_examples()?;
    for example in examples {
        let mut compiler = Compiler::new(false, example.path, None, None, None)?;
        print!("Compiling {}", example.name);
        compiler.compile()?;
        println!("... Ok");
    }
    Ok(())
}
