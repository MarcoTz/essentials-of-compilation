use super::{
    arg::{Arg, VarArg},
    reg::Reg,
};
use crate::patch_instructions::{PatchInstructions, remove_double_deref};
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
    XorQ { src: Arg, dest: Arg },
    CmpQ { left: Arg, right: Arg },
    SetCC { cc: Cc, dest: Arg },
    MovZBQ { src: Arg, dest: Arg },
    JumpCC { cc: Cc, label: String },
    AndQ { src: Arg, dest: Arg },
    OrQ { src: Arg, dest: Arg },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cc {
    E,
    Ne,
    L,
    Le,
    G,
    Ge,
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

impl PatchInstructions for Instruction<Arg> {
    type Target = Vec<Instruction<Arg>>;
    fn patch_instructions(self) -> Vec<Instruction<Arg>> {
        match self {
            Instruction::NegQ { arg } => vec![Instruction::NegQ { arg }],
            Instruction::PushQ { arg } => vec![Instruction::PushQ { arg }],
            Instruction::PopQ { arg } => vec![Instruction::PopQ { arg }],
            Instruction::CallQ { label } => vec![Instruction::CallQ { label }],
            Instruction::RetQ => vec![Instruction::RetQ],
            Instruction::Jump { label } => vec![Instruction::Jump { label }],
            Instruction::JumpCC { cc, label } => vec![Instruction::JumpCC { cc, label }],
            Instruction::SetCC { cc, dest } => vec![Instruction::SetCC { cc, dest }],

            Instruction::CmpQ {
                left: Arg::Immediate(i),
                right,
            } => vec![
                Instruction::MovQ {
                    src: Arg::Immediate(i),
                    dest: Reg::Rax.into(),
                },
                Instruction::CmpQ {
                    left: Reg::Rax.into(),
                    right,
                },
            ],
            Instruction::MovZBQ {
                src,
                dest: Arg::Register(reg),
            } => vec![Instruction::MovZBQ {
                src,
                dest: Arg::Register(reg),
            }],

            Instruction::AddQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::AddQ { src, dest })
            }
            Instruction::SubQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::SubQ { src, dest })
            }
            Instruction::MovQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::MovQ { src, dest })
            }
            Instruction::XorQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::XorQ { src, dest })
            }
            Instruction::CmpQ { left, right } => {
                remove_double_deref(left, right, |left, right| Instruction::CmpQ { left, right })
            }
            Instruction::MovZBQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::MovZBQ { src, dest })
            }
            Instruction::AndQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::AndQ { src, dest })
            }
            Instruction::OrQ { src, dest } => {
                remove_double_deref(src, dest, |src, dest| Instruction::OrQ { src, dest })
            }
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
            Instruction::XorQ { src, dest } => write!(f, "xorq {src}, {dest}"),
            Instruction::CmpQ { left, right } => write!(f, "cmpq {right}, {left}"),
            Instruction::SetCC { cc, dest: arg } => write!(f, "set{cc} {arg}"),
            Instruction::MovZBQ { src, dest } => write!(f, "movzbq {src}, {dest}"),
            Instruction::JumpCC { cc, label } => write!(f, "j{cc} {label}"),
            Instruction::AndQ { src, dest } => write!(f, "andq {src}, {dest}"),
            Instruction::OrQ { src, dest } => write!(f, "orq {src}, {dest}"),
        }
    }
}

impl fmt::Display for Cc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cc::E => f.write_str("e"),
            Cc::Ne => f.write_str("ne"),
            Cc::L => f.write_str("l"),
            Cc::Le => f.write_str("le"),
            Cc::G => f.write_str("g"),
            Cc::Ge => f.write_str("ge"),
        }
    }
}
