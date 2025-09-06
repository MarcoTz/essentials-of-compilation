use crate::edge::Edge;
use std::{collections::HashSet, fmt};
use uncover_live::Location;

#[derive(PartialEq, Eq, Debug)]
pub struct InterferenceGraph {
    verts: HashSet<Location>,
    edges: HashSet<Edge>,
}

impl InterferenceGraph {
    pub fn new() -> InterferenceGraph {
        InterferenceGraph {
            verts: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_vert(&mut self, v: Location) {
        self.verts.insert(v);
    }

    pub fn add_edge(&mut self, left: Location, right: Location) {
        self.add_vert(left.clone());
        self.add_vert(right.clone());
        self.edges.insert(Edge::new(left, right));
    }
}

impl fmt::Display for InterferenceGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vert in self.verts.iter() {
            writeln!(f, "{vert}")?;
        }
        writeln!(f,)?;
        for edg in self.edges.iter() {
            writeln!(f, "{edg}")?;
        }
        Ok(())
    }
}

impl Default for InterferenceGraph {
    fn default() -> InterferenceGraph {
        InterferenceGraph::new()
    }
}
