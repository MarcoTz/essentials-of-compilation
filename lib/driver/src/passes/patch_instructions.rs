use super::Pass;
use crate::CompilerPaths;
use asm::PatchInstructions;
use std::convert::Infallible;

pub struct PatchInstrs;

impl Pass for PatchInstrs {
    type Input = asm::Program;
    type Output = asm::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Patch Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.patch_instructions())
    }
}
