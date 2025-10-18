use super::SelectInstructions;

impl SelectInstructions for core::Program {
    type Target = lang_x86::VarProgram;
    type Arg = ();
    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        let mut lang_x86_prog = lang_x86::VarProgram::new();
        for block in self.blocks {
            lang_x86_prog.add_block(&block.label, block.tail.select_instructions(()));
        }
        lang_x86_prog
    }
}
