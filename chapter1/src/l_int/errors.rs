use super::functions::{ExpFunction, StmtFunction};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    WrongNumArgsStmt {
        name: StmtFunction,
        found: usize,
        expected: usize,
    },
    WrongNumArgsExp {
        name: ExpFunction,
        found: usize,
        expected: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongNumArgsStmt {
                name,
                found,
                expected,
            } => write!(
                f,
                "Wrong number of arguments for statement call {}: found {}, expected {}.",
                name, found, expected
            ),
            Error::WrongNumArgsExp {
                name,
                found,
                expected,
            } => write!(
                f,
                "Wrong number of arguments for expression call {}: found {}, expected {}",
                name, found, expected
            ),
        }
    }
}
