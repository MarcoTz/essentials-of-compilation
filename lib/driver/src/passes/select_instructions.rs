use super::{Explicate, Pass, UncoverLive};
use crate::CompilerPaths;
use core::Program;
use core2asm::SelectInstructions;
use std::convert::Infallible;

pub struct SelectInstrs {
    pub prog: Program,
}

impl Pass for SelectInstrs {
    type Next = UncoverLive;
    type Prev = Explicate;
    type Error = Infallible;

    fn description() -> &'static str {
        "Select Instructions"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = self.prog.select_instructions(());
        Ok(UncoverLive { prog })
    }
}
