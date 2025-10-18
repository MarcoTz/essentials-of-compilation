use super::SelectInstructions;

impl SelectInstructions for core::Program {
    type Target = asm::VarProgram;
    type Arg = ();
    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        let mut asm_prog = asm::VarProgram::new();
        for block in self.blocks {
            asm_prog.add_block(&block.label, block.tail.select_instructions(()));
        }
        asm_prog
    }
}
