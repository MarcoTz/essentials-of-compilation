use super::Var;
use std::collections::HashSet;
pub mod syntax;

pub struct ReduceState {
    vars: HashSet<Var>,
}

impl ReduceState {
    pub fn fresh_var(&mut self) -> Var {
        let prefix = "x".to_owned();
        let mut i = 0;
        while self.vars.contains(&(prefix.clone() + &i.to_string())) {
            i += 1;
        }
        prefix.clone() + &i.to_string()
    }
}

pub trait Reduce {
    type Target;
    fn reduce(self, st: &mut ReduceState) -> Self::Target;
}
