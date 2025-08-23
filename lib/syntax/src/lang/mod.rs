use std::fmt;

mod expr;
mod ops;

pub use expr::Expression;
pub use ops::{BinaryOperation, UnaryOperation};

#[derive(Debug, Clone)]
pub struct Program {
    pub exp: Expression,
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
