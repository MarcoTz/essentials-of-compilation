use std::collections::HashMap;

pub mod atm;
pub mod exp;
pub mod ops;
pub mod stmt;
pub mod tail;

pub use atm::Atom;
pub use exp::Exp;
pub use ops::{Cmp, Op};
pub use stmt::Stmt;
pub use tail::Tail;

pub type Label = String;

pub struct Program {
    pub blocks: HashMap<Label, Tail>,
}
