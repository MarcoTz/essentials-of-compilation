use super::{AssignHomes, AssignState};
use crate::{x86_int, x86_var};
use std::collections::HashMap;

impl AssignHomes for x86_var::Program {
    type Target = x86_int::Program;
    fn assign_homes(self, st: &mut AssignState) -> Self::Target {
        let mut new_blocks = HashMap::new();
        for (label, instrs) in self.blocks.into_iter() {
            let new_instrs = instrs
                .into_iter()
                .map(|instr| instr.assign_homes(st))
                .collect();
            new_blocks.insert(label, new_instrs);
        }
        x86_int::Program {
            blocks: new_blocks,
            stack_space: st.stack_size,
        }
    }
}
