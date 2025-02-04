use crate::{
    assign_homes::AssignHomes,
    c_var::typecheck::typecheck,
    explicate_control::ExplicateControl,
    l_var::uniquify::Uniquify,
    parser::{errors::Error, parse_program},
    remove_complex_operands::RemoveComplexOperands,
    select_instructions::SelectInstructions,
    x86_int,
    x86_int::{
        patch_instructions::PatchInstructions, prelude_conclusion::generate_prelude_conclusion,
    },
    x86_var,
};

/// Compiles a l_var program in the following steps:
/// uniquify : l_var -> l_var
/// remove_complex_operands : l_var -> l_var_reduced
/// explicate_control: l_var_reduced -> c_var
/// select_instructions: c_var -> x86_var
/// assign_homes : x86_var -> x86_var
/// patch_instructions : x86_var -> x86_int
/// prelude_conclusion : x86_int -> x86_int
pub fn compile(input: &str) -> Result<x86_int::Program, Error> {
    let prog_selected = compile_lvar(input)?;
    let prog_homes = prog_selected.assign_homes(&mut Default::default());
    let prog_patched = prog_homes.patch();
    Ok(generate_prelude_conclusion(prog_patched))
}

pub fn compile_lvar(input: &str) -> Result<x86_var::Program, Error> {
    let (_, parsed) = parse_program(input)?;
    let prog_unique = parsed.uniquify(&mut Default::default());
    let prog_reduced = prog_unique.remove_complex_operands(&mut Default::default());
    let mut prog_explicated = prog_reduced.explicate_control();
    typecheck(&mut prog_explicated);
    Ok(prog_explicated.select_instructions())
}
