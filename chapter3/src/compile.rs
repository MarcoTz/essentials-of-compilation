use super::errors::Error;
use super::patch_instructions::patch_instructions;
use super::{
    assign_homes::assign_homes, color_graph::color_graph, interference_graph::build_graph,
};

use chapter2::x86_int::Program as IntProg;
use chapter2::x86_var::Program as VarProg;

pub fn compile(prog: VarProg) -> Result<IntProg, Error> {
    let inter_graph = build_graph(&prog);
    let coloring = color_graph(inter_graph);
    let prog = assign_homes(prog, coloring);
    let patched = patch_instructions(prog);
    todo!()
}
