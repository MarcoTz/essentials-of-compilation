use super::Pass;
use crate::CompilerPaths;
use lang_x86::Program;
use lang_x86::generate_prelude_conclusion;
use std::convert::Infallible;

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
