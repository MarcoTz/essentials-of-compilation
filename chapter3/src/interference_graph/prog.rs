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

#[cfg(test)]
mod prog_tests {
    use super::{BuildGraph, InterferenceGraph, Program};
    use crate::uncover_live::UncoverLive;
    use chapter2::x86_var::{Arg, Instr, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn build_empty() {
        let mut graph = InterferenceGraph::default();
        Program {
            blocks: HashMap::new(),
            types: HashMap::new(),
        }
        .build(&mut graph, &HashMap::new());
        assert_eq!(graph.vertices, HashSet::new());
        assert_eq!(graph.edges, HashSet::new());
    }

    #[test]
    fn build_no_write() {
        let mut graph = InterferenceGraph::default();
        Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::AddQ(Arg::Immediate(1), Arg::Immediate(3)),
                    Instr::CallQ("print".to_owned(), 0),
                    Instr::Jump("main".to_owned()),
                ],
            )]),
            types: HashMap::new(),
        }
        .build(&mut graph, &HashMap::new());
        assert_eq!(graph.vertices, HashSet::new());
        assert_eq!(graph.edges, HashSet::new());
    }

    #[test]
    fn build_mov() {
        let mut graph = InterferenceGraph::default();
        let prog = Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Reg(Reg::Rbx)),
                    Instr::AddQ(Arg::Reg(Reg::Rbx), Arg::Immediate(2)),
                    Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("x".to_owned())),
                    Instr::MovQ(Arg::Immediate(2), Arg::Var("y".to_owned())),
                    Instr::MovQ(Arg::Var("x".to_owned()), Arg::Var("z".to_owned())),
                    Instr::AddQ(Arg::Var("x".to_owned()), Arg::Var("z".to_owned())),
                ],
            )]),
            types: HashMap::new(),
        };
        prog.build(&mut graph, &prog.uncover());
        println!("{:?}", graph.edges);
        assert_eq!(
            graph.vertices,
            HashSet::from(["x".to_owned(), "y".to_owned(),])
        );
        assert_eq!(
            graph.edges,
            HashSet::from([("y".to_owned(), "x".to_owned())])
        );
    }
}
