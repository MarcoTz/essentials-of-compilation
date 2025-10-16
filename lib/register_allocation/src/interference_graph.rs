use crate::{
    graph::LocationGraph,
    program::{AnnotProg, LiveInstruction, Location},
    uncover_live::written_locations,
};
use lang_x86::Instruction;
use std::collections::HashSet;

pub fn build_interference_graph(prog: &AnnotProg) -> LocationGraph {
    let mut graph = LocationGraph::new();
    for (_, block) in prog.blocks.iter() {
        build_block(block, &mut graph);
    }
    graph
}

fn build_block(block: &[LiveInstruction], graph: &mut LocationGraph) {
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
            if let Location::Variable(_) = write_loc {
                graph.add_vert(write_loc.clone());
            }
            for after_loc in instr.live_after.iter() {
                if let Location::Variable(_) = after_loc {
                    graph.add_vert(after_loc.clone());
                }
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
    graph: &mut LocationGraph,
) {
    let dest = match dest {
        Some(dst) => dst,
        None => return,
    };
    if let Location::Variable(_) = dest {
        graph.add_vert(dest.clone());
    }

    for after in live_after {
        if *after == dest {
            continue;
        }
        if let Some(ref s) = src
            && after == s
        {
            if let Location::Variable(_) = *s {
                graph.add_vert(s.clone());
            }
            continue;
        }

        graph.add_edge(dest.clone(), after.clone())
    }
}

#[cfg(test)]
mod interference_graph_tests {
    use super::{LocationGraph, build_interference_graph};
    use crate::program::{AnnotProg, LiveInstruction};
    use lang_x86::{Instruction, Reg};
    use std::collections::HashSet;

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
        let result = build_interference_graph(&example);
        let mut expected = LocationGraph::new();
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
