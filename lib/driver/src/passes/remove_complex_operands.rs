use super::{Explicate, Pass, UniquifyVariables};
use crate::CompilerPaths;
use std::convert::Infallible;
use surface::Program;
use surface2monadic::RemoveComplexOperands;

pub struct Rco {
    pub prog: Program,
}

impl Pass for Rco {
    type Next = Explicate;
    type Prev = UniquifyVariables;
    type Error = Infallible;

    fn description() -> &'static str {
        "Remove Complex Operands"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = self.prog.remove_complex_operands(&mut Default::default());
        Ok(Explicate { prog })
    }
}
