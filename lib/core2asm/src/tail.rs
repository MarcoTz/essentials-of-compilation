use super::SelectInstructions;

impl SelectInstructions for core::Tail {
    type Target = Vec<asm::Instruction<asm::VarArg>>;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        let mut instrs = vec![];
        for stmt in self.stmts {
            instrs.extend(stmt.select_instructions(()));
        }
        instrs.extend(self.cont.select_instructions(()));
        instrs
    }
}
