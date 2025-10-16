use super::{Block, Statement};
use definitions::traits::UsedVars;
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub main: Block,
}

impl Program {
    pub fn new(stmts: Vec<Statement>) -> Program {
        Program {
            main: Block { stmts },
        }
    }
}

impl UsedVars for Program {
    fn used_vars(&self) -> HashSet<String> {
        self.main.used_vars()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.main.fmt(f)
    }
}
