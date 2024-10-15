use super::{
    functions::{Call, ExpFunction},
    BinOp, UnaryOp,
};
use std::fmt;

pub enum Exp {
    Constant(i32),
    Call(Call<ExpFunction>),
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
            Exp::Call(call) => call.fmt(f),
            Exp::UnaryOp { op, exp } => write!(f, "{}{}", op, exp),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "{}{}{}", exp1, op, exp2),
        }
    }
}

impl Exp {
    pub fn leaf(&self) -> bool {
        matches!(self, Exp::Constant(_) | Exp::Call(_))
    }

    pub fn is_exp(&self) -> bool {
        match self {
            Exp::Constant(_) => true,
            Exp::Call(Call {
                name: ExpFunction::InputInt,
                args,
            }) => args.is_empty(),
            Exp::UnaryOp { op: _, exp: e } => e.is_exp(),
            Exp::BinOp { op: _, exp1, exp2 } => exp1.is_exp() && exp2.is_exp(),
        }
    }
}
