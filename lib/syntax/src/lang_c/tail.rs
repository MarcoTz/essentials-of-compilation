use super::{Atom, Statement};
use crate::Comparator;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub stmts: Vec<Statement>,
    pub cont: Continuation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continuation {
    Return(Atom),
    Goto(String),
    If {
        left: Atom,
        cmp: Comparator,
        right: Atom,
        then_label: String,
        else_label: String,
    },
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.stmts.iter() {
            writeln!(f, "{stmt}")?;
        }
        self.cont.fmt(f)
    }
}

impl fmt::Display for Continuation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Continuation::Return(exp) => write!(f, "return {exp}"),
            Continuation::Goto(label) => write!(f, "goto {label}"),
            Continuation::If {
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
