use super::{Arg, Instruction, Reg};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub blocks: HashMap<String, Vec<Instruction<Arg>>>,
    pub stack_space: u64,
    pub used_callee: HashSet<Reg>,
}

impl Program {
    pub fn new(stack_space: u64, used_callee: HashSet<Reg>) -> Program {
        Program {
            stack_space,
            blocks: HashMap::new(),
            used_callee,
        }
    }

    pub fn add_block(&mut self, label: &str, block: Vec<Instruction<Arg>>) {
        self.blocks.insert(label.to_owned(), block);
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, ".global main")?;
        for (label, instrs) in self.blocks.iter() {
            writeln!(
                f,
                "{label}:\n{}",
                instrs
                    .iter()
                    .map(|instr| format!("\t{instr}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            )?;
        }
        Ok(())
    }
}
