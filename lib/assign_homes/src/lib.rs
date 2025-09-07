use color_graph::{Coloring, color_to_arg};
use std::collections::{HashMap, HashSet};
use syntax::x86::{Arg, Instruction, Program, Reg, VarArg, VarProgram};

pub mod color_graph;
pub mod interference_graph;
pub mod uncover_live;
use uncover_live::Location;

fn coloring_to_assignment(coloring: Coloring) -> HashMap<String, Arg> {
    let mut assignments = HashMap::new();
    for (loc, color) in coloring {
        let arg = color_to_arg(color);
        match (loc, &arg) {
            (Location::Variable(v), _) => {
                assignments.insert(v, arg);
            }
            (Location::Register(reg1), Arg::Register(reg2)) if reg1 == *reg2 => (),
            (Location::Stack(offset1), Arg::Deref(Reg::Rbp, offset2)) if offset1 == *offset2 => (),
            (loc, _) => panic!("Locations {loc} and {arg} do not match"),
        }
    }
    assignments
}

pub fn assign_homes(prog: VarProgram, coloring: Coloring) -> Program {
    let used_callee = collect_callee(&prog);
    let vars = collect_vars(&prog);
    let stack_space = vars.len() as u64 * 8;
    let assignments = coloring_to_assignment(coloring);
    let mut assigned = Program::new(stack_space, used_callee);
    for (label, instrs) in prog.blocks {
        assigned.add_block(
            &label,
            instrs
                .into_iter()
                .map(|instr| assign_instr(instr, &assignments))
                .collect(),
        );
    }
    assigned
}

fn collect_callee(prog: &VarProgram) -> HashSet<Reg> {
    let mut callee = HashSet::new();
    for (_, block) in prog.blocks.iter() {
        for instr in block.iter() {
            callee = &callee | &collect_callee_instr(instr);
        }
    }
    callee
}

fn collect_callee_instr(instr: &Instruction<VarArg>) -> HashSet<Reg> {
    match instr {
        Instruction::AddQ { src, dest } => &collect_callee_arg(src) | &collect_callee_arg(dest),
        Instruction::SubQ { src, dest } => &collect_callee_arg(src) | &collect_callee_arg(dest),
        Instruction::NegQ { arg } => collect_callee_arg(arg),
        Instruction::MovQ { src, dest } => &collect_callee_arg(src) | &collect_callee_arg(dest),
        Instruction::PushQ { arg } => collect_callee_arg(arg),
        Instruction::PopQ { arg } => collect_callee_arg(arg),
        Instruction::CallQ { .. } => HashSet::new(),
        Instruction::RetQ => HashSet::new(),
        Instruction::Jump { .. } => HashSet::new(),
    }
}

fn collect_callee_arg(arg: &VarArg) -> HashSet<Reg> {
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
    }
}

fn collect_vars(prog: &VarProgram) -> HashSet<String> {
    let mut vars = HashSet::new();
    for (_, instrs) in prog.blocks.iter() {
        for var_set in instrs.iter().map(collect_instr) {
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
    }
}

fn collect_arg(arg: &VarArg) -> HashSet<String> {
    match arg {
        VarArg::Arg(_) => HashSet::new(),
        VarArg::Var(v) => HashSet::from([v.clone()]),
    }
}

fn assign_instr(
    instr: Instruction<VarArg>,
    assignments: &HashMap<String, Arg>,
) -> Instruction<Arg> {
    match instr {
        Instruction::AddQ { src, dest } => Instruction::AddQ {
            src: assign_arg(src, assignments),
            dest: assign_arg(dest, assignments),
        },
        Instruction::SubQ { src, dest } => Instruction::SubQ {
            src: assign_arg(src, assignments),
            dest: assign_arg(dest, assignments),
        },
        Instruction::NegQ { arg } => Instruction::NegQ {
            arg: assign_arg(arg, assignments),
        },
        Instruction::MovQ { src, dest } => Instruction::MovQ {
            src: assign_arg(src, assignments),
            dest: assign_arg(dest, assignments),
        },
        Instruction::PushQ { arg } => Instruction::PushQ {
            arg: assign_arg(arg, assignments),
        },
        Instruction::PopQ { arg } => Instruction::PopQ {
            arg: assign_arg(arg, assignments),
        },
        Instruction::CallQ { label } => Instruction::CallQ { label },
        Instruction::RetQ => Instruction::RetQ,
        Instruction::Jump { label } => Instruction::Jump { label },
    }
}

fn assign_arg(arg: VarArg, assignments: &HashMap<String, Arg>) -> Arg {
    match arg {
        VarArg::Arg(arg) => arg,
        VarArg::Var(v) => assignments.get(&v).unwrap().clone(),
    }
}

#[cfg(test)]
mod assign_homes_tests {
    use super::assign_homes;
    use std::collections::{HashMap, HashSet};
    use syntax::x86::{Arg, Instruction, Program, Reg, VarArg, VarProgram};
    #[test]
    fn assign_ab() {
        let mut prog = VarProgram::new();
        prog.add_block(
            "start",
            vec![
                Instruction::MovQ {
                    src: Arg::Immediate(42).into(),
                    dest: VarArg::Var("a".to_owned()),
                },
                Instruction::MovQ {
                    src: VarArg::Var("a".to_owned()),
                    dest: VarArg::Var("b".to_owned()),
                },
                Instruction::MovQ {
                    src: VarArg::Var("b".to_owned()),
                    dest: Reg::Rax.into(),
                },
            ],
        );
        let result = assign_homes(prog, HashMap::from([("a".into(), 11), ("b".into(), 12)]));
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
        expected1.add_block("start", block_fun(-8, -16));
        let mut expected2 = Program::new(16, HashSet::new());
        expected2.add_block("start", block_fun(-16, -8));
        assert!(result == expected1 || result == expected2)
    }
}
