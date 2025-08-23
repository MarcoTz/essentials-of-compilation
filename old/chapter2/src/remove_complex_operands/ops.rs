use super::{ReduceState, RemoveComplexOperands};
use crate::{l_var::syntax as l_var, l_var_reduced};

impl RemoveComplexOperands for l_var::UnaryOp {
    type Target = l_var_reduced::UnaryOp;
    fn remove_complex_operands(self, _: &mut ReduceState) -> Self::Target {
        match self {
            l_var::UnaryOp::Neg => l_var_reduced::UnaryOp::Neg,
        }
    }
}

impl RemoveComplexOperands for l_var::BinOp {
    type Target = l_var_reduced::BinOp;
    fn remove_complex_operands(self, _: &mut ReduceState) -> Self::Target {
        match self {
            l_var::BinOp::Add => l_var_reduced::BinOp::Add,
            l_var::BinOp::Sub => l_var_reduced::BinOp::Sub,
        }
    }
}

#[cfg(test)]
mod ops_tests {
    use super::{l_var, l_var_reduced, RemoveComplexOperands};

    #[test]
    fn remove_unary() {
        let result = l_var::UnaryOp::Neg.remove_complex_operands(&mut Default::default());
        let expected = l_var_reduced::UnaryOp::Neg;
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_add() {
        let result = l_var::BinOp::Add.remove_complex_operands(&mut Default::default());
        let expected = l_var_reduced::BinOp::Add;
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_sub() {
        let result = l_var::BinOp::Sub.remove_complex_operands(&mut Default::default());
        let expected = l_var_reduced::BinOp::Sub;
        assert_eq!(result, expected)
    }
}
