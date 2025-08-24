use std::{
    collections::{HashMap, HashSet},
    fmt,
};

pub mod arg;
pub mod instr;
pub mod reg;

pub use arg::{Arg, VarArg};
pub use instr::Instruction;
pub use reg::Reg;

#[derive(Debug, Clone)]
pub struct VarProgram {
    pub blocks: HashMap<String, Vec<Instruction<VarArg>>>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub blocks: HashMap<String, Vec<Instruction<Arg>>>,
    pub stack_space: u64,
    pub used_callee: HashSet<Reg>,
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
