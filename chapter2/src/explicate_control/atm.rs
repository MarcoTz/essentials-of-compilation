use super::ExplicateControl;
use crate::{c_var, l_var_reduced};

impl ExplicateControl for l_var_reduced::Atm {
    type Target = c_var::Atm;
    fn explicate_control(self) -> Self::Target {
        match self {
            l_var_reduced::Atm::Int(i) => c_var::Atm::Int(i),
            l_var_reduced::Atm::Var(v) => c_var::Atm::Var(v),
        }
    }
}

#[cfg(test)]
mod atm_tests {
    use super::{c_var, l_var_reduced, ExplicateControl};

    #[test]
    fn explicate_int() {
        let result = l_var_reduced::Atm::Int(2).explicate_control();
        let expected = c_var::Atm::Int(2);
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_var() {
        let result = l_var_reduced::Atm::Var("x".to_owned()).explicate_control();
        let expected = c_var::Atm::Var("x".to_owned());
        assert_eq!(result, expected)
    }
}
