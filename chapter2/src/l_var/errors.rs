use super::{
    remove_complex_operands::{Exp, Stmt},
    Var,
};
use std::fmt;
#[derive(Debug)]
pub enum Error {
    VarNotFound { name: Var },
    NotAnExpression(Stmt),
    NotAnAtom(Exp),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VarNotFound { name } => {
                write!(f, "Could not find variable {name} in environemnt.")
            }
            Error::NotAnExpression(st) => write!(f, "Statement {st} is not an expression."),
            Error::NotAnAtom(exp) => write!(f, "Expression {exp} is not an atom."),
        }
    }
}

impl std::error::Error for Error {}
