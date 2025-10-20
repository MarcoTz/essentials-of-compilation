use super::{GeneratePreludeConclusion, Link, Pass};
use crate::{CompilerPaths, Error};
use asm::Program;
use std::{
    fs::{File, create_dir_all},
    io::Write,
    process::Command,
};

pub struct Assemble {
    pub prog: Program,
}

impl Pass for Assemble {
    type Next = Link;
    type Prev = GeneratePreludeConclusion;
    type Error = Error;

    fn description() -> &'static str {
        "Assembled"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, compiler: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let asm_dir = compiler
            .asm_out
            .parent()
            .ok_or(Error::ParentNotFound(compiler.asm_out.clone()))?;
        create_dir_all(asm_dir).map_err(|_| Error::CreateDir(asm_dir.to_path_buf()))?;
        let mut asm_file = File::create(&compiler.asm_out)
            .map_err(|_| Error::CreateFile(compiler.asm_out.clone()))?;
        asm_file
            .write_all(self.prog.to_string().as_bytes())
            .map_err(|_| Error::WriteFile(compiler.asm_out.clone()))?;

        let object_dir = compiler
            .object_out
            .parent()
            .ok_or(Error::ParentNotFound(compiler.object_out.clone()))?;
        create_dir_all(object_dir).map_err(|_| Error::CreateDir(object_dir.to_path_buf()))?;

        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd
            .arg("-c")
            .arg("-x")
            .arg("assembler")
            .arg(&compiler.asm_out)
            .arg("-o")
            .arg(&compiler.object_out);
        let res = gcc_cmd
            .status()
            .map_err(|_| Error::RunCommand("gcc -c".to_owned()))?;
        if !res.success() {
            return Err(Error::RunCommand("gcc -c".to_owned()));
        }
        Ok(Link)
    }
}
