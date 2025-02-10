use crate::x86_int::Program;
use std::{
    fmt,
    fs::{create_dir_all, write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum Error {
    OutPath(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutPath(path) => write!(f, "Could not write to out path {path:?}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn assemble(prog: Program, file_name: String, out_dir: PathBuf) -> Result<(), Error> {
    create_dir_all(&out_dir).map_err(|_| Error::OutPath(out_dir.clone()))?;
    let mut out_file = out_dir.join(file_name);
    out_file.set_extension("asm");
    write(&out_file, prog.to_string()).map_err(|_| Error::OutPath(out_file))?;
    Ok(())
}
