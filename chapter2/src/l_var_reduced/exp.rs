use super::{Atm, Stmt};
use crate::{BinOp, UnaryOp, Var};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Exp {
    Atm(Atm),
    InputInt,
    UnaryOp { op: UnaryOp, exp: Atm },
    BinOp { exp1: Atm, op: BinOp, exp2: Atm },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(at) => at.fmt(f),
            Exp::InputInt => f.write_str("input_int"),
            Exp::UnaryOp { op, exp } => write!(f, "{op}{exp}"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{exp1}{op}{exp2}"),
        }
    }
}

impl From<Exp> for Stmt {
    fn from(exp: Exp) -> Stmt {
        Stmt::Exp(exp)
    }
}

impl Exp {
    pub fn occurs(&self, var: &Var) -> bool {
        match self {
            Exp::Atm(atm) => atm.occurs(var),
            Exp::InputInt => false,
            Exp::UnaryOp { op: _, exp } => exp.occurs(var),
            Exp::BinOp { exp1, op: _, exp2 } => exp1.occurs(var) || exp2.occurs(var),
        }
    }
}
