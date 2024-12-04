use super::{ReduceState, RemoveComplexOperands};
use crate::{l_var::syntax as l_var, l_var_reduced};

impl RemoveComplexOperands for l_var::Program {
    type Target = l_var_reduced::Program;
    fn remove_complex_operands(self, st: &mut ReduceState) -> Self::Target {
        l_var_reduced::Program {
            exp: self.exp.remove_complex_operands(st),
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{l_var, l_var_reduced, ReduceState, RemoveComplexOperands};

    #[test]
    fn reduce_prog() {
        let result = l_var::Program {
            exp: l_var::Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(l_var::Exp::InputInt),
                in_term: Box::new(l_var::Exp::Assign {
                    name: "y".to_owned(),
                    bound_term: Box::new(l_var::Exp::InputInt),
                    in_term: Box::new(l_var::Exp::BinOp {
                        exp1: Box::new(l_var::Exp::Name("x".to_owned())),
                        exp2: Box::new(l_var::Exp::Name("y".to_owned())),
                        op: l_var::BinOp::Add,
                    }),
                }),
            },
        }
        .remove_complex_operands(&mut ReduceState::default());
        let expected = l_var_reduced::Program {
            exp: l_var_reduced::Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(l_var_reduced::Exp::InputInt),
                in_term: Box::new(l_var_reduced::Exp::Assign {
                    name: "y".to_owned(),
                    bound_term: Box::new(l_var_reduced::Exp::InputInt),
                    in_term: Box::new(l_var_reduced::Exp::BinOp {
                        exp1: l_var_reduced::Atm::Var("x".to_owned()),
                        exp2: l_var_reduced::Atm::Var("y".to_owned()),
                        op: l_var_reduced::BinOp::Add,
                    }),
                }),
            },
        };
        assert_eq!(result, expected)
    }
}
