use super::{
    assign_homes::assign_homes, color_graph::color_graph, interference_graph::build_graph,
    patch_instructions::patch_instructions, prelude_conclusion::generate_prelude_conclusion,
    x86_int::Program as IntProg,
};

use chapter2::{compile::compile_lvar, parser::errors::Error};

pub fn compile(input: &str) -> Result<IntProg, Error> {
    let prog = compile_lvar(input)?;
    let inter_graph = build_graph(&prog);
    let coloring = color_graph(inter_graph);
    let prog = assign_homes(prog, coloring);
    let patched = patch_instructions(prog);
    Ok(generate_prelude_conclusion(patched))
}
