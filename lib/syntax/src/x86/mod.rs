use std::{collections::HashMap, fmt};

pub mod arg;
pub mod instr;
pub mod reg;

pub use arg::{Arg, VarArg};
pub use instr::Instruction;
pub use reg::Reg;

pub type Block<Arg> = Vec<Instruction<Arg>>;

#[derive(Debug, Clone)]
pub struct Program<Arg> {
    pub blocks: HashMap<String, Block<Arg>>,
}

impl<Arg> Program<Arg> {
    pub fn new() -> Program<Arg> {
        Program {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, lb: &str, block: Block<Arg>) {
        self.blocks.insert(lb.to_owned(), block);
    }
}

pub type VarProg = Program<VarArg>;
pub type Prog = Program<Arg>;

impl<Arg> fmt::Display for Program<Arg>
where
    Arg: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, ".global main")?;
        for (label, instrs) in self.blocks.iter() {
            write!(
                f,
                "{label}:\n{}",
                instrs
                    .iter()
                    .map(|instr| format!("\t{}", instr.to_string()))
                    .collect::<Vec<String>>()
                    .join("\n")
            )?;
        }
        Ok(())
    }
}
