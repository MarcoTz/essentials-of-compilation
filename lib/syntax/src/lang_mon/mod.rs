use std::fmt;

mod atom;
mod expr;

pub use atom::Atom;
pub use expr::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub exps: Vec<Expression>,
}

impl Program {
    pub fn new(exps: Vec<Expression>) -> Program {
        Program { exps }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for exp in self.exps.iter() {
            writeln!(f, "{exp};")?;
        }
        Ok(())
    }
}
