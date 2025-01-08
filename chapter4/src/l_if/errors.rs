use crate::{
    l_if::{
        eval::Value,
        parse::Error as ParseErr,
        syntax::{types::Type, Op},
    },
    Var,
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    ArityMismatch {
        op: Op,
        found: usize,
        expected: usize,
    },
    BadValue {
        found: Value,
        expected: Type,
    },
    TypeMismatch {
        found: Type,
        expected: Type,
    },
    Parse(ParseErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} appears free"),
            Error::ArityMismatch {
                op,
                found,
                expected,
            } => write!(
                f,
                "Arity Mismatch for {op}: found {found}, expected {expected}"
            ),
            Error::BadValue { found, expected } => {
                write!(f, "Expected value of type {expected}, got {found}")
            }
            Error::TypeMismatch { found, expected } => {
                write!(f, "Expected type {expected}, found {found}")
            }
            Error::Parse(err) => write!(f, "Error during parsing: {err}"),
        }
    }
}

impl From<ParseErr> for Error {
    fn from(err: ParseErr) -> Error {
        Error::Parse(err)
    }
}
