use super::{color_graph::RegisterAssignment, errors::Error};
use chapter2::{
    x86_int::{Arg as IntArg, Instr as IntInstr, Program as IntProg},
    x86_var::{Arg as VarArg, Instr as VarInstr, Program as VarProg},
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
        let instrs: Vec<VarInstr> = todo!();
        for instr in instrs {
            let new_instr = instr.remove_vars(assignment)?;
            new_instrs.push(new_instr);
        }
        let stack_space: usize = todo!(); //get_stack_space(&new_instrs);
                                          /*
                                          Ok(IntProg {
                                              instrs: new_instrs,
                                              labels: self.labels,
                                              stack_space,
                                              used_callee: HashSet::new(),
                                          })*/
        todo!()
    }
}

impl RemoveVars for VarInstr {
    type Target = IntInstr;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error> {
        match self {
            VarInstr::AddQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(IntInstr::AddQ(a1_new, a2_new))
            }
            VarInstr::SubQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(IntInstr::SubQ(a1_new, a2_new))
            }
            VarInstr::NegQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(IntInstr::NegQ(a_new))
            }
            VarInstr::MovQ(a1, a2) => {
                let a1_new = a1.remove_vars(assignment)?;
                let a2_new = a2.remove_vars(assignment)?;
                Ok(IntInstr::MovQ(a1_new, a2_new))
            }
            VarInstr::PushQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(IntInstr::PushQ(a_new))
            }
            VarInstr::PopQ(a) => {
                let a_new = a.remove_vars(assignment)?;
                Ok(IntInstr::PopQ(a_new))
            }
            _ => todo!(), //Ok(self.try_into().unwrap()),
        }
    }
}

impl RemoveVars for VarArg {
    type Target = IntArg;
    fn remove_vars(self, assignment: &RegisterAssignment) -> Result<Self::Target, Error> {
        match self {
            VarArg::Var(v) => match assignment.get(&v) {
                None => Err(Error::VariableNotFound(v)),
                Some(reg) => todo!(), //Ok(IntArg::Reg(reg.clone())),
            },
            _ => todo!(), //Ok(self.try_into().unwrap()),
        }
    }
}
