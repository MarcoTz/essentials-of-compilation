use std::{convert::Infallible, fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    Infallible,
    Parse(parser::Error),
    AssignHomes(register_allocation::Error),
    Typecheck(surface::typecheck::Error),
    ExplicateControl(monadic2lang_c::Error),
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

impl From<register_allocation::Error> for Error {
    fn from(err: register_allocation::Error) -> Error {
        Error::AssignHomes(err)
    }
}

impl From<surface::typecheck::Error> for Error {
    fn from(err: surface::typecheck::Error) -> Error {
        Error::Typecheck(err)
    }
}

impl From<monadic2lang_c::Error> for Error {
    fn from(err: monadic2lang_c::Error) -> Error {
        Error::ExplicateControl(err)
    }
}
