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
