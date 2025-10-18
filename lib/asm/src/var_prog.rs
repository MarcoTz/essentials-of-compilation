use super::{Block, Instruction, VarArg};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarProgram {
    pub blocks: Vec<Block<VarArg>>,
}

impl VarProgram {
    pub fn new() -> VarProgram {
        VarProgram { blocks: vec![] }
    }

    pub fn add_block(&mut self, lb: &str, instrs: Vec<Instruction<VarArg>>) {
        self.blocks.push(Block::new(lb, instrs));
    }

    pub fn remove_block(&mut self, label: &str) -> Option<Block<VarArg>> {
        let mut to_remove = -1;
        for (ind, block) in self.blocks.iter().enumerate() {
            if block.label == label {
                to_remove = ind as i32;
                break;
            }
        }
        if to_remove < 0 {
            return None;
        }
        Some(self.blocks.remove(to_remove as usize))
    }
}

impl fmt::Display for VarProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, ".global main")?;
        for block in self.blocks.iter() {
            block.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Default for VarProgram {
    fn default() -> VarProgram {
        VarProgram::new()
    }
}
