pub mod instr;

use chapter2::x86_var::{Instr, Label, Program, Var};
use instr::l_before;
use std::collections::{HashMap, HashSet};

pub type LiveMap = HashMap<Label, Vec<HashSet<Var>>>;

pub fn uncover_live(prog: &Program) -> LiveMap {
    let mut map = HashMap::new();
    let mut jumps = HashMap::new();
    for (label, instrs) in prog.blocks.iter() {
        let mut live_sets = vec![];
        let mut last_live = HashSet::new();
        for instr in instrs.iter().rev() {
            if let Instr::Jump(l) = instr {
                jumps.insert(
                    l.clone(),
                    (label.clone(), instrs.len() - live_sets.len() - 1),
                );
            }
            last_live = l_before(instr, &last_live);
            live_sets.insert(0, last_live.clone());
        }
        map.insert(label.clone(), live_sets);
    }

    for (target_label, (block_label, block_index)) in jumps.into_iter() {
        let target_set = get_target_set(&map, &target_label).unwrap_or(HashSet::new());
        let label_sets = map.get_mut(&block_label).unwrap();
        label_sets[block_index] = target_set;
    }
    map
}

fn get_target_set(map: &LiveMap, target_label: &Label) -> Option<HashSet<Var>> {
    let target_sets = map.get(target_label)?;
    target_sets.first().cloned()
}

#[cfg(test)]
mod uncover_tests {
    use super::{uncover_live, Instr, Program};
    use chapter2::x86_var::{Arg, Reg};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn uncover_example1() {
        let result = uncover_live(&Program {
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
        });
        let expected = HashMap::from([(
            "main".to_owned(),
            vec![
                HashSet::new(),
                HashSet::from(["a".to_owned()]),
                HashSet::from(["a".to_owned()]),
                HashSet::from(["c".to_owned()]),
                HashSet::from(["c".to_owned(), "b".to_owned()]),
            ],
        )]);
        assert_eq!(result, expected)
    }

    #[test]
    fn uncover_example2() {
        let result = uncover_live(&Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Var("v".to_owned())),
                    Instr::MovQ(Arg::Immediate(42), Arg::Var("w".to_owned())),
                    Instr::MovQ(Arg::Var("v".to_owned()), Arg::Var("x".to_owned())),
                    Instr::AddQ(Arg::Immediate(7), Arg::Var("x".to_owned())),
                    Instr::MovQ(Arg::Var("x".to_owned()), Arg::Var("y".to_owned())),
                    Instr::MovQ(Arg::Var("x".to_owned()), Arg::Var("z".to_owned())),
                    Instr::AddQ(Arg::Var("w".to_owned()), Arg::Var("z".to_owned())),
                    Instr::MovQ(Arg::Var("y".to_owned()), Arg::Var("t".to_owned())),
                    Instr::NegQ(Arg::Var("t".to_owned())),
                    Instr::MovQ(Arg::Var("z".to_owned()), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Var("t".to_owned()), Arg::Reg(Reg::Rax)),
                    Instr::Jump("conclusion".to_owned()),
                ],
            )]),
            types: HashMap::new(),
        });
        let expected = HashMap::from([(
            "main".to_owned(),
            vec![
                HashSet::new(),
                HashSet::from(["v".to_owned()]),
                HashSet::from(["v".to_owned(), "w".to_owned()]),
                HashSet::from(["w".to_owned(), "x".to_owned()]),
                HashSet::from(["w".to_owned(), "x".to_owned()]),
                HashSet::from(["w".to_owned(), "x".to_owned(), "y".to_owned()]),
                HashSet::from(["w".to_owned(), "z".to_owned(), "y".to_owned()]),
                HashSet::from(["z".to_owned(), "y".to_owned()]),
                HashSet::from(["z".to_owned(), "t".to_owned()]),
                HashSet::from(["z".to_owned(), "t".to_owned()]),
                HashSet::from(["t".to_owned()]),
                HashSet::new(),
            ],
        )]);
        assert_eq!(result, expected)
    }
}
