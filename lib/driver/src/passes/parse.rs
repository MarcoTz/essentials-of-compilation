use super::Pass;
use crate::CompilerPaths;
use parser::parse_program;
use surface::Program;

pub struct Parse;

impl Pass for Parse {
    type Input = String;
    type Output = Program;
    type Error = parser::Error;

    fn description() -> &'static str {
        "Parse"
    }

    fn run(source: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        parse_program(&source)
    }
}
