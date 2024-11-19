use std::fmt;

pub mod atm;
pub mod exp;
pub mod prog;
pub mod stmt;
pub mod tail;

pub use atm::Atm;
pub use exp::Exp;
pub use prog::Prog;
pub use stmt::Stmt;
pub use tail::Tail;

pub type Var = String;
pub type Label = String;

#[derive(Debug)]
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

#[derive(Debug)]
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
