use super::SelectInstructions;

impl SelectInstructions for lang_c::Tail {
    type Target = Vec<lang_x86::Instruction<lang_x86::VarArg>>;
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
