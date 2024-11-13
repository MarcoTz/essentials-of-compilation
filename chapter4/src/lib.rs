pub mod color_graph;
pub mod errors;
pub mod interference_graph;
pub mod patch_instructions;
pub mod remove_vars;
pub mod uncover_live;

use chapter2::{x86_int::Prog as IntProg, x86_var::Prog as VarProg};
use color_graph::assign_registers::assign_registers;
use color_graph::color_graph;
use errors::Error;
use interference_graph::{BuildGraph, InterferenceGraph};
use patch_instructions::PatchInstructions;
use remove_vars::RemoveVars;
use uncover_live::UncoverLive;

pub fn compile(prog: VarProg) -> Result<IntProg, Error> {
    let live = prog.uncover();
    let mut graph = InterferenceGraph::default();
    prog.build(&mut graph, &live);
    let coloring = color_graph(graph)?;
    let assignment = assign_registers(coloring)?;
    let vars_removed = prog.remove_vars(&assignment)?;
    Ok(vars_removed.patch())
}
