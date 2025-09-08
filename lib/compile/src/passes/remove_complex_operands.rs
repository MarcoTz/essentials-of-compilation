use super::Pass;
use crate::CompilerPaths;
use remove_complex_operands::remove_complex_operands;
use std::convert::Infallible;
use syntax::{lang, lang_mon};

pub struct RemoveComplexOperands;

impl Pass for RemoveComplexOperands {
    type Input = lang::Program;
    type Output = lang_mon::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Remove Complex Operands"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(remove_complex_operands(input))
    }
}
