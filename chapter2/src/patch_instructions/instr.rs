use super::PatchInstructions;
use crate::x86_int::{Arg, Instr};

impl PatchInstructions for Instr<Arg> {
    type Target = Vec<Instr<Arg>>;
    fn patch(self) -> Self::Target {
        let max_immediate = 2_i64.pow(16);
        if self.double_deref() {
            self.remove_double_deref()
        } else {
            self.remove_max_immediate(max_immediate)
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{Arg, Instr, PatchInstructions};
    use crate::x86_int::Reg;

    #[test]
    fn add_reg_reg() {
        let result = Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)).patch();
        let expected = vec![Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))];
        assert_eq!(result, expected)
    }

    #[test]
    fn add_stack_stack() {
        let result = Instr::AddQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -16)).patch();
        let expected = vec![
            Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn add_stack_immediate() {
        let result =
            Instr::AddQ(Arg::Immediate((2 as i64).pow(18)), Arg::Deref(Reg::Rbp, -8)).patch();
        let expected = vec![
            Instr::MovQ(Arg::Immediate((2 as i64).pow(18)), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -8)),
        ];
        assert_eq!(result, expected)
    }
}
