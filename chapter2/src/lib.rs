pub mod l_var;
pub mod l_var_reduced;

use std::fmt;

pub type Var = String;

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => f.write_str("-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add => f.write_str("+"),
            BinOp::Sub => f.write_str("-"),
        }
    }
}
