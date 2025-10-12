use std::fmt;

mod atm;
mod expr;
mod stmt;
mod tail;

pub use atm::Atom;
pub use expr::Expression;
pub use stmt::Statement;
pub use tail::{Tail, TailEnd};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub label: String,
    pub tail: Tail,
}

impl Block {
    pub fn new(lb: &str, tail: Tail) -> Block {
        Block {
            label: lb.to_owned(),
            tail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub blocks: Vec<Block>,
}

impl Program {
    pub fn new() -> Program {
        Program { blocks: vec![] }
    }

    pub fn add_block(&mut self, label: &str, tail: Tail) {
        self.blocks.push(Block::new(label, tail));
    }

    pub fn get_block_mut(&mut self, label: &str) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|bl| bl.label == label)
    }

    pub fn get_labels(&self) -> Vec<&String> {
        self.blocks.iter().map(|block| &block.label).collect()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.blocks.iter() {
            block.fmt(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:\n{}", self.label, self.tail)
    }
}

impl Default for Program {
    fn default() -> Program {
        Program::new()
    }
}
