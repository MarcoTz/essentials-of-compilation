use super::{
    assign_homes::{AssignHomes, AssignState},
    l_var::syntax::Module,
    patch_instructions::PatchInstructions,
    prelude_conclusion::generate_prelude_conclusion,
    reduce::{Reduce, ReduceState},
    select_instructions::SelectInstructions,
    x86_int::Prog,
};

/// Compiles a l_var program in the following steps:
/// uniquify : l_var -> l_var
/// remove_complex_operands : l_var -> l_var_reduced
/// explicate_control: l_var_reduced -> c_var
/// select_instructions: c_var -> x86_var
/// assign_homes : x86_var -> x86_var
/// patch_instructions : x86_var -> x86_int
/// prelude_conclusion : x86_int -> x86_int
pub fn compile(md: Module) -> Prog {
    let md_reduced = md.reduce(&mut ReduceState::default());
    let prog_var = md_reduced.select_instructions();
    let prog_int = prog_var.assign_homes(&mut AssignState::default());
    let prog_patched = prog_int.patch();
    generate_prelude_conclusion(prog_patched)
}
