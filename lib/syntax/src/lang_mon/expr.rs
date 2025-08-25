use super::Atom;
use crate::{BinaryOperation, PRINT_CALL, READ_INT_CALL, UnaryOperation};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Atm(Atom),
    ReadInt,
    Print(Atom),
    UnaryOp {
        arg: Atom,
        op: UnaryOperation,
    },
    BinaryOp {
        fst: Atom,
        op: BinaryOperation,
        snd: Atom,
    },
    LetIn {
        var: String,
        bound: Box<Expression>,
    },
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg: arg, op }
    }

    pub fn bin(fst: Atom, op: BinaryOperation, snd: Atom) -> Expression {
        Expression::BinaryOp { fst, op, snd }
    }

    pub fn let_in(var: &str, bound_exp: Expression) -> Expression {
        Expression::LetIn {
            var: var.to_owned(),
            bound: Box::new(bound_exp),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atm(atm) => atm.fmt(f),
            Expression::ReadInt => f.write_str(READ_INT_CALL),
            Expression::Print(atm) => write!(f, "{PRINT_CALL}({atm})"),
            Expression::LetIn { var, bound } => write!(f, "let {var} = {bound}"),
            Expression::UnaryOp { arg, op } => write!(f, "{op}({arg})"),
            Expression::BinaryOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
        }
    }
}
