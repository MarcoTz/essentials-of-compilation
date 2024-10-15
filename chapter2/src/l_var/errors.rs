use crate::Var;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VarNotFound { name: Var },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VarNotFound { name } => {
                write!(f, "Could not find variable {name} in environemnt.")
            }
        }
    }
}

impl std::error::Error for Error {}
