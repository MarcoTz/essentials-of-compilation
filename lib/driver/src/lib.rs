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
        let parse = Parse {
            source: self.source.clone(),
        };
        let check_types = parse.run_debug(&self.paths, self.debug)?;
        let uniquify = check_types.run_debug(&self.paths, self.debug)?;
        let rco = uniquify.run_debug(&self.paths, self.debug)?;
        let explicate = rco.run_debug(&self.paths, self.debug)?;
        let select_instrs = explicate.run_debug(&self.paths, self.debug)?;
        let uncover = select_instrs.run_debug(&self.paths, self.debug)?;
        let build_interference = uncover.run_debug(&self.paths, self.debug)?;
        let color_graph = build_interference.run_debug(&self.paths, self.debug)?;
        let assign_hoems = color_graph.run_debug(&self.paths, self.debug)?;
        let patch_instrs = assign_hoems.run_debug(&self.paths, self.debug)?;
        let generate_prelude_conclusion = patch_instrs.run_debug(&self.paths, self.debug)?;
        let assemble = generate_prelude_conclusion.run_debug(&self.paths, self.debug)?;
        let link = assemble.run_debug(&self.paths, self.debug)?;
        link.run_debug(&self.paths, self.debug)?;
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
