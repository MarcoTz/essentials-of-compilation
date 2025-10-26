use definitions::{
    BinaryOperation, Comparator, READ_INT_CALL, UnaryOperation,
    traits::{SubstVar, UsedVars},
};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(i64),
    Bool(bool),
    Variable(String),
    ReadInt,
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
    Tuple {
        inner: Vec<Expression>,
    },
    TupleAccess {
        tup: Box<Expression>,
        index: usize,
    },
    Reference {
        inner: Box<Expression>,
    },
    Dereference {
        inner: Box<Expression>,
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
}

impl UsedVars for Expression {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Literal(_) => HashSet::new(),
            Expression::Bool(_) => HashSet::new(),
            Expression::Variable(v) => HashSet::from([v.clone()]),
            Expression::ReadInt => HashSet::new(),
            Expression::BinOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::UnOp { arg, .. } => arg.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
            Expression::Tuple { inner } => inner
                .iter()
                .fold(HashSet::new(), |vars, exp| &vars | &exp.used_vars()),
            Expression::TupleAccess { tup, .. } => tup.used_vars(),
            Expression::Reference { inner } => inner.used_vars(),
            Expression::Dereference { inner } => inner.used_vars(),
        }
    }
}

impl SubstVar for Expression {
    fn subst_var(self, old: &str, new: &str) -> Expression {
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
            Expression::BinOp { fst, op, snd } => {
                let fst_subst = fst.subst_var(old, new);
                let snd_subst = snd.subst_var(old, new);
                Expression::bin(fst_subst, op, snd_subst)
            }
            Expression::UnOp { arg, op } => Expression::un(arg.subst_var(old, new), op),
            Expression::Cmp { left, cmp, right } => {
                Expression::cmp(left.subst_var(old, new), cmp, right.subst_var(old, new))
            }
            Expression::Tuple { inner } => Expression::Tuple {
                inner: inner
                    .into_iter()
                    .map(|exp| exp.subst_var(old, new))
                    .collect(),
            },
            Expression::TupleAccess { tup, index } => Expression::TupleAccess {
                tup: Box::new(tup.subst_var(old, new)),
                index,
            },
            Expression::Reference { inner } => Expression::Reference {
                inner: Box::new(inner.subst_var(old, new)),
            },
            Expression::Dereference { inner } => Expression::Dereference {
                inner: Box::new(inner.subst_var(old, new)),
            },
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
            Expression::BinOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
            Expression::UnOp { arg, op } => write!(f, "{op}{arg}"),
            Expression::Cmp { left, cmp, right } => write!(f, "{left}{cmp}{right}"),
            Expression::Tuple { inner } => write!(
                f,
                "({})",
                inner
                    .iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Expression::TupleAccess { tup, index } => write!(f, "{tup}[{index}]"),
            Expression::Reference { inner } => write!(f, "&{inner}"),
            Expression::Dereference { inner } => write!(f, "*{inner}"),
        }
    }
}
