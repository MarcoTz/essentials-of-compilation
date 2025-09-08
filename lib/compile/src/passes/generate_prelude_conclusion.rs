use super::Pass;
use crate::CompilerPaths;
use generate_prelude_conclusion::generate_prelude_conclusion;
use std::convert::Infallible;
use syntax::x86::Program;

pub struct GeneratePreludeConclusion;

impl Pass for GeneratePreludeConclusion {
    type Input = Program;
    type Output = Program;
    type Error = Infallible;

    fn description() -> &'static str {
        "Generate Prelude and Conclusion"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        Ok(generate_prelude_conclusion(input))
    }
}
