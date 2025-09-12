use super::Atom;
use crate::{BinaryOperation, Comparator, PRINT_CALL, READ_INT_CALL, UnaryOperation};
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
    Cmp {
        left: Atom,
        cmp: Comparator,
        right: Atom,
    },
    LetIn {
        var: String,
        bound: Box<Expression>,
    },
    If {
        cond_exp: Box<Expression>,
        then_block: Vec<Expression>,
        else_block: Vec<Expression>,
    },
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg, op }
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

    pub fn cmp(left: Atom, cmp: Comparator, right: Atom) -> Expression {
        Expression::Cmp { left, cmp, right }
    }

    pub fn if_exp(
        cond_exp: Expression,
        then_block: Vec<Expression>,
        else_block: Vec<Expression>,
    ) -> Expression {
        Expression::If {
            cond_exp: Box::new(cond_exp),
            then_block,
            else_block,
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
            Expression::Cmp { left, cmp, right } => write!(f, "{left} {cmp} {right}"),
            Expression::If {
                cond_exp,
                then_block,
                else_block,
            } => write!(
                f,
                "if {cond_exp} {{ {} }} else {{ {} }}",
                then_block
                    .iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<_>>()
                    .join(";\n"),
                else_block
                    .iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<_>>()
                    .join(";\n")
            ),
        }
    }
}
