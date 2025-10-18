use super::Pass;
use crate::CompilerPaths;
use monadic2core::{Error, explicate_control};

pub struct Explicate;

impl Pass for Explicate {
    type Input = monadic::Program;
    type Output = core::Program;
    type Error = Error;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        explicate_control(input)
    }
}
