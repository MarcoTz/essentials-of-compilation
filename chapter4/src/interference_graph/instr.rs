use super::{BuildGraph, InterferenceGraph};
use crate::uncover_live::{LiveMap, UncoverLive};
use chapter2::x86_var::{Arg, Instr};
use std::collections::HashSet;

impl BuildGraph for Instr<Arg> {
    fn build(&self, graph: &mut InterferenceGraph, live: &LiveMap) {
        match self {
            Instr::MovQ(Arg::Var(v1), Arg::Var(v2)) => {
                let live_here = live.get(self).cloned().unwrap_or(HashSet::new());
                for var in live_here.into_iter() {
                    if var == *v1 || var == *v2 {
                        continue;
                    }
                    graph.add_edge(var, v2.clone());
                }
            }
            _ => {
                let live_vars = self.uncover();
                let after = live.get(self).cloned().unwrap_or(HashSet::new());
                for live_var in live_vars.written {
                    for after_var in after.iter() {
                        if live_var == *after_var {
                            continue;
                        }
                        graph.add_edge(live_var.clone(), after_var.clone());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{Arg, BuildGraph, Instr, InterferenceGraph};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn build_mov() {
        let mut graph = InterferenceGraph::default();
        let mut live = HashMap::new();
        let instr = Instr::MovQ(Arg::Var("v1".to_owned()), Arg::Var("v2".to_owned()));
        live.insert(
            instr.clone(),
            HashSet::from(["v1".to_owned(), "x".to_owned()]),
        );

        instr.build(&mut graph, &live);
        assert_eq!(
            graph.vertices,
            HashSet::from(["v2".to_owned(), "x".to_owned()])
        );
        assert_eq!(
            graph.edges,
            HashSet::from([("x".to_owned(), "v2".to_owned())])
        );
    }

    #[test]
    fn build_other() {
        let mut graph = InterferenceGraph::default();
        let instr = Instr::PopQ(Arg::Var("v1".to_owned()));
        let mut live = HashMap::new();
        live.insert(
            instr.clone(),
            HashSet::from(["v1".to_owned(), "x".to_owned()]),
        );
        instr.build(&mut graph, &live);
        assert_eq!(
            graph.vertices,
            HashSet::from(["v1".to_owned(), "x".to_owned()])
        );
        assert_eq!(
            graph.edges,
            HashSet::from([("v1".to_owned(), "x".to_owned())])
        );
    }
}
