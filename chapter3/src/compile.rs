use super::assign_homes::assign_homes;
use super::errors::Error;
use super::patch_instructions::PatchInstructions;

use chapter2::x86_int::Program as IntProg;
use chapter2::x86_var::Program as VarProg;

pub fn compile(prog: VarProg) -> Result<IntProg, Error> {
    let prog = assign_homes(prog);
    todo!()
    /*
    let assignment = assign_registers(coloring)?;
    let vars_removed = prog.remove_vars(&assignment)?;
    Ok(vars_removed.patch())*/
}
