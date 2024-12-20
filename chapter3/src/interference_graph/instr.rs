use super::{BuildGraph, InterferenceGraph};
use crate::uncover_live::LiveMap;
use chapter2::x86_var::{Arg, Instr, Var};
use std::collections::HashSet;

impl BuildGraph for Instr {
    fn build(&self, graph: &mut InterferenceGraph, live: &LiveMap) {
        match self {
            Instr::MovQ(Arg::Var(v1), Arg::Var(v2)) => {
                let live_here: HashSet<Var> = todo!(); //= live.get(self).cloned().unwrap_or(HashSet::new());
                for var in live_here.into_iter() {
                    if var == *v1 || var == *v2 {
                        continue;
                    }
                    graph.add_edge(var, v2.clone());
                }
            }
            _ => {
                let live_vars: HashSet<Var> = todo!(); //= self.uncover();
                let after: HashSet<Var> = todo!(); //= live.get(self).cloned().unwrap_or(HashSet::new());
                for live_var in live_vars {
                    //live_vars.written {
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
