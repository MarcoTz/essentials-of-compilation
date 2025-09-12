use super::Pass;
use crate::CompilerPaths;
use assign_homes::{AnnotProg, LocationGraph, build_interference_graph};
use std::{convert::Infallible, fmt};

pub struct BuildGraph;

#[derive(Debug)]
pub struct Built {
    pub prog: AnnotProg,
    pub graph: LocationGraph,
}

impl Pass for BuildGraph {
    type Input = AnnotProg;
    type Output = Built;
    type Error = Infallible;

    fn description() -> &'static str {
        "Interference Graph"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        let graph = build_interference_graph(&input);
        Ok(Built { prog: input, graph })
    }
}

impl fmt::Display for Built {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.graph.fmt(f)
    }
}
