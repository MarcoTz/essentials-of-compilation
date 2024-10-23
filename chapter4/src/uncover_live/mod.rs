pub mod arg;
pub mod instr;
pub mod prog;

pub trait UncoverLive {
    type Target;
    fn uncover(&self) -> Self::Target;
}
