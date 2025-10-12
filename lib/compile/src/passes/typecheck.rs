use super::Pass;
use crate::CompilerPaths;
use syntax::lang::Program;
use typecheck::typecheck;

pub struct Typecheck;

impl Pass for Typecheck {
    type Input = Program;
    type Output = Program;
    type Error = typecheck::Error;

    fn description() -> &'static str {
        "Typecheck"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        typecheck(&input)?;
        Ok(input)
    }
}
