use std::collections::HashSet;

pub mod exp;
pub mod ops;
pub mod prog;

pub type Var = String;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ReduceState {
    used_vars: HashSet<Var>,
}

impl ReduceState {
    pub fn fresh_var(&mut self) -> Var {
        let prefix = "x".to_owned();
        let mut num = 0;
        let mut new_var = prefix.clone() + &num.to_string();

        while self.used_vars.contains(&new_var) {
            num += 1;
            new_var = prefix.clone() + &num.to_string();
        }
        self.used_vars.insert(new_var.clone());
        new_var
    }
}

pub trait RemoveComplexOperands {
    type Target;
    fn remove_complex_operands(self, st: &mut ReduceState) -> Self::Target;
}

#[cfg(test)]
mod reduce_tests {
    use super::ReduceState;
    use std::collections::HashSet;

    fn example_state() -> ReduceState {
        ReduceState {
            used_vars: HashSet::new(),
        }
    }

    #[test]
    fn fresh_var_default() {
        let result = ReduceState::default().fresh_var();
        let expected = "x0";
        assert_eq!(result, expected)
    }

    #[test]
    fn fresh_var_ex() {
        let mut st = example_state();
        let result1 = st.fresh_var();
        let expected1 = "x0";
        let result2 = st.fresh_var();
        let expected2 = "x1";
        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2)
    }
}
