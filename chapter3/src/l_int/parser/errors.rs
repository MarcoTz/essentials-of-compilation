use super::tokens::Token;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotADigit { input: String },
    NotAnOp { input: String },
    NotAToken { input: String },
    UnexpectedEndOfInput,
    UnexpectedToken { token: Token },
    NotAnExpr { token: Token },
    BracketMismatch,
    RemainingInput,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotADigit { input } => write!(f, "Not a digit: {input}"),
            Error::NotAnOp { input } => write!(f, "Not an operation: {input}"),
            Error::NotAToken { input } => write!(f, "Not a token: {input}"),
            Error::UnexpectedEndOfInput => f.write_str("Unexpected end of input"),
            Error::UnexpectedToken { token } => write!(f, "Unexpected token {token}"),
            Error::NotAnExpr { token } => write!(f, "Not an expression: {token}"),
            Error::BracketMismatch => f.write_str("Mismatched brackets"),
            Error::RemainingInput => f.write_str("Input remaining after end of parsing"),
        }
    }
}

impl std::error::Error for Error {}
