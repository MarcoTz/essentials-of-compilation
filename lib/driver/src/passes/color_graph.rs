use super::{Pass, build_interference_graph::Built};
use crate::CompilerPaths;
use register_allocation::{AnnotProg, Coloring, color_graph};
use std::fmt;

pub struct ColorGraph;

#[derive(Debug)]
pub struct Colored {
    pub prog: AnnotProg,
    pub color: Coloring,
}

impl Pass for ColorGraph {
    type Input = Built;
    type Output = Colored;
    type Error = register_allocation::Error;

    fn description() -> &'static str {
        "Colored Graph"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        let coloring = color_graph(input.interference_graph, input.move_graph)?;
        Ok(Colored {
            prog: input.prog,
            color: coloring,
        })
    }
}

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.color.fmt(f)
    }
}
