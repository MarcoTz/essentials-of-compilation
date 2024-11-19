use std::collections::HashSet;

pub mod atm;
pub mod exp;
pub mod ops;

pub use atm::Atm;
pub use exp::Exp;
pub use ops::{BinOp, UnaryOp};

pub type Var = String;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub exp: Exp,
}

pub trait UsedVars {
    fn used_vars(&self) -> HashSet<Var>;
}

impl UsedVars for Program {
    fn used_vars(&self) -> HashSet<Var> {
        self.exp.used_vars()
    }
}
