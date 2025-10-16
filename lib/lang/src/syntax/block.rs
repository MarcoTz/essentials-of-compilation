use super::{Program, Statement};
use definitions::traits::{SubstVar, UsedVars};
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
            used.extend(stmt.used_vars())
        }
        used
    }
}

impl SubstVar for Block {
    fn subst_var(self, old: &str, new: &str) -> Block {
        let mut stmts_subst = Vec::with_capacity(self.stmts.len());
        for stmt in self.stmts {
            stmts_subst.push(stmt.subst_var(old, new));
        }
        Block { stmts: stmts_subst }
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
