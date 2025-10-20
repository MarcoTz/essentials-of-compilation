use super::{BuildFlowGraph, BuildInterferenceGraph, Pass};
use crate::CompilerPaths;
use asm::VarProgram;
use register_allocation::{FlowGraph, uncover_live};

pub struct UncoverLive {
    pub prog: VarProgram,
    pub flow_graph: FlowGraph,
}

impl Pass for UncoverLive {
    type Next = BuildInterferenceGraph;
    type Prev = BuildFlowGraph;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Uncover Live"
    }

    fn show_input(&self) -> String {
        self.flow_graph.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let prog = uncover_live(self.prog, self.flow_graph)?;
        Ok(BuildInterferenceGraph { prog })
    }
}
