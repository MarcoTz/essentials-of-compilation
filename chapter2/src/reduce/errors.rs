use crate::l_var_reduced::Stmt;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    StmtShouldBeExp(Stmt),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StmtShouldBeExp(st) => write!(f, "Statement {st} should be an expression"),
        }
    }
}

impl std::error::Error for Error {}
