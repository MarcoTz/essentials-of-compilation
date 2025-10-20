use super::{Atom, Expression};
use definitions::PRINT_CALL;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Assign { var: String, bound: Expression },
    Set { var: String, bound: Expression },
    Print(Atom),
}

impl Statement {
    pub fn assign(var: &str, bound: Expression) -> Statement {
        Statement::Assign {
            var: var.to_owned(),
            bound,
        }
    }

    pub fn set(var: &str, bound: Expression) -> Statement {
        Statement::Set {
            var: var.to_owned(),
            bound,
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Assign { var, bound } => write!(f, "{var} = {bound};"),
            Statement::Set { var, bound } => write!(f, "{var} := {bound};"),
            Statement::Print(exp) => write!(f, "{PRINT_CALL}({exp});"),
        }
    }
}
