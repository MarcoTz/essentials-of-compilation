use super::{Pass, color_graph::Colored};
use crate::CompilerPaths;
use assign_homes::assign_homes;
use syntax::x86::Program;

pub struct AssignHomes;

impl Pass for AssignHomes {
    type Input = Colored;
    type Output = Program;
    type Error = assign_homes::Error;

    fn description() -> &'static str {
        "Assign Homes"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        assign_homes(input.prog, input.color)
    }
}
