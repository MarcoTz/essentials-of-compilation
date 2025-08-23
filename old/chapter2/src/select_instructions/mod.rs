pub mod atm;
pub mod prog;
pub mod stmt;
pub mod tail;

pub trait SelectInstructions {
    type Target;
    fn select_instructions(self) -> Self::Target;
}
