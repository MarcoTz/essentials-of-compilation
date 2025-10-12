use super::Pass;
use crate::CompilerPaths;
use explicate_control::{Error, explicate_control};
use syntax::{lang_c, lang_mon};

pub struct ExplicateControl;

impl Pass for ExplicateControl {
    type Input = lang_mon::Program;
    type Output = lang_c::Program;
    type Error = Error;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        explicate_control(input)
    }
}
