pub mod arg;
pub mod instr;
pub mod prog;

use crate::Var;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssignState {
    pub stack_size: usize,
    pub stack_vars: HashMap<Var, i64>,
}

pub trait AssignHomes {
    type Target;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target;
}
