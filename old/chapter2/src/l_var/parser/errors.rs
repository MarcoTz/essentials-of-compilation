use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOI,
    RemainingInput(String),
    ParenMismatch,
    UnexpectedCharacter(char),
    NoInt,
    NoName,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedEOI => f.write_str("Unexpected end of input."),
            Error::RemainingInput(inp) => write!(f, "Remaining Input after parsing: {inp}"),
            Error::ParenMismatch => f.write_str("Mismatched parentheses"),
            Error::UnexpectedCharacter(c) => write!(f, "Unecpected character {c}"),
            Error::NoInt => f.write_str("Could not parse integer"),
            Error::NoName => f.write_str("Could not parse identifier"),
        }
    }
}

impl std::error::Error for Error {}
