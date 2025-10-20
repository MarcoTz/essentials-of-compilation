use std::{fs::read_to_string, path::PathBuf, process::Command};

mod errors;
mod passes;
mod paths;
use passes::{Parse, Pass};

pub use errors::Error;
use paths::{C_RUNTIME, get_asm_out, get_exe_out, get_object_out, get_runtime_object_out};

pub struct CompilerPaths {
    asm_out: PathBuf,
    object_out: PathBuf,
    pub exe_out: PathBuf,
}

pub struct Driver {
    debug: bool,
    pub paths: CompilerPaths,
    source: String,
}

impl Driver {
    pub fn new(
        debug: bool,
        source: PathBuf,
        asm_out: Option<PathBuf>,
        object_out: Option<PathBuf>,
        exe_out: Option<PathBuf>,
    ) -> Result<Driver, Error> {
        let prog_name = source
            .file_stem()
            .ok_or(Error::GetFileName(source.clone()))?;
        let asm_out = get_asm_out(asm_out, prog_name)?;
        let object_out = get_object_out(object_out, prog_name)?;
        let exe_out = get_exe_out(exe_out, prog_name)?;
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;

        Ok(Driver {
            debug,
            paths: CompilerPaths {
                asm_out,
                object_out,
                exe_out,
            },
            source: source_contents,
        })
    }

    pub fn run(self) -> Result<(), Error> {
        let parsed = Parse {
            source: self.source.clone(),
        }
        .run_debug(&self.paths, self.debug)?;
        let checked = parsed.run_debug(&self.paths, self.debug)?;
        let uniquified = checked.run_debug(&self.paths, self.debug)?;
        let removed = uniquified.run_debug(&self.paths, self.debug)?;
        let explicated = removed.run_debug(&self.paths, self.debug)?;
        let selected = explicated.run_debug(&self.paths, self.debug)?;
        let flow_built = selected.run_debug(&self.paths, self.debug)?;
        let uncovered = flow_built.run_debug(&self.paths, self.debug)?;
        let interference_built = uncovered.run_debug(&self.paths, self.debug)?;
        let colored = interference_built.run_debug(&self.paths, self.debug)?;
        let assigned = colored.run_debug(&self.paths, self.debug)?;
        let patched = assigned.run_debug(&self.paths, self.debug)?;
        let finalized = patched.run_debug(&self.paths, self.debug)?;
        let assembled = finalized.run_debug(&self.paths, self.debug)?;
        assembled.run_debug(&self.paths, self.debug)?;
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
        Err(Error::RunCommand("gcc -c".to_owned()))?;
    }
    Ok(())
}
