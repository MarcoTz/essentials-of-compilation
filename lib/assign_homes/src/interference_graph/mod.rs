use crate::uncover_live::{AnnotProg, LiveInstruction, Location, written_locations};
use std::collections::HashSet;
use syntax::x86::Instruction;

mod edge;
mod graph;
pub use graph::InterferenceGraph;

pub fn build_graph(prog: &AnnotProg) -> InterferenceGraph {
    let mut graph = InterferenceGraph::new();
    for (_, block) in prog.blocks.iter() {
        build_block(block, &mut graph);
    }
    graph
}

fn build_block(block: &[LiveInstruction], graph: &mut InterferenceGraph) {
    for instr in block {
        if let Instruction::MovQ { ref src, ref dest } = instr.instr {
            mov_edges(
                Location::arg_loc(src.clone()),
                Location::arg_loc(dest.clone()),
                &instr.live_after,
                graph,
            );
            continue;
        }
        let written = written_locations(&instr.instr);
        for write_loc in written.iter() {
            for after_loc in instr.live_after.iter() {
                if write_loc != after_loc {
                    graph.add_edge(write_loc.clone(), after_loc.clone())
                }
            }
        }
    }
}

fn mov_edges(
    src: Option<Location>,
    dest: Option<Location>,
    live_after: &HashSet<Location>,
    graph: &mut InterferenceGraph,
) {
    let dest = match dest {
        Some(dst) => dst,
        None => return,
    };

    for after in live_after {
        if *after == dest {
            continue;
        }
        if let Some(ref s) = src
            && after == s
        {
            continue;
        }

        graph.add_edge(dest.clone(), after.clone())
    }
}

#[cfg(test)]
mod interference_graph_tests {
    use super::{InterferenceGraph, build_graph};
    use std::collections::HashSet;
    use syntax::x86::{Instruction, Reg};
    use uncover_live::{AnnotProg, LiveInstruction};

    #[test]
    fn build_example() {
        let mut example = AnnotProg::new();
        example.add_block(
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
        let result = build_graph(&example);
        let mut expected = InterferenceGraph::new();
        expected.add_edge("t".into(), Reg::Rax.into());
        expected.add_edge("t".into(), "z".into());
        expected.add_edge("t".into(), Reg::Rsp.into());
        expected.add_edge(Reg::Rax.into(), Reg::Rsp.into());
        expected.add_edge("z".into(), "y".into());
        expected.add_edge("z".into(), Reg::Rsp.into());
        expected.add_edge("z".into(), "w".into());
        expected.add_edge("y".into(), "w".into());
        expected.add_edge("y".into(), Reg::Rsp.into());
        expected.add_edge("w".into(), "x".into());
        expected.add_edge("w".into(), Reg::Rsp.into());
        expected.add_edge("w".into(), "v".into());
        expected.add_edge("x".into(), Reg::Rsp.into());
        expected.add_edge("y".into(), Reg::Rsp.into());
        expected.add_edge("v".into(), Reg::Rsp.into());
        assert_eq!(result, expected)
    }
}
