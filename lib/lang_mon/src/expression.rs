use super::Atom;
use definitions::{BinaryOperation, Comparator, READ_INT_CALL, UnaryOperation, traits::UsedVars};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Atm(Atom),
    ReadInt,
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
}

impl Expression {
    pub fn un(arg: Atom, op: UnaryOperation) -> Expression {
        Expression::UnaryOp { arg, op }
    }

    pub fn bin(fst: Atom, op: BinaryOperation, snd: Atom) -> Expression {
        Expression::BinaryOp { fst, op, snd }
    }

    pub fn cmp(left: Atom, cmp: Comparator, right: Atom) -> Expression {
        Expression::Cmp { left, cmp, right }
    }
}

impl UsedVars for Expression {
    fn used_vars(&self) -> HashSet<String> {
        match self {
            Expression::Atm(atm) => atm.used_vars(),
            Expression::ReadInt => HashSet::new(),
            Expression::UnaryOp { arg, .. } => arg.used_vars(),
            Expression::BinaryOp { fst, snd, .. } => &fst.used_vars() | &snd.used_vars(),
            Expression::Cmp { left, right, .. } => &left.used_vars() | &right.used_vars(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atm(atm) => atm.fmt(f),
            Expression::ReadInt => f.write_str(READ_INT_CALL),
            Expression::UnaryOp { arg, op } => write!(f, "{op}({arg})"),
            Expression::BinaryOp { fst, op, snd } => write!(f, "{fst} {op} {snd}"),
            Expression::Cmp { left, cmp, right } => write!(f, "{left} {cmp} {right}"),
        }
    }
}
