use super::{Instruction, VarArg};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarProgram {
    pub blocks: HashMap<String, Vec<Instruction<VarArg>>>,
}

impl VarProgram {
    pub fn new() -> VarProgram {
        VarProgram {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, lb: &str, block: Vec<Instruction<VarArg>>) {
        self.blocks.insert(lb.to_owned(), block);
    }
}

impl fmt::Display for VarProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, ".global main")?;
        for (label, instrs) in self.blocks.iter() {
            write!(
                f,
                "{label}:\n{}",
                instrs
                    .iter()
                    .map(|instr| format!("\t{instr}",))
                    .collect::<Vec<String>>()
                    .join("\n")
            )?;
        }
        Ok(())
    }
}
impl Default for VarProgram {
    fn default() -> VarProgram {
        VarProgram::new()
    }
}
