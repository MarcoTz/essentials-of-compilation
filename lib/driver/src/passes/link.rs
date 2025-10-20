use super::{Assemble, Done, Pass};
use crate::{CompilerPaths, Error, assemble_runtime, paths::get_runtime_object_out};
use std::process::Command;

pub struct Link;

impl Pass for Link {
    type Next = Done;
    type Prev = Assemble;
    type Error = Error;

    fn description() -> &'static str {
        "Link"
    }

    fn show_input(&self) -> String {
        "".to_owned()
    }

    fn run(self, compiler: &CompilerPaths) -> Result<Self::Next, Self::Error> {
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
        Ok(Done)
    }
}
