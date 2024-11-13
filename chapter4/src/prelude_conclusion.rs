use crate::x86_int::Prog;

pub fn generate_prelude_conclusion(prog: Prog) -> Prog {
    Prog {
        instrs: prog.instrs,
        stack_space: prog.stack_space,
        labels: prog.labels,
        used_callee: prog.used_callee,
    }
}
