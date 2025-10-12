use std::{convert::Infallible, fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Infallible,
    Parse(parser::Error),
    AssignHomes(assign_homes::Error),
    Typecheck(typecheck::Error),
    ExplicateControl(explicate_control::Error),
    ReadFile(PathBuf),
    ParentNotFound(PathBuf),
    CreateDir(PathBuf),
    CreateFile(PathBuf),
    WriteFile(PathBuf),
    GetFileName(PathBuf),
    RunCommand(String),
    RemoveFile(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Infallible => f.write_str(""),
            Error::Parse(err) => write!(f, "Error during parsing:\n{err}"),
            Error::AssignHomes(err) => write!(f, "Error during assign homes:\n{err}"),
            Error::Typecheck(err) => write!(f, "Error during typechecking:\n{err}"),
            Error::ExplicateControl(err) => write!(f, "Error in explicate control:\n{err}"),
            Error::ReadFile(path) => write!(f, "Could not read source file {path:?}"),
            Error::ParentNotFound(path) => write!(f, "Could not find parent of {path:?}"),
            Error::CreateDir(path) => write!(f, "Could not create dir {path:?}"),
            Error::CreateFile(path) => write!(f, "Could not create file {path:?}"),
            Error::WriteFile(path) => write!(f, "Could not write to file {path:?}"),
            Error::GetFileName(path) => write!(f, "Could not get file name of {path:?}"),
            Error::RunCommand(cmd) => write!(f, "Could not run command {cmd}"),
            Error::RemoveFile(path) => write!(f, "Could not remove file {path:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Error {
        Error::Infallible
    }
}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parse(err)
    }
}

impl From<assign_homes::Error> for Error {
    fn from(err: assign_homes::Error) -> Error {
        Error::AssignHomes(err)
    }
}

impl From<typecheck::Error> for Error {
    fn from(err: typecheck::Error) -> Error {
        Error::Typecheck(err)
    }
}

impl From<explicate_control::Error> for Error {
    fn from(err: explicate_control::Error) -> Error {
        Error::ExplicateControl(err)
    }
}
