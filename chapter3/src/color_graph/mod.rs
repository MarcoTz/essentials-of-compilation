use super::interference_graph::InterferenceGraph;
use chapter2::x86_var::{Reg, Var};
use std::collections::{HashMap, HashSet};

pub mod assign_registers;

use super::errors::Error;

pub type Color = i32;
pub type Coloring = HashMap<Var, Color>;
pub type RegisterAssignment = HashMap<Var, Reg>;

pub fn saturation(vert: &Var, graph: &InterferenceGraph, coloring: &Coloring) -> HashSet<Color> {
    let mut colors = HashSet::new();
    let adj = graph.adjacent(vert);
    for v in adj.iter() {
        match coloring.get(v) {
            None => continue,
            Some(c) => {
                colors.insert(*c);
            }
        }
    }
    colors
}

pub fn color_graph(graph: InterferenceGraph) -> Result<Coloring, Error> {
    let mut coloring = HashMap::new();
    let mut verts = graph.vertices.clone();
    while !verts.is_empty() {
        let saturations = verts
            .iter()
            .map(|v| (v.clone(), saturation(v, &graph, &coloring)));
        let (next_vert, next_saturation) = saturations
            .max_by(|(_, color1), (_, color2)| color1.len().cmp(&color2.len()))
            .unwrap();
        let vert_color = (0..=11)
            .filter(|i| !next_saturation.contains(i))
            .min()
            .ok_or(Error::RegistersFull)?;
        coloring.insert(next_vert.clone(), vert_color);
        verts.remove(&next_vert);
    }
    Ok(coloring)
}

#[cfg(test)]
mod color_tests {
    use super::{color_graph, InterferenceGraph};
    use std::collections::HashSet;

    #[test]
    fn color_ex() {
        let graph = InterferenceGraph {
            vertices: HashSet::from([
                "tmp_0".to_owned(),
                "tmp_1".to_owned(),
                "y".to_owned(),
                "z".to_owned(),
                "x".to_owned(),
                "w".to_owned(),
                "v".to_owned(),
            ]),
            edges: HashSet::from([
                ("tmp_0".to_owned(), "tmp_1".to_owned()),
                ("tmp_0".to_owned(), "z".to_owned()),
                ("y".to_owned(), "z".to_owned()),
                ("z".to_owned(), "w".to_owned()),
                ("y".to_owned(), "w".to_owned()),
                ("x".to_owned(), "w".to_owned()),
                ("w".to_owned(), "v".to_owned()),
            ]),
        };
        let result = color_graph(graph);
        assert!(result.is_ok())
    }
}
