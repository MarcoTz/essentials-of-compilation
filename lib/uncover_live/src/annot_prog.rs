use super::LiveInstruction;
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnnotProg {
    pub blocks: HashMap<String, Vec<LiveInstruction>>,
}

impl AnnotProg {
    pub fn new() -> AnnotProg {
        AnnotProg {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, label: &str, instrs: Vec<LiveInstruction>) {
        self.blocks.insert(label.to_owned(), instrs);
    }
}

impl fmt::Display for AnnotProg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (label, block) in self.blocks.iter() {
            write!(f, "{label}")?;
            for instr in block.iter() {
                writeln!(f, "{instr}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for AnnotProg {
    fn default() -> AnnotProg {
        AnnotProg::new()
    }
}
