use super::{
    assign_homes::{AssignHomes, AssignState},
    l_var::syntax::Module,
    patch_instructions::PatchInstructions,
    prelude_conclusion::generate_prelude_conclusion,
    reduce::{Reduce, ReduceState},
    select_instructions::SelectInstructions,
    x86_int::Prog,
};

pub fn compile(md: Module) -> Prog {
    let md_reduced = md.reduce(&mut ReduceState::default());
    let prog_var = md_reduced.select_instructions();
    let prog_int = prog_var.assign_homes(&mut AssignState::default());
    let prog_patched = prog_int.patch();
    generate_prelude_conclusion(prog_patched)
}
