use super::Pass;
use crate::CompilerPaths;
use assign_homes::{AnnotProg, uncover_live};
use syntax::x86::VarProgram;

pub struct UncoverLive;

impl Pass for UncoverLive {
    type Input = VarProgram;
    type Output = AnnotProg;
    type Error = assign_homes::Error;

    fn description() -> &'static str {
        "Uncover Live"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        uncover_live(input)
    }
}
