use crate::Label;
use std::collections::HashMap;

pub mod arg;
pub mod instr;
pub mod reg;
pub use arg::{Arg, CC};
pub use instr::Instr;
pub use reg::{ByteReg, Reg};

pub struct Program {
    pub blocks: HashMap<Label, Vec<Instr>>,
}
