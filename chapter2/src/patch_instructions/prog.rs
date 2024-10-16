use super::PatchInstructions;
use crate::x86_int::Prog;

impl PatchInstructions for Prog {
    type Target = Prog;
    fn patch(self) -> Self::Target {
        let mut new_instrs = vec![];
        for instr in self.instrs.into_iter() {
            let instrs = instr.patch();
            new_instrs.extend(instrs)
        }
        Prog {
            instrs: new_instrs,
            stack_space: self.stack_space,
        }
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{PatchInstructions, Prog};
    use crate::x86_int::{Arg, Instr, Reg};
    #[test]
    fn patch_empty() {
        let result = Prog {
            instrs: vec![],
            stack_space: 0,
        }
        .patch();
        let expected = Prog {
            instrs: vec![],
            stack_space: 0,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn patch_prog() {
        let result = Prog {
            instrs: vec![
                Instr::AddQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -16)),
                Instr::CallQ("print_int".to_owned(), 1),
            ],
            stack_space: 16,
        }
        .patch();
        let expected = Prog {
            instrs: vec![
                Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
                Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
                Instr::CallQ("print_int".to_owned(), 1),
            ],
            stack_space: 16,
        };
        assert_eq!(result, expected)
    }
}
