use super::Atom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continuation {
    Return(Atom),
    Goto(String),
    If {
        cond: Atom,
        then_label: String,
        else_label: String,
    },
}

impl fmt::Display for Continuation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Continuation::Return(exp) => write!(f, "return {exp}"),
            Continuation::Goto(label) => write!(f, "goto {label}"),
            Continuation::If {
                cond,
                then_label,
                else_label,
            } => write!(f, "if {cond} goto {then_label} else goto {else_label}"),
        }
    }
}
