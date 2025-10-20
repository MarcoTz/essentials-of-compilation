use super::{Assemble, Pass, PatchInstrs};
use crate::CompilerPaths;
use asm::{Program, generate_prelude_conclusion};
use std::convert::Infallible;

pub struct GeneratePreludeConclusion {
    pub prog: Program,
}

impl Pass for GeneratePreludeConclusion {
    type Next = Assemble;
    type Prev = PatchInstrs;
    type Error = Infallible;

    fn description() -> &'static str {
        "Generate Prelude and Conclusion"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = generate_prelude_conclusion(self.prog);
        Ok(Assemble { prog })
    }
}
