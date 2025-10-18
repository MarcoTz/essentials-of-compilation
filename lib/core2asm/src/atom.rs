use super::SelectInstructions;

impl SelectInstructions for core::Atom {
    type Target = asm::VarArg;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> asm::VarArg {
        match self {
            core::Atom::Integer(i) => asm::Arg::Immediate(i).into(),
            core::Atom::Variable(v) => asm::VarArg::Var(v),
            core::Atom::Bool(b) => {
                if b {
                    asm::Arg::Immediate(1).into()
                } else {
                    asm::Arg::Immediate(0).into()
                }
            }
            core::Atom::Unit => asm::Arg::Immediate(0).into(),
        }
    }
}
