use super::{BuildGraph, InterferenceGraph};
use crate::uncover_live::LiveMap;
use chapter2::x86_var::{Instr, Program};

impl BuildGraph for Program {
    fn build(&self, graph: &mut InterferenceGraph, live: &LiveMap) {
        let instrs: Vec<Instr> = todo!();
        for instr in instrs {
            instr.build(graph, live)
        }
    }
}
