use crate::l_if::errors::Error as LErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    LIf(LErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LIf(err) => write!(f, "Error in l_if: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<LErr> for Error {
    fn from(err: LErr) -> Error {
        Error::LIf(err)
    }
}
