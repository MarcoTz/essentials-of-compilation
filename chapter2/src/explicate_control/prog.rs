use super::ExplicateControl;
use crate::{c_var, l_var_reduced};
use std::collections::HashMap;

impl ExplicateControl for l_var_reduced::Program {
    type Target = c_var::Program;
    fn explicate_control(self) -> Self::Target {
        c_var::Program {
            blocks: HashMap::from([("start".to_owned(), self.exp.explicate_control())]),
        }
    }
}
