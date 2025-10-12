use syntax::x86::{Arg, Block, Instruction, Program, Reg};

pub fn patch_instructions(prog: Program) -> Program {
    let mut patched = Program::new(prog.stack_space, prog.used_callee);
    for block in prog.blocks {
        patched.blocks.push(patch_block(block));
    }
    patched
}

fn patch_block(block: Block<Arg>) -> Block<Arg> {
    let mut new_instrs = vec![];
    for instr in block.instrs {
        new_instrs.extend(patch_instr(instr));
    }
    Block::new(&block.label, new_instrs)
}

fn patch_instr(instr: Instruction<Arg>) -> Vec<Instruction<Arg>> {
    match instr {
        Instruction::NegQ { arg } => vec![Instruction::NegQ { arg }],
        Instruction::PushQ { arg } => vec![Instruction::PushQ { arg }],
        Instruction::PopQ { arg } => vec![Instruction::PopQ { arg }],
        Instruction::CallQ { label } => vec![Instruction::CallQ { label }],
        Instruction::RetQ => vec![Instruction::RetQ],
        Instruction::Jump { label } => vec![Instruction::Jump { label }],
        Instruction::JumpCC { cc, label } => vec![Instruction::JumpCC { cc, label }],
        Instruction::NotQ { arg } => vec![Instruction::NotQ { arg }],
        Instruction::SetCC { cc, dest } => vec![Instruction::SetCC { cc, dest }],

        Instruction::CmpQ {
            left: Arg::Immediate(i),
            right,
        } => vec![
            Instruction::MovQ {
                src: Arg::Immediate(i),
                dest: Reg::Rax.into(),
            },
            Instruction::CmpQ {
                left: Reg::Rax.into(),
                right,
            },
        ],
        Instruction::MovZBQ {
            src,
            dest: Arg::Register(reg),
        } => vec![Instruction::MovZBQ {
            src,
            dest: Arg::Register(reg),
        }],

        Instruction::AddQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::AddQ { src, dest })
        }
        Instruction::SubQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::SubQ { src, dest })
        }
        Instruction::MovQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::MovQ { src, dest })
        }
        Instruction::XorQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::XorQ { src, dest })
        }
        Instruction::CmpQ { left, right } => {
            remove_double_deref(left, right, |left, right| Instruction::CmpQ { left, right })
        }
        Instruction::MovZBQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::MovZBQ { src, dest })
        }
        Instruction::AndQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::AddQ { src, dest })
        }
        Instruction::OrQ { src, dest } => {
            remove_double_deref(src, dest, |src, dest| Instruction::OrQ { src, dest })
        }
    }
}

fn remove_double_deref(
    src: Arg,
    dest: Arg,
    instr: impl FnOnce(Arg, Arg) -> Instruction<Arg>,
) -> Vec<Instruction<Arg>> {
    match (&src, &dest) {
        (Arg::Deref(_, _), Arg::Deref(_, _)) => vec![
            Instruction::MovQ {
                src,
                dest: Reg::Rax.into(),
            },
            instr(Reg::Rax.into(), dest),
        ],
        _ => vec![instr(src, dest)],
    }
}

#[cfg(test)]
mod patch_instructions_tests {
    use super::patch_instructions;
    use std::collections::HashSet;
    use syntax::x86::{Arg, Block, Instruction, Program, Reg};

    #[test]
    fn patch_mov() {
        let mut prog = Program::new(16, HashSet::new());
        prog.blocks.push(Block::new(
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
        ));
        let result = patch_instructions(prog);
        let mut expected = Program::new(16, HashSet::new());
        expected.blocks.push(Block::new(
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
        ));
        assert_eq!(result, expected)
    }
}
