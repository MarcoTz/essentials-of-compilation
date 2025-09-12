use std::fmt;
use syntax::lang::Type;

#[derive(Debug)]
pub enum Error {
    FreeVar(String),
    TypeMismatch { fst: Type, snd: Type },
}

impl Error {
    pub fn mismatch(fst: Type, snd: Type) -> Error {
        Error::TypeMismatch { fst, snd }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Free variable {v}"),
            Error::TypeMismatch { fst, snd } => write!(f, "Type mismatch {fst} != {snd}"),
        }
    }
}

impl std::error::Error for Error {}
