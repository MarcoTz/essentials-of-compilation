use super::{BinaryOperation, UnaryOperation};
use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Literal(i64),
    Variable(String),
    InputInt,
    LetIn {
        var: String,
        bound_exp: Box<Expression>,
        in_exp: Box<Expression>,
    },
    BinOp {
        fst: Box<Expression>,
        op: BinaryOperation,
        snd: Box<Expression>,
    },
    UnOp {
        arg: Box<Expression>,
        op: UnaryOperation,
    },
}

impl Expression {
    pub fn lit(i: i64) -> Expression {
        Expression::Literal(i)
    }

    pub fn var(v: &str) -> Expression {
        Expression::Variable(v.to_owned())
    }

    pub fn let_in(v: &str, bound_exp: Expression, in_exp: Expression) -> Expression {
        Expression::LetIn {
            var: v.to_owned(),
            bound_exp: Box::new(bound_exp),
            in_exp: Box::new(in_exp),
        }
    }

    pub fn bin(fst: Expression, op: BinaryOperation, snd: Expression) -> Expression {
        Expression::BinOp {
            fst: Box::new(fst),
            op,
            snd: Box::new(snd),
        }
    }

    pub fn un(arg: Expression, op: UnaryOperation) -> Expression {
        Expression::UnOp {
            arg: Box::new(arg),
            op,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{lit}"),
            Expression::Variable(v) => f.write_str(v),
            Expression::InputInt => f.write_str("input_int"),
            Expression::LetIn {
                var,
                bound_exp,
                in_exp,
            } => write!(f, "let {var} = {bound_exp};\n{in_exp}"),
            Expression::BinOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
            Expression::UnOp { arg, op } => write!(f, "{op}{arg}"),
        }
    }
}
