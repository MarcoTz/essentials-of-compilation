use std::{ffi::OsStr, path::PathBuf};

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
) -> PathBuf {
    match cli_arg {
        Some(path) => path,
        None => {
            let mut out_path = PathBuf::from(default_out).join(prog_name);
            out_path.set_extension(default_ext);
            out_path
        }
    }
}

pub fn get_asm_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> PathBuf {
    get_out(cli_arg, prog_name, DEFAULT_ASM_OUT, ASM_EXT)
}

pub fn get_object_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> PathBuf {
    get_out(cli_arg, prog_name, DEFAULT_OBJECT_OUT, OBJECT_EXT)
}

pub fn get_exe_out(cli_arg: Option<PathBuf>, prog_name: &OsStr) -> PathBuf {
    get_out(cli_arg, prog_name, DEFAULT_EXE_OUT, EXE_EXT)
}

pub fn get_runtime_object_out() -> PathBuf {
    PathBuf::from(DEFAULT_OBJECT_OUT).join("runtime.o")
}
