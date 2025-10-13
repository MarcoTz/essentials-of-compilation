use syntax::x86::{Arg, Block, Instruction, Program, Reg};

pub fn generate_prelude_conclusion(prog: Program) -> Program {
    let prelude = generate_prelude(&prog);
    let conclusion = generate_conclusion(&prog);
    let mut finalized = Program::new(prog.stack_space, prog.used_callee);
    finalized.blocks.push(Block::new("main", prelude));
    finalized.blocks.push(Block::new("conclusion", conclusion));
    for block in prog.blocks {
        finalized
            .blocks
            .push(Block::new(&block.label, block.instrs));
    }
    finalized
}

fn used_space(prog: &Program) -> i64 {
    let mut used_space = prog.stack_space + 8 * prog.used_callee.len() as u64;
    used_space = used_space + (used_space % 16);
    used_space -= 8 * prog.used_callee.len() as u64;
    used_space as i64
}

fn generate_prelude(prog: &Program) -> Vec<Instruction<Arg>> {
    let mut prelude = vec![
        Instruction::PushQ {
            arg: Reg::Rbp.into(),
        },
        Instruction::MovQ {
            src: Reg::Rsp.into(),
            dest: Reg::Rbp.into(),
        },
    ];
    let mut callee_vec: Vec<&Reg> = prog.used_callee.iter().collect();
    callee_vec.sort();
    for callee_saved in callee_vec {
        prelude.push(Instruction::PushQ {
            arg: callee_saved.clone().into(),
        });
    }
    let used_space = used_space(prog);
    prelude.push(Instruction::SubQ {
        src: Arg::Immediate(used_space),
        dest: Reg::Rsp.into(),
    });
    prelude.push(Instruction::Jump {
        label: "start".to_owned(),
    });
    prelude
}

fn generate_conclusion(prog: &Program) -> Vec<Instruction<Arg>> {
    let used_space = used_space(prog);
    let mut conc = vec![Instruction::AddQ {
        src: Arg::Immediate(used_space),
        dest: Reg::Rsp.into(),
    }];

    let mut callee_vec: Vec<&Reg> = prog.used_callee.iter().collect();
    callee_vec.sort();
    callee_vec.reverse();
    for callee_saved in callee_vec {
        conc.push(Instruction::PopQ {
            arg: callee_saved.clone().into(),
        });
    }
    conc.push(Instruction::PopQ {
        arg: Reg::Rbp.into(),
    });
    conc.push(Instruction::MovQ {
        src: Arg::Immediate(0),
        dest: Reg::Rax.into(),
    });
    conc.push(Instruction::RetQ);
    conc
}

#[cfg(test)]
mod generate_prelude_conclusion_tests {
    use super::generate_prelude_conclusion;
    use std::collections::HashSet;
    use syntax::x86::{Arg, Block, Cc, Instruction, Program, Reg};

    #[test]
    fn generate_example() {
        let start = vec![
            Instruction::MovQ {
                src: Arg::Immediate(1),
                dest: Reg::Rbx.into(),
            },
            Instruction::MovQ {
                src: Arg::Immediate(42),
                dest: Reg::Rcx.into(),
            },
            Instruction::AddQ {
                src: Arg::Immediate(7),
                dest: Reg::Rbx.into(),
            },
            Instruction::MovQ {
                src: Reg::Rbx.into(),
                dest: Arg::Deref(Reg::Rbp, -16),
            },
            Instruction::AddQ {
                src: Reg::Rcx.into(),
                dest: Reg::Rbx.into(),
            },
            Instruction::MovQ {
                src: Arg::Deref(Reg::Rbp, -16),
                dest: Reg::Rcx.into(),
            },
            Instruction::NegQ {
                arg: Reg::Rcx.into(),
            },
            Instruction::MovQ {
                src: Reg::Rbx.into(),
                dest: Reg::Rax.into(),
            },
            Instruction::AddQ {
                src: Reg::Rcx.into(),
                dest: Reg::Rax.into(),
            },
            Instruction::Jump {
                label: "conclusion".to_owned(),
            },
        ];
        let mut prog = Program::new(8, HashSet::from([Reg::Rbx]));
        prog.blocks.push(Block::new("start", start.clone()));
        let result = generate_prelude_conclusion(prog);
        let mut expected = Program::new(8, HashSet::from([Reg::Rbx]));

        expected.blocks.push(Block::new("start", start));
        expected.blocks.push(Block::new(
            "main",
            vec![
                Instruction::PushQ {
                    arg: Reg::Rbp.into(),
                },
                Instruction::MovQ {
                    src: Reg::Rsp.into(),
                    dest: Reg::Rbp.into(),
                },
                Instruction::PushQ {
                    arg: Reg::Rbx.into(),
                },
                Instruction::SubQ {
                    src: Arg::Immediate(8),
                    dest: Reg::Rsp.into(),
                },
                Instruction::Jump {
                    label: "start".to_owned(),
                },
            ],
        ));
        expected.blocks.push(Block::new(
            "conclusion",
            vec![
                Instruction::AddQ {
                    src: Arg::Immediate(8),
                    dest: Reg::Rsp.into(),
                },
                Instruction::PopQ {
                    arg: Reg::Rbx.into(),
                },
                Instruction::PopQ {
                    arg: Reg::Rbp.into(),
                },
                Instruction::MovQ {
                    src: Arg::Immediate(0),
                    dest: Reg::Rax.into(),
                },
                Instruction::RetQ,
            ],
        ));
        assert_eq!(result, expected)
    }

    #[test]
    fn generate_if() {
        let mut prog = Program::new(0, HashSet::from([Reg::R13, Reg::R12, Reg::R14, Reg::Rbx]));
        prog.blocks.push(Block::new(
            "start",
            vec![
                Instruction::CallQ {
                    label: "read_int".to_owned(),
                },
                Instruction::MovQ {
                    src: Reg::Rax.into(),
                    dest: Reg::Rcx.into(),
                },
                Instruction::CmpQ {
                    left: Reg::Rcx.into(),
                    right: 1.into(),
                },
                Instruction::JumpCC {
                    cc: Cc::E,
                    label: "block_0".to_owned(),
                },
                Instruction::Jump {
                    label: "block_1".to_owned(),
                },
            ],
        ));
        prog.blocks.push(Block::new(
            "block_0",
            vec![
                Instruction::MovQ {
                    src: 0.into(),
                    dest: Reg::Rax.into(),
                },
                Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        ));
        prog.blocks.push(Block::new(
            "block_1",
            vec![
                Instruction::MovQ {
                    src: 42.into(),
                    dest: Reg::Rax.into(),
                },
                Instruction::Jump {
                    label: "conclusion".to_owned(),
                },
            ],
        ));
        let result = generate_prelude_conclusion(prog.clone());
        let mut expected = prog;
        expected.blocks.push(Block::new(
            "main",
            vec![
                Instruction::PushQ {
                    arg: Reg::Rbp.into(),
                },
                Instruction::MovQ {
                    src: Reg::Rsp.into(),
                    dest: Reg::Rbp.into(),
                },
                Instruction::PushQ {
                    arg: Reg::Rbx.into(),
                },
                Instruction::PushQ {
                    arg: Reg::R12.into(),
                },
                Instruction::PushQ {
                    arg: Reg::R13.into(),
                },
                Instruction::PushQ {
                    arg: Reg::R14.into(),
                },
                Instruction::SubQ {
                    src: 0.into(),
                    dest: Reg::Rsp.into(),
                },
                Instruction::Jump {
                    label: "start".to_owned(),
                },
            ],
        ));
        expected.blocks.push(Block::new(
            "conclusion",
            vec![
                Instruction::AddQ {
                    src: 0.into(),
                    dest: Reg::Rsp.into(),
                },
                Instruction::PopQ {
                    arg: Reg::R14.into(),
                },
                Instruction::PopQ {
                    arg: Reg::R13.into(),
                },
                Instruction::PopQ {
                    arg: Reg::R12.into(),
                },
                Instruction::PopQ {
                    arg: Reg::Rbx.into(),
                },
                Instruction::PopQ {
                    arg: Reg::Rbp.into(),
                },
                Instruction::MovQ {
                    src: 0.into(),
                    dest: Reg::Rax.into(),
                },
                Instruction::RetQ,
            ],
        ));
        assert_eq!(result, expected)
    }
}
