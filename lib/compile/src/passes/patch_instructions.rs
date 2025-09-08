use super::Pass;
use crate::CompilerPaths;
use patch_instructions::patch_instructions;
use std::convert::Infallible;
use syntax::x86::Program;

pub struct PatchInstructions;

impl Pass for PatchInstructions {
    type Input = Program;
    type Output = Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Patch Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(patch_instructions(input))
    }
}
