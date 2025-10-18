use super::{Pass, color_graph::Colored};
use crate::CompilerPaths;
use asm::Program;
use register_allocation::assign_homes;

pub struct AssignHomes;

impl Pass for AssignHomes {
    type Input = Colored;
    type Output = Program;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Assign Homes"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        assign_homes(input.prog, input.color)
    }
}
