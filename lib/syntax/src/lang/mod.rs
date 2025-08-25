use std::{collections::HashSet, fmt};

mod expr;

pub use expr::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub exps: Vec<Expression>,
}

impl Program {
    pub fn new(exps: Vec<Expression>) -> Program {
        Program { exps }
    }

    pub fn used_vars(&self) -> HashSet<String> {
        let mut used = HashSet::new();
        for exp in self.exps.iter() {
            used.extend(exp.used_vars().into_iter());
        }
        used
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
