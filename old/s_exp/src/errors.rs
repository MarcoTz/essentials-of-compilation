use std::fmt;

#[derive(Debug)]
pub enum Error {
    MissingParens,
    UnexpectedSymbol(String),
    ArgumentMismatch(String),
    UnexpectedEOI,
    RemainingInput(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingParens => f.write_str("Parenthesis mismatch"),
            Error::UnexpectedSymbol(s) => write!(f, "Unexpected symbol {s}"),
            Error::ArgumentMismatch(s) => write!(f, "Argument Mismatch {s}"),
            Error::UnexpectedEOI => f.write_str("Unexpected End of Input"),
            Error::RemainingInput(inp) => write!(f, "Remaining Input {inp}"),
        }
    }
}

impl std::error::Error for Error {}
