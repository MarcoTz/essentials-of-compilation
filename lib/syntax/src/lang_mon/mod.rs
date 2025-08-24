use std::fmt;

mod atom;
mod expr;

pub use atom::Atom;
pub use expr::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
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
