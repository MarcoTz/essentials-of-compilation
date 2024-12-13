pub mod arg;
pub mod instr;
pub mod prog;

use crate::{
    x86_var::{Arg, Instr},
    Var,
};
use std::collections::{HashMap, HashSet};

pub trait UncoverLive {
    type Target;
    fn uncover(&self) -> Self::Target;
}

pub type LiveMap = HashMap<Instr<Arg>, HashSet<Var>>;
