use driver::Compiler;
use std::process::Command;
use test_utils::{Error, load_examples, set_working_dir};

fn main() -> Result<(), Error> {
    set_working_dir()?;
    let examples = load_examples()?;
    for example in examples {
        let compiler = Compiler::new(false, example.path, None, None, None)?;
        let exe_path = compiler.paths.exe_out.clone();
        println!("Compiling {}", example.name);
        compiler.run()?;
        println!("\t...Ok");
        println!("Checking output of {}", example.name);
        let mut check_cmd = Command::new(&exe_path);
        let output = check_cmd
            .output()
            .map_err(|_| Error::ReadCommandOut(format!("{:?}", exe_path)))?;
        if !output.status.success() {
            return Err(Error::RunCommand(format!("{:?}", exe_path)));
        }
        let result = str::from_utf8(&output.stdout)
            .map_err(|_| Error::ReadCommandOut(format!("{:?}", exe_path)))?;
        if result != example.expected {
            return Err(Error::unexpected(
                format!("{:?}", exe_path),
                &result,
                &example.expected,
            ));
        }
        println!("\t...Ok");
    }
    Ok(())
}
