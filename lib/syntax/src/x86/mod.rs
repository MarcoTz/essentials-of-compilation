use std::{collections::HashMap, fmt};

pub mod arg;
pub mod instr;
pub mod reg;

use instr::Instr;

pub type Block = Vec<Instr>;

#[derive(Debug)]
struct Program {
    blocks: HashMap<String, Block>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".global main")?;
        for (label, instrs) in self.blocks.iter() {
            write!(
                f,
                "{label}: {}",
                instrs
                    .iter()
                    .map(|instr| instr.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )?;
        }
        Ok(())
    }
}
