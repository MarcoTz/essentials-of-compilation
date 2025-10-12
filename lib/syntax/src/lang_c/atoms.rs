use super::{Continuation, Expression, Tail};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    Integer(i64),
    Variable(String),
    Bool(bool),
    Unit,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Integer(i) => write!(f, "{i}"),
            Atom::Variable(v) => f.write_str(v),
            Atom::Bool(b) => write!(f, "{b}"),
            Atom::Unit => f.write_str("()"),
        }
    }
}

impl From<Atom> for Expression {
    fn from(atm: Atom) -> Expression {
        Expression::Atm(atm)
    }
}

impl From<Atom> for Tail {
    fn from(atm: Atom) -> Tail {
        Tail {
            stmts: vec![],
            cont: Continuation::Return(atm),
        }
    }
}

impl From<Atom> for Continuation {
    fn from(atm: Atom) -> Continuation {
        Continuation::Return(atm)
    }
}
