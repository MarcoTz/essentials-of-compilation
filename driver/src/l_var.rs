use super::Driver;
use chapter2::{
    assign_homes::AssignHomes,
    c_var::typecheck::typecheck,
    explicate_control::ExplicateControl,
    l_var::uniquify::Uniquify,
    parser::parse_program,
    remove_complex_operands::RemoveComplexOperands,
    select_instructions::SelectInstructions,
    x86_int::{
        patch_instructions::PatchInstructions, prelude_conclusion::generate_prelude_conclusion,
    },
};

pub struct LVarDriver {
    print_intermediary: bool,
}

impl LVarDriver {
    pub fn new(print_intermediary: bool) -> LVarDriver {
        LVarDriver { print_intermediary }
    }
}

impl LVarDriver {
    pub fn compile_lvar(
        &self,
        input: chapter2::l_var::Program,
    ) -> Result<chapter2::x86_var::Program, Box<dyn std::error::Error>> {
        let prog_unique = input.uniquify(&mut Default::default());
        self.debug("------ Uniquified -----");
        self.debug(&prog_unique.to_string());

        let prog_reduced = prog_unique.remove_complex_operands(&mut Default::default());
        self.debug("------ Reduced -----");
        self.debug(&prog_reduced.to_string());

        let mut prog_explicated = prog_reduced.explicate_control();
        self.debug("------ Explicated -----");
        self.debug(&prog_explicated.to_string());

        typecheck(&mut prog_explicated);
        self.debug("------ Typechecked -----");
        self.debug(&prog_explicated.to_string());

        let selected = prog_explicated.select_instructions();
        self.debug("------ Selected Instructions -----");
        self.debug(&selected.to_string());
        Ok(selected)
    }
}

impl Driver for LVarDriver {
    type Target = chapter2::x86_int::Program;
    type Parsed = chapter2::l_var::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>> {
        let (_, parsed) = parse_program(input)?;
        self.debug("----- Parsed ----");
        self.debug(&parsed.to_string());
        Ok(parsed)
    }

    fn compile(&self, input: Self::Parsed) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog_selected = self.compile_lvar(input)?;

        let prog_homes = prog_selected.assign_homes(&mut Default::default());
        self.debug("------ Assigned Homes -----");
        self.debug(&prog_homes.to_string());

        let prog_patched = prog_homes.patch();
        self.debug("------ Patched Instructions -----");
        self.debug(&prog_patched.to_string());

        let prog_prel_conc = generate_prelude_conclusion(prog_patched);
        self.debug("------ Generated Prelude and Conclusion -----");
        self.debug(&prog_prel_conc.to_string());

        Ok(prog_prel_conc)
    }

    fn evaluate(&self, _: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl Default for LVarDriver {
    fn default() -> LVarDriver {
        LVarDriver::new(false)
    }
}
