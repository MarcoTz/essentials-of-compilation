use super::{AssignHomes, GeneratePreludeConclusion, Pass};
use crate::CompilerPaths;
use asm::{PatchInstructions, Program};
use std::convert::Infallible;

pub struct PatchInstrs {
    pub prog: Program,
}

impl Pass for PatchInstrs {
    type Next = GeneratePreludeConclusion;
    type Prev = AssignHomes;
    type Error = Infallible;

    fn description() -> &'static str {
        "Patch Instructions"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = self.prog.patch_instructions();
        Ok(GeneratePreludeConclusion { prog })
    }
}
