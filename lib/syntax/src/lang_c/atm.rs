use super::{Expression, Tail, TailEnd};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    Integer(i64),
    Variable(String),
    Bool(bool),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Integer(i) => write!(f, "{i}"),
            Atom::Variable(v) => f.write_str(v),
            Atom::Bool(b) => write!(f, "{b}"),
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
        Tail::ret(atm.into())
    }
}

impl From<Atom> for TailEnd {
    fn from(atm: Atom) -> TailEnd {
        TailEnd::Return(atm.into())
    }
}
