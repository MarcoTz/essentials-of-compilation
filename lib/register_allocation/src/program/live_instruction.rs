use super::Location;
use lang_x86::{Instruction, VarArg};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiveInstruction {
    pub instr: Instruction<VarArg>,
    pub live_before: HashSet<Location>,
    pub live_after: HashSet<Location>,
}

impl LiveInstruction {
    pub fn new(
        instr: Instruction<VarArg>,
        live_before: HashSet<Location>,
        live_after: HashSet<Location>,
    ) -> LiveInstruction {
        LiveInstruction {
            instr,
            live_before,
            live_after,
        }
    }
}

impl fmt::Display for LiveInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}; L_before = {:?}; L_after = {:?}",
            self.instr, self.live_before, self.live_after
        )
    }
}
