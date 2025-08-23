use crate::Var;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    RegistersFull,
    VariableNotFound(Var),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RegistersFull => {
                f.write_str("Cannot assign all registers, all registers already occupied.")
            }
            Error::VariableNotFound(var) => write!(f, "Register for variable {var} not found"),
        }
    }
}

impl std::error::Error for Error {}
