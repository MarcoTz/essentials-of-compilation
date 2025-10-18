use super::{Continuation, Statement};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub stmts: Vec<Statement>,
    pub cont: Continuation,
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.stmts.iter() {
            writeln!(f, "{stmt}")?;
        }
        self.cont.fmt(f)
    }
}
