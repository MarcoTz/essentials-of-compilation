use crate::{
    graph::LocationGraph,
    program::{AnnotProg, LiveInstruction, Location},
};
use asm::{Instruction, VarArg};

pub fn build_move_graph(prog: &AnnotProg) -> LocationGraph {
    let mut graph = LocationGraph::new();
    for (_, block) in prog.blocks.iter() {
        build_block(block, &mut graph);
    }
    graph
}

fn build_block(block: &[LiveInstruction], graph: &mut LocationGraph) {
    for instr in block.iter() {
        build_instr(instr, graph);
    }
}

fn build_instr(instr: &LiveInstruction, graph: &mut LocationGraph) {
    if let Instruction::MovQ {
        src: VarArg::Var(v1),
        dest: VarArg::Var(v2),
    } = &instr.instr
    {
        graph.add_edge(
            Location::Variable(v1.clone()),
            Location::Variable(v2.clone()),
        )
    }
}

#[cfg(test)]
mod move_graph_tests {
    use super::{AnnotProg, LiveInstruction, LocationGraph, build_move_graph};
    use asm::{Instruction, Reg};
    use std::collections::HashSet;

    #[test]
    fn build_example() {
        let mut prog = AnnotProg::new();
        prog.add_block(
            "main",
            vec![
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov(1, "v"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov(42, "w"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov("v", "x"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::add(7, "x"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov("x", "y"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov("x", "z"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::add("w", "z"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov("y", "t"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::neg("t"),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::mov("z", Reg::Rax),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::add("t", Reg::Rax),
                    live_after: HashSet::new(),
                },
                LiveInstruction {
                    live_before: HashSet::new(),
                    instr: Instruction::jmp("conclusion"),
                    live_after: HashSet::new(),
                },
            ],
        );
        let result = build_move_graph(&prog);
        let mut expected = LocationGraph::new();
        expected.add_edge("t".into(), "y".into());
        expected.add_edge("y".into(), "x".into());
        expected.add_edge("z".into(), "x".into());
        expected.add_edge("x".into(), "v".into());
        assert_eq!(result, expected)
    }
}
