use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Parse(parser::Error),
    ReadFile(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "Error during parsing:\n{err}"),
            Error::ReadFile(path) => write!(f, "Could not read source file {path:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parse(err)
    }
}
