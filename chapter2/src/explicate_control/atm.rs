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
