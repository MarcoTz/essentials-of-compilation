use super::{Atm, Exp};
use crate::Var;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Vec<Atm>),
    Exp(Exp),
    Assign(Var, Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(atms) => write!(
                f,
                "print({})",
                atms.iter()
                    .map(|at| format!("{}", at))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Stmt::Exp(e) => e.fmt(f),
            Stmt::Assign(var, exp) => write!(f, "{var} = {exp}"),
        }
    }
}
