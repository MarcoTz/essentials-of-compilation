use super::Pass;
use crate::CompilerPaths;
use lang_x86::PatchInstructions;
use std::convert::Infallible;

pub struct PatchInstrs;

impl Pass for PatchInstrs {
    type Input = lang_x86::Program;
    type Output = lang_x86::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Patch Instructions"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.patch_instructions())
    }
}
