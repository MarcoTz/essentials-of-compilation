use super::{Pass, SelectInstrs, UncoverLive};
use crate::CompilerPaths;
use asm::VarProgram;
use register_allocation::FlowGraph;
use std::convert::Infallible;

pub struct BuildFlowGraph {
    pub prog: VarProgram,
}

impl Pass for BuildFlowGraph {
    type Next = UncoverLive;
    type Prev = SelectInstrs;
    type Error = Infallible;

    fn description() -> &'static str {
        "Build Flow Graph"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let mut graph = FlowGraph::new();
        graph.build(&self.prog);
        Ok(UncoverLive {
            prog: self.prog,
            flow_graph: graph,
        })
    }
}
