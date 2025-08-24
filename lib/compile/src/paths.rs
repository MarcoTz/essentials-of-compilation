use crate::Error;
use std::{
    ffi::OsStr,
    fs::{create_dir_all, remove_file},
    path::PathBuf,
};

const DEFAULT_ASM_OUT: &str = "target/asm";
const ASM_EXT: &str = "asm";
const DEFAULT_OBJECT_OUT: &str = "target/object";
const OBJECT_EXT: &str = "o";
const DEFAULT_EXE_OUT: &str = "target/exe";
const EXE_EXT: &str = "";
pub const C_RUNTIME: &str = "include/runtime.c";

fn get_out(
    cli_arg: Option<PathBuf>,
    prog_name: &OsStr,
    default_out: &str,
    default_ext: &str,
) -> Result<PathBuf, Error> {
    let out_path = match cli_arg {
        Some(path) => path,
        None => {
            let mut out_path = PathBuf::from(default_out).join(prog_name);
            out_path.set_extension(default_ext);
            out_path
        }
    };
    if out_path.exists() {
        remove_file(&out_path).map_err(|_| Error::RemoveFile(out_path.clone()))?;
    }
    let parent = out_path
        .parent()
        .ok_or(Error::ParentNotFound(out_path.clone()))?;
    if !parent.exists() {
        create_dir_all(parent).map_err(|_| Error::CreateDir(parent.to_path_buf()))?;
    }

    Ok(out_path)
}

pub fn get_asm_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> Result<PathBuf, Error> {
    get_out(cli_arg, prog_name, DEFAULT_ASM_OUT, ASM_EXT)
}

pub fn get_object_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> Result<PathBuf, Error> {
    get_out(cli_arg, prog_name, DEFAULT_OBJECT_OUT, OBJECT_EXT)
}

pub fn get_exe_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> Result<PathBuf, Error> {
    get_out(cli_arg, prog_name, DEFAULT_EXE_OUT, EXE_EXT)
}

pub fn get_runtime_object_out() -> PathBuf {
    PathBuf::from(DEFAULT_OBJECT_OUT).join("runtime.o")
}
