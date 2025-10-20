use super::{Parse, Pass, UniquifyVariables};
use crate::CompilerPaths;
use surface::{Program, Typecheck};

pub struct CheckTypes {
    pub prog: Program,
}

impl Pass for CheckTypes {
    type Next = UniquifyVariables;
    type Prev = Parse;
    type Error = surface::typecheck::Error;

    fn description() -> &'static str {
        "Typecheck"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        self.prog.check(&mut Default::default())?;
        Ok(UniquifyVariables { prog: self.prog })
    }
}
