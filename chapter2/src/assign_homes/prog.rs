use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};

impl AssignHomes for x86_var::Prog {
    type Target = x86_int::Prog;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        let mut new_instrs = vec![];
        for instr in self.instrs.into_iter() {
            let new_instr = instr.assign_homes(st);
            new_instrs.push(new_instr)
        }
        x86_int::Prog {
            stack_space: st.stack_size,
            instrs: new_instrs,
            labels: self.labels,
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{
        x86_int,
        x86_int::{Instr, Prog, Reg},
        x86_var, AssignHomes, AssignState,
    };
    use std::collections::HashMap;

    #[test]
    fn assign_empty() {
        let result = x86_var::Prog {
            instrs: vec![],
            labels: HashMap::new(),
        }
        .assign_homes(&mut AssignState::default());
        let expected = Prog {
            stack_space: 0,
            instrs: vec![],
            labels: HashMap::new(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_prog() {
        let result = x86_var::Prog {
            instrs: vec![
                Instr::CallQ("read_int".to_owned(), 0),
                Instr::MovQ(
                    x86_var::Arg::Reg(Reg::Rax),
                    x86_var::Arg::Var("x".to_owned()),
                ),
            ],
            labels: HashMap::new(),
        }
        .assign_homes(&mut AssignState::default());
        let expected = Prog {
            stack_space: 8,
            labels: HashMap::new(),
            instrs: vec![
                Instr::CallQ("read_int".to_owned(), 0),
                Instr::MovQ(
                    x86_int::Arg::Reg(Reg::Rax),
                    x86_int::Arg::Deref(Reg::Rbp, -8),
                ),
            ],
        };
        assert_eq!(result, expected)
    }
}
