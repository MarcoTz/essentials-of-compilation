use super::{Expression, Statement};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Tail {
    pub stmts: Vec<Statement>,
    pub ret: Expression,
}

impl Tail {
    pub fn new(ret: Expression, stmts: Vec<Statement>) -> Tail {
        Tail { ret, stmts }
    }

    pub fn ret(exp: Expression) -> Tail {
        Tail {
            ret: exp,
            stmts: vec![],
        }
    }
}

impl fmt::Display for Tail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.stmts.iter() {
            writeln!(f, "{stmt}")?;
        }
        writeln!(f, "return {}", self.ret)
    }
}
