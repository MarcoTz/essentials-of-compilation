use syntax::x86::{Arg, Instruction, Program, Reg};

pub fn patch_instructions(prog: Program) -> Program {
    let mut patched = Program::new(prog.stack_space, prog.used_callee);
    for (label, instrs) in prog.blocks {
        let mut new_block = vec![];
        for instr in instrs {
            new_block.extend(patch_instr(instr));
        }
        patched.add_block(&label, new_block);
    }
    patched
}

fn patch_instr(instr: Instruction<Arg>) -> Vec<Instruction<Arg>> {
    match instr {
        Instruction::AddQ {
            src: Arg::Deref(r1, o1),
            dest: Arg::Deref(r2, o2),
        } => vec![
            Instruction::MovQ {
                src: Arg::Deref(r1, o1),
                dest: Reg::Rax.into(),
            },
            Instruction::AddQ {
                src: Reg::Rax.into(),
                dest: Arg::Deref(r2, o2),
            },
        ],
        Instruction::AddQ { src, dest } => vec![Instruction::AddQ { src, dest }],
        Instruction::SubQ {
            src: Arg::Deref(r1, o1),
            dest: Arg::Deref(r2, o2),
        } => vec![
            Instruction::MovQ {
                src: Arg::Deref(r1, o1),
                dest: Reg::Rax.into(),
            },
            Instruction::SubQ {
                src: Reg::Rax.into(),
                dest: Arg::Deref(r2, o2),
            },
        ],
        Instruction::SubQ { src, dest } => vec![Instruction::SubQ { src, dest }],
        Instruction::NegQ { arg } => vec![Instruction::NegQ { arg }],
        Instruction::MovQ {
            src: Arg::Deref(r1, o1),
            dest: Arg::Deref(r2, o2),
        } => vec![
            Instruction::MovQ {
                src: Arg::Deref(r1, o1),
                dest: Reg::Rax.into(),
            },
            Instruction::MovQ {
                src: Reg::Rax.into(),
                dest: Arg::Deref(r2, o2),
            },
        ],
        Instruction::MovQ { src, dest } => vec![Instruction::MovQ { src, dest }],
        Instruction::PushQ { arg } => vec![Instruction::PushQ { arg }],
        Instruction::PopQ { arg } => vec![Instruction::PopQ { arg }],
        Instruction::CallQ { label } => vec![Instruction::CallQ { label }],
        Instruction::RetQ => vec![Instruction::RetQ],
        Instruction::Jump { label } => vec![Instruction::Jump { label }],
    }
}

#[cfg(test)]
mod patch_instructions_tests {
    use super::patch_instructions;
    use std::collections::HashSet;
    use syntax::x86::{Arg, Instruction, Program, Reg};

    #[test]
    fn patch_mov() {
        let mut prog = Program::new(16, HashSet::new());
        prog.add_block(
            "start",
            vec![
                Instruction::MovQ {
                    src: Arg::Immediate(42),
                    dest: Arg::Deref(Reg::Rbp, -8),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, -8),
                    dest: Arg::Deref(Reg::Rbp, -16),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, -16),
                    dest: Reg::Rax.into(),
                },
            ],
        );
        let result = patch_instructions(prog);
        let mut expected = Program::new(16, HashSet::new());
        expected.add_block(
            "start",
            vec![
                Instruction::MovQ {
                    src: Arg::Immediate(42),
                    dest: Arg::Deref(Reg::Rbp, -8),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, -8),
                    dest: Reg::Rax.into(),
                },
                Instruction::MovQ {
                    src: Reg::Rax.into(),
                    dest: Arg::Deref(Reg::Rbp, -16),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, -16),
                    dest: Reg::Rax.into(),
                },
            ],
        );
        assert_eq!(result, expected)
    }
}
