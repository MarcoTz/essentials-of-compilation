use super::uncover_live::{get_written, uncover_live};
use chapter2::x86_var::{Arg, Instr, Program};
use std::collections::HashSet;

pub mod graph;

pub use graph::InterferenceGraph;

pub fn build_graph(prog: &Program) -> InterferenceGraph {
    let mut graph = InterferenceGraph::default();
    let live = uncover_live(prog);
    for (label, instrs) in prog.blocks.iter() {
        let mut block_sets = live.get(label).unwrap().clone();
        block_sets.push(HashSet::new());
        let mut block_iter = block_sets.iter();
        block_iter.next();

        for (instr, instr_set) in instrs.iter().zip(block_iter) {
            if let Instr::MovQ(a1, a2) = instr {
                build_mov(a1, a2, instr_set, &mut graph);
            } else {
                build_instr(instr, instr_set, &mut graph);
            }
        }
    }
    graph
}

fn build_instr(instr: &Instr, live: &HashSet<Arg>, graph: &mut InterferenceGraph) {
    let written = get_written(instr);
    for arg1 in live.iter() {
        for arg2 in written.iter() {
            graph.add_edge(arg1.clone(), arg2.clone());
        }
    }
}

fn build_mov(src: &Arg, dest: &Arg, live: &HashSet<Arg>, graph: &mut InterferenceGraph) {
    for arg in live.iter() {
        if src == arg || src == dest {
            continue;
        }
        graph.add_edge(dest.clone(), arg.clone());
    }
}

#[cfg(test)]
mod graph_tests {
    use super::{build_graph, InterferenceGraph};
    use crate::test_examples::example_prog2;
    use chapter2::x86_var::Reg;

    #[test]
    fn build_example() {
        let result = build_graph(&example_prog2());
        let mut expected = InterferenceGraph::default();
        expected.add_edge(Reg::Rax.into(), "t".into());
        expected.add_edge(Reg::Rax.into(), Reg::Rsp.into());
        expected.add_edge("t".into(), "z".into());
        expected.add_edge("t".into(), Reg::Rsp.into());
        expected.add_edge("z".into(), "y".into());
        expected.add_edge("z".into(), "w".into());
        expected.add_edge("z".into(), Reg::Rsp.into());
        expected.add_edge("y".into(), "w".into());
        expected.add_edge("y".into(), Reg::Rsp.into());
        expected.add_edge("w".into(), "x".into());
        expected.add_edge("w".into(), Reg::Rsp.into());
        expected.add_edge("w".into(), "v".into());
        expected.add_edge("x".into(), Reg::Rsp.into());
        expected.add_edge("v".into(), Reg::Rsp.into());
        expected.add_edge(Reg::Rax.into(), Reg::Rsp.into());
        assert_eq!(result, expected)
    }
}
