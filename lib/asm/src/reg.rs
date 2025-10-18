use super::{Arg, VarArg};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reg {
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Reg {
    pub const fn caller_saved() -> [Reg; 9] {
        [
            Reg::Rax,
            Reg::Rcx,
            Reg::Rdx,
            Reg::Rsi,
            Reg::Rdi,
            Reg::R8,
            Reg::R9,
            Reg::R10,
            Reg::R11,
        ]
    }

    pub const fn callee_saved() -> [Reg; 7] {
        [
            Reg::Rsp,
            Reg::Rbp,
            Reg::Rbx,
            Reg::R12,
            Reg::R13,
            Reg::R14,
            Reg::R15,
        ]
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::Rsp => f.write_str("rsp"),
            Reg::Rbp => f.write_str("rbp"),
            Reg::Rax => f.write_str("rax"),
            Reg::Rbx => f.write_str("rbx"),
            Reg::Rcx => f.write_str("rcx"),
            Reg::Rdx => f.write_str("rdx"),
            Reg::Rsi => f.write_str("rsi"),
            Reg::Rdi => f.write_str("rdi"),
            Reg::R8 => f.write_str("r8"),
            Reg::R9 => f.write_str("r9"),
            Reg::R10 => f.write_str("r10"),
            Reg::R11 => f.write_str("r11"),
            Reg::R12 => f.write_str("r12"),
            Reg::R13 => f.write_str("r13"),
            Reg::R14 => f.write_str("r14"),
            Reg::R15 => f.write_str("r15"),
        }
    }
}

impl From<Reg> for Arg {
    fn from(reg: Reg) -> Arg {
        Arg::Register(reg)
    }
}

impl From<Reg> for VarArg {
    fn from(reg: Reg) -> VarArg {
        VarArg::Arg(reg.into())
    }
}
