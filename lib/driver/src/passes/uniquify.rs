use super::Pass;
use crate::CompilerPaths;
use std::convert::Infallible;
use surface::{Program, Uniquify};

pub struct UniquifyVariables;

impl Pass for UniquifyVariables {
    type Input = Program;
    type Output = Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Uniquify"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.uniquify(&mut Default::default()))
    }
}
