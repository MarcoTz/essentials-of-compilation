use super::{Reduce, ReduceState};
use crate::{l_var::syntax as l_var, l_var_reduced};

impl Reduce for l_var::UnaryOp {
    type Target = l_var_reduced::UnaryOp;
    fn reduce(self, _: &mut ReduceState) -> Self::Target {
        match self {
            l_var::UnaryOp::Neg => l_var_reduced::UnaryOp::Neg,
        }
    }
}

impl Reduce for l_var::BinOp {
    type Target = l_var_reduced::BinOp;
    fn reduce(self, _: &mut ReduceState) -> Self::Target {
        match self {
            l_var::BinOp::Add => l_var_reduced::BinOp::Add,
            l_var::BinOp::Sub => l_var_reduced::BinOp::Sub,
        }
    }
}
