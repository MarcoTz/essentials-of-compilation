use super::Statement;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

impl Block {
    pub fn new(stmts: Vec<Statement>) -> Block {
        Block { stmts }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in self.stmts.iter() {
            stmt.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
