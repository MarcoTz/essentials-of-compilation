use super::Pass;
use crate::CompilerPaths;
use assign_homes::{AnnotProg, LocationGraph, build_interference_graph, build_move_graph};
use std::{convert::Infallible, fmt};

pub struct BuildGraph;

#[derive(Debug)]
pub struct Built {
    pub prog: AnnotProg,
    pub interference_graph: LocationGraph,
    pub move_graph: LocationGraph,
}

impl Pass for BuildGraph {
    type Input = AnnotProg;
    type Output = Built;
    type Error = Infallible;

    fn description() -> &'static str {
        "Interference Graph"
    }

    fn run(input: Self::Input, _: &CompilerPaths) -> Result<Self::Output, Self::Error> {
        let interference_graph = build_interference_graph(&input);
        let move_graph = build_move_graph(&input);
        Ok(Built {
            prog: input,
            interference_graph,
            move_graph,
        })
    }
}

impl fmt::Display for Built {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Interference Graph:")?;
        self.interference_graph.fmt(f);
        writeln!(f, "Move GFraph:")?;
        self.move_graph.fmt(f)
    }
}
