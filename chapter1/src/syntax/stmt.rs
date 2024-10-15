use super::exp::Exp;
use std::fmt;

pub enum Stmt {
    Print(Exp),
    Exp(Exp),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Print(exp) => write!(f, "print({exp})"),
            Stmt::Exp(e) => e.fmt(f),
        }
    }
}

impl Stmt {
    pub fn is_stmt(&self) -> bool {
        match self {
            Stmt::Print(exp) => exp.is_exp(),
            Stmt::Exp(e) => e.is_exp(),
        }
    }
}
