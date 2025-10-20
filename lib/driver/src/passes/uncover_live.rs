use super::{BuildInterferenceGraph, Pass, SelectInstrs};
use crate::CompilerPaths;
use asm::VarProgram;
use register_allocation::uncover_live;

pub struct UncoverLive {
    pub prog: VarProgram,
}

impl Pass for UncoverLive {
    type Next = BuildInterferenceGraph;
    type Prev = SelectInstrs;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Uncover Live"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = uncover_live(self.prog)?;
        Ok(BuildInterferenceGraph { prog })
    }
}
