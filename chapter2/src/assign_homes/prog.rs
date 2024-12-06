use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};
use std::collections::{HashMap, HashSet};

impl AssignHomes for x86_var::Program {
    type Target = x86_int::Program;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        let mut new_blocks = HashMap::new();
        for (label, instrs) in self.blocks.into_iter() {
            let new_instrs = instrs
                .into_iter()
                .map(|instr| instr.assign_homes(st))
                .collect();
            new_blocks.insert(label, new_instrs);
        }
        x86_int::Program {
            blocks: new_blocks,
            stack_space: st.stack_size,
            global_labels: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{x86_int, x86_var, AssignHomes, AssignState};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn assign_no_stac() {
        let result = x86_var::Program {
            blocks: HashMap::from([("main".to_owned(), vec![x86_var::Instr::RetQ])]),
            types: HashMap::new(),
        }
        .assign_homes(&mut Default::default());
        let expected = x86_int::Program {
            blocks: HashMap::from([("main".to_owned(), vec![x86_int::Instr::RetQ])]),
            stack_space: 0,
            global_labels: HashSet::new(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_stack() {
        let mut state = AssignState::default();
        let result = x86_var::Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    x86_var::Instr::MovQ(
                        x86_var::Arg::Var("x".to_owned()),
                        x86_var::Arg::Immediate(10),
                    ),
                    x86_var::Instr::MovQ(
                        x86_var::Arg::Var("y".to_owned()),
                        x86_var::Arg::Immediate(5),
                    ),
                    x86_var::Instr::SubQ(
                        x86_var::Arg::Var("x".to_owned()),
                        x86_var::Arg::Var("y".to_owned()),
                    ),
                ],
            )]),
            types: HashMap::from([
                ("x".to_owned(), x86_var::Type::Int),
                ("y".to_owned(), x86_var::Type::Int),
            ]),
        }
        .assign_homes(&mut state);
        let expected = x86_int::Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    x86_int::Instr::MovQ(
                        x86_int::Arg::Deref(x86_int::Reg::Rbp, -8),
                        x86_int::Arg::Immediate(10),
                    ),
                    x86_int::Instr::MovQ(
                        x86_int::Arg::Deref(x86_int::Reg::Rbp, -16),
                        x86_int::Arg::Immediate(5),
                    ),
                    x86_int::Instr::SubQ(
                        x86_int::Arg::Deref(x86_int::Reg::Rbp, -8),
                        x86_int::Arg::Deref(x86_int::Reg::Rbp, -16),
                    ),
                ],
            )]),
            stack_space: 16,
            global_labels: HashSet::new(),
        };
        assert_eq!(result, expected)
    }
}
