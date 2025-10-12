use std::fmt;
use syntax::lang_mon::Expression;

#[derive(Debug)]
pub enum Error {
    EmptyProgram,
    BadCmp(Expression),
    BlockNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyProgram => f.write_str("Cannot explicate an empty Program"),
            Error::BadCmp(exp) => write!(f, "Cannot use expression {exp} as if condition"),
            Error::BlockNotFound(label) => write!(f, "Could not find block {label}"),
        }
    }
}

impl std::error::Error for Error {}
