use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};

impl AssignHomes for x86_var::Arg {
    type Target = x86_int::Arg;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        match self {
            x86_var::Arg::Immediate(i) => x86_int::Arg::Immediate(i),
            x86_var::Arg::Reg(reg) => x86_int::Arg::Reg(reg.assign_homes(st)),
            x86_var::Arg::Deref(reg, offset) => x86_int::Arg::Deref(reg.assign_homes(st), offset),
            x86_var::Arg::Var(v) => match st.stack_vars.get(&v) {
                Some(i) => x86_int::Arg::Deref(x86_int::Reg::Rbp, *i),
                None => {
                    st.stack_size += 8;
                    let i = -(st.stack_size as i64);
                    st.stack_vars.insert(v.clone(), i);
                    x86_int::Arg::Deref(x86_int::Reg::Rbp, i)
                }
            },
        }
    }
}

#[cfg(test)]
mod arg_tests {
    use super::{x86_int, x86_var, AssignHomes, AssignState};

    #[test]
    fn assign_immediate() {
        let result = x86_var::Arg::Immediate(10).assign_homes(&mut Default::default());
        let expected = x86_int::Arg::Immediate(10);
        assert_eq!(result, expected)
    }

    #[test]
    fn assign_var_none() {
        let mut st = AssignState::default();
        let result = x86_var::Arg::Var("x".to_owned()).assign_homes(&mut st);
        let expected = x86_int::Arg::Deref(x86_int::Reg::Rbp, -8);
        let mut new_st = AssignState::default();
        new_st.stack_size = 8;
        new_st.stack_vars.insert("x".to_owned(), -8);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn assign_var_some() {
        let mut st = AssignState::default();
        st.stack_vars.insert("x".to_owned(), 15);
        let result = x86_var::Arg::Var("x".to_owned()).assign_homes(&mut st);
        let expected = x86_int::Arg::Deref(x86_int::Reg::Rbp, 15);
        let mut new_st = AssignState::default();
        new_st.stack_vars.insert("x".to_owned(), 15);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
