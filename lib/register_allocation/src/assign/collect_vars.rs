use crate::program::AnnotProg;
use lang_x86::{Instruction, VarArg};
use std::collections::HashSet;

pub fn collect_vars(prog: &AnnotProg) -> HashSet<String> {
    let mut vars = HashSet::new();
    for (_, instrs) in prog.blocks.iter() {
        for var_set in instrs.iter().map(|instr| collect_instr(&instr.instr)) {
            vars.extend(var_set.into_iter());
        }
    }
    vars
}

fn collect_instr(instr: &Instruction<VarArg>) -> HashSet<String> {
    match instr {
        Instruction::AddQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::SubQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::NegQ { arg } => collect_arg(arg),
        Instruction::MovQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::PushQ { arg } => collect_arg(arg),
        Instruction::PopQ { arg } => collect_arg(arg),
        Instruction::CallQ { .. } => HashSet::new(),
        Instruction::RetQ => HashSet::new(),
        Instruction::Jump { .. } => HashSet::new(),
        Instruction::XorQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::CmpQ { left, right } => &collect_arg(left) | &collect_arg(right),
        Instruction::SetCC { dest, .. } => collect_arg(dest),
        Instruction::MovZBQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::JumpCC { .. } => HashSet::new(),
        Instruction::AndQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::OrQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
    }
}

fn collect_arg(arg: &VarArg) -> HashSet<String> {
    match arg {
        VarArg::Arg(_) => HashSet::new(),
        VarArg::Var(v) => HashSet::from([v.clone()]),
    }
}
