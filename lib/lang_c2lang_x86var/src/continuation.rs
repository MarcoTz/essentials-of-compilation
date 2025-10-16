use super::SelectInstructions;

impl SelectInstructions for lang_c::Continuation {
    type Target = Vec<lang_x86::Instruction<lang_x86::VarArg>>;
    type Arg = ();

    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        match self {
            lang_c::Continuation::Return(atm) => {
                let arg_dest = atm.select_instructions(());
                vec![
                    lang_x86::Instruction::MovQ {
                        src: arg_dest,
                        dest: lang_x86::Reg::Rax.into(),
                    },
                    lang_x86::Instruction::Jump {
                        label: "conclusion".to_owned(),
                    },
                ]
            }
            lang_c::Continuation::Goto(label) => vec![lang_x86::Instruction::Jump { label }],
            lang_c::Continuation::If {
                cond,
                then_label,
                else_label,
            } => {
                let cond_dest = cond.select_instructions(());
                let cmp = lang_x86::Instruction::CmpQ {
                    left: cond_dest,
                    right: 1.into(),
                };
                let jump_true = lang_x86::Instruction::JumpCC {
                    cc: lang_x86::Cc::E,
                    label: then_label,
                };
                let jump_false = lang_x86::Instruction::Jump { label: else_label };
                vec![cmp, jump_true, jump_false]
            }
        }
    }
}
