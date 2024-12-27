pub mod instr;

use chapter2::x86_var::{Arg, Instr, Label, Program, Reg};
pub use instr::get_written;
use instr::l_before;
use std::collections::{HashMap, HashSet};

pub type LiveMap = HashMap<Label, Vec<HashSet<Arg>>>;

pub fn uncover_live(prog: &Program) -> LiveMap {
    let mut map = HashMap::new();
    for (label, instrs) in prog.blocks.iter() {
        let mut live_sets = vec![];
        let mut last_live = HashSet::new();
        for instr in instrs.iter().rev() {
            //only jump is to conclusion for now
            if let Instr::Jump(_) = instr {
                last_live = HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]);
            } else {
                last_live = l_before(instr, &last_live);
            }
            live_sets.insert(0, last_live.clone());
        }
        map.insert(label.clone(), live_sets);
    }

    let main_sets = map.get_mut(&"main".to_owned()).unwrap();
    for arg_set in main_sets.iter_mut() {
        arg_set.insert(Reg::Rsp.into());
    }

    map
}

#[cfg(test)]
mod uncover_tests {
    use super::uncover_live;
    use crate::test_examples::{example_prog1, example_prog2};
    use chapter2::x86_var::{Arg, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn uncover_example1() {
        let result = uncover_live(&example_prog1());
        let expected = HashMap::from([(
            "main".to_owned(),
            vec![
                HashSet::from([Reg::Rsp.into()]),
                HashSet::from([Arg::Var("a".to_owned()), Reg::Rsp.into()]),
                HashSet::from([Arg::Var("a".to_owned()), Reg::Rsp.into()]),
                HashSet::from([Arg::Var("c".to_owned()), Reg::Rsp.into()]),
                HashSet::from([
                    Arg::Var("c".to_owned()),
                    Arg::Var("b".to_owned()),
                    Reg::Rsp.into(),
                ]),
            ],
        )]);
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_example2() {
        let result = uncover_live(&example_prog2());
        let expected = HashMap::from([(
            "main".to_owned(),
            vec![
                HashSet::from([Reg::Rsp.into()]),
                HashSet::from([Arg::Var("v".to_owned()), Reg::Rsp.into()]),
                HashSet::from([
                    Arg::Var("v".to_owned()),
                    Arg::Var("w".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("w".to_owned()),
                    Arg::Var("x".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("w".to_owned()),
                    Arg::Var("x".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("w".to_owned()),
                    Arg::Var("x".to_owned()),
                    Arg::Var("y".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("w".to_owned()),
                    Arg::Var("z".to_owned()),
                    Arg::Var("y".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("z".to_owned()),
                    Arg::Var("y".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("z".to_owned()),
                    Arg::Var("t".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("z".to_owned()),
                    Arg::Var("t".to_owned()),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([
                    Arg::Var("t".to_owned()),
                    Arg::Reg(Reg::Rax),
                    Reg::Rsp.into(),
                ]),
                HashSet::from([Reg::Rax.into(), Reg::Rsp.into()]),
            ],
        )]);
        assert_eq!(result, expected)
    }
}
