use super::color_graph::assign_registers::assign_registers;
use super::color_graph::color_graph;
use super::errors::Error;
use super::interference_graph::{BuildGraph, InterferenceGraph};
use super::patch_instructions::PatchInstructions;
use super::remove_vars::RemoveVars;
use super::uncover_live::uncover_live;

use chapter2::x86_int::Program as IntProg;
use chapter2::x86_var::Program as VarProg;

pub fn compile(prog: VarProg) -> Result<IntProg, Error> {
    let livemap = uncover_live(&prog);
    todo!()
    /*    let live = prog.uncover();
    let mut graph = InterferenceGraph::default();
    prog.build(&mut graph, &live);
    let coloring = color_graph(graph)?;
    let assignment = assign_registers(coloring)?;
    let vars_removed = prog.remove_vars(&assignment)?;
    Ok(vars_removed.patch())*/
}
