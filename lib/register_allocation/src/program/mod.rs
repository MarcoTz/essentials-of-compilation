use asm::VarProgram;
use std::fmt;

mod live_block;
mod live_instruction;
pub mod location;
pub use live_block::LiveBlock;
pub use live_instruction::LiveInstruction;
pub use location::Location;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiveProg {
    pub blocks: Vec<LiveBlock>,
}

impl LiveProg {
    pub fn new() -> LiveProg {
        LiveProg { blocks: vec![] }
    }
}

impl fmt::Display for LiveProg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in self.blocks.iter() {
            block.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for LiveProg {
    fn default() -> LiveProg {
        LiveProg::new()
    }
}

impl From<VarProgram> for LiveProg {
    fn from(prog: VarProgram) -> LiveProg {
        let mut annot_prog = LiveProg::new();
        for block in prog.blocks {
            annot_prog.blocks.push(block.into())
        }
        annot_prog
    }
}
