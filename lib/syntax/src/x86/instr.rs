use super::arg::Arg;
use std::fmt;

#[derive(Debug)]
pub enum Instr {
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

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::AddQ { src, dest } => write!(f, "addq {src}, {dest}"),
            Instr::SubQ { src, dest } => write!(f, "subq {src}, {dest}"),
            Instr::NegQ { arg } => write!(f, "negq {arg}"),
            Instr::MovQ { src, dest } => write!(f, "movq {src}, {dest}"),
            Instr::PushQ { arg } => write!(f, "pushq {arg}"),
            Instr::PopQ { arg } => write!(f, "popq {arg}"),
            Instr::CallQ { label } => write!(f, "callq {label}"),
            Instr::RetQ => write!(f, "retq"),
            Instr::Jump { label } => write!(f, "jmp {label}"),
        }
    }
}
