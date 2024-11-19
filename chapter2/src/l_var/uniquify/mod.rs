use super::syntax::Var;
use std::collections::HashSet;

pub struct UniqueState {
    pub used_vars: HashSet<Var>,
}

pub trait Uniquify {
    type Target;
    fn uniquify(self, st: &mut UniqueState) -> Self::Target;
}
