use crate::{
    color_graph::{assign_registers, Coloring, RegisterAssignment},
    x86_int::{Arg as IntArg, Instr as IntInstr, Program as IntProg, Reg as IntReg},
};
use chapter2::x86_var::{Arg as VarArg, Instr as VarInstr, Program as VarProg, Reg as VarReg, Var};

use std::collections::{HashMap, HashSet};

pub fn assign_homes(prog: VarProg, coloring: Coloring) -> IntProg {
    let assignment = assign_registers(coloring);
    apply_assignment(prog, assignment)
}

fn apply_assignment(prog: VarProg, assignment: RegisterAssignment) -> IntProg {
    let callee_saved = assignment
        .vars
        .iter()
        .filter_map(|(_, arg)| {
            let reg = if let VarArg::Reg(reg) = arg {
                reg.clone()
            } else {
                return None;
            };
            if VarReg::callee_saved().contains(&reg) && !matches!(reg, VarReg::Rsp) {
                Some(reg.into())
            } else {
                None
            }
        })
        .collect();
    IntProg {
        blocks: prog
            .blocks
            .into_iter()
            .map(|(label, instrs)| (label, apply_block(instrs, &assignment.vars)))
            .collect(),
        stack_space: assignment.stack_space,
        global_labels: HashSet::new(),
        callee_saved,
    }
}

fn apply_block(instrs: Vec<VarInstr>, assignments: &HashMap<Var, VarArg>) -> Vec<IntInstr> {
    let mut new_instrs = vec![];
    for instr in instrs {
        new_instrs.push(apply_instr(instr, assignments));
    }
    new_instrs
}

fn apply_instr(instr: VarInstr, assignments: &HashMap<Var, VarArg>) -> IntInstr {
    match instr {
        VarInstr::AddQ(a1, a2) => {
            IntInstr::AddQ(apply_arg(a1, assignments), apply_arg(a2, assignments))
        }
        VarInstr::RetQ => IntInstr::RetQ,
        VarInstr::SubQ(a1, a2) => {
            IntInstr::SubQ(apply_arg(a1, assignments), apply_arg(a2, assignments))
        }
        VarInstr::NegQ(a) => IntInstr::NegQ(apply_arg(a, assignments)),
        VarInstr::MovQ(a1, a2) => {
            IntInstr::MovQ(apply_arg(a1, assignments), apply_arg(a2, assignments))
        }
        VarInstr::CallQ(fun, args) => IntInstr::CallQ(fun, args),
        VarInstr::PushQ(a) => IntInstr::PushQ(apply_arg(a, assignments)),
        VarInstr::PopQ(a) => IntInstr::PopQ(apply_arg(a, assignments)),
        VarInstr::Jump(lb) => IntInstr::Jump(lb),
    }
}

fn apply_arg(arg: VarArg, assignments: &HashMap<Var, VarArg>) -> IntArg {
    match arg {
        VarArg::Reg(rg) => IntArg::Reg(conv_reg(rg)),
        VarArg::Immediate(i) => IntArg::Immediate(i),
        VarArg::Deref(rg, offset) => IntArg::Deref(conv_reg(rg), offset),
        VarArg::Var(v) => apply_arg(assignments.get(&v).unwrap().clone(), assignments),
    }
}

fn conv_reg(reg: VarReg) -> IntReg {
    match reg {
        VarReg::Rsp => IntReg::Rsp,
        VarReg::Rbp => IntReg::Rbp,
        VarReg::Rax => IntReg::Rax,
        VarReg::R8 => IntReg::R8,
        VarReg::Rbx => IntReg::Rbx,
        VarReg::Rcx => IntReg::Rcx,
        VarReg::Rdx => IntReg::Rdx,
        VarReg::Rsi => IntReg::Rsi,
        VarReg::Rdi => IntReg::Rdi,
        VarReg::R9 => IntReg::R9,
        VarReg::R10 => IntReg::R10,
        VarReg::R11 => IntReg::R11,
        VarReg::R12 => IntReg::R12,
        VarReg::R13 => IntReg::R13,
        VarReg::R14 => IntReg::R14,
        VarReg::R15 => IntReg::R15,
    }
}
