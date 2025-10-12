use super::{Atom, Expression, Statement};
use crate::Comparator;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub stmts: Vec<Statement>,
    pub ret: TailEnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TailEnd {
    Return(Expression),
    Goto(String),
    If {
        left: Atom,
        cmp: Comparator,
        right: Atom,
        then_label: String,
        else_label: String,
    },
}

impl Tail {
    pub fn ret(exp: Expression) -> Tail {
        Tail {
            ret: exp.into(),
            stmts: vec![],
        }
    }
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.stmts.iter() {
            writeln!(f, "{stmt}")?;
        }
        self.ret.fmt(f)
    }
}

impl fmt::Display for TailEnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TailEnd::Return(exp) => write!(f, "return {exp}"),
            TailEnd::Goto(label) => write!(f, "goto {label}"),
            TailEnd::If {
                left,
                cmp,
                right,
                then_label,
                else_label,
            } => write!(
                f,
                "if {left}{cmp}{right} goto {then_label} else goto {else_label}"
            ),
        }
    }
}
