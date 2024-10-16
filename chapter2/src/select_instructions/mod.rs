pub mod atm;
pub mod exp;
pub mod prog;
pub mod stmt;

pub trait SelectInstructions {
    type Target;
    fn select_instructions(self) -> Self::Target;
}
