use super::{Arg, Block, Instruction, Reg};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub blocks: Vec<Block<Arg>>,
    pub stack_space: u64,
    pub used_callee: HashSet<Reg>,
}

impl Program {
    pub fn new(stack_space: u64, used_callee: HashSet<Reg>) -> Program {
        Program {
            stack_space,
            blocks: vec![],
            used_callee,
        }
    }

    pub fn add_block(&mut self, label: &str, block: Vec<Instruction<Arg>>) {
        self.blocks.push(Block::new(label, block));
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, ".global main")?;
        for block in self.blocks.iter() {
            block.fmt(f)?;
        }
        Ok(())
    }
}
