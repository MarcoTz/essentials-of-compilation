use super::LiveInstruction;
use asm::{Block, VarArg};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiveBlock {
    pub label: String,
    pub instrs: Vec<LiveInstruction>,
}

impl fmt::Display for LiveBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: ", self.label)?;
        for instr in self.instrs.iter() {
            writeln!(f, "\t{instr}")?;
        }
        Ok(())
    }
}

impl From<Block<VarArg>> for LiveBlock {
    fn from(block: Block<VarArg>) -> LiveBlock {
        let mut live_block = LiveBlock {
            label: block.label,
            instrs: vec![],
        };
        for instr in block.instrs {
            live_block.instrs.push(instr.into());
        }
        live_block
    }
}
