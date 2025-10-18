use crate::{
    colors::{Coloring, coloring_to_assignment},
    errors::Error,
    program::AnnotProg,
};
use asm::{Arg, Block, Instruction, Program, VarArg};
use std::collections::HashMap;

mod collect_callee;
mod collect_vars;
use collect_callee::collect_callee;
use collect_vars::collect_vars;

pub fn assign_homes(prog: AnnotProg, coloring: Coloring) -> Result<Program, Error> {
    let used_callee = collect_callee(&prog);
    let vars = collect_vars(&prog);
    let stack_space = vars.len() as u64 * 8;
    let assignments = coloring_to_assignment(coloring);
    let mut assigned = Program::new(stack_space, used_callee);
    for (label, instrs) in prog.blocks {
        let assigned_instrs = instrs
            .into_iter()
            .map(|instr| assign_instr(instr.instr, &assignments))
            .collect::<Result<Vec<_>, Error>>()?;
        assigned.blocks.push(Block::new(&label, assigned_instrs));
    }
    Ok(assigned)
}

fn assign_instr(
    instr: Instruction<VarArg>,
    assignments: &HashMap<String, Arg>,
) -> Result<Instruction<Arg>, Error> {
    match instr {
        Instruction::AddQ { src, dest } => Ok(Instruction::AddQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::SubQ { src, dest } => Ok(Instruction::SubQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::NegQ { arg } => Ok(Instruction::NegQ {
            arg: assign_arg(arg, assignments)?,
        }),
        Instruction::MovQ { src, dest } => Ok(Instruction::MovQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::PushQ { arg } => Ok(Instruction::PushQ {
            arg: assign_arg(arg, assignments)?,
        }),
        Instruction::PopQ { arg } => Ok(Instruction::PopQ {
            arg: assign_arg(arg, assignments)?,
        }),
        Instruction::CallQ { label } => Ok(Instruction::CallQ { label }),
        Instruction::RetQ => Ok(Instruction::RetQ),
        Instruction::Jump { label } => Ok(Instruction::Jump { label }),
        Instruction::XorQ { src, dest } => Ok(Instruction::XorQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::CmpQ { left, right } => Ok(Instruction::CmpQ {
            left: assign_arg(left, assignments)?,
            right: assign_arg(right, assignments)?,
        }),
        Instruction::SetCC { cc, dest } => Ok(Instruction::SetCC {
            cc,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::MovZBQ { src, dest } => Ok(Instruction::MovZBQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::JumpCC { cc, label } => Ok(Instruction::JumpCC { cc, label }),
        Instruction::AndQ { src, dest } => Ok(Instruction::AndQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
        Instruction::OrQ { src, dest } => Ok(Instruction::OrQ {
            src: assign_arg(src, assignments)?,
            dest: assign_arg(dest, assignments)?,
        }),
    }
}
fn assign_arg(arg: VarArg, assignments: &HashMap<String, Arg>) -> Result<Arg, Error> {
    match arg {
        VarArg::Arg(arg) => Ok(arg),
        VarArg::Var(v) => Ok(assignments.get(&v).ok_or(Error::NoAssignment(v))?.clone()),
    }
}

#[cfg(test)]
mod assign_homes_tests {
    use super::{Coloring, assign_homes};
    use crate::program::{AnnotProg, LiveInstruction};
    use asm::{Arg, Block, Instruction, Program, Reg, VarArg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn assign_ab() {
        let mut prog = AnnotProg::new();
        prog.add_block(
            "start",
            vec![
                LiveInstruction {
                    instr: Instruction::MovQ {
                        src: Arg::Immediate(42).into(),
                        dest: VarArg::Var("a".to_owned()),
                    },
                    live_before: HashSet::new(),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    instr: Instruction::MovQ {
                        src: VarArg::Var("a".to_owned()),
                        dest: VarArg::Var("b".to_owned()),
                    },
                    live_before: HashSet::new(),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    instr: Instruction::MovQ {
                        src: VarArg::Var("b".to_owned()),
                        dest: Reg::Rax.into(),
                    },
                    live_before: HashSet::new(),
                    live_after: HashSet::new(),
                },
            ],
        );
        let result = assign_homes(
            prog,
            Coloring(HashMap::from([("a".into(), 11), ("b".into(), 12)])),
        )
        .unwrap();
        let block_fun = |offset1: i64, offset2: i64| {
            vec![
                Instruction::MovQ {
                    src: Arg::Immediate(42),
                    dest: Arg::Deref(Reg::Rbp, offset1),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, offset1),
                    dest: Arg::Deref(Reg::Rbp, offset2),
                },
                Instruction::MovQ {
                    src: Arg::Deref(Reg::Rbp, offset2),
                    dest: Reg::Rax.into(),
                },
            ]
        };
        let mut expected1 = Program::new(16, HashSet::new());
        expected1
            .blocks
            .push(Block::new("start", block_fun(-8, -16)));
        let mut expected2 = Program::new(16, HashSet::new());
        expected2
            .blocks
            .push(Block::new("start", block_fun(-16, -8)));
        assert!(result == expected1 || result == expected2)
    }
}
