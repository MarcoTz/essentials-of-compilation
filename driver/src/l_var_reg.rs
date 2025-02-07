use super::{l_var::LVarDriver, Driver};
use chapter3::{
    assign_homes::assign_homes, color_graph::color_graph, interference_graph::build_graph,
    patch_instructions::patch_instructions, prelude_conclusion::generate_prelude_conclusion,
};

pub struct LVarRegDriver {
    print_intermediary: bool,
    l_var_driver: LVarDriver,
}

impl LVarRegDriver {
    pub fn new(print_intermediary: bool) -> LVarRegDriver {
        LVarRegDriver {
            print_intermediary,
            l_var_driver: LVarDriver::new(print_intermediary),
        }
    }
}

impl Driver for LVarRegDriver {
    type Target = chapter3::x86_int::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn compile(&self, input: &str) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog = self.l_var_driver.compile_lvar(input)?;
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

impl Default for LVarRegDriver {
    fn default() -> LVarRegDriver {
        LVarRegDriver::new(false)
    }
}
