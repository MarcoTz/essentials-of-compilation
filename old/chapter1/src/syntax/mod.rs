pub mod exp;
pub mod ops;

pub use exp::Exp;
pub use ops::{BinOp, UnaryOp};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub exp: Exp,
}
