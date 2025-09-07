use crate::program::{AnnotProg, LiveInstruction, Location, location::arg_locations};
use std::collections::{HashMap, HashSet};
use syntax::{
    PRINT_CALL,
    x86::{Instruction, Reg, VarArg, VarProgram},
};

pub fn uncover_live(prog: VarProgram) -> AnnotProg {
    let mut annot = AnnotProg::new();
    let mut label2live = HashMap::new();
    label2live.insert(
        "conclusion".to_owned(),
        HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
    );
    for (label, instrs) in prog.blocks {
        let uncovered = uncover_block(instrs, &label2live);
        label2live.insert(label.clone(), (uncovered[0]).live_before.clone());
        annot.add_block(&label, uncovered);
    }
    annot
}

fn uncover_block(
    mut block: Vec<Instruction<VarArg>>,
    label2live: &HashMap<String, HashSet<Location>>,
) -> Vec<LiveInstruction> {
    let mut live_sets = vec![];
    let mut last_after = HashSet::new();

    let num_instrs = block.len();
    for _ in 0..num_instrs {
        let curr_instr = block.remove(block.len() - 1);
        let current_live = live_before(&curr_instr, &last_after, label2live);
        live_sets.push(LiveInstruction::new(
            curr_instr,
            current_live.clone(),
            last_after,
        ));
        last_after = current_live;
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

pub fn written_locations(instr: &Instruction<VarArg>) -> HashSet<Location> {
    match instr {
        Instruction::AddQ { dest, .. } => arg_locations(dest),
        Instruction::SubQ { dest, .. } => arg_locations(dest),
        Instruction::NegQ { arg } => arg_locations(arg),
        Instruction::MovQ { dest, .. } => arg_locations(dest),
        Instruction::PushQ { .. } => HashSet::new(),
        Instruction::PopQ { .. } => HashSet::new(),
        Instruction::CallQ { .. } => Reg::caller_saved()
            .into_iter()
            .map(Location::Register)
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

#[cfg(test)]
mod uncover_live_tests {
    use super::{AnnotProg, LiveInstruction, uncover_live};
    use std::collections::HashSet;
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
        let result = uncover_live(example);
        let mut expected = AnnotProg::new();
        expected.add_block(
            "main",
            vec![
                LiveInstruction::new(
                    Instruction::mov(5, "a"),
                    HashSet::from([]),
                    HashSet::from(["a".into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov(30, "b"),
                    HashSet::from(["a".into()]),
                    HashSet::from(["a".into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("a", "c"),
                    HashSet::from(["a".into()]),
                    HashSet::from(["c".into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov(10, "b"),
                    HashSet::from(["c".into()]),
                    HashSet::from(["b".into(), "c".into()]),
                ),
                LiveInstruction::new(
                    Instruction::add("b", "c"),
                    HashSet::from(["b".into(), "c".into()]),
                    HashSet::new(),
                ),
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
        let result = uncover_live(example);
        let mut expected = AnnotProg::new();
        expected.add_block(
            "main",
            vec![
                LiveInstruction::new(
                    Instruction::mov(1, "v"),
                    HashSet::from([Reg::Rsp.into()]),
                    HashSet::from(["v".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov(42, "w"),
                    HashSet::from(["v".into(), Reg::Rsp.into()]),
                    HashSet::from(["v".into(), "w".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("v", "x"),
                    HashSet::from(["v".into(), "w".into(), Reg::Rsp.into()]),
                    HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::add(7, "x"),
                    HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                    HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("x", "y"),
                    HashSet::from(["w".into(), "x".into(), Reg::Rsp.into()]),
                    HashSet::from(["w".into(), "x".into(), "y".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("x", "z"),
                    HashSet::from(["w".into(), "x".into(), "y".into(), Reg::Rsp.into()]),
                    HashSet::from(["w".into(), "y".into(), "z".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::add("w", "z"),
                    HashSet::from(["w".into(), "y".into(), "z".into(), Reg::Rsp.into()]),
                    HashSet::from(["y".into(), "z".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("y", "t"),
                    HashSet::from(["y".into(), "z".into(), Reg::Rsp.into()]),
                    HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::neg("t"),
                    HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                    HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::mov("z", Reg::Rax),
                    HashSet::from(["t".into(), "z".into(), Reg::Rsp.into()]),
                    HashSet::from([Reg::Rax.into(), "t".into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::add("t", Reg::Rax),
                    HashSet::from([Reg::Rax.into(), "t".into(), Reg::Rsp.into()]),
                    HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
                ),
                LiveInstruction::new(
                    Instruction::jmp("conclusion"),
                    HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
                    HashSet::new(),
                ),
            ],
        );
        assert_eq!(result, expected)
    }
}
