use super::PatchInstructions;
use chapter2::x86_int::{Instr, Label, Program};

impl PatchInstructions for Program {
    type Target = Program;
    fn patch(self) -> Self::Target {
        let mut new_instrs = vec![];
        let mut new_labels: Vec<(Label, usize)> = todo!(); //self.labels;
        let instrs: Vec<Instr> = todo!();
        for instr in instrs.into_iter() {
            //self.instrs.into_iter() {
            let instrs = instr.patch();
            if instrs.len() > 1 {
                new_labels = new_labels
                    .into_iter()
                    .map(|(lb, line)| (lb, line + 1))
                    .collect();
            }
            new_instrs.extend(instrs)
        }
        todo!()
        /*Program {
            labels: new_labels,
            instrs: new_instrs,
            stack_space: self.stack_space,
            used_callee: self.used_callee,
        }*/
    }
}
