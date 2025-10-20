use super::{CheckTypes, Pass, Rco};
use crate::CompilerPaths;
use std::convert::Infallible;
use surface::{Program, Uniquify};

pub struct UniquifyVariables {
    pub prog: Program,
}

impl Pass for UniquifyVariables {
    type Next = Rco;
    type Prev = CheckTypes;
    type Error = Infallible;

    fn description() -> &'static str {
        "Uniquify"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = self.prog.uniquify(&mut Default::default());
        Ok(Rco { prog })
    }
}
