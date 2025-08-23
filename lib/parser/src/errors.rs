use crate::Rule;
use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    Pest(String),
    MissingInput { rule: Rule },
    RemainingInput { rule: Rule },
    UnexpectedRule { rule: Rule, expected: String },
    ParseInt { reason: String },
    UnknownSymbol { sym: String },
}

impl Error {
    pub fn missing(r: Rule) -> Error {
        Error::MissingInput { rule: r }
    }

    pub fn remaining(r: Rule) -> Error {
        Error::RemainingInput { rule: r }
    }

    pub fn unexpected(r: Rule, expected: &str) -> Error {
        Error::UnexpectedRule {
            rule: r,
            expected: expected.to_owned(),
        }
    }

    pub fn unknown(sym: &str) -> Error {
        Error::UnknownSymbol {
            sym: sym.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(msg) => write!(f, "Error in Pest:\n{msg}"),
            Error::MissingInput { rule } => write!(f, "Missing input {rule:?}"),
            Error::RemainingInput { rule } => write!(f, "Remaining input {rule:?}"),
            Error::UnexpectedRule { rule, expected } => {
                write!(f, "Unexpected rule {rule:?}, expected {expected}")
            }
            Error::ParseInt { reason } => write!(f, "Could not parse integer ({reason})"),
            Error::UnknownSymbol { sym } => write!(f, "Unknown symbol {sym}"),
        }
    }
}

impl std::error::Error for Error {}

impl<T> From<pest::error::Error<T>> for Error
where
    T: fmt::Debug,
{
    fn from(err: pest::error::Error<T>) -> Error {
        Error::Pest(format!("{err:?}"))
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt {
            reason: format!("{:?}", err.kind()),
        }
    }
}
