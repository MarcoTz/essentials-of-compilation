use super::{Program, Statement};
use definitions::traits::UsedVars;
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

impl Block {
    pub fn new(stmts: Vec<Statement>) -> Block {
        Block { stmts }
    }
}

impl UsedVars for Block {
    fn used_vars(&self) -> HashSet<String> {
        let mut used = HashSet::new();
        for stmt in self.stmts.iter() {
            used.extend(stmt.used_vars());
        }
        used
    }
}

impl From<Block> for Program {
    fn from(b: Block) -> Program {
        Program { main: b }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.stmts
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
