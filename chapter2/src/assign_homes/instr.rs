use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};

impl AssignHomes for x86_var::Instr {
    type Target = x86_int::Instr;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        match self {
            x86_var::Instr::AddQ(a1, a2) => {
                x86_int::Instr::AddQ(a1.assign_homes(st), a2.assign_homes(st))
            }
            x86_var::Instr::SubQ(a1, a2) => {
                x86_int::Instr::SubQ(a1.assign_homes(st), a2.assign_homes(st))
            }
            x86_var::Instr::NegQ(a) => x86_int::Instr::NegQ(a.assign_homes(st)),
            x86_var::Instr::MovQ(a1, a2) => {
                x86_int::Instr::MovQ(a1.assign_homes(st), a2.assign_homes(st))
            }
            x86_var::Instr::CallQ(l, i) => x86_int::Instr::CallQ(l, i),
            x86_var::Instr::PushQ(a) => x86_int::Instr::PushQ(a.assign_homes(st)),
            x86_var::Instr::PopQ(a) => x86_int::Instr::PopQ(a.assign_homes(st)),
            x86_var::Instr::RetQ => x86_int::Instr::RetQ,
            x86_var::Instr::Jump(l) => x86_int::Instr::Jump(l),
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{x86_int, x86_var, AssignHomes};

    #[test]
    fn assign_add() {
        let result = x86_var::Instr::AddQ(
            x86_var::Arg::Immediate(10),
            x86_var::Arg::Reg(x86_var::Reg::Rax),
        )
        .assign_homes(&mut Default::default());
        let expected = x86_int::Instr::AddQ(
            x86_int::Arg::Immediate(10),
            x86_int::Arg::Reg(x86_int::Reg::Rax),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_jump() {
        let result =
            x86_var::Instr::Jump("conclusion".to_owned()).assign_homes(&mut Default::default());
        let expected = x86_int::Instr::Jump("conclusion".to_owned());
        assert_eq!(result, expected)
    }
}
