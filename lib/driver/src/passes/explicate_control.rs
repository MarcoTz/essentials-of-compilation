use super::{Pass, Rco, SelectInstrs};
use crate::CompilerPaths;
use monadic::Program;
use monadic2core::{Error, explicate_control};

pub struct Explicate {
    pub prog: Program,
}

impl Pass for Explicate {
    type Next = SelectInstrs;
    type Prev = Rco;
    type Error = Error;

    fn description() -> &'static str {
        "Explicate Control"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = explicate_control(self.prog)?;
        Ok(SelectInstrs { prog })
    }
}
