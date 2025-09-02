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

impl<Arg> Instruction<Arg> {
    pub fn add<A1, A2>(src: A1, dest: A2) -> Instruction<Arg>
    where
        A1: Into<Arg>,
        A2: Into<Arg>,
    {
        Instruction::AddQ {
            src: src.into(),
            dest: dest.into(),
        }
    }

    pub fn sub<A1, A2>(src: A1, dest: A2) -> Instruction<Arg>
    where
        A1: Into<Arg>,
        A2: Into<Arg>,
    {
        Instruction::SubQ {
            src: src.into(),
            dest: dest.into(),
        }
    }

    pub fn neg<A>(arg: A) -> Instruction<Arg>
    where
        A: Into<Arg>,
    {
        Instruction::NegQ { arg: arg.into() }
    }

    pub fn mov<A1, A2>(src: A1, dest: A2) -> Instruction<Arg>
    where
        A1: Into<Arg>,
        A2: Into<Arg>,
    {
        Instruction::MovQ {
            src: src.into(),
            dest: dest.into(),
        }
    }

    pub fn push<A>(arg: A) -> Instruction<Arg>
    where
        A: Into<Arg>,
    {
        Instruction::PushQ { arg: arg.into() }
    }

    pub fn pop<A>(arg: A) -> Instruction<Arg>
    where
        A: Into<Arg>,
    {
        Instruction::PopQ { arg: arg.into() }
    }

    pub fn call(lb: &str) -> Instruction<Arg> {
        Instruction::CallQ {
            label: lb.to_owned(),
        }
    }

    pub fn jmp(lb: &str) -> Instruction<Arg> {
        Instruction::Jump {
            label: lb.to_owned(),
        }
    }
}

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
