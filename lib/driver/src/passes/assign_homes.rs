use super::{ColorGraph, Pass, PatchInstrs};
use crate::CompilerPaths;
use register_allocation::{Coloring, LiveProg, assign_homes};

pub struct AssignHomes {
    pub prog: LiveProg,
    pub coloring: Coloring,
}

impl Pass for AssignHomes {
    type Next = PatchInstrs;
    type Prev = ColorGraph;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Assign Homes"
    }

    fn show_input(&self) -> String {
        self.coloring.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let out = assign_homes(self.prog, self.coloring)?;
        Ok(PatchInstrs { prog: out })
    }
}
