use std::collections::{HashMap, HashSet};
use syntax::{
    PRINT_CALL,
    x86::{Arg, Instruction, Reg, VarArg, VarProgram},
};

mod location;
use location::Location;

pub fn uncover_live(prog: &VarProgram) -> HashMap<String, Vec<HashSet<Location>>> {
    let mut live = HashMap::new();
    let mut label2live = HashMap::new();
    label2live.insert(
        "conclusion".to_owned(),
        HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
    );
    for (label, instrs) in prog.blocks.iter() {
        let uncovered = uncover_block(&instrs, &mut label2live);
        label2live.insert(label.clone(), uncovered[0].clone());
        live.insert(label.clone(), uncovered);
    }
    live
}

fn uncover_block(
    block: &[Instruction<VarArg>],
    label2live: &HashMap<String, HashSet<Location>>,
) -> Vec<HashSet<Location>> {
    let mut live_sets = vec![];
    let mut last_after = HashSet::new();

    for i in 1..=block.len() {
        let curr_instr = &block[block.len() - i];
        println!("getting live for {curr_instr} ({i})");
        let current_live = live_before(curr_instr, &last_after, &label2live);
        last_after = current_live.clone();
        live_sets.push(current_live);
    }
    live_sets.reverse();
    live_sets
}

fn live_before(
    instr: &Instruction<VarArg>,
    live_after: &HashSet<Location>,
    label2live: &HashMap<String, HashSet<Location>>,
) -> HashSet<Location> {
    if let Instruction::Jump { label } = instr {
        return label2live.get(label).unwrap().clone();
    }
    let written = written_locations(instr);
    let read = read_locations(instr);
    &(live_after - &written) | &read
}

fn written_locations(instr: &Instruction<VarArg>) -> HashSet<Location> {
    match instr {
        Instruction::AddQ { dest, .. } => arg_locations(dest),
        Instruction::SubQ { dest, .. } => arg_locations(dest),
        Instruction::NegQ { arg } => arg_locations(arg),
        Instruction::MovQ { dest, .. } => arg_locations(dest),
        Instruction::PushQ { .. } => HashSet::new(),
        Instruction::PopQ { .. } => HashSet::new(),
        Instruction::CallQ { .. } => Reg::caller_saved()
            .into_iter()
            .map(|reg| Location::Register(reg))
            .collect(),
        Instruction::RetQ => HashSet::new(),
        Instruction::Jump { .. } => HashSet::new(),
    }
}

fn read_locations(instr: &Instruction<VarArg>) -> HashSet<Location> {
    match instr {
        Instruction::AddQ { src, dest } => &arg_locations(src) | &arg_locations(dest),
        Instruction::SubQ { dest, src } => &arg_locations(src) | &arg_locations(dest),
        Instruction::NegQ { arg } => arg_locations(arg),
        Instruction::MovQ { src, .. } => arg_locations(src),
        Instruction::PushQ { arg } => arg_locations(arg),
        Instruction::PopQ { arg } => arg_locations(arg),
        Instruction::CallQ { label } => {
            if label == PRINT_CALL {
                HashSet::from([Location::Register(Reg::Rdi)])
            } else {
                HashSet::new()
            }
        }
        Instruction::RetQ => HashSet::new(),
        Instruction::Jump { .. } => HashSet::new(),
    }
}

fn arg_locations(arg: &VarArg) -> HashSet<Location> {
    match arg {
        VarArg::Var(v) => HashSet::from([Location::Variable(v.clone())]),
        VarArg::Arg(Arg::Register(r)) => HashSet::from([Location::Register(r.clone())]),
        VarArg::Arg(Arg::Deref(_, offset)) => HashSet::from([Location::Stack(*offset)]),
        _ => HashSet::new(),
    }
}

#[cfg(test)]
mod uncover_live_tests {
    use super::uncover_live;
    use std::collections::{HashMap, HashSet};
    use syntax::x86::{Instruction, Reg, VarProgram};

    #[test]
    fn uncover_example() {
        let mut example = VarProgram::new();
        example.add_block(
            "main",
            vec![
                Instruction::mov(5, "a"),
                Instruction::mov(30, "b"),
                Instruction::mov("a", "c"),
                Instruction::mov(10, "b"),
                Instruction::add("b", "c"),
            ],
        );
        let result = uncover_live(&example);
        let mut expected = HashMap::new();
        expected.insert(
            "main".to_owned(),
            vec![
                HashSet::from([]),
                HashSet::from(["a".into()]),
                HashSet::from(["a".into()]),
                HashSet::from(["c".into()]),
                HashSet::from(["b".into(), "c".into()]),
            ],
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_registers() {
        let mut example = VarProgram::new();
        example.add_block(
            "main",
            vec![
                Instruction::mov(1, "v"),
                Instruction::mov(42, "w"),
                Instruction::mov("v", "x"),
                Instruction::add(7, "x"),
                Instruction::mov("x", "y"),
                Instruction::mov("x", "z"),
                Instruction::add("w", "z"),
                Instruction::mov("y", "t"),
                Instruction::neg("t"),
                Instruction::mov("z", Reg::Rax),
                Instruction::add("t", Reg::Rax),
                Instruction::jmp("conclusion"),
            ],
        );
        let result = uncover_live(&example);
        let mut expected = HashMap::new();
        expected.insert(
            "main".to_owned(),
            vec![
                HashSet::from([Reg::Rsp.into()]),
                HashSet::from(["v".into(), Reg::Rsp.into()]),
                HashSet::from(["v".into(), "w".into(), Reg::Rsp.into()]),
                HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                HashSet::from(["w".into(), "x".into(), "y".into(), Reg::Rsp.into()]),
                HashSet::from(["w".into(), "y".into(), "z".into(), Reg::Rsp.into()]),
                HashSet::from(["y".into(), "z".into(), Reg::Rsp.into()]),
                HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                HashSet::from([Reg::Rax.into(), "t".into(), Reg::Rsp.into()]),
                HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
            ],
        );
        assert_eq!(result, expected)
    }
}
