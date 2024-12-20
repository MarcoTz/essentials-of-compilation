use chapter2::x86_var::{Arg, Instr, Reg, Var};
use std::collections::HashSet;

pub fn l_before(instr: &Instr, l_after: &HashSet<Arg>) -> HashSet<Arg> {
    let written = get_written(instr);
    let read = get_read(instr);
    let mut l_before = l_after.clone();
    for arg in written.iter() {
        l_before.remove(arg);
    }
    l_before.extend(read);
    l_before
}

fn get_written(instr: &Instr) -> HashSet<Arg> {
    match instr {
        Instr::AddQ(_, a2) => args_to_set(vec![a2.clone()]),
        Instr::RetQ => HashSet::new(),
        Instr::SubQ(_, a2) => args_to_set(vec![a2.clone()]),
        Instr::NegQ(a) => args_to_set(vec![a.clone()]),
        Instr::MovQ(_, a2) => args_to_set(vec![a2.clone()]),
        Instr::PopQ(a) => args_to_set(vec![a.clone(), Reg::Rsp.into()]),
        Instr::Jump(_) => HashSet::new(),
        Instr::CallQ(_, _) => {
            let mut live = HashSet::new();
            for reg in Reg::caller_saved().into_iter() {
                live.insert(reg.into());
            }
            live
        }
        Instr::PushQ(_) => args_to_set(vec![Reg::Rsp.into()]),
    }
}

fn get_read(instr: &Instr) -> HashSet<Arg> {
    match instr {
        Instr::AddQ(a1, a2) => args_to_set(vec![a1.clone(), a2.clone()]),
        Instr::RetQ => HashSet::new(),
        Instr::SubQ(a1, a2) => args_to_set(vec![a1.clone(), a2.clone()]),
        Instr::NegQ(a) => args_to_set(vec![a.clone()]),
        Instr::MovQ(a1, _) => args_to_set(vec![a1.clone()]),
        Instr::PopQ(_) => args_to_set(vec![Reg::Rsp.into()]),
        Instr::Jump(_) => HashSet::new(),
        Instr::CallQ(_, num_args) => {
            let mut live = HashSet::new();
            for (ind, reg) in Reg::callee_saved().into_iter().enumerate() {
                if ind >= *num_args {
                    break;
                }
                live.insert(reg.into());
            }
            live
        }
        Instr::PushQ(a) => args_to_set(vec![a.clone(), Reg::Rsp.into()]),
    }
}

fn args_to_set(args: Vec<Arg>) -> HashSet<Arg> {
    let mut set = HashSet::new();
    for arg in args.into_iter() {
        if matches!(arg, Arg::Immediate(_)) {
            continue;
        }
        set.insert(arg);
    }
    set
}
