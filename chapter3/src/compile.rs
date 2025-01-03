use super::assign_homes::assign_homes;
use super::errors::Error;
use super::patch_instructions::patch_instructions;

use chapter2::x86_int::Program as IntProg;
use chapter2::x86_var::Program as VarProg;

pub fn compile(prog: VarProg) -> Result<IntProg, Error> {
    let prog = assign_homes(prog);
    let patched = patch_instructions(prog);
    todo!()
}
