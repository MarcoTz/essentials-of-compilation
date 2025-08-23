use super::keywords::Keyword;
use nom::{
    error::{ErrorKind, ParseError},
    IResult,
};
use std::fmt;

pub type ParseRes<'a, T> = IResult<&'a str, T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingInput,
    RemainingInput(String),
    UsedKeyword(Keyword),
    UnknownKeyword(String),
    Nom(ErrorKind),
    Cons(Box<Error>, Box<Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Nom(knd) => write!(f, "{}", knd.description()),
            Error::Cons(err1, err2) => write!(f, "{err1}\n{err2}"),
            Error::RemainingInput(inp) => write!(f, "Remaining Input: {inp}"),
            Error::MissingInput => f.write_str("Missing input"),
            Error::UsedKeyword(kw) => write!(f, "Variable cannot be keyword {kw}"),
            Error::UnknownKeyword(input) => write!(f, "{input} is not a keyword"),
        }
    }
}

impl std::error::Error for Error {}

impl From<nom::Err<Error>> for Error {
    fn from(err: nom::Err<Error>) -> Error {
        match err {
            nom::Err::Incomplete(_) => Error::MissingInput,
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Error::Nom(kind)
    }

    fn append(i: I, kind: ErrorKind, other: Self) -> Self {
        Error::Cons(Box::new(other), Box::new(Self::from_error_kind(i, kind)))
    }
}
