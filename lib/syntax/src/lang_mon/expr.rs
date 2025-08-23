use super::Atom;
use crate::{BinaryOperation, UnaryOperation};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expression {
    Atm(Atom),
    InputInt,
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
        bound_exp: Box<Expression>,
        in_exp: Box<Expression>,
    },
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg: arg, op }
    }

    pub fn bin(fst: Atom, op: BinaryOperation, snd: Atom) -> Expression {
        Expression::BinaryOp { fst, op, snd }
    }

    pub fn let_in(var: &str, bound_exp: Expression, in_exp: Expression) -> Expression {
        Expression::LetIn {
            var: var.to_owned(),
            bound_exp: Box::new(bound_exp),
            in_exp: Box::new(in_exp),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atm(atm) => atm.fmt(f),
            Expression::InputInt => f.write_str("input_int"),
            Expression::LetIn {
                var,
                bound_exp,
                in_exp,
            } => write!(f, "let {var} = {bound_exp};\n{in_exp}"),
            Expression::UnaryOp { arg, op } => write!(f, "{op}({arg})"),
            Expression::BinaryOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
        }
    }
}
