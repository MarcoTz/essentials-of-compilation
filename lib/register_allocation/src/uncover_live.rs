use crate::{
    errors::Error,
    program::{LiveBlock, LiveInstruction, LiveProg, Location, location::arg_locations},
};
use asm::{Instruction, Reg, VarProgram};
use definitions::PRINT_CALL;
use std::collections::{HashMap, HashSet};

pub fn uncover_live(prog: VarProgram) -> Result<LiveProg, Error> {
    let mut annot: LiveProg = prog.into();
    let mut label2live = HashMap::new();
    label2live.insert(
        "conclusion".to_owned(),
        HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
    );

    let mut changing = true;
    while changing {
        changing = false;
        for block in annot.blocks.iter_mut() {
            if block.label == "conclusion" || block.label == "main" {
                continue;
            }
            changing = changing || uncover_block(block, &label2live)?;
            label2live.insert(block.label.clone(), (block.instrs[0]).live_before.clone());
        }
    }

    Ok(annot)
}

fn uncover_block(
    block: &mut LiveBlock,
    label2live: &HashMap<String, HashSet<Location>>,
) -> Result<bool, Error> {
    let mut last_after = HashSet::new();

    block.instrs.reverse();
    let mut changing = false;
    for instr in block.instrs.iter_mut() {
        changing = changing || live_before(instr, &last_after, label2live)?;
        instr.live_after = last_after;
        last_after = instr.live_before.clone();
    }
    block.instrs.reverse();
    Ok(changing)
}

fn live_before(
    instr: &mut LiveInstruction,
    live_after: &HashSet<Location>,
    label2live: &HashMap<String, HashSet<Location>>,
) -> Result<bool, Error> {
    if let Instruction::Jump { label } = &instr.instr {
        let live_before = label2live
            .get(label)
            .ok_or(Error::MissingLiveBefore(label.clone()))?
            .clone();
        let changed = live_before != instr.live_before;
        instr.live_before = &instr.live_before | &live_before;
        return Ok(changed);
    }
    let written = written_locations(instr);
    let read = read_locations(instr);
    let next_live_before = &(live_after - &written) | &read;
    let changed = next_live_before != instr.live_before;
    instr.live_before = &instr.live_before | &next_live_before;
    Ok(changed)
}

pub fn written_locations(instr: &LiveInstruction) -> HashSet<Location> {
    match &instr.instr {
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
        Instruction::XorQ { dest, .. } => arg_locations(dest),
        Instruction::CmpQ { .. } => HashSet::new(),
        Instruction::SetCC { dest, .. } => arg_locations(dest),
        Instruction::MovZBQ { dest, .. } => arg_locations(dest),
        Instruction::JumpCC { .. } => HashSet::new(),
        Instruction::AndQ { dest, .. } => arg_locations(dest),
        Instruction::OrQ { dest, .. } => arg_locations(dest),
    }
}

fn read_locations(instr: &LiveInstruction) -> HashSet<Location> {
    match &instr.instr {
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
        Instruction::XorQ { src, dest, .. } => &arg_locations(src) | &arg_locations(dest),
        Instruction::CmpQ { left, right } => &arg_locations(left) | &arg_locations(right),
        Instruction::SetCC { .. } => HashSet::new(),
        Instruction::MovZBQ { src, .. } => arg_locations(src),
        Instruction::JumpCC { .. } => HashSet::new(),
        Instruction::AndQ { src, dest } => &arg_locations(src) | &arg_locations(dest),
        Instruction::OrQ { src, dest } => &arg_locations(src) | &arg_locations(dest),
    }
}

#[cfg(test)]
mod uncover_live_tests {
    use super::{LiveBlock, LiveInstruction, LiveProg, uncover_live};
    use asm::{Instruction, Reg, VarProgram};
    use std::collections::HashSet;

    #[test]
    fn uncover_example() {
        let mut example = VarProgram::new();
        example.add_block(
            "start",
            vec![
                Instruction::mov(5, "a"),
                Instruction::mov(30, "b"),
                Instruction::mov("a", "c"),
                Instruction::mov(10, "b"),
                Instruction::add("b", "c"),
            ],
        );
        let result = uncover_live(example).unwrap();
        let mut expected = LiveProg::new();
        expected.blocks.push(LiveBlock {
            label: "start".to_owned(),
            instrs: vec![
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
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_registers() {
        let mut example = VarProgram::new();
        example.add_block(
            "start",
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
        let result = uncover_live(example).unwrap();
        let mut expected = LiveProg::new();
        expected.blocks.push(LiveBlock {
            label: "start".to_owned(),
            instrs: vec![
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
        });
        assert_eq!(result, expected)
    }
}
