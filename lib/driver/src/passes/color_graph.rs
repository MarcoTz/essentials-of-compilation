use super::{AssignHomes, BuildInterferenceGraph, Pass};
use crate::CompilerPaths;
use register_allocation::{LiveProg, LocationGraph, color_graph};

pub struct ColorGraph {
    pub prog: LiveProg,
    pub interference_graph: LocationGraph,
    pub move_graph: LocationGraph,
}

impl Pass for ColorGraph {
    type Next = AssignHomes;
    type Prev = BuildInterferenceGraph;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Colored Graph"
    }

    fn show_input(&self) -> String {
        format!(
            "Interfecence Graph:\n{}\n\nMove GFraph:\n{}",
            self.interference_graph, self.move_graph
        )
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let coloring = color_graph(self.interference_graph, self.move_graph)?;
        Ok(AssignHomes {
            prog: self.prog,
            coloring,
        })
    }
}
