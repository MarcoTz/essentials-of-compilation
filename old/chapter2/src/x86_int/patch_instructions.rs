use super::{Arg, Instr, Program, Reg};
use std::collections::{HashMap, HashSet};

pub trait PatchInstructions {
    type Target;
    fn patch(self) -> Self::Target;
}

impl PatchInstructions for Program {
    type Target = Program;
    fn patch(self) -> Self::Target {
        let mut new_blocks = HashMap::new();
        for (label, instrs) in self.blocks.into_iter() {
            let mut new_instrs = vec![];
            for instr in instrs {
                new_instrs.extend(instr.patch())
            }
            new_blocks.insert(label, new_instrs);
        }
        Program {
            blocks: new_blocks,
            stack_space: self.stack_space,
            global_labels: HashSet::new(),
        }
    }
}

impl PatchInstructions for Instr {
    type Target = Vec<Instr>;
    fn patch(self) -> Self::Target {
        match self {
            Instr::AddQ(Arg::Deref(reg1, offset1), Arg::Deref(reg2, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(reg1, offset1), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(reg2, offset2)),
                ]
            }
            Instr::SubQ(Arg::Deref(reg1, offset1), Arg::Deref(reg2, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(reg1, offset1), Arg::Reg(Reg::Rax)),
                    Instr::SubQ(Arg::Reg(Reg::Rax), Arg::Deref(reg2, offset2)),
                ]
            }
            Instr::MovQ(Arg::Deref(reg1, offset1), Arg::Deref(reg2, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(reg1, offset1), Arg::Reg(Reg::Rax)),
                    Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(reg2, offset2)),
                ]
            }
            _ => vec![self],
        }
    }
}
