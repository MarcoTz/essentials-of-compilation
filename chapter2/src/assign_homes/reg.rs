use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};

impl AssignHomes for x86_var::Reg {
    type Target = x86_int::Reg;
    fn assign_homes(self, _: &mut AssignState) -> Self::Target {
        match self {
            x86_var::Reg::Rsp => x86_int::Reg::Rsp,
            x86_var::Reg::Rbp => x86_int::Reg::Rbp,
            x86_var::Reg::Rax => x86_int::Reg::Rax,
            x86_var::Reg::Rbx => x86_int::Reg::Rbx,
            x86_var::Reg::Rcx => x86_int::Reg::Rcx,
            x86_var::Reg::Rdx => x86_int::Reg::Rdx,
            x86_var::Reg::Rsi => x86_int::Reg::Rsi,
            x86_var::Reg::Rdi => x86_int::Reg::Rdi,
            x86_var::Reg::R8 => x86_int::Reg::R8,
            x86_var::Reg::R9 => x86_int::Reg::R9,
            x86_var::Reg::R10 => x86_int::Reg::R10,
            x86_var::Reg::R11 => x86_int::Reg::R11,
            x86_var::Reg::R12 => x86_int::Reg::R12,
            x86_var::Reg::R13 => x86_int::Reg::R13,
            x86_var::Reg::R14 => x86_int::Reg::R14,
            x86_var::Reg::R15 => x86_int::Reg::R15,
        }
    }
}

#[cfg(test)]
mod reg_tests {
    use super::{x86_int, x86_var, AssignHomes};

    #[test]
    fn assign_rdx() {
        let result = x86_var::Reg::Rdx.assign_homes(&mut Default::default());
        let expected = x86_int::Reg::Rdx;
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_r15() {
        let result = x86_var::Reg::R15.assign_homes(&mut Default::default());
        let expected = x86_int::Reg::R15;
        assert_eq!(result, expected)
    }
}
