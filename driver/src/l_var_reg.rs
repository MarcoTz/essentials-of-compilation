use super::{l_var::LVarDriver, Driver};
use chapter3::{
    assign_homes::assign_homes,
    color_graph::{color_graph, coloring_to_string},
    interference_graph::build_graph,
    patch_instructions::patch_instructions,
    prelude_conclusion::generate_prelude_conclusion,
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
        self.debug(prog.to_string());

        let inter_graph = build_graph(&prog);
        self.debug(inter_graph.to_string());

        let coloring = color_graph(inter_graph);
        self.debug(coloring_to_string(&coloring));

        let prog = assign_homes(prog, coloring);
        self.debug(prog.to_string());

        let patched = patch_instructions(prog);
        self.debug(patched.to_string());

        let prel_conc = generate_prelude_conclusion(patched);
        self.debug(prel_conc.to_string());
        Ok(prel_conc)
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
