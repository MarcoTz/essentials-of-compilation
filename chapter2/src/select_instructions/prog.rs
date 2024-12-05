use super::SelectInstructions;
use crate::{c_var, x86_var};

impl SelectInstructions for c_var::Program {
    type Target = x86_var::Program;
    fn select_instructions(self) -> Self::Target {
        x86_var::Program {
            blocks: self
                .blocks
                .into_iter()
                .map(|(label, tl)| (label, tl.select_instructions()))
                .collect(),
        }
    }
}
