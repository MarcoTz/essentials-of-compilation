use crate::x86_int::{Arg, Instr, Program, Reg};

pub fn patch_instructions(prog: Program) -> Program {
    Program {
        blocks: prog
            .blocks
            .into_iter()
            .map(|(label, instrs)| (label, patch_instrs(instrs)))
            .collect(),
        stack_space: prog.stack_space,
        global_labels: prog.global_labels,
        callee_saved: prog.callee_saved,
    }
}

fn patch_instrs(instrs: Vec<Instr>) -> Vec<Instr> {
    let mut new_instrs = vec![];
    for instr in instrs {
        new_instrs.extend(patch_instr(instr));
    }
    new_instrs
}

fn patch_instr(instr: Instr) -> Vec<Instr> {
    match instr {
        Instr::AddQ(a1, a2) => match (a1, a2) {
            (Arg::Deref(rg1, o1), Arg::Deref(rg2, o2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(rg1, o1), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(rg2, o2)),
                ]
            }
            (a1, a2) => vec![Instr::AddQ(a1, a2)],
        },
        Instr::SubQ(a1, a2) => match (a1, a2) {
            (Arg::Deref(rg1, o1), Arg::Deref(rg2, o2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(rg1, o1), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(rg2, o2)),
                ]
            }
            (a1, a2) => vec![Instr::AddQ(a1, a2)],
        },
        Instr::MovQ(a1, a2) => match (a1, a2) {
            (a1, a2) if a1 == a2 => vec![],
            (Arg::Deref(rg1, o1), Arg::Deref(rg2, o2)) => vec![
                Instr::MovQ(Arg::Deref(rg1, o1), Arg::Reg(Reg::Rax)),
                Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(rg2, o2)),
            ],
            (a1, a2) => vec![Instr::MovQ(a1, a2)],
        },
        _ => vec![instr],
    }
}

#[cfg(test)]
mod patchinstructions_tests {
    use super::patch_instructions;
    use crate::x86_int::{Arg, Instr, Program, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn patch_example() {
        let prog = Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Immediate(42), Arg::Reg(Reg::Rcx)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -8)),
                    Instr::AddQ(Arg::Immediate(7), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -16)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -8)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rcx)),
                    Instr::NegQ(Arg::Reg(Reg::Rcx)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
                    Instr::Jump("conclusion".to_owned()),
                ],
            )]),
            stack_space: 16,
            global_labels: HashSet::new(),
            callee_saved: HashSet::new(),
        };
        let result = patch_instructions(prog);
        let expected = Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Immediate(42), Arg::Reg(Reg::Rcx)),
                    Instr::AddQ(Arg::Immediate(7), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                    Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Deref(Reg::Rbp, -8)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -16), Arg::Reg(Reg::Rcx)),
                    Instr::NegQ(Arg::Reg(Reg::Rcx)),
                    Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rcx), Arg::Reg(Reg::Rax)),
                    Instr::Jump("conclusion".to_owned()),
                ],
            )]),
            stack_space: 16,
            global_labels: HashSet::new(),
            callee_saved: HashSet::new(),
        };
        assert_eq!(result, expected)
    }
}
