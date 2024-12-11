use super::keywords::Keyword;
use nom::{
    error::{ErrorKind, ParseError},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOI,
    NotAKeyword(String),
    VarIsKeyword(Keyword),
    Multiple(Vec<Error>),
    Nom(String),
}

pub type ParseRes<'a, T> = IResult<&'a str, T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedEOI => f.write_str("Unexpected end of input."),
            Error::NotAKeyword(s) => write!(
                f,
                "Got {s}, expected one of {}",
                Keyword::all()
                    .iter()
                    .map(|kw| kw.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Error::VarIsKeyword(kw) => write!(f, "Cannot use {kw} as variable name"),
            Error::Multiple(errs) => write!(
                f,
                "Parser encountered multiple errors:\n{}",
                errs.iter()
                    .map(|err| format!("\t{err}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Error::Nom(msg) => write!(f, "Parser encountered an error: {msg}."),
        }
    }
}

impl std::error::Error for Error {}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Error::Nom(kind.description().to_owned())
    }

    fn append(_: I, kind: ErrorKind, other: Self) -> Self {
        let nom_err = Error::Nom(kind.description().to_owned());
        match other {
            Error::Multiple(errs) => {
                let mut new_errs = errs;
                new_errs.push(nom_err);
                Error::Multiple(new_errs)
            }
            err => Error::Multiple(vec![err, nom_err]),
        }
    }
}
