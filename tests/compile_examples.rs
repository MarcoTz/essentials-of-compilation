use compile::Compiler;
use std::{
    env::{current_dir, set_current_dir},
    process::Command,
};
use test_utils::load_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_current_dir(current_dir()?.parent().unwrap())?;
    let examples = load_examples()?;
    for example in examples {
        let mut compiler = Compiler::new(false, example.path, None, None, None)?;
        print!("Compiling {}", example.name);
        compiler.compile()?;
        println!("... Ok");
        let mut check_cmd = Command::new(compiler.exe_out);
        let output = check_cmd.output()?;
        assert!(output.status.success());
        assert_eq!(output.stdout, example.expected.as_bytes())
    }
    Ok(())
}
