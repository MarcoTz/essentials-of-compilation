use crate::{Arg, Instruction, Reg};

pub trait PatchInstructions {
    type Target;
    fn patch_instructions(self) -> Self::Target;
}

pub fn remove_double_deref(
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
    use super::PatchInstructions;
    use crate::{Arg, Block, Instruction, Program, Reg};
    use std::collections::HashSet;

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
        let result = prog.patch_instructions();
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
