use super::Pass;
use crate::CompilerPaths;
use explicate_control::explicate_control;
use std::convert::Infallible;
use syntax::{lang_c, lang_mon};

pub struct ExplicateControl;

impl Pass for ExplicateControl {
    type Input = lang_mon::Program;
    type Output = lang_c::Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(explicate_control(input))
    }
}
