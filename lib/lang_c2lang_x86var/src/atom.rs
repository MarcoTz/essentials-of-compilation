use super::SelectInstructions;

impl SelectInstructions for lang_c::Atom {
    type Target = lang_x86::VarArg;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> lang_x86::VarArg {
        match self {
            lang_c::Atom::Integer(i) => lang_x86::Arg::Immediate(i).into(),
            lang_c::Atom::Variable(v) => lang_x86::VarArg::Var(v),
            lang_c::Atom::Bool(b) => {
                if b {
                    lang_x86::Arg::Immediate(1).into()
                } else {
                    lang_x86::Arg::Immediate(0).into()
                }
            }
            lang_c::Atom::Unit => lang_x86::Arg::Immediate(0).into(),
        }
    }
}
