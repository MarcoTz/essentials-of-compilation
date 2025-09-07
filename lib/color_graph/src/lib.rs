use interference_graph::InterferenceGraph;
use std::collections::{HashMap, HashSet};
use syntax::x86::{Arg, Reg};
use uncover_live::Location;

type Color = i64;
pub type Coloring = HashMap<Location, Color>;

pub fn color_to_arg(color: Color) -> Arg {
    match color {
        0 => Reg::Rcx.into(),
        1 => Reg::Rdx.into(),
        2 => Reg::Rsi.into(),
        3 => Reg::Rdi.into(),
        4 => Reg::R8.into(),
        5 => Reg::R9.into(),
        6 => Reg::R10.into(),
        7 => Reg::Rbx.into(),
        8 => Reg::R12.into(),
        9 => Reg::R13.into(),
        10 => Reg::R14.into(),
        -1 => Reg::Rax.into(),
        -2 => Reg::Rsp.into(),
        -3 => Reg::Rbp.into(),
        -4 => Reg::R11.into(),
        -5 => Reg::R15.into(),
        i if i > 10 => Arg::Deref(Reg::Rbp, -8 * (i - 10)),
        _ => panic!("Cannot assign color {color}"),
    }
}

pub fn color_graph(graph: &InterferenceGraph) -> Coloring {
    let mut vert_set = graph.verts.clone();
    let mut coloring = empty_coloring();
    while !vert_set.is_empty() {
        let next = vert_set
            .iter()
            .min_by(|v1, v2| {
                saturation(graph, v1, &coloring)
                    .len()
                    .cmp(&saturation(graph, v2, &coloring).len())
            })
            .unwrap()
            .clone();
        vert_set.remove(&next);
        let adjacent_colors: Vec<Color> = graph
            .adjacent(&next)
            .iter()
            .filter_map(|c| coloring.get(c))
            .copied()
            .collect();
        let mut next_color = 0;
        while adjacent_colors.contains(&next_color) {
            next_color += 1;
        }
        coloring.insert(next, next_color);
    }
    coloring
}

fn empty_coloring() -> HashMap<Location, Color> {
    HashMap::from([
        (Reg::Rax.into(), -1),
        (Reg::Rsp.into(), -2),
        (Reg::Rbp.into(), -3),
        (Reg::R11.into(), -4),
        (Reg::R15.into(), -5),
    ])
}

fn saturation(graph: &InterferenceGraph, vert: &Location, coloring: &Coloring) -> HashSet<Color> {
    let mut colors = HashSet::new();
    for v in graph.adjacent(vert) {
        if let Some(c) = coloring.get(&v) {
            colors.insert(*c);
        }
    }
    colors
}

#[cfg(test)]
mod color_graph_tests {
    use super::color_graph;
    use interference_graph::InterferenceGraph;
    use std::collections::HashMap;
    use syntax::x86::Reg;

    #[test]
    fn color_example() {
        let mut graph = InterferenceGraph::new();
        graph.add_edge("t".into(), Reg::Rax.into());
        graph.add_edge("t".into(), "z".into());
        graph.add_edge("t".into(), Reg::Rsp.into());
        graph.add_edge(Reg::Rax.into(), Reg::Rsp.into());
        graph.add_edge("z".into(), "y".into());
        graph.add_edge("z".into(), Reg::Rsp.into());
        graph.add_edge("z".into(), "w".into());
        graph.add_edge("y".into(), "w".into());
        graph.add_edge("y".into(), Reg::Rsp.into());
        graph.add_edge("x".into(), "w".into());
        graph.add_edge("x".into(), Reg::Rsp.into());
        graph.add_edge("w".into(), Reg::Rsp.into());
        graph.add_edge("w".into(), "v".into());
        let result = color_graph(&graph);
        let expected = HashMap::from([
            (Reg::Rax.into(), -1),
            (Reg::Rsp.into(), -2),
            (Reg::Rbp.into(), -3),
            (Reg::R11.into(), -4),
            (Reg::R15.into(), -5),
            ("t".into(), 0),
            ("z".into(), 1),
            ("x".into(), 1),
            ("y".into(), 2),
            ("w".into(), 0),
            ("v".into(), 1),
        ]);
        assert_eq!(result, expected)
    }
}
