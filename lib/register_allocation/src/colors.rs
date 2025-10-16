use crate::{graph::LocationGraph, program::Location};
use lang_x86::{Arg, Reg};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

pub type Color = i64;
#[derive(Debug, PartialEq)]
pub struct Coloring(pub HashMap<Location, Color>);

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

pub fn empty_coloring() -> Coloring {
    Coloring(HashMap::from([
        (Reg::Rax.into(), -1),
        (Reg::Rsp.into(), -2),
        (Reg::Rbp.into(), -3),
        (Reg::R11.into(), -4),
        (Reg::R15.into(), -5),
    ]))
}

pub fn saturation(graph: &LocationGraph, vert: &Location, coloring: &Coloring) -> HashSet<Color> {
    let mut colors = HashSet::new();
    for v in graph.adjacent(vert) {
        if let Some(c) = coloring.0.get(&v) {
            colors.insert(*c);
        }
    }
    colors
}

pub fn coloring_to_assignment(coloring: Coloring) -> HashMap<String, Arg> {
    let mut assignments = HashMap::new();
    for (loc, color) in coloring.0 {
        let arg = color_to_arg(color);
        match (loc, &arg) {
            (Location::Variable(v), _) => {
                assignments.insert(v, arg);
            }
            (Location::Register(reg1), Arg::Register(reg2)) if reg1 == *reg2 => (),
            (Location::Stack(offset1), Arg::Deref(Reg::Rbp, offset2)) if offset1 == *offset2 => (),
            (loc, _) => panic!("Locations {loc} and {arg} do not match"),
        }
    }
    assignments
}

impl fmt::Display for Coloring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (loc, color) in self.0.iter() {
            writeln!(f, "{loc} -> {color}")?;
        }
        Ok(())
    }
}
