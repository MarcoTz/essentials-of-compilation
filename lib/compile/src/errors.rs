use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Parse(parser::Error),
    ReadFile(PathBuf),
    ParentNotFound(PathBuf),
    CreateDir(PathBuf),
    CreateFile(PathBuf),
    WriteFile(PathBuf),
    GetFileName(PathBuf),
    RunCommand(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "Error during parsing:\n{err}"),
            Error::ReadFile(path) => write!(f, "Could not read source file {path:?}"),
            Error::ParentNotFound(path) => write!(f, "Could not find parent of {path:?}"),
            Error::CreateDir(path) => write!(f, "Could not create dir {path:?}"),
            Error::CreateFile(path) => write!(f, "Could not create file {path:?}"),
            Error::WriteFile(path) => write!(f, "Could not write to file {path:?}"),
            Error::GetFileName(path) => write!(f, "Could not get file name of {path:?}"),
            Error::RunCommand(cmd) => write!(f, "Could not run command {cmd}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parse(err)
    }
}
