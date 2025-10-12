use super::{Atom, Tail, TailEnd};
use crate::{BinaryOperation, Comparator, READ_INT_CALL, UnaryOperation};
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
    Cmp {
        left: Atom,
        cmp: Comparator,
        right: Atom,
    },
    Unit,
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg, op }
    }

    pub fn bin(fst: Atom, op: BinaryOperation, snd: Atom) -> Expression {
        Expression::BinOp { fst, op, snd }
    }

    pub fn cmp(left: Atom, cmp: Comparator, right: Atom) -> Expression {
        Expression::Cmp { left, cmp, right }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atm(atm) => atm.fmt(f),
            Expression::ReadInt => f.write_str(READ_INT_CALL),
            Expression::UnaryOp { arg, op } => write!(f, "{op}({arg})"),
            Expression::BinOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
            Expression::Unit => f.write_str("()"),
            Expression::Cmp { left, cmp, right } => write!(f, "{left}{cmp}{right}"),
        }
    }
}

impl From<Expression> for Tail {
    fn from(exp: Expression) -> Tail {
        Tail::ret(exp)
    }
}

impl From<Expression> for TailEnd {
    fn from(exp: Expression) -> TailEnd {
        TailEnd::Return(exp)
    }
}
