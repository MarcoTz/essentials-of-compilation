use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_int::Reg, x86_var};

impl AssignHomes for x86_var::Arg {
    type Target = x86_int::Arg;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        match self {
            x86_var::Arg::Intermediate(i) => x86_int::Arg::Intermediate(i),
            x86_var::Arg::Reg(reg) => x86_int::Arg::Reg(reg),
            x86_var::Arg::Deref(reg, offset) => x86_int::Arg::Deref(reg, offset),
            x86_var::Arg::Var(v) => match st.stack_vars.get(&v) {
                None => {
                    st.stack_size += 8;
                    let offset = -(st.stack_size as i64);
                    x86_int::Arg::Deref(Reg::Rbp, offset)
                }
                Some(offset) => x86_int::Arg::Deref(Reg::Rbp, offset.to_owned()),
            },
        }
    }
}

#[cfg(test)]
mod arg_tests {
    use super::{x86_int, x86_var, AssignHomes, AssignState, Reg};

    #[test]
    fn assign_int() {
        let result = x86_var::Arg::Intermediate(1).assign_homes(&mut AssignState::default());
        let expected = x86_int::Arg::Intermediate(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_reg() {
        let result = x86_var::Arg::Reg(Reg::Rdx).assign_homes(&mut AssignState::default());
        let expected = x86_int::Arg::Reg(Reg::Rdx);
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_deref() {
        let result = x86_var::Arg::Deref(Reg::Rax, 0).assign_homes(&mut AssignState::default());
        let expected = x86_int::Arg::Deref(Reg::Rax, 0);
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_var() {
        let result = x86_var::Arg::Var("x".to_owned()).assign_homes(&mut AssignState::default());
        let expected = x86_int::Arg::Deref(Reg::Rbp, -8);
        assert_eq!(result, expected)
    }
}
