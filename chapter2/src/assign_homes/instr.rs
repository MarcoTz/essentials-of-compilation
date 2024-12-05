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
