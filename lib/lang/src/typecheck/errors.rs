use crate::Type;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(String),
    TypeMismatch { fst: Type, snd: Type },
    EmptyBlock,
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
            Error::EmptyBlock => write!(f, "Cannot have block with no expressions"),
        }
    }
}

impl std::error::Error for Error {}
