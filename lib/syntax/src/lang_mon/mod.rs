use std::fmt;

mod atoms;
mod blocks;
mod expressions;
mod statements;

pub use atoms::Atom;
pub use blocks::Block;
pub use expressions::Expression;
pub use statements::Statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub main: Block,
}

impl Program {
    pub fn new(exps: Vec<Statement>) -> Program {
        Program {
            main: Block::new(exps),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.main.fmt(f)
    }
}
