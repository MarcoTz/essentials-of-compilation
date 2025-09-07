use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Parse(parser::Error),
    Compiler(compile::Error),
    ReadDir(String),
    FileAccess(PathBuf),
    GetFileName(PathBuf),
    SetWorkingDir(PathBuf),
    ReadCommandOut(String),
    RunCommand(String),
    UnexpectedOutput {
        cmd: String,
        result: String,
        expected: String,
    },
}

impl Error {
    pub fn unexpected(cmd: String, result: &str, expected: &str) -> Error {
        Error::UnexpectedOutput {
            cmd,
            result: result.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "Error during parsing:\n{err}"),
            Error::Compiler(err) => write!(f, "Error during compiling:\n{err}"),
            Error::ReadDir(path) => write!(f, "Could not read directory {path}"),
            Error::FileAccess(path) => write!(f, "Could not read file {path:?}"),
            Error::GetFileName(path) => write!(f, "Could not get file name of {path:?}"),
            Error::SetWorkingDir(path) => write!(f, "Could not set working directory to {path:?}"),
            Error::ReadCommandOut(cmd) => write!(f, "Could not read command output of {cmd}"),
            Error::RunCommand(cmd) => write!(f, "Command {cmd} exited with non-zero exit status"),
            Error::UnexpectedOutput {
                cmd,
                result,
                expected,
            } => write!(
                f,
                "Unexpected output of {cmd}:\nresult:{result}\nexpected:{expected}"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parse(err)
    }
}

impl From<compile::Error> for Error {
    fn from(err: compile::Error) -> Error {
        Error::Compiler(err)
    }
}
