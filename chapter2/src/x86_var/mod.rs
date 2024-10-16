pub use crate::x86_int::{instr::Instr, reg::Reg};

pub mod arg;
pub use arg::Arg;

pub type Prog = Vec<Instr<Arg>>;
