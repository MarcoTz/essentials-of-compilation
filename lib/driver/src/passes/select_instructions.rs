use super::Pass;
use crate::CompilerPaths;
use core2lang_x86var::SelectInstructions;
use std::convert::Infallible;

pub struct SelectInstrs;

impl Pass for SelectInstrs {
    type Input = core::Program;
    type Output = lang_x86::VarProgram;
    type Error = Infallible;

    fn description() -> &'static str {
        "Select Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.select_instructions(()))
    }
}
