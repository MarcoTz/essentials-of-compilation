use crate::l_var_reduced::Stmt;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    StmtShouldBeExp(Stmt),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StmtShouldBeExp(st) => write!(f, "Statement {st} should be an expression."),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod error_tests {
    use super::{Error, Stmt};

    #[test]
    fn display_stmt() {
        let result = format!("{}", Error::StmtShouldBeExp(Stmt::Print(1.into())));
        let expected = "Statement print(1) should be an expression.";
        assert_eq!(result, expected)
    }
}
