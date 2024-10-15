use super::{Exp, Stmt};
use crate::Var;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Atm {
    Constant(i64),
    Name(Var),
}

impl fmt::Display for Atm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atm::Constant(i) => i.fmt(f),
            Atm::Name(var) => var.fmt(f),
        }
    }
}

impl From<Atm> for Exp {
    fn from(at: Atm) -> Exp {
        Exp::Atm(at)
    }
}
impl From<Atm> for Stmt {
    fn from(at: Atm) -> Stmt {
        Stmt::Exp(at.into())
    }
}

impl Atm {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Atm::Constant(_) => false,
            Atm::Name(v) => var == v,
        }
    }
}
