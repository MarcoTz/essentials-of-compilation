use crate::{
    l_if::{
        eval::Value,
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
        }
    }
}
