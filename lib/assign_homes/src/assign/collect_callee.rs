use crate::program::AnnotProg;
use std::collections::HashSet;
use syntax::x86::{Arg, Instruction, Reg, VarArg};

pub fn collect_callee(prog: &AnnotProg) -> HashSet<Reg> {
    let mut callee = HashSet::new();
    for (_, block) in prog.blocks.iter() {
        for instr in block.iter() {
            callee = &callee | &collect_instr(&instr.instr);
        }
    }
    callee
}

fn collect_instr(instr: &Instruction<VarArg>) -> HashSet<Reg> {
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
        Instruction::NotQ { arg } => collect_arg(arg),
        Instruction::AndQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
        Instruction::OrQ { src, dest } => &collect_arg(src) | &collect_arg(dest),
    }
}

fn collect_arg(arg: &VarArg) -> HashSet<Reg> {
    match arg {
        VarArg::Var(_) => HashSet::new(),
        VarArg::Arg(Arg::Immediate(_)) => HashSet::new(),
        VarArg::Arg(Arg::Register(reg)) => {
            if Reg::callee_saved().contains(reg) {
                HashSet::from([reg.clone()])
            } else {
                HashSet::new()
            }
        }
        VarArg::Arg(Arg::Deref(reg, _)) => {
            if Reg::callee_saved().contains(reg) {
                HashSet::from([reg.clone()])
            } else {
                HashSet::new()
            }
        }
        VarArg::Arg(Arg::ByteReg(_)) => HashSet::new(),
    }
}
