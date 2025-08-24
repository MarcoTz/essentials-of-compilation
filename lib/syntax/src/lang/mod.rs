use std::{collections::HashSet, fmt};

mod expr;

pub use expr::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub exp: Expression,
}

impl Program {
    pub fn new(exp: Expression) -> Program {
        Program { exp }
    }

    pub fn used_vars(&self) -> HashSet<String> {
        self.exp.used_vars()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.exp.fmt(f)
    }
}
