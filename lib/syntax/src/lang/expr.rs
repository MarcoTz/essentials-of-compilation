use crate::{BinaryOperation, UnaryOperation};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone)]
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

    pub fn subst_var(self, old: &str, new: &str) -> Expression {
        match self {
            Expression::Literal(_) => self,
            Expression::Variable(ref v) => {
                if v == old {
                    Expression::var(new)
                } else {
                    self
                }
            }
            Expression::InputInt => self,
            Expression::LetIn {
                var,
                bound_exp,
                in_exp,
            } => {
                let bound_subst = bound_exp.subst_var(old, new);
                if old == var {
                    Expression::let_in(&var, bound_subst, *in_exp)
                } else {
                    let in_subst = in_exp.subst_var(old, new);
                    Expression::let_in(&var, bound_subst, in_subst)
                }
            }
            Expression::BinOp { fst, op, snd } => {
                let fst_subst = fst.subst_var(old, new);
                let snd_subst = snd.subst_var(old, new);
                Expression::bin(fst_subst, op, snd_subst)
            }
            Expression::UnOp { arg, op } => Expression::un(arg.subst_var(old, new), op),
        }
    }

    pub fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Literal(_) => HashSet::new(),
            Expression::Variable(v) => HashSet::from([v.clone()]),
            Expression::InputInt => HashSet::new(),
            Expression::LetIn {
                var,
                bound_exp,
                in_exp,
            } => &(&HashSet::from([var.clone()]) | &bound_exp.used_vars()) | &in_exp.used_vars(),
            Expression::BinOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::UnOp { arg, .. } => arg.used_vars(),
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
