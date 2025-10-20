use super::{CheckTypes, Done, Pass};
use crate::CompilerPaths;
use parser::parse_program;

pub struct Parse {
    pub source: String,
}

impl Pass for Parse {
    type Next = CheckTypes;
    type Prev = Done;
    type Error = parser::Error;

    fn description() -> &'static str {
        "Parse"
    }

    fn show_input(&self) -> String {
        self.source.clone()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = parse_program(&self.source)?;
        Ok(CheckTypes { prog })
    }
}
