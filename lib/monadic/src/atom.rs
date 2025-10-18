use super::expression::Expression;
use definitions::traits::UsedVars;
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    Integer(i64),
    Variable(String),
    Bool(bool),
}

impl UsedVars for Atom {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Atom::Variable(v) => HashSet::from([v.clone()]),
            _ => HashSet::new(),
        }
    }
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

impl From<i64> for Atom {
    fn from(i: i64) -> Atom {
        Atom::Integer(i)
    }
}

impl From<bool> for Atom {
    fn from(b: bool) -> Atom {
        Atom::Bool(b)
    }
}

impl From<&str> for Atom {
    fn from(v: &str) -> Atom {
        Atom::Variable(v.to_owned())
    }
}
