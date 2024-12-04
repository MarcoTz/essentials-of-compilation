use crate::{
    explicate_control::ExplicateControl,
    l_var::{syntax::Program, uniquify::Uniquify},
    remove_complex_operands::RemoveComplexOperands,
};

/// Compiles a l_var program in the following steps:
/// uniquify : l_var -> l_var
/// remove_complex_operands : l_var -> l_var_reduced
/// explicate_control: l_var_reduced -> c_var
/// select_instructions: c_var -> x86_var
/// assign_homes : x86_var -> x86_var
/// patch_instructions : x86_var -> x86_int
/// prelude_conclusion : x86_int -> x86_int
pub fn compile(prog: Program) -> Program {
    let prog_unique = prog.uniquify(&mut Default::default());
    let prog_reduced = prog_unique.remove_complex_operands(&mut Default::default());
    let prog_explicated = prog_reduced.explicate_control();
    todo!()
}
