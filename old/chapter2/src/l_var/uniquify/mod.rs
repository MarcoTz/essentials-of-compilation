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

#[cfg(test)]
mod fresh_var_tests {
    use super::UniqueState;

    #[test]
    fn fresh_empty() {
        let result = UniqueState::default().fresh_var(&"x".to_owned());
        let expected = "x0";
        assert_eq!(result, expected)
    }

    #[test]
    fn fresh_nonempty() {
        let mut st = UniqueState::default();
        st.var_subst.insert("x3".to_owned(), "y".to_owned());
        st.var_subst.insert("x".to_owned(), "x0".to_owned());
        st.var_subst.insert("y".to_owned(), "x1".to_owned());
        let result = st.fresh_var(&"x2".to_owned());
        let expected = "x3";
        assert_eq!(result, expected)
    }
}
