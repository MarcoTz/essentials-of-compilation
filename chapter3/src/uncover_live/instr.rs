use chapter2::x86_var::{Instr, Var};
use std::collections::HashSet;

pub fn l_before(instr: &Instr, l_after: &HashSet<Var>) -> HashSet<Var> {
    let written = get_written(instr);
    let read = get_read(instr);
    let mut l_before = l_after.clone();
    for var in written.iter() {
        l_before.remove(var);
    }
    l_before.extend(read);
    l_before
}

fn get_written(instr: &Instr) -> HashSet<Var> {
    match instr {
        Instr::AddQ(_, a2) => vars_to_set(vec![a2.as_var()]),
        Instr::RetQ => HashSet::new(),
        Instr::SubQ(_, a2) => vars_to_set(vec![a2.as_var()]),
        Instr::NegQ(a) => vars_to_set(vec![a.as_var()]),
        Instr::MovQ(_, a2) => vars_to_set(vec![a2.as_var()]),
        Instr::PopQ(a) => vars_to_set(vec![a.as_var()]),
        Instr::Jump(_) => HashSet::new(),
        Instr::CallQ(_, _) => HashSet::new(),
        Instr::PushQ(_) => HashSet::new(),
    }
}

fn get_read(instr: &Instr) -> HashSet<Var> {
    match instr {
        Instr::AddQ(a1, a2) => vars_to_set(vec![a1.as_var(), a2.as_var()]),
        Instr::RetQ => HashSet::new(),
        Instr::SubQ(a1, a2) => vars_to_set(vec![a1.as_var(), a2.as_var()]),
        Instr::NegQ(a) => vars_to_set(vec![a.as_var()]),
        Instr::MovQ(a1, _) => vars_to_set(vec![a1.as_var()]),
        Instr::PopQ(_) => HashSet::new(),
        Instr::Jump(_) => HashSet::new(),
        Instr::CallQ(_, _) => HashSet::new(),
        Instr::PushQ(a) => vars_to_set(vec![a.as_var()]),
    }
}

fn vars_to_set(vars: Vec<Option<Var>>) -> HashSet<Var> {
    let mut set = HashSet::new();
    for var in vars.into_iter() {
        if let Some(v) = var {
            set.insert(v);
        }
    }
    set
}
