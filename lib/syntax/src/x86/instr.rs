use super::arg::{Arg, VarArg};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction<Arg> {
    AddQ { src: Arg, dest: Arg },
    SubQ { src: Arg, dest: Arg },
    NegQ { arg: Arg },
    MovQ { src: Arg, dest: Arg },
    PushQ { arg: Arg },
    PopQ { arg: Arg },
    CallQ { label: String },
    RetQ,
    Jump { label: String },
}

pub type VarInstr = Instruction<VarArg>;
pub type Instr = Instruction<Arg>;

impl<Arg> fmt::Display for Instruction<Arg>
where
    Arg: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::AddQ { src, dest } => write!(f, "addq {src}, {dest}"),
            Instruction::SubQ { src, dest } => write!(f, "subq {src}, {dest}"),
            Instruction::NegQ { arg } => write!(f, "negq {arg}"),
            Instruction::MovQ { src, dest } => write!(f, "movq {src}, {dest}"),
            Instruction::PushQ { arg } => write!(f, "pushq {arg}"),
            Instruction::PopQ { arg } => write!(f, "popq {arg}"),
            Instruction::CallQ { label } => write!(f, "callq {label}"),
            Instruction::RetQ => write!(f, "retq"),
            Instruction::Jump { label } => write!(f, "jmp {label}"),
        }
    }
}
