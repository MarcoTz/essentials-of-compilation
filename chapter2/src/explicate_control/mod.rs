pub mod atm;
pub mod exp;
pub mod prog;

use crate::{c_var, l_var_reduced};

pub trait ExplicateControl {
    type Target;
    fn explicate_control(self) -> Self::Target;
}

impl ExplicateControl for l_var_reduced::ops::UnaryOp {
    type Target = c_var::UnaryOp;
    fn explicate_control(self) -> Self::Target {
        match self {
            l_var_reduced::UnaryOp::Neg => c_var::UnaryOp::Neg,
        }
    }
}

impl ExplicateControl for l_var_reduced::ops::BinOp {
    type Target = c_var::BinOp;
    fn explicate_control(self) -> Self::Target {
        match self {
            l_var_reduced::BinOp::Add => c_var::BinOp::Add,
            l_var_reduced::BinOp::Sub => c_var::BinOp::Sub,
        }
    }
}

#[cfg(test)]
mod op_tests {
    use super::{c_var, l_var_reduced, ExplicateControl};

    #[test]
    fn explicate_unary() {
        let result = l_var_reduced::UnaryOp::Neg.explicate_control();
        let expected = c_var::UnaryOp::Neg;
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_add() {
        let result = l_var_reduced::BinOp::Add.explicate_control();
        let expected = c_var::BinOp::Add;
        assert_eq!(result, expected)
    }

    #[test]
    fn explicate_sub() {
        let result = l_var_reduced::BinOp::Sub.explicate_control();
        let expected = c_var::BinOp::Sub;
        assert_eq!(result, expected)
    }
}
