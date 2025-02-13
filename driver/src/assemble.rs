use std::{fmt, fs::write, path::PathBuf, process::Command};

#[derive(Debug)]
pub enum Error {
    OutPath(PathBuf),
    Asm(String),
    Gcc(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutPath(path) => write!(f, "Could not write to out path {path:?}"),
            Error::Asm(msg) => write!(f, "as exited with error: {msg}"),
            Error::Gcc(msg) => write!(f, "gcc exited with error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn write_asm(source: String, file_name: String, out_dir: &PathBuf) -> Result<PathBuf, Error> {
    let mut out_file = out_dir.join(file_name);
    out_file.set_extension("asm");
    write(&out_file, source).map_err(|_| Error::OutPath(out_file.clone()))?;
    Ok(out_file)
}

pub fn assemble(asm_file: &PathBuf, obj_dir: &PathBuf) -> Result<PathBuf, Error> {
    let stem = asm_file
        .file_stem()
        .ok_or(Error::OutPath(asm_file.clone()))?
        .to_str()
        .ok_or(Error::OutPath(asm_file.clone()))?;
    let mut out_path = obj_dir.join(stem);
    out_path.set_extension("o");
    Command::new("as")
        .arg(asm_file)
        .arg("-o")
        .arg(&out_path)
        .status()
        .map_err(|err| Error::Asm(err.to_string()))?;
    Ok(out_path)
}

pub fn link_obj(obj_file: &PathBuf, exe_dir: &PathBuf, lib_c: &PathBuf) -> Result<PathBuf, Error> {
    let stem = obj_file
        .file_stem()
        .ok_or(Error::OutPath(obj_file.clone()))?
        .to_str()
        .ok_or(Error::OutPath(obj_file.clone()))?;
    let mut out_path = exe_dir.join(stem);
    out_path.set_extension("");
    Command::new("gcc")
        .arg("-o")
        .arg(&out_path)
        .arg(&obj_file)
        .arg(&lib_c)
        .status()
        .map_err(|err| Error::Gcc(err.to_string()))?;
    Ok(out_path)
}
