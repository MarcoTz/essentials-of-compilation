use std::{fs::read_to_string, path::PathBuf, process::Command};

mod errors;
mod passes;
mod paths;
mod pipeline;
use pipeline::Pipeline;

pub use errors::Error;
use paths::{C_RUNTIME, get_asm_out, get_exe_out, get_object_out, get_runtime_object_out};

pub struct CompilerPaths {
    asm_out: PathBuf,
    object_out: PathBuf,
    pub exe_out: PathBuf,
}

pub struct Compiler {
    debug: bool,
    pub paths: CompilerPaths,
    current_step: Option<Pipeline>,
}

impl Compiler {
    pub fn new(
        debug: bool,
        source: PathBuf,
        asm_out: Option<PathBuf>,
        object_out: Option<PathBuf>,
        exe_out: Option<PathBuf>,
    ) -> Result<Compiler, Error> {
        let prog_name = source
            .file_stem()
            .ok_or(Error::GetFileName(source.clone()))?;
        let asm_out = get_asm_out(asm_out, prog_name)?;
        let object_out = get_object_out(object_out, prog_name)?;
        let exe_out = get_exe_out(exe_out, prog_name)?;
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;

        Ok(Compiler {
            debug,
            paths: CompilerPaths {
                asm_out,
                object_out,
                exe_out,
            },
            current_step: Some(Pipeline::Parse(source_contents)),
        })
    }

    pub fn run(mut self) -> Result<(), Error> {
        while let Some(step) = self.current_step {
            self.current_step = step.step(&self.paths, self.debug)?;
        }
        Ok(())
    }
}

pub fn assemble_runtime() -> Result<(), Error> {
    let runtime_in = PathBuf::from(C_RUNTIME);
    let runtime_out = get_runtime_object_out();
    if runtime_out.exists() {
        return Ok(());
    }

    let mut gcc_cmd = Command::new("gcc");
    gcc_cmd
        .arg("-c")
        .arg(&runtime_in)
        .arg("-o")
        .arg(&runtime_out);
    let res = gcc_cmd
        .status()
        .map_err(|_| Error::RunCommand("gcc -c".to_owned()))?;
    if !res.success() {
        return Err(Error::RunCommand("gcc -c".to_owned()))?;
    }
    Ok(())
}
