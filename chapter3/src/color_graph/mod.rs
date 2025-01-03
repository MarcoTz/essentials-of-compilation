use super::interference_graph::InterferenceGraph;
use chapter2::x86_var::{Arg, Reg, Var};
use std::collections::{HashMap, HashSet};

pub mod registers_colors;
use registers_colors::{color_to_reg, reg_to_color};

pub type Color = i32;
pub type Coloring = HashMap<Var, Color>;
pub struct RegisterAssignment {
    pub vars: HashMap<Var, Arg>,
    pub stack_space: usize,
}

pub fn saturation(vert: &Var, graph: &InterferenceGraph, coloring: &Coloring) -> HashSet<Color> {
    let mut colors = HashSet::new();
    let adj: HashSet<Arg> = graph.adjacent(&Arg::Var(vert.clone()));
    for arg in adj.iter() {
        match arg {
            Arg::Reg(reg) => {
                colors.insert(reg_to_color(reg));
            }
            Arg::Deref(reg, _) => {
                colors.insert(reg_to_color(reg));
            }
            Arg::Var(v) => match coloring.get(v) {
                None => continue,
                Some(c) => {
                    colors.insert(*c);
                }
            },
            Arg::Immediate(_) => continue,
        }
    }
    colors
}

pub fn color_graph(graph: InterferenceGraph) -> Coloring {
    let mut coloring = HashMap::new();
    let mut verts: HashSet<Var> = graph
        .vertices
        .iter()
        .filter_map(|arg| {
            if let Arg::Var(v) = arg {
                Some(v.clone())
            } else {
                None
            }
        })
        .collect();
    while !verts.is_empty() {
        let saturations = verts
            .iter()
            .map(|v| (v.clone(), saturation(v, &graph, &coloring)));
        let (next_vert, next_saturation) = saturations
            .max_by(|(_, color1), (_, color2)| color1.len().cmp(&color2.len()))
            .unwrap();
        let mut vert_color = 0;
        while next_saturation.contains(&vert_color) {
            vert_color += 1
        }
        coloring.insert(next_vert.clone(), vert_color);
        verts.remove(&next_vert);
    }
    coloring
}

pub fn assign_registers(coloring: Coloring) -> RegisterAssignment {
    let mut assignment = HashMap::new();
    let mut stack_space = -8;
    for (var, color) in coloring.into_iter() {
        match color_to_reg(color) {
            Ok(reg) => {
                assignment.insert(var, Arg::Reg(reg));
            }
            Err(_) => {
                assignment.insert(var, Arg::Deref(Reg::Rbp, stack_space));
                stack_space -= 8;
            }
        }
    }
    RegisterAssignment {
        vars: assignment,
        stack_space: stack_space.abs() as usize,
    }
}
