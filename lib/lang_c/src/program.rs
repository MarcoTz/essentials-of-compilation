use super::{Block, Tail};
use std::fmt;

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
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tail_str = self.tail.to_string().replace("\n", "\n\t");
        write!(f, "{}:\n\t{}", self.label, tail_str)
    }
}

impl Default for Program {
    fn default() -> Program {
        Program::new()
    }
}
