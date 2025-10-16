use super::Pass;
use crate::CompilerPaths;
use lang_mon2lang_c::{Error, ExplicateControl};

pub struct Explicate;

impl Pass for Explicate {
    type Input = lang_mon::Program;
    type Output = lang_c::Program;
    type Error = Error;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        input.explicate_control(&mut Default::default())
    }
}
