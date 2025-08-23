use super::SelectInstructions;
use crate::{c_var, x86_var};

impl SelectInstructions for c_var::Atm {
    type Target = x86_var::Arg;
    fn select_instructions(self) -> Self::Target {
        match self {
            c_var::Atm::Int(i) => x86_var::Arg::Immediate(i),
            c_var::Atm::Var(v) => x86_var::Arg::Var(v),
        }
    }
}
