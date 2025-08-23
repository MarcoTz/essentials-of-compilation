use std::fmt;

mod expr;
mod ops;

pub use expr::Expression;
pub use ops::{BinaryOperation, UnaryOperation};

#[derive(Debug)]
pub struct Program {
    exp: Expression,
}

impl Program {
    pub fn new(exp: Expression) -> Program {
        Program { exp }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.exp.fmt(f)
    }
}
