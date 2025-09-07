use crate::{
    colors::{Color, Coloring, empty_coloring, saturation},
    errors::Error,
    graph::InterferenceGraph,
};

pub fn color_graph(graph: &InterferenceGraph) -> Result<Coloring, Error> {
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
            .ok_or(Error::NextVertex)?
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
    Ok(coloring)
}

#[cfg(test)]
mod color_graph_tests {
    use super::color_graph;
    use crate::graph::InterferenceGraph;
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
        let result = color_graph(&graph).unwrap();
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
