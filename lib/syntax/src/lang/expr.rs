use crate::{BinaryOperation, Comparator, PRINT_CALL, READ_INT_CALL, UnaryOperation};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(i64),
    Bool(bool),
    Variable(String),
    ReadInt,
    Print(Box<Expression>),
    LetIn {
        var: String,
        bound: Box<Expression>,
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
    Cmp {
        left: Box<Expression>,
        cmp: Comparator,
        right: Box<Expression>,
    },
    If {
        cond_exp: Box<Expression>,
        then_block: Vec<Expression>,
        else_block: Vec<Expression>,
    },
}

impl Expression {
    pub fn lit(i: i64) -> Expression {
        Expression::Literal(i)
    }

    pub fn bool(b: bool) -> Expression {
        Expression::Bool(b)
    }

    pub fn var(v: &str) -> Expression {
        Expression::Variable(v.to_owned())
    }

    pub fn let_in(v: &str, bound_exp: Expression) -> Expression {
        Expression::LetIn {
            var: v.to_owned(),
            bound: Box::new(bound_exp),
        }
    }

    pub fn if_exp(
        cond: Expression,
        then: Vec<Expression>,
        else_exp: Vec<Expression>,
    ) -> Expression {
        Expression::If {
            cond_exp: Box::new(cond),
            then_block: then,
            else_block: else_exp,
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

    pub fn cmp(left: Expression, cmp: Comparator, right: Expression) -> Expression {
        Expression::Cmp {
            left: Box::new(left),
            cmp,
            right: Box::new(right),
        }
    }

    pub fn subst_var(self, old: &str, new: &str) -> Expression {
        match self {
            Expression::Literal(_) => self,
            Expression::Bool(_) => self,
            Expression::Variable(ref v) => {
                if v == old {
                    Expression::var(new)
                } else {
                    self
                }
            }
            Expression::ReadInt => self,
            Expression::Print(exp) => Expression::Print(Box::new(exp.subst_var(old, new))),
            Expression::LetIn { var, bound } => {
                let bound_subst = bound.subst_var(old, new);
                Expression::let_in(&var, bound_subst)
            }
            Expression::BinOp { fst, op, snd } => {
                let fst_subst = fst.subst_var(old, new);
                let snd_subst = snd.subst_var(old, new);
                Expression::bin(fst_subst, op, snd_subst)
            }
            Expression::UnOp { arg, op } => Expression::un(arg.subst_var(old, new), op),
            Expression::Cmp { left, cmp, right } => {
                Expression::cmp(left.subst_var(old, new), cmp, right.subst_var(old, new))
            }
            Expression::If {
                cond_exp,
                then_block,
                else_block,
            } => Expression::if_exp(
                cond_exp.subst_var(old, new),
                then_block
                    .into_iter()
                    .map(|exp| exp.subst_var(old, new))
                    .collect(),
                else_block
                    .into_iter()
                    .map(|exp| exp.subst_var(old, new))
                    .collect(),
            ),
        }
    }

    pub fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Literal(_) => HashSet::new(),
            Expression::Bool(_) => HashSet::new(),
            Expression::Variable(v) => HashSet::from([v.clone()]),
            Expression::ReadInt => HashSet::new(),
            Expression::Print(exp) => exp.used_vars(),
            Expression::LetIn { var, bound } => &HashSet::from([var.clone()]) | &bound.used_vars(),
            Expression::BinOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::UnOp { arg, .. } => arg.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
            Expression::If {
                cond_exp,
                then_block,
                else_block,
            } => {
                &(&cond_exp.used_vars()
                    | &then_block
                        .into_iter()
                        .fold(HashSet::new(), |used, next| &used | &next.used_vars()))
                    | &else_block
                        .into_iter()
                        .fold(HashSet::new(), |used, next| &used | &next.used_vars())
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{lit}"),
            Expression::Bool(b) => write!(f, "{b}"),
            Expression::Variable(v) => f.write_str(v),
            Expression::ReadInt => f.write_str(READ_INT_CALL),
            Expression::Print(exp) => write!(f, "{PRINT_CALL}({exp})"),
            Expression::LetIn { var, bound } => write!(f, "let {var} = {bound}"),
            Expression::BinOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
            Expression::UnOp { arg, op } => write!(f, "{op}{arg}"),
            Expression::Cmp { left, cmp, right } => write!(f, "{left}{cmp}{right}"),
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
