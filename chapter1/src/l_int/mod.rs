use exp::Exp;
use std::fmt;
use stmt::Stmt;

pub mod errors;
pub mod eval;
pub mod exp;
pub mod functions;
pub mod stmt;

pub enum UnaryOp {
    Neg,
}

pub enum BinOp {
    Add,
    Sub,
}

pub type Module = Vec<Stmt>;

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOp::Neg => f.write_str("-"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add => f.write_str("+"),
            BinOp::Sub => f.write_str("-"),
        }
    }
}

pub fn is_l_int(md: &Module) -> bool {
    md.iter().all(|stmt| stmt.is_stmt())
}
