pub mod arg;
pub mod instr;
pub mod reg;

pub use arg::Arg;
pub use instr::Instr;
pub use reg::Reg;

#[derive(Debug, PartialEq, Eq)]
pub struct Prog {
    pub instrs: Vec<Instr<Arg>>,
    pub stack_space: usize,
}
