use super::Pass;
use crate::CompilerPaths;
use surface::{Program, Typecheck};

pub struct CheckTypes;

impl Pass for CheckTypes {
    type Input = Program;
    type Output = Program;
    type Error = surface::typecheck::Error;

    fn description() -> &'static str {
        "Typecheck"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        input.check(&mut Default::default())?;
        Ok(input)
    }
}
