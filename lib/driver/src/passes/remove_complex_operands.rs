use super::Pass;
use crate::CompilerPaths;
use lang2lang_mon::RemoveComplexOperands;
use std::convert::Infallible;

pub struct Rco;

impl Pass for Rco {
    type Input = lang::Program;
    type Output = lang_mon::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Remove Complex Operands"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.remove_complex_operands(&mut Default::default()))
    }
}
