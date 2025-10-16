use lang_x86::{Arg, Reg, VarArg};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Location {
    Variable(String),
    Register(Reg),
    Stack(i64),
}

impl Location {
    pub fn arg_loc(arg: VarArg) -> Option<Location> {
        match arg {
            VarArg::Var(v) => Some(Location::Variable(v)),
            VarArg::Arg(Arg::Register(reg)) => Some(Location::Register(reg)),
            VarArg::Arg(Arg::Deref(_, offset)) => Some(Location::Stack(offset)),
            _ => None,
        }
    }
}

pub fn arg_locations(arg: &VarArg) -> HashSet<Location> {
    match arg {
        VarArg::Var(v) => HashSet::from([Location::Variable(v.clone())]),
        VarArg::Arg(Arg::Register(r)) => HashSet::from([Location::Register(r.clone())]),
        VarArg::Arg(Arg::Deref(_, offset)) => HashSet::from([Location::Stack(*offset)]),
        _ => HashSet::new(),
    }
}

impl From<Reg> for Location {
    fn from(reg: Reg) -> Location {
        Location::Register(reg)
    }
}

impl From<&str> for Location {
    fn from(var: &str) -> Location {
        Location::Variable(var.to_owned())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Variable(v) => write!(f, "{v}"),
            Location::Register(reg) => write!(f, "%{reg}"),
            Location::Stack(offset) => write!(f, "{offset}%Rsp"),
        }
    }
}
