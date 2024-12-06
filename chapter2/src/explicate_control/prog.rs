use super::ExplicateControl;
use crate::{c_var, l_var_reduced};
use std::collections::HashMap;

impl ExplicateControl for l_var_reduced::Program {
    type Target = c_var::Program;
    fn explicate_control(self) -> Self::Target {
        c_var::Program {
            blocks: HashMap::from([("start".to_owned(), self.exp.explicate_control())]),
            types: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{c_var, l_var_reduced, ExplicateControl};
    use std::collections::HashMap;

    #[test]
    fn explicate_prog() {
        let result = l_var_reduced::Program {
            exp: l_var_reduced::Exp::InputInt,
        }
        .explicate_control();
        let expected = c_var::Program {
            blocks: HashMap::from([("start".to_owned(), c_var::Tail::Return(c_var::Exp::Read))]),
            types: HashMap::new(),
        };
        assert_eq!(result, expected)
    }
}
