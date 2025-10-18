use super::Pass;
use crate::CompilerPaths;
use core2asm::SelectInstructions;
use std::convert::Infallible;

pub struct SelectInstrs;

impl Pass for SelectInstrs {
    type Input = core::Program;
    type Output = asm::VarProgram;
    type Error = Infallible;

    fn description() -> &'static str {
        "Select Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.select_instructions(()))
    }
}
