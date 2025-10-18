use super::Pass;
use crate::CompilerPaths;
use std::convert::Infallible;
use surface2monadic::RemoveComplexOperands;

pub struct Rco;

impl Pass for Rco {
    type Input = surface::Program;
    type Output = monadic::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Remove Complex Operands"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(input.remove_complex_operands(&mut Default::default()))
    }
}
