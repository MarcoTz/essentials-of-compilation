use super::syntax::Var;
use std::collections::HashMap;

pub mod exp;
pub mod prog;

#[derive(Default)]
pub struct UniqueState {
    pub var_subst: HashMap<Var, Var>,
}

impl UniqueState {
    pub fn fresh_var(&self, used_name: &Var) -> Var {
        let prefix = "x".to_owned();
        let mut new_ind = 0;
        let mut new_var = prefix.clone() + &new_ind.to_string();
        while new_var == *used_name || self.var_subst.values().any(|var| **var == new_var) {
            new_ind += 1;
            new_var = prefix.clone() + &new_ind.to_string();
        }
        new_var
    }
}

pub trait Uniquify {
    type Target;
    fn uniquify(self, st: &mut UniqueState) -> Self::Target;
}
