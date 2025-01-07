use super::{
    assign_homes::assign_homes, color_graph::color_graph, interference_graph::build_graph,
    patch_instructions::patch_instructions, prelude_conclusion::generate_prelude_conclusion,
    x86_int::Program as IntProg,
};

use chapter2::x86_var::Program as VarProg;

pub fn compile(prog: VarProg) -> IntProg {
    let inter_graph = build_graph(&prog);
    let coloring = color_graph(inter_graph);
    let prog = assign_homes(prog, coloring);
    let patched = patch_instructions(prog);
    generate_prelude_conclusion(patched)
}
