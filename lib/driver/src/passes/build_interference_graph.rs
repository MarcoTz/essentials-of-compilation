use super::{ColorGraph, Pass, UncoverLive};
use crate::CompilerPaths;
use register_allocation::{LiveProg, build_interference_graph, build_move_graph};
use std::convert::Infallible;

pub struct BuildInterferenceGraph {
    pub prog: LiveProg,
}

impl Pass for BuildInterferenceGraph {
    type Next = ColorGraph;
    type Prev = UncoverLive;
    type Error = Infallible;

    fn description() -> &'static str {
        "Interference Graph"
    }

    fn show_input(&self) -> String {
        self.prog.to_string()
    }

    fn run(self, _: &CompilerPaths) -> Result<Self::Next, Self::Error> {
        let interference_graph = build_interference_graph(&self.prog);
        let move_graph = build_move_graph(&self.prog);
        Ok(ColorGraph {
            prog: self.prog,
            interference_graph,
            move_graph,
        })
    }
}
