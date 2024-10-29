use super::uncover_live::LiveMap;
use chapter2::Var;
use std::collections::HashSet;

pub mod instr;
pub mod prog;

#[derive(Default)]
pub struct InterferenceGraph {
    pub vertices: HashSet<Var>,
    pub edges: HashSet<(Var, Var)>,
}

impl InterferenceGraph {
    pub fn add_edge(&mut self, start: Var, end: Var) {
        if !self.vertices.contains(&start) {
            self.vertices.insert(start.clone());
        }

        if !self.vertices.contains(&end) {
            self.vertices.insert(end.clone());
        }

        self.edges.insert((start, end));
    }
}

pub trait BuildGraph {
    fn build(&self, graph: &mut InterferenceGraph, live: &LiveMap);
}
