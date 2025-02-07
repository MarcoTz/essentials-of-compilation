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

pub struct LVarDriver;

impl LVarDriver {
    pub fn compile_lvar(
        input: &str,
    ) -> Result<chapter2::x86_var::Program, Box<dyn std::error::Error>> {
        let (_, parsed) = parse_program(input)?;
        let prog_unique = parsed.uniquify(&mut Default::default());
        let prog_reduced = prog_unique.remove_complex_operands(&mut Default::default());
        let mut prog_explicated = prog_reduced.explicate_control();
        typecheck(&mut prog_explicated);
        Ok(prog_explicated.select_instructions())
    }
}

impl Driver for LVarDriver {
    type Target = chapter2::x86_int::Program;

    fn compile(
        &self,
        input: &str,
        _print_intermediary: bool,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog_selected = Self::compile_lvar(input)?;
        let prog_homes = prog_selected.assign_homes(&mut Default::default());
        let prog_patched = prog_homes.patch();
        Ok(generate_prelude_conclusion(prog_patched))
    }

    fn evaluate(&self, _: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
