pub mod arg;
pub mod instr;
pub mod prog;

use chapter2::x86_var::{Instr, Var};
use std::collections::{HashMap, HashSet};

pub trait UncoverLive {
    type Target;
    fn uncover(&self) -> Self::Target;
}

pub type LiveMap = HashMap<Instr, HashSet<Var>>;
