use super::{Pass, build_graph::Built};
use crate::CompilerPaths;
use assign_homes::{AnnotProg, Coloring, color_graph};
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
    type Error = assign_homes::Error;

    fn description() -> &'static str {
        "Colored Graph"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        let coloring = color_graph(input.graph)?;
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
