pub mod instr;
pub mod prog;

pub trait PatchInstructions {
    type Target;
    fn patch(self) -> Self::Target;
}
