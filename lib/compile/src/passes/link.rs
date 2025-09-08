use super::{Pass, assemble::WrappedPath};
use crate::{CompilerPaths, Error, assemble_runtime, paths::get_runtime_object_out};
use std::process::Command;

pub struct Link;

impl Pass for Link {
    type Input = ();
    type Output = WrappedPath;
    type Error = Error;

    fn description() -> &'static str {
        "Link"
    }

    fn run(_: Self::Input, compiler: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        if !compiler.object_out.exists() {
            return Err(Error::ReadFile(compiler.object_out.clone()));
        }
        let runtime_out = get_runtime_object_out();
        if !runtime_out.exists() {
            assemble_runtime()?;
        }

        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd
            .arg(&compiler.object_out)
            .arg(&runtime_out)
            .arg("-o")
            .arg(&compiler.exe_out);
        let res = gcc_cmd
            .status()
            .map_err(|_| Error::RunCommand("gcc".to_owned()))?;
        if !res.success() {
            return Err(Error::RunCommand("gcc".to_owned()));
        }
        Ok(WrappedPath(compiler.exe_out.clone()))
    }
}
