use compile::Compiler;
use std::process::Command;
use test_utils::{Error, load_examples, set_working_dir};

fn main() -> Result<(), Error> {
    set_working_dir()?;
    let examples = load_examples()?;
    for example in examples {
        let mut compiler = Compiler::new(false, example.path, None, None, None)?;
        print!("Compiling {}", example.name);
        compiler.compile()?;
        println!("... Ok");
        print!("Checking output of {}", example.name);
        let mut check_cmd = Command::new(&compiler.exe_out);
        let output = check_cmd
            .output()
            .map_err(|_| Error::ReadCommandOut(format!("{:?}", compiler.exe_out)))?;
        if !output.status.success() {
            return Err(Error::RunCommand(format!("{:?}", compiler.exe_out)));
        }
        let result = str::from_utf8(&output.stdout)
            .map_err(|_| Error::ReadCommandOut(format!("{:?}", compiler.exe_out)))?;
        if result != example.expected {
            return Err(Error::unexpected(
                format!("{:?}", compiler.exe_out),
                &result,
                &example.expected,
            ));
        }
        println!("... Ok");
    }
    Ok(())
}
