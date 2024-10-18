use super::x86_int::Prog;
use std::{collections::HashMap, env::consts::OS};

pub fn generate_prelude_conclusion(prog: Prog) -> Prog {
    if OS != "macos" {
        return prog;
    }

    let mut new_labels = HashMap::new();

    for (lb, pos) in prog.labels.iter() {
        new_labels.insert(lb.clone(), *pos);
    }
    Prog {
        instrs: prog.instrs,
        stack_space: prog.stack_space,
        labels: new_labels,
    }
}
