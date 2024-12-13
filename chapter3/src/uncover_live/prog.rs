use super::{LiveMap, UncoverLive};
use chapter2::x86_var::{Instr, Program};
use std::collections::{HashMap, HashSet};

impl UncoverLive for Program {
    type Target = LiveMap;
    fn uncover(&self) -> Self::Target {
        let mut live = HashMap::new();
        let mut last = HashSet::new();

        let mut instrs_rev: Vec<Instr> = todo!(); //self.instrs.clone();
        instrs_rev.reverse();
        for instr in instrs_rev {
            let vars = instr.uncover();
            for written in vars.written.iter() {
                last.remove(written);
            }
            last = last.union(&vars.read).cloned().collect();
            live.insert(instr, last.clone());
        }
        live
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Program, UncoverLive};
    use chapter2::x86_var::{Arg, Instr};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn live_sum() {
        let result = Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(5), Arg::Var("a".to_owned())),
                    Instr::MovQ(Arg::Immediate(30), Arg::Var("b".to_owned())),
                    Instr::MovQ(Arg::Var("a".to_owned()), Arg::Var("c".to_owned())),
                    Instr::MovQ(Arg::Immediate(10), Arg::Var("b".to_owned())),
                    Instr::AddQ(Arg::Var("b".to_owned()), Arg::Var("c".to_owned())),
                ],
            )]),
            types: HashMap::new(),
        }
        .uncover();
        let mut expected = HashMap::new();
        expected.insert(
            Instr::MovQ(Arg::Immediate(5), Arg::Var("a".to_owned())),
            HashSet::new(),
        );
        expected.insert(
            Instr::MovQ(Arg::Immediate(30), Arg::Var("b".to_owned())),
            HashSet::from(["a".to_owned()]),
        );
        expected.insert(
            Instr::MovQ(Arg::Var("a".to_owned()), Arg::Var("c".to_owned())),
            HashSet::from(["a".to_owned()]),
        );
        expected.insert(
            Instr::MovQ(Arg::Immediate(10), Arg::Var("b".to_owned())),
            HashSet::from(["c".to_owned()]),
        );
        expected.insert(
            Instr::AddQ(Arg::Var("b".to_owned()), Arg::Var("c".to_owned())),
            HashSet::from(["b".to_owned(), "c".to_owned()]),
        );
        assert_eq!(result, expected)
    }
}
