use std::fmt;

#[derive(Debug)]
pub enum Error {
    MissingParens,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingParens => f.write_str("Parenthesis mismatch"),
        }
    }
}

impl std::error::Error for Error {}
