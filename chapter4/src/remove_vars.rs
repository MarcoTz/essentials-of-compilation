use super::{color_graph::RegisterAssignment, errors::Error};
use chapter2::x86_var::{Arg, Instr, Prog};

pub trait RemoveVars: Sized {
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self, Error>;
}

impl RemoveVars for Prog {
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self, Error> {
        let mut new_instrs = vec![];
        for instr in self.instrs {
            let new_instr = instr.remove_vars(assignment)?;
            new_instrs.push(new_instr);
        }
        Ok(Prog {
            instrs: new_instrs,
            labels: self.labels,
        })
    }
}

impl RemoveVars for Instr<Arg> {
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self, Error> {
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
            _ => Ok(self),
        }
    }
}

impl RemoveVars for Arg {
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self, Error> {
        match self {
            Arg::Var(v) => match assignment.get(&v) {
                None => Err(Error::VariableNotFound(v)),
                Some(reg) => Ok(Arg::Reg(reg.clone())),
            },
            _ => Ok(self),
        }
    }
}
