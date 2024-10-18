use super::x86_int::Prog;
use std::{collections::HashMap, env::consts::OS};

pub fn generate_prelude_conclusion(prog: &mut Prog) {
    if OS != "macos" {
        return;
    }

    let mut new_labels = HashMap::new();

    for (lb, pos) in prog.labels.iter() {
        new_labels.insert(lb.clone(), *pos);
    }
    prog.labels = new_labels;
}
