use super::{l_var::LVarDriver, Driver};
use chapter3::{
    assign_homes::assign_homes, color_graph::color_graph, interference_graph::build_graph,
    patch_instructions::patch_instructions, prelude_conclusion::generate_prelude_conclusion,
};
pub struct LVarRegDriver;

impl Driver for LVarRegDriver {
    type Target = chapter3::x86_int::Program;

    fn compile(
        &self,
        input: &str,
        _include_intermediary: bool,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog = LVarDriver::compile_lvar(input)?;
        let inter_graph = build_graph(&prog);
        let coloring = color_graph(inter_graph);
        let prog = assign_homes(prog, coloring);
        let patched = patch_instructions(prog);
        Ok(generate_prelude_conclusion(patched))
    }

    fn evaluate(&self, _input: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
