use super::reg::Reg;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arg {
    Immediate(i64),
    Register(Reg),
    Deref(Reg, i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarArg {
    Arg(Arg),
    Var(String),
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

impl fmt::Display for VarArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarArg::Arg(arg) => arg.fmt(f),
            VarArg::Var(v) => f.write_str(v),
        }
    }
}

impl From<Arg> for VarArg {
    fn from(arg: Arg) -> VarArg {
        VarArg::Arg(arg)
    }
}

impl From<&str> for VarArg {
    fn from(var: &str) -> VarArg {
        VarArg::Var(var.to_owned())
    }
}

impl From<i64> for Arg {
    fn from(i: i64) -> Arg {
        Arg::Immediate(i)
    }
}

impl From<i64> for VarArg {
    fn from(i: i64) -> VarArg {
        VarArg::Arg(i.into())
    }
}
