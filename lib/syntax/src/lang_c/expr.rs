use super::{Atom, Tail};
use crate::{BinaryOperation, READ_INT_CALL, UnaryOperation};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Atm(Atom),
    ReadInt,
    UnaryOp {
        arg: Atom,
        op: UnaryOperation,
    },
    BinOp {
        fst: Atom,
        op: BinaryOperation,
        snd: Atom,
    },
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg, op }
    }

    pub fn bin(fst: Atom, op: BinaryOperation, snd: Atom) -> Expression {
        Expression::BinOp { fst, op, snd }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atm(atm) => atm.fmt(f),
            Expression::ReadInt => f.write_str(READ_INT_CALL),
            Expression::UnaryOp { arg, op } => write!(f, "{op}({arg})"),
            Expression::BinOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
        }
    }
}

impl From<Expression> for Tail {
    fn from(exp: Expression) -> Tail {
        Tail::ret(exp)
    }
}
