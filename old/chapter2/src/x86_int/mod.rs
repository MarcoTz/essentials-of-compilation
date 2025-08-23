pub mod arg;
pub mod instr;
pub mod patch_instructions;
pub mod prelude_conclusion;
pub mod reg;

pub use arg::Arg;
pub use instr::Instr;
pub use reg::Reg;

use std::fmt;

use std::collections::{HashMap, HashSet};

pub type Label = String;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub blocks: HashMap<Label, Vec<Instr>>,
    pub stack_space: usize,
    pub global_labels: HashSet<Label>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let globl_str = self
            .global_labels
            .iter()
            .map(|label| format!(".globl {label}"))
            .collect::<Vec<String>>()
            .join("\n");
        let block_str = self
            .blocks
            .iter()
            .map(|(label, instrs)| {
                let instr_str = instrs
                    .iter()
                    .map(|instr| format!("\t{instr}"))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("{label}:\n{instr_str}")
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{globl_str}\n{block_str}\n")
    }
}
