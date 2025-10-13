use super::{Arg, Block, Reg};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone)]
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
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        if self.stack_space != other.stack_space || (self.used_callee != other.used_callee) {
            return false;
        }
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
        writeln!(f, ".global main")?;
        for block in self.blocks.iter() {
            block.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
