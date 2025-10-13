use std::fmt;

mod atoms;
mod expressions;
mod statements;
mod tail;

pub use atoms::Atom;
pub use expressions::Expression;
pub use statements::Statement;
pub use tail::{Continuation, Tail};

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

#[derive(Debug, Clone)]
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

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        for block in self.blocks.iter() {
            match other.blocks.iter().find(|bl| bl.label == block.label) {
                None => return false,
                Some(bl) => {
                    if block != bl {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Eq for Program {}

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
