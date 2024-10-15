use super::{Atm, Exp};
use crate::Var;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Atm),
    Exp(Exp),
    Assign { name: Var, exp: Exp },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(atm) => write!(f, "print({})", atm),
            Stmt::Exp(e) => e.fmt(f),
            Stmt::Assign { name, exp } => write!(f, "{name} = {exp}"),
        }
    }
}

impl Stmt {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Stmt::Print(atm) => atm.occurs(var),
            Stmt::Exp(e) => e.occurs(var),
            Stmt::Assign { name, exp } => name == var || exp.occurs(var),
        }
    }
}
