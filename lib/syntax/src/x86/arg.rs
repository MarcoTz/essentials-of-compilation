use super::reg::Reg;
use std::fmt;

#[derive(Debug)]
pub enum Arg {
    Immediate(i64),
    Register(Reg),
    Deref(Reg, i64),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::Immediate(i) => write!(f, "${i}"),
            Arg::Register(reg) => write!(f, "%{reg}"),
            Arg::Deref(reg, offset) => write!(f, "{offset}(%{reg})"),
        }
    }
}
