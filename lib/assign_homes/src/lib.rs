use std::collections::{HashMap, HashSet};
use syntax::x86::{Arg, Instruction, Program, Reg, VarArg, VarProgram};

pub fn assign_homes(prog: VarProgram) -> Program {
    let vars = collect_vars(&prog);
    let used_callee = collect_callee(&prog);
    let stack_space = vars.len() as u64 * 8;
    let assignments = assign_vars(vars);
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
        for var_set in instrs.iter().map(|instr| collect_instr(instr)) {
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

fn assign_vars(vars: HashSet<String>) -> HashMap<String, i64> {
    let mut offset = -8;
    let mut assignments = HashMap::new();
    for var in vars {
        assignments.insert(var, offset);
        offset -= 8;
    }
    assignments
}

fn assign_instr(
    instr: Instruction<VarArg>,
    assignments: &HashMap<String, i64>,
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

fn assign_arg(arg: VarArg, assignments: &HashMap<String, i64>) -> Arg {
    match arg {
        VarArg::Arg(arg) => arg,
        VarArg::Var(v) => {
            let offset = assignments.get(&v).unwrap();
            Arg::Deref(Reg::Rbp, *offset)
        }
    }
}
