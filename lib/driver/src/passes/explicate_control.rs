use super::Pass;
use crate::CompilerPaths;
use monadic2lang_c::{Error, ExplicateControl};

pub struct Explicate;

impl Pass for Explicate {
    type Input = monadic::Program;
    type Output = core::Program;
    type Error = Error;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        input.explicate_control(&mut Default::default())
    }
}
