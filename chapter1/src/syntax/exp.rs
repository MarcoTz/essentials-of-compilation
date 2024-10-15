use super::{BinOp, UnaryOp};
use std::fmt;

pub enum Exp {
    Constant(i32),
    InputInt,
    UnaryOp {
        op: UnaryOp,
        exp: Box<Exp>,
    },
    BinOp {
        exp1: Box<Exp>,
        op: BinOp,
        exp2: Box<Exp>,
    },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Constant(i) => write!(f, "{}", i),
            Exp::InputInt => f.write_str("input_int"),
            Exp::UnaryOp { op, exp } => write!(f, "{}{}", op, exp),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{}{}{}", exp1, op, exp2),
        }
    }
}

impl Exp {
    pub fn leaf(&self) -> bool {
        matches!(self, Exp::Constant(_) | Exp::InputInt)
    }

    pub fn is_exp(&self) -> bool {
        match self {
            Exp::Constant(_) => true,
            Exp::InputInt => true,
            Exp::UnaryOp { op: _, exp: e } => e.is_exp(),
            Exp::BinOp { op: _, exp1, exp2 } => exp1.is_exp() && exp2.is_exp(),
        }
    }
}
