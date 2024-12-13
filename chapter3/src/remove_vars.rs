use super::{color_graph::RegisterAssignment, errors::Error};
use crate::{
    x86_int::{get_stack_space, Arg as IntArg, Prog as IntProg},
    x86_var::{Arg as VarArg, Instr, Prog as VarProg},
};
use std::collections::HashSet;

pub trait RemoveVars: Sized {
    type Target;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error>;
}

impl RemoveVars for VarProg {
    type Target = IntProg;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error> {
        let mut new_instrs = vec![];
        for instr in self.instrs {
            let new_instr = instr.remove_vars(assignment)?;
            new_instrs.push(new_instr);
        }
        let stack_space = get_stack_space(&new_instrs);
        Ok(IntProg {
            instrs: new_instrs,
            labels: self.labels,
            stack_space,
            used_callee: HashSet::new(),
        })
    }
}

impl RemoveVars for Instr<VarArg> {
    type Target = Instr<IntArg>;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error> {
        match self {
            Instr::AddQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(Instr::AddQ(a1_new, a2_new))
            }
            Instr::SubQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(Instr::SubQ(a1_new, a2_new))
            }
            Instr::NegQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(Instr::NegQ(a_new))
            }
            Instr::MovQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(Instr::MovQ(a1_new, a2_new))
            }
            Instr::PushQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(Instr::PushQ(a_new))
            }
            Instr::PopQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(Instr::PopQ(a_new))
            }
            _ => Ok(self.try_into().unwrap()),
        }
    }
}

impl RemoveVars for VarArg {
    type Target = IntArg;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error> {
        match self {
            VarArg::Var(v) => match assignment.get(&v) {
                None => Err(Error::VariableNotFound(v)),
                Some(reg) => Ok(IntArg::Reg(reg.clone())),
            },
            _ => Ok(self.try_into().unwrap()),
        }
    }
}
