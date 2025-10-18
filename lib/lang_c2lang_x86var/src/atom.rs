use super::SelectInstructions;

impl SelectInstructions for core::Atom {
    type Target = lang_x86::VarArg;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> lang_x86::VarArg {
        match self {
            core::Atom::Integer(i) => lang_x86::Arg::Immediate(i).into(),
            core::Atom::Variable(v) => lang_x86::VarArg::Var(v),
            core::Atom::Bool(b) => {
                if b {
                    lang_x86::Arg::Immediate(1).into()
                } else {
                    lang_x86::Arg::Immediate(0).into()
                }
            }
            core::Atom::Unit => lang_x86::Arg::Immediate(0).into(),
        }
    }
}
