use super::Pass;
use crate::CompilerPaths;
use asm::VarProgram;
use register_allocation::FlowGraph;
use std::{convert::Infallible, fmt};

pub struct BuildFlowGraph;

#[derive(Debug)]
pub struct FlowProgram {
    pub prog: VarProgram,
    pub graph: FlowGraph,
}

impl Pass for BuildFlowGraph {
    type Input = VarProgram;
    type Output = FlowProgram;
    type Error = Infallible;

    fn description() -> &'static str {
        "Build Flow Graph"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        let mut graph = FlowGraph::new();
        graph.build(&input);
        Ok(FlowProgram { prog: input, graph })
    }
}

impl fmt::Display for FlowProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.graph.fmt(f)
    }
}
