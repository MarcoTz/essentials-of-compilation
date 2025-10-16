use super::{Pass, build_flow_graph::FlowProgram};
use crate::CompilerPaths;
use register_allocation::{AnnotProg, uncover_live};

pub struct UncoverLive;

impl Pass for UncoverLive {
    type Input = FlowProgram;
    type Output = AnnotProg;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Uncover Live"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        uncover_live(input.prog, input.graph)
    }
}
