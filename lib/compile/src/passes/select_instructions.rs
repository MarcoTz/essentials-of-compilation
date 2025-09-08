use super::Pass;
use crate::CompilerPaths;
use select_instructions::select_instructions;
use std::convert::Infallible;
use syntax::{lang_c, x86};

pub struct SelectInstructions;

impl Pass for SelectInstructions {
    type Input = lang_c::Program;
    type Output = x86::VarProgram;
    type Error = Infallible;

    fn description() -> &'static str {
        "Select Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(select_instructions(input))
    }
}
