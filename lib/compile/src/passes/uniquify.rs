use super::Pass;
use crate::CompilerPaths;
use std::convert::Infallible;
use syntax::lang::Program;
use uniquify::uniquify;

pub struct Uniquify;

impl Pass for Uniquify {
    type Input = Program;
    type Output = Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Uniquify"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(uniquify(input))
    }
}
