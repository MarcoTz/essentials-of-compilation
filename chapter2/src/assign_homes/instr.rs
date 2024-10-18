use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_int::Instr, x86_var};

impl AssignHomes for Instr<x86_var::Arg> {
    type Target = Instr<x86_int::Arg>;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        match self {
            Instr::AddQ(a1, a2) => Instr::AddQ(a1.assign_homes(st), a2.assign_homes(st)),
            Instr::SubQ(a1, a2) => Instr::SubQ(a1.assign_homes(st), a2.assign_homes(st)),
            Instr::NegQ(a) => Instr::NegQ(a.assign_homes(st)),
            Instr::MovQ(a1, a2) => Instr::MovQ(a1.assign_homes(st), a2.assign_homes(st)),
            Instr::CallQ(l, offset) => Instr::CallQ(l, offset),
            Instr::PushQ(a) => Instr::PushQ(a.assign_homes(st)),
            Instr::PopQ(a) => Instr::PopQ(a.assign_homes(st)),
            Instr::RetQ => Instr::RetQ,
            Instr::Jump(l) => Instr::Jump(l),
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{x86_int, x86_var, AssignHomes, AssignState, Instr};
    use crate::x86_int::Reg;

    #[test]
    fn assign_no_var() {
        let result = Instr::AddQ(x86_var::Arg::Immediate(1), x86_var::Arg::Reg(Reg::Rdi))
            .assign_homes(&mut AssignState::default());
        let expected = Instr::AddQ(x86_int::Arg::Immediate(1), x86_int::Arg::Reg(Reg::Rdi));
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_var() {
        let result = Instr::AddQ(
            x86_var::Arg::Immediate(1),
            x86_var::Arg::Var("x".to_owned()),
        )
        .assign_homes(&mut AssignState::default());
        let expected = Instr::AddQ(
            x86_int::Arg::Immediate(1),
            x86_int::Arg::Deref(Reg::Rbp, -8),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_no_arg() {
        let result = Instr::Jump("exit".to_owned()).assign_homes(&mut AssignState::default());
        let expected = Instr::Jump("exit".to_owned());
        assert_eq!(result, expected)
    }
}
