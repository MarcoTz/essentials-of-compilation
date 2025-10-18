use crate::{
    colors::{Color, Coloring, empty_coloring, saturation},
    errors::Error,
    graph::LocationGraph,
    program::Location,
};
use std::collections::HashSet;

pub fn color_graph(
    interference_graph: LocationGraph,
    move_graph: LocationGraph,
) -> Result<Coloring, Error> {
    let mut vert_set = interference_graph.verts.clone();
    let mut coloring = empty_coloring();
    vert_set.retain(|vert| matches!(vert, Location::Variable(_)));
    'outer: while !vert_set.is_empty() {
        let mut next_candidates = get_next_candidates(&vert_set, &interference_graph, &coloring)?;
        next_candidates.sort();
        for candidate in next_candidates.iter() {
            if let Some(color) =
                get_move_related(candidate, &interference_graph, &move_graph, &coloring)
            {
                vert_set.remove(candidate);
                coloring.0.insert(candidate.clone(), color);
                continue 'outer;
            }
        }
        let next_candidate = next_candidates.first().ok_or(Error::NextVertex)?;
        let adjacent_colors: Vec<&Color> = interference_graph
            .verts
            .iter()
            .filter_map(|vert| coloring.0.get(vert))
            .collect();
        let mut next_color = 0;
        while adjacent_colors.contains(&&next_color) {
            next_color += 1;
        }
        vert_set.remove(next_candidate);
        coloring.0.insert(next_candidate.clone(), next_color);
    }
    Ok(coloring)
}

fn get_next_candidates(
    vert_set: &HashSet<Location>,
    interference_graph: &LocationGraph,
    coloring: &Coloring,
) -> Result<Vec<Location>, Error> {
    let saturations: Vec<(&Location, usize)> = vert_set
        .iter()
        .map(|vert| {
            (
                vert,
                saturation(interference_graph, vert, coloring).len() as usize,
            )
        })
        .collect();
    let min_saturation = saturations
        .iter()
        .map(|(_, s)| s)
        .min()
        .copied()
        .unwrap_or(0);
    let min_verts: Vec<Location> = saturations
        .iter()
        .filter_map(|(vert, sat)| (*sat == min_saturation).then_some(*vert))
        .cloned()
        .collect();
    Ok(min_verts)
}

fn get_move_related(
    loc: &Location,
    interference_graph: &LocationGraph,
    move_graph: &LocationGraph,
    coloring: &Coloring,
) -> Option<Color> {
    if !matches!(loc, Location::Variable(_)) {
        return None;
    };
    let interfering = interference_graph.adjacent(loc);
    let interfering_colors: Vec<Color> = interfering
        .iter()
        .filter_map(|vert| coloring.0.get(vert))
        .copied()
        .collect();
    for move_related in move_graph.adjacent(loc) {
        if interfering.contains(&move_related) {
            continue;
        }
        let related_color = if let Some(color) = coloring.0.get(&move_related) {
            color
        } else {
            continue;
        };
        if interfering_colors.contains(related_color) {
            continue;
        }
        return Some(*related_color);
    }
    None
}

#[cfg(test)]
mod color_graph_tests {
    use super::{Coloring, color_graph};
    use crate::graph::LocationGraph;
    use asm::Reg;
    use std::collections::HashMap;

    #[test]
    fn color_example() {
        let mut interference_graph = LocationGraph::new();
        interference_graph.add_edge("t".into(), Reg::Rax.into());
        interference_graph.add_edge("t".into(), "z".into());
        interference_graph.add_edge("t".into(), Reg::Rsp.into());
        interference_graph.add_edge(Reg::Rax.into(), Reg::Rsp.into());
        interference_graph.add_edge("z".into(), "y".into());
        interference_graph.add_edge("z".into(), Reg::Rsp.into());
        interference_graph.add_edge("z".into(), "w".into());
        interference_graph.add_edge("y".into(), "w".into());
        interference_graph.add_edge("y".into(), Reg::Rsp.into());
        interference_graph.add_edge("x".into(), "w".into());
        interference_graph.add_edge("x".into(), Reg::Rsp.into());
        interference_graph.add_edge("w".into(), Reg::Rsp.into());
        interference_graph.add_edge("w".into(), "v".into());
        let mut move_graph = LocationGraph::new();
        move_graph.add_edge("t".into(), "y".into());
        move_graph.add_edge("y".into(), "x".into());
        move_graph.add_edge("z".into(), "x".into());
        move_graph.add_edge("x".into(), "v".into());
        let result = color_graph(interference_graph, move_graph).unwrap();
        let expected = Coloring(HashMap::from([
            (Reg::Rax.into(), -1),
            (Reg::Rsp.into(), -2),
            (Reg::Rbp.into(), -3),
            (Reg::R11.into(), -4),
            (Reg::R15.into(), -5),
            ("v".into(), 0),
            ("x".into(), 0),
            ("y".into(), 0),
            ("t".into(), 0),
            ("z".into(), 2),
            ("w".into(), 1),
        ]));
        assert_eq!(result, expected)
    }
}
