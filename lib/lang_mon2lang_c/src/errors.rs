use lang_c::Expression;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    EmptyProgram,
    BadCmp(Expression),
    BlockExists(String),
    BlockNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyProgram => f.write_str("Cannot explicate an empty Program"),
            Error::BadCmp(exp) => write!(f, "Cannot use expression {exp} as if condition"),
            Error::BlockExists(label) => write!(f, "Block {label} already exists"),
            Error::BlockNotFound(label) => write!(f, "Could not find block {label}"),
        }
    }
}

impl std::error::Error for Error {}
