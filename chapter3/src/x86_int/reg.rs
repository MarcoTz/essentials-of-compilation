use chapter2::x86_var::Reg as VarReg;
use std::fmt;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum Reg {
    Rsp, // stack pointer
    Rbp, // base pointer
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

impl From<VarReg> for Reg {
    fn from(reg: VarReg) -> Reg {
        match reg {
            VarReg::Rsp => Reg::Rsp,
            VarReg::Rbp => Reg::Rbp,
            VarReg::Rax => Reg::Rax,
            VarReg::Rbx => Reg::Rbx,
            VarReg::Rcx => Reg::Rcx,
            VarReg::Rdx => Reg::Rdx,
            VarReg::Rsi => Reg::Rsi,
            VarReg::Rdi => Reg::Rdi,
            VarReg::R8 => Reg::R8,
            VarReg::R9 => Reg::R9,
            VarReg::R10 => Reg::R10,
            VarReg::R11 => Reg::R11,
            VarReg::R12 => Reg::R12,
            VarReg::R13 => Reg::R13,
            VarReg::R14 => Reg::R14,
            VarReg::R15 => Reg::R15,
        }
    }
}
