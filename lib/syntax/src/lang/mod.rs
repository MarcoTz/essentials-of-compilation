use std::fmt;

mod blocks;
mod expressions;
mod statements;
mod types;

pub use blocks::Block;
pub use expressions::Expression;
pub use statements::Statement;
pub use types::Type;

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

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.main.fmt(f)
    }
}
